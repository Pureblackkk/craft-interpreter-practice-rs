use crate::grammer::expression::{Expr, ExprLiteral};
use crate::scanner::token;
use super::*;

impl Parser {
    pub fn parse_expr_debug(&mut self) -> Result<Expr, ParserError> {
        self.expression()
    }

    pub fn expression(&mut self) -> Result<Expr, ParserError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParserError> {
        let expr = self.equality()?;

        if self.matches(TokenType::Equal) {
            let token_equal = self.previous().clone();
            let value: Expr = self.assignment()?;

            match expr {
                Expr::Variable(token) => {
                    return Ok(Expr::Assign(token.clone(), Box::new(value)))
                },
                Expr::Get(object, propery) => {
                    return Ok(Expr::Set(object, propery, Box::new(value)));
                },
                _ => {
                    return Err(ParserError::InvalidAssignmentTarget { line: token_equal.line });
                }
            }            
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.and()?;

        while self.matches(TokenType::Or) {
            let operator = self.previous().clone();
            let right = self.and()?;
            left = Expr::Logical(Box::new(left), operator, Box::new(right));
        }

        Ok(left)
    }

    fn and(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.equality()?;

        while self.matches(TokenType::And) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            left = Expr::Logical(Box::new(left), operator, Box::new(right));
        }

        Ok(left)
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparision()?;

        while self.match_one_of(vec![
            TokenType::BangEqual,
            TokenType::EqualEqual
        ]) {
            let operator = self.previous().clone();
            let right = self.comparision()?;
            expr = Expr::Binary(
                Box::new(expr),
                operator,
                Box::new(right),
            );
        }

        Ok(expr)
    }

    fn comparision(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;

        while self.match_one_of(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(
                Box::new(expr),
                operator,
                Box::new(right),
            );
        }

        Ok(expr)
    }


    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        while self.match_one_of(vec![
            TokenType::Minus,
            TokenType::Plus,
        ]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(
                Box::new(expr),
                operator,
                Box::new(right),
            );
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        while self.match_one_of(vec![
            TokenType::Slash,
            TokenType::Star
        ]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(
                Box::new(expr),
                operator,
                Box::new(right)
            );
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.match_one_of(vec![
            TokenType::Bang,
            TokenType::Minus
        ]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        return self.class_init();
    }

    fn class_init(&mut self) -> Result<Expr, ParserError> {
        if self.matches(TokenType::New) {
            let init_token = self.previous().clone();
            let call_expr = self.call()?;

            return Ok(Expr::New(init_token, Box::new(call_expr)));
        }

        return self.call();
    }

    fn call(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.primary()?;

        while true {
            if self.matches(TokenType::LeftParen) {
                expr = self.finsh_call(expr)?;
            } else if self.matches(TokenType::Dot) {
                let property = self.consume(
                    TokenType::Identifier,
                    "Expect ')' after expression.",
                )?.clone();

                expr = Expr::Get(Box::new(expr), property);
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self.matches(TokenType::False) {
            return Ok(Expr::Literal(ExprLiteral::False));
        }

        if self.matches(TokenType::True) {
            return Ok(Expr::Literal(ExprLiteral::True));
        }

        if self.matches(TokenType::Nil) {
            return Ok(Expr::Literal(ExprLiteral::Nil));
        }

        if self.matches(TokenType::Number) {
            match &self.previous().literal {
                Some(token::Literal::Number(n)) => {
                    return Ok(Expr::Literal(ExprLiteral::Number(*n)))
                },
                Some(l) => panic!(
                    "internal error in parser: when parsing number, found literal {:?}",
                    l
                ),
                None => panic!("internal error in parser: when parsing number, found no literal"),
            }
        }

        if self.matches(TokenType::String) {
            match &self.previous().literal {
                Some(token::Literal::Str(s)) => {
                    return Ok(Expr::Literal(ExprLiteral::String(s.clone())))
                },
                Some(l) => panic!(
                    "internal error in parser: when parsing string, found literal {:?}",
                    l
                ),
                None => panic!("internal error in parser: when parsing string, found no literal"),
            }
        }

        if self.matches(TokenType::Identifier) {
            return Ok(Expr::Variable(self.previous().clone()));
        }

        if self.matches(TokenType::LeftParen) {
            let expr = self.expression()?;

            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;

            return Ok(Expr::Grouping(Box::new(expr)));
        }

        if self.matches(TokenType::This) {
            return Ok(Expr::This(self.previous().clone()));
        }

        if self.matches(TokenType::Super) {
            let token = self.previous().clone();
            self.consume(TokenType::Dot, "Expect \'.\' afer super")?;
            let method = self.consume(TokenType::Identifier, "Expect superclass method name.")?.clone();
            return Ok(Expr::Super(token, Box::new(Expr::Variable(method))))
        }

        Err(ParserError::ExpectedExpression { 
            token_type: self.peek().typee,
            line: self.peek().line,
        })
    }
}

/**
 * Helper function
 */
impl Parser {
    fn finsh_call(&mut self, callee: Expr) -> Result<Expr, ParserError> {
        let mut arguments: Vec<Expr> = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(ParserError::FunctionParamUpperLimit { token: self.peek().clone() });
                }

                arguments.push(self.expression()?);

                if !self.matches(TokenType::Comma) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect \')\' after argumnets")?;
        
        Ok(Expr::Call(Box::new(callee), paren.clone(), arguments))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::Scanner;
    use crate::vistor::ast_printer::AstPrinter;

    #[test]
    fn expression_primary() {
        let source_expected: Vec<(String, String)> = vec![
            (String::from("\"abc\""), String::from("abc")),
            (String::from("123"), String::from("123")),
            (String::from("123"), String::from("123")),
            (String::from("nil"), String::from("nil")),
            (String::from("true"), String::from("true")),
            (String::from("false"), String::from("false")),
        ];

        for (source, expected) in source_expected {
            let tokens: Vec<Token> = Scanner::new(source).scan_tokens().unwrap();
            let expr = Parser::new(tokens).parse_expr_debug().unwrap();
            assert_eq!(AstPrinter::new().print(expr), expected);
        }
    }

    #[test]
    fn expression_general() {
        let source_expected: Vec<(String, String)> = vec![
            (String::from("(1+2) * 3 + 5"), String::from("(+ (* (group (+ 1 2)) 3) 5)")),
            (String::from("-1 * 2"), String::from("(* (- 1) 2)")),
        ];

        for (source, expected) in source_expected {
            let tokens: Vec<Token> = Scanner::new(source).scan_tokens().unwrap();
            let expr = Parser::new(tokens).parse_expr_debug().unwrap();
            assert_eq!(AstPrinter::new().print(expr), expected);
        }
    }


    #[test]
    fn expression_error() {
        let source_expected: Vec<(String, ParserError)> = vec![
            (
                String::from("(1 + 2"),
                ParserError::TokenMisMatch {
                    expected: TokenType::RightParen,
                    found: Token { typee: TokenType::Eof, lexeme: "".as_bytes().to_vec(), literal: None, line: 1, col: 7 },
                    message: String::from("Expect ')' after expression."),
                }
            ),
        ];

        for (source, expected) in source_expected {
            let tokens: Vec<Token> = Scanner::new(source).scan_tokens().unwrap();
            let expr = Parser::new(tokens).parse_expr_debug().unwrap_err();
            assert_eq!(expr, expected);
        }
    }
}
