pub mod operation;
pub mod condition;
pub mod function;
pub mod class;

use std::{cell::RefCell, rc::Rc};
use function::Function;
use class::{Class, ClassInstance};


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LValue {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    Function(Rc<Function>),
    Class(Rc<Class>),
    ClassInstance(Rc<ClassInstance>),
}