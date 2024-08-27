use std::ops::Deref;
use std::rc::Rc;

use crate::grammer::expression::*;
use crate::runner::error::{CommonError, RunTimeError};
use crate::scanner::token::{Token, TokenType};
use crate::value::class::ClassInstance;
use crate::value::condition::IsTruthy;
use crate::value::function::Callable;
use crate::value::LValue;
use crate::interpreter::Interpreter;

impl Interpreter {
    pub fn interpret_expr_debug(&mut self, expr: &Expr) -> Result<LValue, RunTimeError> {
        self.evaluate(expr)
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<LValue, RunTimeError>{
        expr.accept(self)
    }

    fn result_expr_helper(operation_result: Result<LValue, String>, token: &Token) -> Result<LValue, RunTimeError> {
        match operation_result {
            Ok(l_value) => Ok(l_value),
            Err(message) => Err(RunTimeError::Error(
                CommonError {
                    message: message,
                    token: Some(token.clone()),
                }
            ))
        }
    }

    fn lookup_variable(&mut self, name: &Token) -> Result<LValue, RunTimeError> {
        let distance_option: Option<&usize> = self.locals.get(&name);

        if let Some(distance) = distance_option {
            return self.environment.borrow().get_at(*distance, name);
        } else {
            return self.globals.borrow().get(name)
        }
    }
}

impl ExprVistor<Result<LValue, RunTimeError>> for Interpreter {
    fn visit(&mut self, root_expr: &Expr) -> Result<LValue, RunTimeError>{
        match root_expr {
            Expr::Literal(literal) => {
                match literal {
                    ExprLiteral::Nil => Ok(LValue::Nil),
                    ExprLiteral::False => Ok(LValue::Bool(false)),
                    ExprLiteral::True => Ok(LValue::Bool(true)),
                    ExprLiteral::Number(n) => Ok(LValue::Number(*n)),
                    ExprLiteral::String(s) => Ok(LValue::String(s.clone())),
                }
            },
            Expr::Grouping(expr) => {
                self.evaluate(expr)
            },
            Expr::Unary(token, expr) => {
                let right = self.evaluate(expr)?;

                match token.typee {
                    TokenType::Minus => Interpreter::result_expr_helper(-right, token),
                    TokenType::Bang => Interpreter::result_expr_helper(!right, token),
                    _ => Err(RunTimeError::Error(
                        CommonError {
                            token: Some(token.clone()),
                            message: String::from("Wrong token type evaluating for unary expression"),
                        }
                    )),
                }
            },
            Expr::Binary(l, token, r) => {
                let left = self.evaluate(l)?;
                let right = self.evaluate(r)?;

                match token.typee {
                    TokenType::Minus => Interpreter::result_expr_helper(left - right, token),
                    TokenType::Plus => Interpreter::result_expr_helper(left + right, token),
                    TokenType::Slash => Interpreter::result_expr_helper(left / right, token),
                    TokenType::Star => Interpreter::result_expr_helper(left * right, token),
                    TokenType::Greater => Ok(LValue::Bool(left > right)),
                    TokenType::GreaterEqual => Ok(LValue::Bool(left >= right)),
                    TokenType::Less => Ok(LValue::Bool(left < right)),
                    TokenType::LessEqual => Ok(LValue::Bool(left <= right)),
                    TokenType::BangEqual => Ok(LValue::Bool(left != right)),
                    TokenType::EqualEqual => Ok(LValue::Bool(left == right)),
                    _ => Err(RunTimeError::Error(CommonError {
                        token: Some(token.clone()),
                        message: String::from("Wrong token type evaluating for binary expression"),
                    })),
                }
            },
            Expr::Variable(token) => {
                match token.typee {
                    TokenType::Identifier => {
                        self.lookup_variable(token)
                    },
                    _ => Err(RunTimeError::Error(CommonError {
                        token: Some(token.clone()),
                        message: String::from("Wrong token type evaluating for variable expression"),
                    })),
                }
            },
            Expr::Assign(token, expr) => {
                match token.typee {
                    TokenType::Identifier => {
                        let value = self.evaluate(expr)?;
                        let distance_option = self.locals.get(token);
                        
                        if let Some(distance) = distance_option {
                            self.environment.borrow_mut().assign_at(*distance, token, value.clone())?;
                            return Ok(value);
                        } else {
                            let assgin_res = self.globals.borrow_mut().assign(token, value.clone());
                            match assgin_res {
                                Ok(_) => return Ok(value),
                                Err(err) => return Err(err),
                            }
                        }
                    },
                    _ => Err(RunTimeError::Error(CommonError {
                        token: Some(token.clone()),
                        message: String::from("Wrong token type evaluating for assignment expression"),
                    })),
                }
            },
            Expr::Logical(left, token, right) => {
                let left_val = self.evaluate(left)?;

                match token.typee {
                    TokenType::Or => {
                        if left_val.is_truthy() {
                            return Ok(LValue::Bool(true));
                        }
                        
                        let right_val = self.evaluate(right)?;
                        
                        if right_val.is_truthy() {
                            return Ok(LValue::Bool(true));
                        } else {
                            return Ok(LValue::Bool(false));
                        }
                    },
                    TokenType::And => {
                        if !left_val.is_truthy() {
                            return Ok(LValue::Bool(false));
                        }

                        let right_val = self.evaluate(right)?;

                        if right_val.is_truthy() {
                            return Ok(LValue::Bool(true));
                        } else {
                            return Ok(LValue::Bool(false));
                        }
                    },
                    _ => Err(RunTimeError::Error(CommonError {
                        token: Some(token.clone()),
                        message: String::from("Wrong token type evaluating for logical expression"),
                    })),
                }
            },
            Expr::Call(callee, paren, arguments) => {
                let callee_val = self.evaluate(callee)?;

                let mut arguments_val: Vec<LValue> = Vec::new();

                // TODO: arity check
                for argument in arguments {
                    arguments_val.push(self.evaluate(argument)?);
                }

                match callee_val {
                    LValue::Function(function) => {
                        return function.call(self, paren, arguments_val);
                    },
                    LValue::Class(class) => {
                        return class.call(self, paren, arguments_val);
                    },
                    _ => {
                        Err(RunTimeError::Error(CommonError {
                            token: Some(paren.clone()),
                            message: String::from("Can only call functions and classes."),
                        }))
                    },
                }
            },
            Expr::New(token, caller) => {
                let val = self.evaluate(caller);
                return val;
            },
            Expr::Get(object, field) => {
                let object_val = self.evaluate(object)?;

                match object_val {
                    LValue::ClassInstance(instance) => {
                        return Ok(instance.clone().get(field)?);
                    },
                    _ => {
                        Err(RunTimeError::Error(CommonError {
                            token: Some(field.clone()),
                            message: String::from("Only instances have fields."),
                        }))
                    }
                }
            },
            Expr::Set(object, field, val) => {
                let object_val = self.evaluate(object)?;

                match object_val {
                    LValue::ClassInstance(instance) => {
                        let value = self.evaluate(val)?;
                        instance.set(field, value.clone())?;
                        return Ok(value);
                    },
                    _ => {
                        Err(RunTimeError::Error(CommonError {
                            token: Some(field.clone()),
                            message: String::from("Only instances have fields."),
                        }))
                    }
                }
            },
            Expr::This(token) => {
                Ok(self.lookup_variable(token)?)
            },
            Expr::Super(token, method) => {
                let distance = self.locals.get(token).unwrap();
                let super_class = self.environment.borrow_mut().get_at(*distance, token)?;
                let object = self.environment.borrow_mut().get_at(distance - 1, &Token {
                    typee: TokenType::This,
                    line: 0,
                    col: 0,
                    literal: None,
                    lexeme: String::from("this").as_bytes().to_vec(),
                })?;

                if let LValue::Class(super_class_val) = super_class {
                    if let Expr::Variable(method_token) = method.deref() {
                        let method_val = super_class_val.find_method(method_token).unwrap();
                        if let LValue::ClassInstance(class_instance) = object {
                            return Ok(LValue::Function(method_val.bind(class_instance)));
                        }
                    }
                    
                    // Won't go in to this branch
                    return Ok(LValue::Nil);
                } else {
                    return Err(RunTimeError::Error(CommonError {
                        token: Some(token.clone()),
                        message: String::from("No super class found"),
                    }))
                }
            },
        }
    } 
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::Scanner;
    use crate::parser::Parser;

    #[test]
    fn interpreter_expression() {
        let source_expected: Vec<(String, LValue)> = vec![
            (String::from("\"abc\""), LValue::String(String::from("abc"))),
            (String::from("123"), LValue::Number(123.0)),
            (String::from("nil"), LValue::Nil),
            (String::from("true"), LValue::Bool(true)),
            (String::from("false"), LValue::Bool(false)),
            (String::from("1 * 2"), LValue::Number(2.0)),
            (String::from("(1 + 2) * 2"), LValue::Number(6.0)),
            (String::from("-1 * 2 + 3"), LValue::Number(1.0)),
            (String::from("1 + nil"), LValue::Number(1.0)),
            (String::from("1 - nil"), LValue::Number(1.0)),
            (String::from("1 + true"), LValue::Number(2.0)),
            (String::from("1 - true"), LValue::Number(0.0)),
            (String::from("1 + false"), LValue::Number(1.0)),
            (String::from("1 - false"), LValue::Number(1.0)),
            (String::from("\"abc\" + 1"), LValue::String(String::from("abc1"))),
            (String::from("\"abc\" + true"), LValue::String(String::from("abctrue"))),
            (String::from("\"abc\" + false"), LValue::String(String::from("abcfalse"))),
        ];

        for (source, expected) in source_expected {
            let tokens: Vec<Token> = Scanner::new(source).scan_tokens().unwrap();
            let expr = Parser::new(tokens).parse_expr_debug().unwrap();

            // AstPrinter::new().print(expr.clone());
            let interpreter_output = Interpreter::new().interpret_expr_debug(&expr).unwrap();
            println!("{:?}", interpreter_output);            
            assert_eq!(interpreter_output, expected)
        }
    }

    #[test]
    fn interpreter_expression_error() {
        let source_expected: Vec<(String, RunTimeError)> = vec![
            (
                String::from("1 / true"),
                RunTimeError::Error(CommonError {
                    token: Some(Token {
                        typee: TokenType::Slash,
                        lexeme: String::from("/").into(),
                        literal: None,
                        line: 1,
                        col: 3,
                    }),
                    message: String::from("Operation divide only supports for Number"),
                }),
            ),
            (
                String::from("2 * true"),
                RunTimeError::Error(CommonError {
                    token: Some(Token {
                        typee: TokenType::Star,
                        lexeme: String::from("*").into(),
                        literal: None,
                        line: 1,
                        col: 3,
                    }),
                    message: String::from("Operation multiple only supports for Number"),
                }),
            ),
            (
                String::from("1 - \"abc\""),
                RunTimeError::Error(CommonError {
                    token: Some(Token {
                        typee: TokenType::Minus,
                        lexeme: String::from("-").into(),
                        literal: None,
                        line: 1,
                        col: 3,
                    }),
                    message: String::from("Invalid operation subtract between number and string"),
                }),
            ),
        ];

        for (source, expected) in source_expected {
            let tokens: Vec<Token> = Scanner::new(source).scan_tokens().unwrap();
            let expr = Parser::new(tokens).parse_expr_debug().unwrap();

            // AstPrinter::new().print(expr.clone());
            let interpreter_error = Interpreter::new().interpret_expr_debug(&expr).unwrap_err();           
            assert_eq!(interpreter_error, expected)
        }
    }
}

