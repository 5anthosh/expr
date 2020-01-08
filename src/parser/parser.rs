use crate::error::ExprError;
use crate::lexer::token::TokenType::{
    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Identifier, Lesser, LesserEqual,
    Minus, Plus, Print, SemiColon, Slash, Star,
};
use crate::lexer::token::{Token, TokenType};
use crate::lexer::Lexer;
use crate::parser::expr::{Binary, ExprType, Group, Literal};
use crate::parser::{self, Assign, Expression, Unary, Var, Variable};
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

    pub fn parse(&mut self) -> Result<Vec<ExprType>, ExprError> {
        let lexer = Lexer::new(&self.source);
        for token in lexer {
            self.tokens.push(token?);
        }

        let mut statements = Vec::new();
        while !self.at_end() {
            statements.push(self.statement()?);
        }
        // println!("{:?}", self.tokens);
        Ok(statements)
    }

    fn statement(&self) -> Result<ExprType, ExprError> {
        if self.match_token(&[Print]) {
            return self.print_statement();
        }
        if self.match_token(&[TokenType::Var]) {
            return self.var_statement();
        }
        self.expression_statement()
    }

    fn var_statement(&self) -> Result<ExprType, ExprError> {
        if self.match_token(&[Identifier]) {
            let t = self.previous();
            if self.match_token(&[Equal]) {
                let value = self.expression()?;
                if self.match_token(&[SemiColon]) {
                    return Ok(ExprType::Var(Var {
                        name: t.lexeme.clone(),
                        initializer: Some(Box::new(value)),
                    }));
                }
                return Err(ExprError::ParserErrorMessage(String::from(
                    "Expect ';' after variable declaration",
                )));
            }
            if self.match_token(&[SemiColon]) {
                return Ok(ExprType::Var(Var {
                    name: t.lexeme.clone(),
                    initializer: None,
                }));
            }
            return Err(ExprError::ParserErrorMessage(String::from(
                "Expect ';' after variable declaration",
            )));
        }
        Err(ExprError::ParserErrorMessage(String::from(
            "Expecting variable name",
        )))
    }

    fn print_statement(&self) -> Result<ExprType, ExprError> {
        let expr = self.expression()?;
        if self.match_token(&[SemiColon]) {
            return Ok(ExprType::Print(parser::Print {
                expression: Box::new(expr),
            }));
        }
        return Err(ExprError::ParserErrorMessage(String::from(
            "Expect ';' after value",
        )));
    }
    fn expression_statement(&self) -> Result<ExprType, ExprError> {
        let expr = self.expression()?;
        if self.match_token(&[SemiColon]) {
            return Ok(ExprType::ExpressionStmt(Expression {
                expression: Box::new(expr),
            }));
        }
        return Err(ExprError::ParserErrorMessage(String::from(
            "Expect ';' after expression",
        )));
    }
    fn expression(&self) -> Result<ExprType, ExprError> {
        self.assignment()
    }

    fn assignment(&self) -> Result<ExprType, ExprError> {
        let left = self.equality()?;
        if self.match_token(&[Equal]) {
            return match &left {
                ExprType::Variable(var) => {
                    let name = var.name.clone();
                    let value = self.assignment()?;
                    Ok(ExprType::Assign(Assign {
                        name,
                        initializer: Box::new(value),
                    }))
                }
                _ => Err(ExprError::ParserErrorMessage(String::from(
                    "Expecting variable in left side of assignment",
                ))),
            };
        }
        return Ok(left);
    }

    fn equality(&self) -> Result<ExprType, ExprError> {
        let left = self.comparator()?;
        while self.match_token(&[EqualEqual, BangEqual]) {
            let operator = self.previous();
            let right = self.comparator()?;
            return Ok(ExprType::Binary(Binary {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            }));
        }
        return Ok(left);
    }

    fn comparator(&self) -> Result<ExprType, ExprError> {
        let left = self.addition()?;
        while self.match_token(&[Greater, GreaterEqual, Lesser, LesserEqual]) {
            let operator = self.previous();
            let right = self.addition()?;
            return Ok(ExprType::Binary(Binary {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            }));
        }
        return Ok(left);
    }

    fn addition(&self) -> Result<ExprType, ExprError> {
        let left = self.multiply()?;
        while self.match_token(&[Plus, Minus]) {
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
        while self.match_token(&[Star, Slash]) {
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
        while self.match_token(&[Plus, Minus, Bang]) {
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
        if self.match_token(&[TokenType::Number]) {
            let t = self.previous();
            let number: f64 = t.lexeme.parse().unwrap();
            return Ok(ExprType::Literal(Literal {
                value: Value::Float(number),
            }));
        }

        if self.match_token(&[TokenType::String]) {
            let t = self.previous();
            let string_value = &t.lexeme[1..t.lexeme.len() - 1];
            return Ok(ExprType::Literal(Literal {
                value: Value::String(String::from(string_value)),
            }));
        }

        if self.match_token(&[TokenType::True]) {
            return Ok(ExprType::Literal(Literal {
                value: Value::Boolean(true),
            }));
        }

        if self.match_token(&[TokenType::False]) {
            return Ok(ExprType::Literal(Literal {
                value: Value::Boolean(false),
            }));
        }

        if self.match_token(&[TokenType::Nil]) {
            return Ok(ExprType::Literal(Literal { value: Value::Nil }));
        }

        if self.match_token(&[TokenType::Identifier]) {
            let t = self.previous();
            return Ok(ExprType::Variable(Variable {
                name: t.lexeme.clone(),
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
