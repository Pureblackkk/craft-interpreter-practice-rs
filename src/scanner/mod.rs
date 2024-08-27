use self::token::{
    Token,
    Literal,
    TokenType,
    RESERVED_KEYWORD,
};

pub mod token;

#[derive(Clone, Debug, PartialEq)]
pub struct ScannerError {
    pub reason: String,
    pub line: usize,
}

pub struct Scanner {
    pub source: Vec<u8>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    col: usize,
    error: Option<ScannerError>,
}

impl Scanner {
    pub fn new(src: String) -> Scanner {
        Scanner {
            source: src.into_bytes(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            col: 0,
            error: None,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, ScannerError> {
        while !self.done() {
            self.start = self.current;
            self.scan_token();
        }

        match &self.error {
            Some(error) => Err(error.clone()),
            None => {
                self.tokens.push(Token {
                    typee: TokenType::Eof,
                    lexeme: Vec::new(),
                    literal: None,
                    line: self.line,
                    col: self.col + 1,
                });
                Ok(self.tokens.to_vec())
            }
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            '[' => self.add_token(TokenType::LeftBracket, None),
            ']' => self.add_token(TokenType::RightBracket, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                if self.match_next('=') {
                    self.add_token(TokenType::BangEqual, None)
                } else {
                    self.add_token(TokenType::Bang, None)
                }
            },
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenType::EqualEqual, None)
                } else {
                    self.add_token(TokenType::Equal, None)
                }
            },
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::GreaterEqual, None)
                } else {
                    self.add_token(TokenType::Greater, None)
                }
            },
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::LessEqual, None)
                } else {
                    self.add_token(TokenType::Less, None)
                }
            },
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None)
                }
            }
            ' ' | '\r' | '\t' => {},
            '\n' => {
                self.line += 1;
                // Reset col num
                self.col = 0;
            },
            '"' => {
                self.string();
            },
            _ => {
                if Scanner::is_digit(c) {
                    self.number()
                } else if Scanner::is_alpha(c) {
                    self.identifier()
                } else {
                    self.error = Some(ScannerError{
                        reason: format!("Unexpected character {}", c),
                        line: self.line
                    })
                }
            },
        }
    }

    fn advance(&mut self) -> char {
        let c = char::from(self.source[self.current]);
        self.current += 1;
        self.col += 1;
        c
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if char::from(self.source[self.current]) != expected {
            return false;
        }

        self.current += 1;
        self.col += 1;
        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        char::from(self.source[self.current])
    }

    fn peek_next(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        char::from(self.source[self.current + 1])
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start..self.current].to_vec();
        let start_col = self.col - (self.current - self.start - 1);

        self.tokens.push(Token {
            typee: token_type,
            lexeme: text,
            literal,
            line: self.line,
            col: start_col,
        })
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error = Some(ScannerError{
                reason: String::from("Unterminated string!"),
                line: self.line,
            });
            return;
        }

        self.advance();

        self.add_token(TokenType::String, Some(Literal::Str(
            String::from_utf8(self.source[self.start + 1..self.current - 1].to_vec()).unwrap()
        )));
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            self.advance();

            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        let val: f64 = String::from_utf8(self.source[self.start..self.current].to_vec())
            .unwrap()
            .parse()
            .unwrap();

        self.add_token(TokenType::Number, Some(Literal::Number(val)))
    }

    fn identifier(&mut self) {
        while Scanner::is_alpha_numberic(self.peek()) {
            self.advance();
        }

        let text = String::from_utf8(self.source[self.start..self.current].to_vec()).unwrap();
        let token_type = match RESERVED_KEYWORD.get(text.as_str()) {
            Some(reserved_token_type) => *reserved_token_type,
            None => TokenType::Identifier,
        };

        match token_type {
            TokenType::Identifier => self.add_token(token_type, Some(Literal::Identifier(text))),
            _ => self.add_token(token_type, None)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn done(&self) -> bool {
        self.is_at_end() || self.error.is_some()
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') ||
        (c >= 'A' && c <= 'Z') ||
        c == '_'
    }

    fn is_alpha_numberic(c: char) -> bool {
        return Scanner::is_alpha(c) || Scanner::is_digit(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_token(
        typee: TokenType,
        lexeme: &str,
        literal: Option<Literal>,
        line: usize,
        col: usize,
    ) -> Token {
        Token {
            typee,
            lexeme: lexeme.as_bytes().to_vec(),
            literal,
            line,
            col,
        }
    }

    #[test]
    fn unit_single_character_token() {
        let test_source: String = String::from("(){}[],.-+;/*");
        let expected_tokens: Vec<Token> = vec![
            (
                TokenType::LeftParen,
                "(",
                None::<Literal>,
                1,
                1,
            ),
            (
                TokenType::RightParen,
                ")",
                None,
                1,
                2,
            ),
            (
                TokenType::LeftBrace,
                "{",
                None,
                1,
                3,
            ),
            (
                TokenType::RightBrace,
                "}",
                None,
                1,
                4,
            ),
            (
                TokenType::LeftBracket,
                "[",
                None,
                1,
                5,
            ),
            (
                TokenType::RightBracket,
                "]",
                None,
                1,
                6,
            ),
            (
                TokenType::Comma,
                ",",
                None,
                1,
                7,
            ),
            (
                TokenType::Dot,
                ".",
                None,
                1,
                8,
            ),
            (
                TokenType::Minus,
                "-",
                None,
                1,
                9,
            ),
            (
                TokenType::Plus,
                "+",
                None,
                1,
                10,
            ),
            (
                TokenType::Semicolon,
                ";",
                None,
                1,
                11,
            ),
            (
                TokenType::Slash,
                "/",
                None,
                1,
                12,
            ),
            (
                TokenType::Star,
                "*",
                None,
                1,
                13,
            ),
            (
                TokenType::Eof,
                "",
                None,
                1,
                14,
            ),
        ].into_iter()
        .map(|(t, le, li, line, col)| create_token(t, le, li, line, col))
        .collect();

        let mut scanner = Scanner::new(test_source);
        let tokens = scanner.scan_tokens().unwrap();
            
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn unit_one_or_two_character_token() {
        let test_source: String = String::from("! != = == > >= < <=");
        let expected_tokens: Vec<Token> = vec![
            (
                TokenType::Bang,
                "!",
                None::<Literal>,
                1,
                1,
            ),
            (
                TokenType::BangEqual,
                "!=",
                None,
                1,
                3,
            ),
            (
                TokenType::Equal,
                "=",
                None,
                1,
                6,
            ),
            (
                TokenType::EqualEqual,
                "==",
                None,
                1,
                8,
            ),
            (
                TokenType::Greater,
                ">",
                None,
                1,
                11,
            ),
            (
                TokenType::GreaterEqual,
                ">=",
                None,
                1,
                13,
            ),
            (
                TokenType::Less,
                "<",
                None,
                1,
                16,
            ),
            (
                TokenType::LessEqual,
                "<=",
                None,
                1,
                18,
            ),
            (
                TokenType::Eof,
                "",
                None,
                1,
                20,
            ),
        ].into_iter()
        .map(|(t, le, li, line, col)| create_token(t, le, li, line, col))
        .collect();

        let mut scanner = Scanner::new(test_source);
        let tokens = scanner.scan_tokens().unwrap();
            
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn unit_string_literals() {
        let source: String = String::from("\"This is a test\"");
        let expected_tokens: Vec<Token> = vec![
            (
                TokenType::String,
                "\"This is a test\"",
                Some(Literal::Str(String::from("This is a test"))),
                1,
                1,
            ),
            (
                TokenType::Eof,
                "",
                None,
                1,
                17,
            ),
        ].into_iter()
        .map(|(t, le, li, line, col)| create_token(t, le, li, line, col))
        .collect();

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn unit_number_literals() {
        let source: String = String::from("12345.12");
        let expected_tokens: Vec<Token> = vec![
            (
                TokenType::Number,
                "12345.12",
                Some(Literal::Number(12345.12)),
                1,
                1,
            ),
            (
                TokenType::Eof,
                "",
                None,
                1,
                9,
            ),
        ].into_iter()
        .map(|(t, le, li, line, col)| create_token(t, le, li, line, col))
        .collect();

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn unit_identifier_literals() {
        let source: String = String::from("a = 1");
        let expected_tokens: Vec<Token> = vec![
            (
                TokenType::Identifier,
                "a",
                Some(Literal::Identifier(String::from("a"))),
                1,
                1,
            ),
            (
                TokenType::Equal,
                "=",
                None,
                1,
                3,
            ),
            (
                TokenType::Number,
                "1",
                Some(Literal::Number(1.0)),
                1,
                5,
            ),
            (
                TokenType::Eof,
                "",
                None,
                1,
                6,
            ),
        ].into_iter()
        .map(|(t, le, li, line, col)| create_token(t, le, li, line, col))
        .collect();

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn unit_reserved_keywords() {
        let source: String = String::from("a or b");
        let expected_tokens: Vec<Token> = vec![
            (
                TokenType::Identifier,
                "a",
                Some(Literal::Identifier(String::from("a"))),
                1,
                1,
            ),
            (
                TokenType::Or,
                "or",
                None,
                1,
                3,
            ),
            (
                TokenType::Identifier,
                "b",
                Some(Literal::Identifier(String::from("b"))),
                1,
                6,
            ),
            (
                TokenType::Eof,
                "",
                None,
                1,
                7,
            ),
        ].into_iter()
        .map(|(t, le, li, line, col)| create_token(t, le, li, line, col))
        .collect();

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn multiple_line() {
        let source: String = String::from("
            a = 1;
            b = 2;
        ");

        let expected_tokens: Vec<Token> = vec![
            (
                TokenType::Identifier,
                "a",
                Some(Literal::Identifier(String::from("a"))),
                2,
                13,
            ),
            (
                TokenType::Equal,
                "=",
                None,
                2,
                15,
            ),
            (
                TokenType::Number,
                "1",
                Some(Literal::Number(1.0)),
                2,
                17,
            ),
            (
                TokenType::Semicolon,
                ";",
                None,
                2,
                18,
            ),
            (
                TokenType::Identifier,
                "b",
                Some(Literal::Identifier(String::from("b"))),
                3,
                13,
            ),
            (
                TokenType::Equal,
                "=",
                None,
                3,
                15,
            ),
            (
                TokenType::Number,
                "2",
                Some(Literal::Number(2.0)),
                3,
                17,
            ),
            (
                TokenType::Semicolon,
                ";",
                None,
                3,
                18,
            ),
            (
                TokenType::Eof,
                "",
                None,
                4,
                9,
            ),
        ].into_iter()
        .map(|(t, le, li, line, col)| create_token(t, le, li, line, col))
        .collect();

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn unit_notation() {
        let source: String = String::from("// Here is notation");
        let expected_tokens: Vec<Token> = vec![
            (
                TokenType::Eof,
                "",
                None,
                1,
                20,
            ),
        ].into_iter()
        .map(|(t, le, li, line, col)| create_token(t, le, li, line, col))
        .collect();

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn error_unterminated_string() {
        let source: String = String::from("\"Unterminated error");

        let expected_error = ScannerError{
            reason: String::from("Unterminated string!"),
            line: 1,
        };

        let mut scanner = Scanner::new(source);
        let error = scanner.scan_tokens().unwrap_err();

        assert_eq!(error, expected_error);
    }

    #[test]
    fn error_unexpected_char() {
        let source: String = String::from("$a");

        let expected_error = ScannerError{
            reason: String::from("Unexpected character $"),
            line: 1,
        };

        let mut scanner = Scanner::new(source);
        let error = scanner.scan_tokens().unwrap_err();

        assert_eq!(error, expected_error);
    }
}