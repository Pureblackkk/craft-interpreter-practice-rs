use crate::{
    grammer::statement::Stmt,
    interpreter::Interpreter,
    scanner::token::Token
};
use std::{collections::HashMap};
use resolve_error::ResolveError;
pub mod resolve_error;

pub struct Resolver<'a> {
    pub scopes: Vec<HashMap<String, bool>>,
    pub interpreter: &'a mut Interpreter,
    pub current_function: FunctionStatus,
    pub current_class: ClassStatus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SetValueType {
    Declar,
    Define,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FunctionStatus {
    None,
    Function,
    Method,
    Initializer,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClassStatus {
    None,
    Class,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Resolver {
        let mut init_scopes: Vec<HashMap<String, bool>> = Vec::new();
        init_scopes.push(HashMap::new());

        Resolver {
            interpreter,
            scopes: init_scopes,
            current_function: FunctionStatus::None,
            current_class: ClassStatus::None,
        }
    }

    pub fn resolve(&mut self, statements: &Vec<Stmt>) -> Result<(), ResolveError> {
        self.resolve_stmt_list(statements)
    }

    pub fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn end_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn is_scope_empty(&mut self) -> bool {
        self.scopes.is_empty()
    }

    pub fn scope_size(&self) -> usize {
        self.scopes.len()
    }

    pub fn set_current_val(&mut self, token: &Token, val: bool, operation: SetValueType) -> Result<(), ResolveError> {
        if self.is_scope_empty() {
            return Ok(());
        }

        let name = String::from_utf8(token.lexeme.to_vec()).unwrap();
        let current_scope = self.scopes.last_mut().unwrap();

        match operation {
            SetValueType::Declar => {
                if current_scope.contains_key(&name) {
                    // Duplicated declar
                    return Err(ResolveError::CommonError { 
                        token: token.clone(),
                        message: String::from("Already a variable with this name in this scope."),
                    })
                }

                current_scope.insert(name, val);
                Ok(())
            },
            SetValueType::Define => {
                current_scope.insert(name, val);
                Ok(())
            }
        }
    }

    pub fn get_current_val(&mut self, name: String) -> Option<bool> {
        if self.is_scope_empty() {
            return None;
        }

        let current_scope = self.scopes.last().unwrap();
        current_scope.get(&name).copied()
    }


    pub fn declare(&mut self, name: &Token) -> Result<(), ResolveError> {
        self.set_current_val(
            name, 
            false,
            SetValueType::Declar,
        )
    }

    pub fn define(&mut self, name: &Token) -> Result<(), ResolveError> {
        self.set_current_val(
            name, 
            true,
            SetValueType::Define,
        )
    }
}