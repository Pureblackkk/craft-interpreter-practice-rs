use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::environment::Environment;
use crate::grammer::expression::Expr;
use crate::resolver::{ClassStatus, FunctionStatus, Resolver, SetValueType};
use crate::grammer::statement::{*};
use crate::resolver::resolve_error::ResolveError;

impl Resolver<'_> {
    pub fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), ResolveError> {
        stmt.accept(self)
    }

    pub fn resolve_stmt_list(&mut self, stmts: &Vec<Stmt>) -> Result<(), ResolveError> {
        for stmt in stmts {
            self.resolve_stmt(stmt)?;
        }

        Ok(())
    }

    pub fn resolve_stmt_function(&mut self, stmt: &Stmt, function_status: FunctionStatus) -> Result<(), ResolveError> {
        if let Stmt::Function(name, argument, body) = stmt {
            let function_name = String::from_utf8(name.lexeme.to_vec()).unwrap();
            let mut function_status_to_assign = function_status;

            if function_name.eq(&String::from("init")) && function_status == FunctionStatus::Method {
                function_status_to_assign = FunctionStatus::Initializer;
            }

            let previous_function_status = self.current_function;
            self.current_function = function_status_to_assign;


            self.begin_scope();

            for token in argument {
                self.declare(token)?;
                self.define(token)?;
            }

            if let Stmt::Block(statmens) = body.deref() {
                self.resolve_stmt_list(statmens)?;
            }

            self.end_scope();
            self.current_function = previous_function_status;
        }

        Ok(())
    }
}

impl StmtVistor<Result<(), ResolveError>> for Resolver<'_> {
    fn visit(&mut self, stmt: &Stmt) -> Result<(), ResolveError> {
        match stmt {
            Stmt::Block(statements) => {
                self.begin_scope();
                self.resolve_stmt_list(statements)?;
                self.end_scope();
                Ok(())
            },
            Stmt::Var(indentifier, expr) => {
                self.declare(indentifier)?;

                if let Some(expr) = expr {
                    self.resolve_expr(expr)?;
                }

                self.define(indentifier)?;

                Ok(())
            },
            Stmt::Function(name, _, _) => {
                self.declare(name)?;
                self.define(name)?;
                self.resolve_stmt_function(stmt, FunctionStatus::Function)?;

                Ok(())
            },
            Stmt::Expr(expr) => {
                self.resolve_expr(expr)?;
                Ok(())
            },
            Stmt::If(condition, if_stmt, else_stmt) => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(if_stmt)?;
                if let Some(else_stmt_val) = else_stmt.deref() {
                    self.resolve_stmt(else_stmt_val)?;
                }
                Ok(())
            },
            Stmt::Print(expr) => {
                self.resolve_expr(expr)?;
                Ok(())
            },
            Stmt::Return(token, expr) => {
                if let Some(return_val) = expr {
                    self.resolve_expr(return_val)?;

                    if self.current_function == FunctionStatus::None {
                        return Err(ResolveError::CommonError {
                            token: token.clone(),
                            message: String::from("Can't return from top-level code."),
                        });
                    }

                    if self.current_function == FunctionStatus::Initializer {
                        return Err(ResolveError::CommonError {
                            token: token.clone(),
                            message: String::from("Can't return a value from an initializer."),
                        });
                    }
                }
                Ok(())
            },
            Stmt::While(condition, body) => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(body)?;
                Ok(())
            },
            Stmt::Class(name, supper_class, methods) => {
                let previous_class_status = self.current_class;
                self.current_class = ClassStatus::Class;

                self.declare(name)?;
                self.define(name)?;

                if let Some(supper_class_expr) = supper_class {
                    if let Expr::Variable(token) = supper_class_expr {
                        if !name.lexeme.eq(&token.lexeme) {
                            self.resolve_expr(supper_class_expr)?;

                            // Add a new scope and insert super keyword
                            self.begin_scope();
                            self.scopes.last_mut().unwrap().insert(String::from("super"), true);
                        } else {
                            return Err(ResolveError::CommonError {
                                token: token.clone(),
                                message: String::from("A class can't inherit from itself."),
                            });
                        }
                    } else {
                        return Err(ResolveError::CommonError {
                            token: name.clone(),
                            message: String::from("Only variable can be extended."),
                        });
                    }
                }

                self.begin_scope();
                // Push this into the class scope
                self.scopes.last_mut().unwrap().insert(String::from("this"), true);

                for method in methods {
                    self.resolve_stmt_function(method, FunctionStatus::Method)?;
                }
                
                // Exit class scope
                self.end_scope();

                // Exit super class scope
                if let Some(supper_class_expr) = supper_class {
                    self.end_scope();
                }

                // Restore class status
                self.current_class = previous_class_status;

                Ok(())
            },
            _ => Ok(()),
        }
    }

    fn visit_env(&mut self, stmt: &Stmt, env: Rc<RefCell<Environment>>) -> Result<(), ResolveError> {
        Ok(())
    }
}