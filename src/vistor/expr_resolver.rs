use crate::resolver::{*};
use crate::grammer::expression::{*};
use crate::scanner::token::Token;
use resolve_error::ResolveError;

impl Resolver<'_> {
    pub fn resolve_expr(&mut self, expr: &Expr) -> Result<(), ResolveError> {
        expr.accept(self)
    }

    fn resolve_local(&mut self, name: &Token) -> Result<(), ResolveError> {
        let scope_size = self.scope_size();
        let name_string = String::from_utf8(name.lexeme.to_vec()).unwrap();

        for index in (0..scope_size).rev() {
            if self.scopes.get(index).unwrap().contains_key(&name_string) {
                self.interpreter.resolve(name, scope_size - index - 1);
                return Ok(());
            }
        }

        Err(ResolveError::CommonError {
            token: name.clone(),
            message: String::from("Can't read local variable in its own initializer."),
        })
    }
}

impl ExprVistor<Result<(), ResolveError>> for Resolver<'_> {
    fn visit(&mut self, expr: &Expr) -> Result<(), ResolveError> {
        match expr {
            Expr::Variable(name) => {
                let name_string = String::from_utf8(name.lexeme.to_vec()).unwrap();
                let current_scope_val = self.get_current_val(name_string);

                if let Some(val) = current_scope_val {
                    if !val {
                        return Err(ResolveError::CommonError {
                            token: name.clone(),
                            message: String::from("Can't read local variable in its own initializer."),
                        });
                    }
                }

                self.resolve_local(name)?;
                Ok(())
            },
            Expr::Assign(token, val) => {
                self.resolve_expr(val)?;
                self.resolve_local(token)?;

                Ok(())
            },
            Expr::Binary(l, _, r) => {
                self.resolve_expr(l)?;
                self.resolve_expr(r)?;
                Ok(())
            },
            Expr::Call(callee, _, param) => {
                self.resolve_expr(callee)?;

                for argument in param {
                    self.resolve_expr(argument)?;
                }

                Ok(())
            },
            Expr::Grouping(val) => {
                self.resolve_expr(val)?;
                Ok(())
            },
            Expr::Unary(_, expr) => {
                self.resolve_expr(expr)?;
                Ok(())
            },
            // TODO: Add rules which is: new only for class not funciton
            Expr::Get(object, property) => {
                self.resolve_expr(object)?;
                Ok(())
            },
            Expr::Set(object, property, val) => {
                self.resolve_expr(object)?;
                self.resolve_expr(val)?;
                Ok(())
            },
            Expr::This(token) => {
                if self.current_class == ClassStatus::None {
                    return Err(ResolveError::CommonError {
                        token: token.clone(),
                        message: String::from("Can't use 'this' outside of a class."),
                    });
                }

                self.resolve_local(token)?;
                Ok(())
            },
            Expr::Super(token, method) => {
                self.resolve_local(token)?;
                Ok(())
            },
            _ => Ok(()),
        }
    }
}