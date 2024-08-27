use crate::scanner::token::{Token, TokenType};
use crate::grammer::statement::Stmt;
use parese_error::ParserError;

mod parser_expr;
mod parser_stmt;
mod parese_error;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parser(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements: Vec<Stmt> = vec![];

        while !self.is_at_end() {
            let statement = self.declaration()?;
            statements.push(statement);   
        }

        Ok(statements)
    }

    fn consume(&mut self, typee: TokenType, message: &str) -> Result<&Token, ParserError> {
        if self.check(typee) {
            return Ok(self.advance());
        }

        Err(ParserError::TokenMisMatch {
            expected: typee,
            found: self.peek().clone(),
            message: message.to_string(),
        })
    }

    fn match_one_of(&mut self, types: Vec<TokenType>) -> bool {
        for typee in types {
            if self.matches(typee) {
                return true;
            }
        }

        false
    }

    fn matches(&mut self, typee: TokenType) -> bool {
        if self.check(typee) {
            self.advance();
            return true;
        }

        false
    }


    fn check(&mut self, typee: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().typee == typee
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;       
        }

        // TODO: Why return previous Token while at the end return current
        self.previous()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().typee == TokenType::Eof
    }

    fn peek(&mut self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&mut self) -> &Token {
        &self.tokens[self.current - 1]
    }
}

