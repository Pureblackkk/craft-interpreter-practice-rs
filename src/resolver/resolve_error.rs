use std::fmt;
use crate::{scanner::token::Token};

pub enum ResolveError {
    CommonError {
        token: Token,
        message: String,
    }
}

impl fmt::Debug for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ResolveError::CommonError { token, message } => {
                write!(
                    f,
                    "Resolve Error, token {:?} found at line {:?}: {:?}",
                    token.literal, token.line, message,
                )
            }
        }
    }
}