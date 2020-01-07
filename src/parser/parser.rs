use crate::lexer::token::{Token, TokenType};
use crate::lexer::Lexer;
use crate::parser::expr::{Binary, Literal, ExprType};

struct Parser {
    pub source: String,
    lexer: Lexer,
    n: usize,
    tokens: Vec<Token>,
}

impl Parser {
    fn addition(&mut self) -> ExprType {
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
        return left
    }
    fn multiply(&mut self) -> ExprType  {
        let left = self.term();
        if self.match_token(&[TokenType::STAR, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.term();
            return ExprType::Binary(Binary{
                left: Box::new(left),
                right: Box::new(right),
                operator,
            });
        }
        return left;
    }

    fn term(&mut self) -> ExprType {
        if self.match_token(&[TokenType::NUMBER]) {
            let t = self.previous();
            match &t.literal {
                Some(val) => {
                    return ExprType::Literal(Literal { value: val });
                }
                None => panic!("literal value is None but Token is number"),
            }
        }
        panic!("Unexpected token {:?}", self.peek());
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types.iter() {
            if self.check(t) {
                self.increment();
                return true;
            }
        }
        return false;
    }
    fn check(&mut self, t: &TokenType) -> bool {
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

    fn at_end(&mut self) -> bool {
        if let None = self.next_token() {
            return true;
        }
        return false;
    }
    fn next_token(&mut self) -> Option<&Token> {
        if self.n > self.tokens.len() {
            return Some(&self.tokens[self.n]);
        }
        return self.get_token();
    }

    fn get_token(&mut self) -> Option<&Token> {
        let token = self.lexer.next();
        match token.tt {
            TokenType::EOL => {
                return None;
            }
            _ => self.tokens.push(token),
        };
        return self.tokens.last();
    }

    fn previous(&self) -> &Token {
        return &self.tokens[self.n - 1];
    }

    fn peek(&mut self) -> Option<&Token> {
        return self.next_token();
    }

    fn increment(&mut self) {
        self.n += 1;
    }
}
