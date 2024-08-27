use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::ops::Deref;
use std::ptr;
use std::rc::Rc;
use crate::scanner::token::Token;
use crate::value::LValue;
use crate::runner::error::{CommonError, RunTimeError};

#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    map: HashMap<String, LValue>,    
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            map: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn define(&mut self, name: String, value: LValue) {
        self.map.insert(name, value);
    }

    pub fn assign(&mut self, token: &Token, value: LValue) -> Result<(), RunTimeError>{
        let name_string = String::from_utf8(token.lexeme.to_vec()).unwrap();

        if self.map.contains_key(&name_string) {
            self.map.insert(name_string, value);
            return Ok(());
        }

        if let Some(ref enclosing) = self.enclosing {
            return enclosing.borrow_mut().assign(token, value);
        }

        // if let Some(env) = &mut self.enclosing {
        //     return env.assign(token, value);
        // }

        Err(RunTimeError::Error(CommonError {
            token: Some(token.clone()),
            message: "Undefined variable".to_owned() + name_string.as_str(),
        }))
    }

    pub fn get(&self, token: &Token) -> Result<LValue, RunTimeError>{
        let name_string = String::from_utf8(token.lexeme.to_vec()).unwrap();
        
        if self.map.contains_key(&name_string) {
            return Ok(self.map.get(&name_string).unwrap().clone());
        }

        if let Some(ref enclosing) = self.enclosing {
            return enclosing.borrow_mut().get(token);
        }
        
        // if let Some(env) = &self.enclosing {
        //     return env.get(token);
        // }

        Err(RunTimeError::Error(CommonError {
            token: Some(token.clone()),
            message: "Undefined variable".to_owned() + name_string.as_str(),
        }))
    }

    pub fn get_at(&self, distance: usize, token: &Token) -> Result<LValue, RunTimeError> {
        let name_string = String::from_utf8(token.lexeme.to_vec()).unwrap();

        if distance == 0 {
            return Ok(self.map.get(&name_string).unwrap().clone());
        }

        let mut ancestor_env = self.enclosing.clone().unwrap();
        
        for _ in 1..distance {
            let clone_env_option = ancestor_env.borrow().enclosing.clone();
            ancestor_env = clone_env_option.unwrap();
        }

        let res = ancestor_env.borrow_mut().map.get(&name_string).unwrap().clone();
        Ok(res)
    }

    pub fn assign_at(&mut self, distance: usize, token: &Token, value: LValue) -> Result<(), RunTimeError> {
        let name_string = String::from_utf8(token.lexeme.to_vec()).unwrap();
        
        if distance == 0 {
            self.map.insert(name_string, value);
            return Ok(());
        }

        let mut ancestor_env = self.enclosing.clone().unwrap();

        for _ in 1..distance {
            let clone_env_option = ancestor_env.borrow().enclosing.clone();
            ancestor_env = clone_env_option.unwrap();
        }       

        ancestor_env.borrow_mut().map.insert(name_string, value);

        Ok(())
    }
}