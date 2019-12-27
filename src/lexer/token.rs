use crate::value::Value;

pub enum TokenType {
    PLUS,
    STAR,
    MINUS,
    SLASH,
    PS,
    EOL,
}

pub struct Token {
    tt: TokenType,
    lexeme: String,
    literal: Option<Value>,
    start: usize,
    end: usize,
}

impl Token {
    pub fn end_of_line() -> Token {
        return Token {
            tt: TokenType::EOL,
            lexeme: String::default(),
            literal: None,
            start: 0,
            end: 0,
        };
    }
    pub fn new(
        tt: TokenType,
        lexeme: String,
        literal: Option<Value>,
        start: usize,
        end: usize,
    ) -> Token {
        Token {
            tt,
            lexeme,
            literal,
            start,
            end,
        }
    }
}
