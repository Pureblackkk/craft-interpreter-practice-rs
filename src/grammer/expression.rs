use crate::scanner::token::Token;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expr {
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Literal(ExprLiteral),
    Variable(Token),
    Assign(Token, Box<Expr>),
    Logical(Box<Expr>, Token, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
    New(Token, Box<Expr>),
    Get(Box<Expr>, Token),
    Set(Box<Expr>, Token, Box<Expr>),
    This(Token),
    Super(Token, Box<Expr>),
}

pub trait ExprVistor<T> {
    fn visit(&mut self, expr: &Expr) -> T;
}

pub trait ExprAccept {
    fn accept<P, T: ExprVistor<P>>(&self, vistor: &mut T) -> P;
}

impl ExprAccept for Expr {
    fn accept<P, T: ExprVistor<P>>(&self, vistor: &mut T) -> P {
        vistor.visit(self)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ExprLiteral {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}