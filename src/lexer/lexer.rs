use crate::lexer::token::{Token, TokenType};
use crate::value::Value;

pub struct Lexer<'a> {
    expr_chars: Vec<char>,
    tokens: Vec<&'a Token>,
    length: usize,
    start: usize,
    current: usize,
}

impl Lexer {
    pub fn new(expr: &str) -> Lexer {
        let a: Vec<char> = expr.chars().collect();
        return Lexer {
            length: a.len(),
            expr_chars: a,
            tokens: Vec::new(),
            start: 0,
            current: 0,
        };
    }
    fn is_at_end(&self) -> bool {
        return self.current >= self.length;
    }

    fn eat(&mut self) -> char {
        self.current += 1;
        self.expr_chars[self.current - 1]
    }

    fn peek(&self, n: usize) -> char {
        if self.current + n >= self.length {
            return '\0';
        }
        self.expr_chars[self.current + n]
    }

    fn lexeme(&self) -> String {
        // let a = &self.expr_chars[self.start..self.current]
        self.expr_chars[self.start..self.current]
            .iter()
            .cloned()
            .collect::<String>()
    }
    fn TT_with_literal(&mut self, tt: TokenType, literal: Option<Value>) ->Token {
        let token = Token::new(tt, self.lexeme(), literal, self.start,  self.current);
        self.start = self.current;
        self.tokens.push(&token);
        token
    }
}
