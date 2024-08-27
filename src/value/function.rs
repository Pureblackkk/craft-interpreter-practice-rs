use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment,
    grammer::statement::Stmt,
    interpreter::Interpreter,
    runner::error::RunTimeError,
    scanner::token::{Token, TokenType},
};
use super::{class::ClassInstance, LValue};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Stmt, // Block statement
    pub closure: Rc<RefCell<Environment>>,
    pub is_initializer: bool,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.params == other.params && self.body == other.body
    }
}

// Just for avoiding complie check 
impl PartialOrd for Function {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Function {
    pub fn bind(self: &Rc<Self>, instance: Rc<ClassInstance>) -> Rc<Function> {
        let mut new_environment = Environment::new();
        new_environment.define(String::from("this"), LValue::ClassInstance(instance));

        // Add closure for original method closure
        new_environment.enclosing = Some(self.closure.clone());

        Rc::new(Function {
            name: self.name.clone(),
            params: self.params.clone(),
            body: self.body.clone(),
            is_initializer: self.is_initializer,
            closure: Rc::new(RefCell::new(new_environment)),
        })
    }
}

pub trait Callable {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        token: &Token,
        arguments: Vec<LValue>,
    ) -> Result<LValue, RunTimeError>;
    fn arity(&self) -> usize;
}

impl Callable for Function {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        token: &Token,
        arguments: Vec<LValue>,
    ) -> Result<LValue, RunTimeError> {
        let mut environment = Environment::new();
        environment.enclosing = Some(self.closure.clone());

        for (index, param) in self.params.iter().enumerate() {
            environment.define(
                String::from_utf8(param.lexeme.to_vec()).unwrap(),
                arguments.get(index).unwrap().clone(),
            );
        }

        let environment_ref = Rc::new(RefCell::new(environment));
        let call_res = interpreter.exectue_with_env(&self.body, environment_ref);

        let init_mock_token = Token {
            typee: TokenType::Identifier,
            col: 0,
            line: 0,
            lexeme: String::from("this").as_bytes().to_vec(),
            literal: None,
        };

        if let Err(e) = call_res {
            match e {
                RunTimeError::Error(_) => return Err(e),
                RunTimeError::Return(val) => {
                    if self.is_initializer {
                        // Return this when it is initializer function
                        return self.closure.borrow_mut().get_at(0, &init_mock_token);
                    }
                    return Ok(val);
                }
            }
        }

        // Return this when it is initializer function
        if self.is_initializer {
            return self.closure.borrow_mut().get_at(0, &init_mock_token);
        }

        Ok(LValue::Nil)
    }

    fn arity(&self) -> usize {
        return 0;
    }
}