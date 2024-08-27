use std::fmt;
use super::*;

#[derive(PartialEq)]
pub enum ParserError {
    TokenMisMatch {
        expected: TokenType,
        found: Token,
        message: String,
    },

    ExpectedExpression {
        token_type: TokenType,
        line: usize,
    },

    InvalidAssignmentTarget {
        line: usize,
    },

    FunctionParamUpperLimit {
        token: Token,
    }
}

impl fmt::Debug for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ParserError::TokenMisMatch{
                expected,
                found,
                message,
            } => {
                write!(
                    f,
                    "Expected token {:?} but found {:?} at line={} : {}",
                    expected, found, found.line, message,
                )
            },
            ParserError::ExpectedExpression { 
                token_type,
                line,
            } => {
                write!(
                    f,
                    "Expected expression, but found {:?} at line = {}",
                    token_type, line,
                )
            },
            ParserError::InvalidAssignmentTarget {
                line 
            } => {
                write!(
                    f,
                    "Invalid assignment target, found at line = {}",
                    line,
                )
            },
            ParserError::FunctionParamUpperLimit { 
                token 
            } => {
                write!(
                    f,
                    "Can't have more than 255 arguments, found at line = {}",
                    token.line,
                )
            },
        }
    }
}
