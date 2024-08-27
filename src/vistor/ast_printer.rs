use std::fmt::Arguments;

use crate::grammer::expression::{Expr, ExprAccept, ExprLiteral, ExprVistor};

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> AstPrinter {
        AstPrinter{}
    }

    pub fn print(&mut self, expr: Expr) -> String {
        let output = expr.accept(self);
        println!("{}", output);
        output
    }

    fn parenthesize(&mut self, name: &str, exprs: &Vec<&Expr>) -> String {
        let mut output = String::from("");
        output.push_str("(");
        output.push_str(name);

        for expr in exprs {
            output.push_str(" ");
            output.push_str(expr.accept(self).as_str());
        }

        output.push_str(")");

        output.to_string()
    }
}

impl ExprVistor<String> for AstPrinter {
    fn visit(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Unary(token, expr) => {
                self.parenthesize(
                    String::from_utf8(token.lexeme.to_vec()).unwrap().as_str(),
                    &vec![expr],
                )
            },
            Expr::Binary(l, op, r) => {
                self.parenthesize(
                    String::from_utf8(op.lexeme.to_vec()).unwrap().as_str(),
                    &vec![l, r],
                )
            },
            Expr::Grouping(expr) => {
                self.parenthesize(
                    "group",
                    &vec![expr],
                )
            },
            Expr::Literal(literal) => {
                match literal {
                    ExprLiteral::Nil => String::from("nil"),
                    ExprLiteral::True => String::from("true"),
                    ExprLiteral::False => String::from("false"),
                    ExprLiteral::String(s) => s.clone(),
                    ExprLiteral::Number(n) => n.to_string(),
                }
            },
            Expr::Variable(expr) => {
                format!("idt {:?}", expr.literal)
            },
            Expr::Assign(token, expr) => {
                self.parenthesize(
                    format!("{:?} = ", String::from_utf8(token.lexeme.to_vec())).as_str(),
                    &vec![expr],
                )
            },
            Expr::Logical(l, op, r) => {
                self.parenthesize(
                    String::from_utf8(op.lexeme.to_vec()).unwrap().as_str(),
                    &vec![l, r],
                )   
            },
            Expr::Call(callee, token, arguments) => {
                self.parenthesize(
                    format!("func {:#?} ", callee).as_str(),
                    &(arguments.iter().collect()),
                )
            },
            Expr::New(_, call_expr) => {
                self.parenthesize(
                    format!("new class").as_str(),
                    &vec![call_expr],
                )
            },
            Expr::Get(object, property) => {
                self.parenthesize(
                    format!("get").as_str(),
                    &vec![object],
                )
            },
            Expr::Set(object, property, value) => {
                self.parenthesize(
                    format!("set").as_str(),
                    &vec![object, value],
                )
            },
            Expr::This(_) => {
                format!("this ")
            },
            Expr::Super(_, method) => {
                self.parenthesize(
                    format!("super").as_str(),
                    &vec![method] 
                )
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::token::{Token, TokenType};
    use crate::grammer::expression::{Expr, ExprLiteral};

    #[test]
    fn display_token() {
        let expression = Expr::Binary(
            Box::new(Expr::Unary(
                Token {
                    typee: TokenType::Minus,
                    lexeme: "-".as_bytes().to_vec(),
                    literal: None,
                    line: 1,
                    col: 1,
                }, 
                Box::new(Expr::Literal(ExprLiteral::Number(123.0))),
            )),
            Token {
                typee: TokenType::Star,
                lexeme: "*".as_bytes().to_vec(),
                literal: None,
                line: 1,
                col: 2,
            },
            Box::new(
                Expr::Grouping(
                    Box::new(
                        Expr::Literal(ExprLiteral::Number(45.67)
                    )
                ))
            ),
        );

        assert_eq!(
            AstPrinter::new().print(expression),
            String::from("(* (- 123) (group 45.67))"),
        );
    }
}
