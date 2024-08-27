use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::grammer::expression::Expr;
use crate::scanner::token::Token;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Box<Option<Stmt>>),
    While(Expr, Box<Stmt>),
    Function(Token, Vec<Token>, Box<Stmt>),
    Class(Token, Option<Expr>, Vec<Stmt>),
    Return(Token, Option<Expr>),
}

#[derive(Debug, Clone, Copy)]
pub enum FunctionType {
    Function,
    Method,
}

pub trait StmtVistor<T> {
    fn visit(&mut self, stmt: &Stmt) -> T;

    fn visit_env(&mut self, stmt: &Stmt, env: Rc<RefCell<Environment>>) -> T;
}

pub trait StmtAccept {
    fn accept<P, T: StmtVistor<P>>(&self, vistor: &mut T) -> P;

    fn accept_with_env<P, T: StmtVistor<P>>(&self, vistor: &mut T, env: Rc<RefCell<Environment>>) -> P;
}

impl StmtAccept for Stmt {
    fn accept<P, T: StmtVistor<P>>(&self, vistor: &mut T) -> P {
        vistor.visit(self)
    }

    fn accept_with_env<P, T: StmtVistor<P>>(&self, vistor: &mut T, env: Rc<RefCell<Environment>>) -> P {
        vistor.visit_env(self, env)
    }
}
