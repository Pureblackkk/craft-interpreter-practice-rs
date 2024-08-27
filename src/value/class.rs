use std::{cell::RefCell, collections::HashMap, rc::Rc};
use crate::{
    interpreter::Interpreter,
    runner::error::{CommonError, RunTimeError},
    scanner::token::{Literal, Token, TokenType},
};
use super::{
    function::Function,
    function::Callable,
    LValue
};

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: String,
    pub methods: HashMap<String, Rc<Function>>,
    pub supper_class: Option<Rc<Class>>,
}

// Just for avoiding complie check, maybe more error introducing here
impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Class {
    pub fn new(
        name: String,
        methods: HashMap<String, Rc<Function>>,
        supper_class: Option<Rc<Class>>,
    ) -> Rc<Class> {
        Rc::new(Class {
            name,
            methods,
            supper_class,
        })
    }

    pub fn to_string(&self) -> String {
        self.name.clone()
    }

    pub fn find_method(self: Rc<Self>, token: &Token) -> Option<Rc<Function>> {
        let method_name = String::from_utf8(token.lexeme.to_vec()).unwrap();

        if self.methods.contains_key(&method_name) {
            if let Some(method) = self.methods.get(&method_name) {
                return Some(method.clone());
            }
        }

        if let Some(ref supper_class) = self.supper_class {
            return supper_class.clone().find_method(token);
        }

        return None;
    }
}

impl Class {
    pub fn call(
        self: Rc<Self>,
        interpreter: &mut Interpreter,
        token: &Token,
        arguments: Vec<super::LValue>,
    ) -> Result<LValue, RunTimeError> {
        let instance = ClassInstance::new(
            self,
            interpreter,
            token,
            arguments,
        );
        Ok(LValue::ClassInstance(instance))
    }

    pub fn arity(&self) -> usize {
        return 0;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassInstance {
    class: Rc<Class>,
    fields: RefCell<HashMap<String, LValue>>,
}

// Just for avoiding complie check, maybe more error introducing here
impl PartialOrd for ClassInstance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.class.name.partial_cmp(&other.class.name)
    }
}

impl ClassInstance {
    pub fn new(class: Rc<Class>,
        interpreter: &mut Interpreter,
        token: &Token,
        arguments: Vec<super::LValue>,
    ) -> Rc<ClassInstance> {
        let instance = Rc::new(ClassInstance {
            class: class.clone(),
            fields: RefCell::new(HashMap::new()),
        });

        // Do the init job
        let mock_init_token = Token {
            typee: TokenType::Identifier,
            line: 0,
            col: 0,
            lexeme: String::from("init").into_bytes(),
            literal: Some(Literal::Str(String::from("init"))),
        };

        let initializer = class.find_method(&mock_init_token);

        if let Some(init_func) = initializer {
            init_func.bind(instance.clone()).call(interpreter, token, arguments);
        }

        return instance;
    }

    pub fn to_string(&self) -> String {
        format!("{} instance", self.class.name)
    }

    pub fn get(self: Rc<Self>, name: &Token) -> Result<LValue, RunTimeError>  {
        let name_string = String::from_utf8(name.lexeme.to_vec()).unwrap();

        if self.fields.borrow().contains_key(&name_string) {
            // TODO: return clone might introduce problem here
            return Ok(self.fields.borrow().get(&name_string).unwrap().clone());
        }

        if let Some(function) = self.class.clone().find_method(name) {
            return Ok(LValue::Function(function.bind(self.clone())));
        }

        Err(RunTimeError::Error(CommonError {
            token: Some(name.clone()),
            message: format!("Undefined property {}", name_string),
        }))
    }

    pub fn set(self: Rc<Self>, name: &Token, value: LValue) -> Result<(), RunTimeError> {
        // TODO: check if we force the object having corresponding fields
        let name_string = String::from_utf8(name.lexeme.to_vec()).unwrap();
        self.fields.borrow_mut().insert(name_string, value);
        Ok(())
    }
}