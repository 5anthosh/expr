use crate::value::Value;

pub enum TokenType {
    PLUS,
    STAR,
    MINUS,
    SLASH,
    PS,
}

pub struct Token {
    tt: TokenType,
    lexeme: String,
    literal: Option<Value>,
    start: usize,
    end: usize,
}

impl Token {
    pub fn new(tt: TokenType, lexeme: String, literal: Option<Value>, start: usize, end: usize) -> Token {
        Token {
            tt,
            lexeme,
            literal,
            start,
            end,
        }
    }
}
