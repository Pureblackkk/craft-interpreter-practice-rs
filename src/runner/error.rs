use crate::{
    scanner::token::Token,
    value::LValue
};

#[derive(Debug, PartialEq)]
pub enum RunTimeError {
    Error(CommonError),
    Return(LValue),
}

#[derive(Debug, PartialEq)]
pub struct CommonError {
    pub message: String,
    pub token: Option<Token>,
}