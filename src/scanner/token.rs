use std::{fmt, hash::Hash};
use phf::{phf_map};

#[derive(Eq, PartialEq, PartialOrd, Debug, Copy, Clone, Hash)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,
 
    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Lambda,
    New,
    Extend,

    Eof,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64),
}

pub static RESERVED_KEYWORD: phf::Map<&'static str, TokenType> = phf_map!{
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While,
    "lambda" => TokenType::Lambda,
    "new" => TokenType::New,
    "extend" => TokenType::Extend,
};

#[derive(Clone, PartialOrd)]
pub struct Token {
    pub typee: TokenType,
    pub lexeme: Vec<u8>,
    pub literal: Option<Literal>,
    pub line: usize,
    pub col: usize,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token {{ type: {:?}, lexeme: \"{}\", literal: {:#?}, line: {:#?}, col: {:#?}}}\n",
            self.typee,
            String::from_utf8(self.lexeme.clone()).unwrap(),
            self.literal,
            self.line,
            self.col,
        )
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.typee == other.typee
        && self.lexeme == other.lexeme
        && self.line == other.line
        && self.col == other.col
    }
}

impl Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.typee.hash(state);
        self.lexeme.hash(state);
        self.line.hash(state);
        self.col.hash(state);
    }
}

impl Eq for Token {}


#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn create_token() {
        let typee = TokenType::Bang;
        let lexeme = String::from("hi, this is a test");
        let literal = Some(Literal::Str(String::from("variable")));
        let line: usize = 10;
        let col: usize = 1;

        let token = Token {
            typee,
            lexeme: lexeme.as_bytes().to_vec(),
            literal: literal.clone(),
            line,
            col,
        };

        assert_eq!(token.typee, typee);
        assert_eq!(token.lexeme, lexeme.as_bytes().to_vec());
        assert_eq!(token.literal, literal);
        assert_eq!(token.line, line);
        assert_eq!(token.col, col);
   } 
}