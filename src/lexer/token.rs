#[derive(Debug, Clone)]
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
    SemiColon,
    OpenBrace,
    CloseBrace,
    If,
    Else,
    While,
    For,
    COMMA,
    Fun,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tt: TokenType,
    pub lexeme: String,
    pub line: usize,
    start: usize,
    end: usize,
}

impl Token {
    pub fn end_of_line() -> Token {
        return Token {
            tt: TokenType::EOL,
            lexeme: String::default(),
            line: 0,
            start: 0,
            end: 0,
        };
    }
    pub fn new(tt: TokenType, lexeme: String, line: usize, start: usize, end: usize) -> Token {
        Token {
            tt,
            lexeme,
            line,
            start,
            end,
        }
    }
}
