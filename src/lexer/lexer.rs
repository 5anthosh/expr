use crate::lexer::token::{Token, TokenType};
use crate::value::Value;

pub struct Lexer {
    expr_chars: Vec<char>,
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
        self.expr_chars[self.start..self.current]
            .iter()
            .cloned()
            .collect::<String>()
    }

    fn token_type_with_literal(&mut self, tt: TokenType, literal: Option<Value>) -> Token {
        let token: Token = Token::new(tt, self.lexeme(), literal, self.start, self.current);
        self.start = self.current;
        return token;
    }

    fn token_type(&mut self, tt: TokenType) -> Token {
        return self.token_type_with_literal(tt, None);
    }

    fn space(&mut self) -> char {
        let mut c = self.eat();
        while c.is_whitespace() {
            self.start = self.current;
            c = self.eat();
        }
        return c;
    }

    pub fn next(&mut self) -> Token {
        if self.is_at_end() {
            return Token::end_of_line();
        }
        self.scan_token()
    }

    fn number(&mut self) -> Token {
        while self.peek(0).is_digit(10) {
            self.eat();
        }
        if self.peek(0) == '.' && self.peek(1).is_digit(10) {
            self.eat();
            while self.peek(0).is_digit(10) {
                self.eat();
            }
        }
        let number = self.lexeme();
        let number: f64 = number.parse().unwrap();
        return self.token_type_with_literal(TokenType::NUMBER, Some(Value::Float(number)));
    }

    fn scan_token(&mut self) -> Token {
        let c = self.space();
        match c {
            '+' => self.token_type(TokenType::PLUS),
            '-' => self.token_type(TokenType::MINUS),
            '*' => self.token_type(TokenType::STAR),
            '/' => self.token_type(TokenType::SLASH),
            '%' => self.token_type(TokenType::PS),
            _ => {
                if c.is_digit(10) {
                    return self.number();
                }
                panic!(format!("Unexpected character {}", c));
            }
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next();
        match token.tt {
            TokenType::EOL => None,
            _ => Some(token),
        }
    }
}
