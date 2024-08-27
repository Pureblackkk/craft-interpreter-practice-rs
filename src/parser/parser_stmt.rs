use crate::grammer::statement::{FunctionType, Stmt};
use crate::grammer::expression::{Expr, ExprLiteral};
use super::*;

impl Parser {
    pub fn declaration(&mut self) -> Result<Stmt, ParserError> {
        // TODO: error synchronize
        if self.matches(TokenType::Var) {
            return self.var_declaration();
        }

        if self.matches(TokenType::Class) {
            return self.class();
        }

        if self.matches(TokenType::Fun) {
            return self.function(FunctionType::Function);
        }

        self.statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParserError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?.clone();

        let mut initializer: Option<Expr> = None;
        
        if self.matches(TokenType::Equal) {
            initializer = Some(self.expression()?);
        }

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::Var(name, initializer))
    }

    fn class(&mut self) -> Result<Stmt, ParserError> {
        let name = self.consume(TokenType::Identifier, "Expect class name.")?.clone();

        let mut superclass: Option<Expr> = None;

        if self.matches(TokenType::Extend) {
            self.consume(TokenType::Identifier, "Expect super class name");
            superclass = Some(Expr::Variable(self.previous().clone()));
        }


        // TODO: No block here might introducing circular reference
        self.consume(TokenType::LeftBrace, "Expect \'{ \' after class name")?;
        let mut methods: Vec<Stmt> = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.function(FunctionType::Method)?);
        }

        self.consume(TokenType::RightBrace, "Expect \'} \' after class body")?;

        Ok(Stmt::Class(name, superclass, methods))
    }

    fn function(&mut self, function_type: FunctionType) -> Result<Stmt, ParserError> {
        let function_name = self.consume(TokenType::Identifier, "Expect function name")?.clone();
        self.consume(TokenType::LeftParen, "Expect \'( \' after function name")?;

        let mut parameters: Vec<Token> = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err(ParserError::FunctionParamUpperLimit { token: self.peek().clone() });
                }

                parameters.push(self.consume(TokenType::Identifier, "Expect parameter name")?.clone());

                if !self.matches(TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect \' ) \' after parameters.")?;

        // Parse body
        self.consume(TokenType::LeftBrace, "Expect \' { \' before function body")?;
        let body = self.block()?;

        Ok(Stmt::Function(function_name, parameters, Box::new(body)))
    }
 
    fn statement(&mut self) -> Result<Stmt, ParserError> {
        if self.matches(TokenType::Print) {
            return self.print_statement();
        }

        if self.matches(TokenType::LeftBrace) {
            return self.block();
        }

        if self.matches(TokenType::If) {
            return self.if_statement();
        }

        if self.matches(TokenType::While) {
            return self.while_statement();
        }

        if self.matches(TokenType::For) {
            return self.for_statement();
        }

        if self.matches(TokenType::Return) {
            return self.return_statement();
        }

        self.expression_statement()
    }

    fn block(&mut self) -> Result<Stmt, ParserError> {
        let mut statements: Vec<Stmt> = vec![];

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let declaration = self.declaration()?;
            statements.push(declaration);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.");

        Ok(Stmt::Block(statements))
    }

    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(expr))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParserError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Expr(expr))
    }

    fn if_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenType::LeftParen, "Expect \' ( \' after if");
        let condition_expr = self.expression()?;
        self.consume(TokenType::RightParen, "Expect \' ) \' after if condition");

        let then_branch = self.statement()?;
        let mut else_branch: Option<Stmt> = None;

        if self.matches(TokenType::Else) {
            else_branch = Some(self.statement()?);
        }

        Ok(Stmt::If(condition_expr, Box::new(then_branch), Box::new(else_branch)))
    }

    fn while_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenType::LeftParen, "Expect \' ( \' after while");
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect \' ) \' after condition");
        let body = self.statement()?;

        Ok(Stmt::While(condition, Box::new(body)))
    }

    fn for_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenType::LeftParen, "Expect \' ( \' after for");

        // Parse initializer
        let mut initializer: Option<Stmt>;

        if self.matches(TokenType::Semicolon) {
            initializer = None;
        } else if self.matches(TokenType::Var) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        // Parse condition
        let mut condition: Option<Expr> = None;
        if !self.check(TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }

        self.consume(TokenType::Semicolon, "Expect \' ; \' after loop condition");

        // Parse increment
        let mut increment: Option<Expr> = None;

        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }

        self.consume(TokenType::RightParen, "Expect \' ) \' after for clauses.");

        // Parse body
        let body = self.statement()?;

        // Convert for loop to while loop, as desugaring
        let mut while_body: Stmt;
        let while_condition: Expr;

        // Create condition for while statement
        if condition.is_none() {
            while_condition = Expr::Literal(ExprLiteral::True);
        } else {
            while_condition = condition.unwrap();
        }

        // { body; increment }
        if !increment.is_none() {
            while_body = Stmt::Block(vec![body, Stmt::Expr(increment.unwrap())]);
        } else {
            while_body = body;
        }

        // while (condition) { body; increment }
        while_body = Stmt::While(while_condition, Box::new(while_body));

        // { initializer; while (condition) { body, increment } }
        if !initializer.is_none() {
            while_body = Stmt::Block(vec![initializer.unwrap(), while_body]);
        }

        Ok(while_body)
    }

    fn return_statement(&mut self) -> Result<Stmt, ParserError> {
        let token = self.previous().clone();
        let mut value: Option<Expr> = None;

        if !self.check(TokenType::Semicolon) {
            value = Some(self.expression()?);
        }

        self.consume(TokenType::Semicolon, "Expect \';\' after return value");

        Ok(Stmt::Return(token, value))
    }
}