use crate::error::ExprError;
use crate::lexer::token::{Token, TokenType};
use crate::lexer::Lexer;
use crate::parser::expr::{Binary, ExprType, Group, Literal};
use crate::parser::Unary;
use crate::value::Value;
use std::cell::Cell;

pub struct Parser {
    pub source: String,
    n: Cell<usize>,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(source: String) -> Parser {
        Parser {
            source,
            n: Cell::new(0),
            tokens: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<ExprType, ExprError> {
        let lexer = Lexer::new(&self.source);
        for token in lexer {
            self.tokens.push(token?);
        }
        self.expression()
    }

    fn expression(&self) -> Result<ExprType, ExprError> {
        self.addition()
    }

    fn addition(&self) -> Result<ExprType, ExprError> {
        let left = self.multiply()?;
        while self.match_token(&[TokenType::PLUS, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.multiply()?;
            return Ok(ExprType::Binary(Binary {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            }));
        }
        return Ok(left);
    }
    fn multiply(&self) -> Result<ExprType, ExprError> {
        let left = self.unary()?;
        while self.match_token(&[TokenType::STAR, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(ExprType::Binary(Binary {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            }));
        }
        return Ok(left);
    }

    fn unary(&self) -> Result<ExprType, ExprError> {
        while self.match_token(&[TokenType::PLUS, TokenType::MINUS]) {
            let operator = self.previous();
            let expression = self.unary()?;
            return Ok(ExprType::Unary(Unary {
                expression: Box::new(expression),
                operator,
            }));
        }
        self.term()
    }

    fn term(&self) -> Result<ExprType, ExprError> {
        if self.match_token(&[TokenType::NUMBER]) {
            let t = self.previous();
            let number: f64 = t.lexeme.parse().unwrap();
            return Ok(ExprType::Literal(Literal {
                value: Value::Float(number),
            }));
        }
        if self.match_token(&[TokenType::OpenParen]) {
            let group = ExprType::Group(Group {
                expression: Box::new(self.expression()?),
            });
            if self.match_token(&[TokenType::CloseParen]) {
                return Ok(group);
            }
            return Err(ExprError::ParserErrorMessage(String::from("Expecting ')'")));
        }
        Err(ExprError::ParserErrorMessage(String::from(format!(
            "Unexpected token {:?}",
            self.peek()
        ))))
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
            Some(t1) => std::mem::discriminant(&t1.tt) == std::mem::discriminant(t),
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
