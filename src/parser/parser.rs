use crate::lexer::token::{Token, TokenType};
use crate::lexer::Lexer;
use crate::parser::expr::{Binary, ExprType, Literal};
use crate::value::Value;
use std::cell::Cell;

pub struct Parser {
    pub source: String,
    n: Cell<usize>,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(source: String) -> Parser {
        let mut tokens = Vec::new();
        let lexer = Lexer::new(&source);
        for token in lexer {
            tokens.push(token);
        }
        Parser {
            source,
            n: Cell::new(0),
            tokens,
        }
    }

    pub fn parse(&self) -> ExprType {
        self.addition()
    }

    fn addition(&self) -> ExprType {
        let left = self.multiply();
        if self.match_token(&[TokenType::PLUS, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.multiply();
            return ExprType::Binary(Binary {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            });
        }
        return left;
    }
    fn multiply(&self) -> ExprType {
        let left = self.term();
        if self.match_token(&[TokenType::STAR, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.term();
            return ExprType::Binary(Binary {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            });
        }
        return left;
    }

    fn term(&self) -> ExprType {
        if self.match_token(&[TokenType::NUMBER]) {
            let t = self.previous();
            let number: f64 = t.lexeme.parse().unwrap();
            return ExprType::Literal(Literal {
                value: Value::Float(number),
            });
        }
        panic!("Unexpected token {:?}", self.peek());
    }

    fn match_token(&self, types: &[TokenType]) -> bool {
        for t in types.iter() {
            if self.check(t) {
                self.increment();
                return true;
            }
        }
        return false;
    }
    fn check(&self, t: &TokenType) -> bool {
        if self.at_end() {
            return false;
        }
        match self.peek() {
            Some(t1) => match &t1.tt {
                t => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn at_end(&self) -> bool {
        if let None = self.next_token() {
            return true;
        }
        return false;
    }
    fn next_token(&self) -> Option<&Token> {
        return self.get_token();
    }

    fn get_token(&self) -> Option<&Token> {
        self.tokens.get(self.n.get())
    }

    fn previous(&self) -> &Token {
        return &self.tokens[self.n.get() - 1];
    }

    fn peek(&self) -> Option<&Token> {
        return self.next_token();
    }

    fn increment(&self) {
        self.n.set(self.n.get() + 1);
    }
}
