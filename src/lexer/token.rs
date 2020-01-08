#[derive(Debug)]
pub enum TokenType {
    Plus,
    Star,
    Minus,
    Slash,
    Percentage,
    EOL,
    Number,
    OpenParen,
    CloseParen,
    True,
    False,
    Nil,
    Print,
    Var,
    String,
    Identifier,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Lesser,
    LesserEqual,
}

#[derive(Debug)]
pub struct Token {
    pub tt: TokenType,
    pub lexeme: String,
    start: usize,
    end: usize,
}

impl Token {
    pub fn end_of_line() -> Token {
        return Token {
            tt: TokenType::EOL,
            lexeme: String::default(),
            start: 0,
            end: 0,
        };
    }
    pub fn new(tt: TokenType, lexeme: String, start: usize, end: usize) -> Token {
        Token {
            tt,
            lexeme,
            start,
            end,
        }
    }
}
