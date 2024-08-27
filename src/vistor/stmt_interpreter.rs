use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::grammer::statement::{*};
use crate::runner::error::{CommonError, RunTimeError};
use crate::value::function::Function;
use crate::value::class::Class;
use crate::value::LValue;
use crate::value::condition::{*};

impl Interpreter {
    pub fn interpret_stmt_debug(&mut self, stmt: &Stmt) -> Result<(), RunTimeError> {
        self.exectue(stmt)
    }

    pub fn exectue(&mut self, stmt: &Stmt) -> Result<(), RunTimeError> {
        stmt.accept(self)
    }

    pub fn exectue_with_env(&mut self, stmt: &Stmt, env: Rc<RefCell<Environment>>) -> Result<(), RunTimeError> {
        stmt.accept_with_env(self, env)
    }
}

impl StmtVistor<Result<(), RunTimeError>> for Interpreter {
    fn visit(&mut self, root_stmt: &Stmt) -> Result<(), RunTimeError> {
        match root_stmt {
            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;
                println!("{:?}", value);
                Ok(())
            },
            Stmt::Expr(expr) => {
                self.evaluate(expr)?;
                Ok(())
            },
            Stmt::Var(indentifier, expr) => {
                let name = String::from_utf8(indentifier.lexeme.to_vec()).unwrap();

                match expr {
                    Some(expr) => {
                        let value = self.evaluate(expr)?;
                        self.environment.borrow_mut().define(name, value);
                        Ok(())
                    },
                    None => {
                        self.environment.borrow_mut().define(name, LValue::Nil);
                        Ok(())
                    }, 
                }
            },
            Stmt::Block(statements) => {
                let previous_environment = self.environment.clone();
                let mut new_environment = Environment::new();
                new_environment.enclosing = Some(previous_environment.clone());

                self.environment = Rc::new(RefCell::new(new_environment));

                for statement in statements {
                    let value_result = self.exectue(statement)?;
                }

                self.environment = previous_environment;
                Ok(())
            },
            Stmt::If(condition, then_stmt, else_stmt) => {
                let condition_val = self.evaluate(condition)?;
                if condition_val.is_truthy() {
                    self.exectue(then_stmt)?;
                    return Ok(());
                } else if else_stmt.is_some() {
                    self.exectue((**else_stmt).as_ref().unwrap())?;
                    return Ok(());
                }

                Ok(())
            },
            Stmt::While(condition, body) => {
                while self.evaluate(condition)?.is_truthy() {
                    self.exectue(&(*body));
                }

                Ok(())
            },
            Stmt::Function(name, param, body) => {
                let function_name = String::from_utf8(name.lexeme.to_vec()).unwrap();
                let function_lvalue = LValue::Function(Rc::new(Function {
                    params: param.to_vec(),
                    name: name.clone(),
                    body: body.deref().clone(),
                    closure: self.environment.clone(),
                    is_initializer: false,
                }));

                self.environment.borrow_mut().define(function_name, function_lvalue);                
                Ok(())
            },
            Stmt::Return(_, value) => {
                if value.is_some() {
                    let return_value = self.evaluate((*value).as_ref().unwrap()).unwrap();
                    // Throw error to pass the return value
                    return Err(RunTimeError::Return(return_value));
                }

                Ok(())
            },
            Stmt::Class(name, supper_class, methods) => {
                // TODO: Reconstruct the code here
                let mut supper_class_val_option: Option<Rc<Class>> = None;

                if let Some(supper_class_expr) = supper_class {
                    let supper_class_val = self.evaluate(supper_class_expr)?;
                    match supper_class_val {
                        LValue::Class(class_rc) => {
                            supper_class_val_option = Some(class_rc);
                        },
                        _ => return Err(RunTimeError::Error(
                            CommonError {
                                message: String::from("Supperclass must be a class"),
                                token: None,
                            }
                        ))
                    }
                }
                
                let class_name = String::from_utf8(name.lexeme.to_vec()).unwrap();
                self.environment.borrow_mut().define(class_name.clone(), LValue::Nil);
                let previous_environment = self.environment.clone();

                // Create env for super class method
                // TODO: Reconstruct the code here
                if let Some(supper_class_expr) = supper_class {
                    let mut super_environment = Environment::new();

                    if let Some(ref supper_class_val) = supper_class_val_option {
                        super_environment.define(String::from("super"), LValue::Class(supper_class_val.clone()));
                    }

                    super_environment.enclosing = Some(self.environment.clone());
                    self.environment = Rc::new(RefCell::new(super_environment));
                }

                // Handle class method
                let mut methods_map: HashMap<String, Rc<Function>> = HashMap::new();

                // Add method into hashmap in the form of stmt::Function
                for method in methods {
                    if let Stmt::Function(name, param, body) = method {
                        let name_string = String::from_utf8(name.lexeme.to_vec()).unwrap();
                        let current_method = Rc::new(Function {
                            name: name.clone(),
                            params: param.to_vec(),
                            body: body.deref().clone(),
                            closure: self.environment.clone(),
                            is_initializer: name_string.eq(String::from("init").as_str()),
                        });

                        methods_map.insert(name_string, current_method);
                    }
                }

                let lclass = LValue::Class(Class::new(
                    class_name.clone(),
                    methods_map,
                    supper_class_val_option
                ));

                if let Some(supper_class_expr) = supper_class {
                    self.environment = previous_environment;
                }

                self.environment.borrow_mut().assign(name, lclass)?;
                Ok(())
            },
        }
    }

    fn visit_env(&mut self, root_stmt: &Stmt, env: Rc<RefCell<Environment>>) -> Result<(), RunTimeError> {
        match root_stmt {
            Stmt::Block(statements) => {
                let previous_environment = self.environment.clone();
                self.environment = env;

                for statement in statements {
                    let value_result = self.exectue(statement);
                    if let Err(e) = value_result {
                        // Restore current environment
                        self.environment = previous_environment;
                        return Err(e);
                    }
                }

                self.environment = previous_environment;
                Ok(())
            },
            _ => Err(RunTimeError::Error(
                CommonError {
                    message: String::from("Wrong statement type running with environment"),
                    token: None,
                }
            ))
        }
    }
}