use std::cell::Cell;

use crate::error::TullyError;
use crate::lexer::token::TokenType::{
    Bang, BangEqual, CloseBrace, CloseParen, Else, Equal, EqualEqual, Greater, GreaterEqual,
    Identifier, Lesser, LesserEqual, Minus, OpenBrace, OpenParen, Plus, Print, SemiColon, Slash,
    Star, COMMA,
};
use crate::lexer::token::{Token, TokenType};
use crate::lexer::Lexer;
use crate::parser::expr::{Binary, Call, ExprType, Function, Group, Literal, Return};
use crate::parser::{
    self, Assign, Block, Expression, IfStatement, Unary, Var, Variable, WhileStatement,
};
use crate::value::LiteralValue;

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

    pub fn parse(&mut self) -> Result<Vec<ExprType>, TullyError> {
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

    fn statement(&self) -> Result<ExprType, TullyError> {
        if self.match_token(&[Print]) {
            return self.print_statement();
        }
        if self.match_token(&[TokenType::Var]) {
            return self.var_statement();
        }
        if self.match_token(&[TokenType::Fun]) {
            return self.function(String::from("function"));
        }
        if self.match_token(&[TokenType::OpenBrace]) {
            return self.block();
        }
        if self.match_token(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.match_token(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.match_token(&[TokenType::For]) {
            return self.for_statement();
        }
        if self.match_token(&[TokenType::Return]) {
            return self.return_statement();
        }
        self.expression_statement()
    }

    fn return_statement(&self) -> Result<ExprType, TullyError> {
        let keyword = self.previous().clone();
        let mut value = None;
        if !self.check(&SemiColon) {
            value = Some(Box::new(self.expression()?));
        }
        self.expect_token_or(SemiColon, "Expecting ';' after return value")?;
        return Ok(ExprType::Return(Return { keyword, value }));
    }

    fn var_statement(&self) -> Result<ExprType, TullyError> {
        self.expect_token_or(Identifier, "Expecting variable name")?;
        let t = self.previous();
        if self.match_token(&[Equal]) {
            let value = self.expression()?;
            self.expect_token_or(SemiColon, "Expect ';' after variable declaration")?;
            return Ok(ExprType::Var(Var {
                name: t.lexeme.clone(),
                initializer: Some(Box::new(value)),
            }));
        }
        self.expect_token_or(SemiColon, "Expect ';' after variable declaration")?;
        return Ok(ExprType::Var(Var {
            name: t.lexeme.clone(),
            initializer: None,
        }));
    }

    fn print_statement(&self) -> Result<ExprType, TullyError> {
        let expr = self.expression()?;
        self.expect_token_or(SemiColon, "Expect ';' after value ")?;
        return Ok(ExprType::Print(parser::Print {
            expression: Box::new(expr),
        }));
    }

    fn if_statement(&self) -> Result<ExprType, TullyError> {
        self.expect_token_or(OpenParen, "Expecting '(' after If")?;
        let condition = Box::new(self.expression()?);
        self.expect_token_or(CloseParen, "Expecting ')' after condition")?;
        let then_branch = Box::new(self.statement()?);
        let mut else_branch = None;
        if self.match_token(&[Else]) {
            else_branch = Some(Box::new(self.statement()?));
        }
        return Ok(ExprType::IfStatement(IfStatement {
            condition,
            then_branch,
            else_branch,
        }));
    }

    fn while_statement(&self) -> Result<ExprType, TullyError> {
        self.expect_token_or(OpenParen, "Expecting '(' after While")?;
        let condition = Box::new(self.expression()?);
        self.expect_token_or(CloseParen, "Expecting ')' after condition")?;
        let body = Box::new(self.statement()?);
        return Ok(ExprType::WhileStatement(WhileStatement { condition, body }));
    }

    fn for_statement(&self) -> Result<ExprType, TullyError> {
        self.expect_token_or(OpenParen, "Expecting '(' after for")?;
        let mut _initializer = None;
        if self.match_token(&[SemiColon]) {
            _initializer = None
        } else if self.match_token(&[TokenType::Var]) {
            _initializer = Some(self.var_statement()?);
        } else {
            _initializer = Some(self.expression_statement()?);
        }
        let mut condition = None;
        if !self.check(&SemiColon) {
            condition = Some(self.expression()?);
        }
        self.expect_token_or(SemiColon, "Expecting ';' after condition")?;
        let mut increment = None;
        if !self.check(&CloseParen) {
            increment = Some(self.expression()?);
        }
        self.expect_token_or(CloseParen, "Expecting ')' after clauses")?;
        let mut body = self.statement()?;
        if let Some(increment) = increment {
            body = ExprType::Block(Block {
                statements: vec![
                    Box::new(body),
                    Box::new(ExprType::ExpressionStmt(Expression {
                        expression: Box::new(increment),
                    })),
                ],
            })
        }
        let condition = Box::new(match condition {
            Some(value) => value,
            None => ExprType::Literal(Literal {
                value: LiteralValue::Boolean(true),
            }),
        });
        body = ExprType::WhileStatement(WhileStatement {
            condition,
            body: Box::new(body),
        });
        if let Some(initializer) = _initializer {
            body = ExprType::Block(Block {
                statements: vec![Box::new(initializer), Box::new(body)],
            })
        }
        return Ok(body);
    }

    fn function(&self, kind: String) -> Result<ExprType, TullyError> {
        self.expect_token_or(Identifier, &format!("Expect {} name", kind))?;
        let name = self.previous().clone();
        self.expect_token_or(OpenParen, &format!("Expect '(' after {} name", kind))?;
        let mut params = Vec::new();
        if !self.check(&CloseParen) {
            loop {
                if params.len() >= 255 {
                    return Err(Parser::error(
                        self.peek(),
                        "Cannot have more than 255 params",
                    ));
                }
                self.expect_token_or(Identifier, "Expect parameter name")?;
                params.push(self.previous().clone());
                if self.match_token(&[COMMA]) {
                    continue;
                }
                break;
            }
        }
        self.expect_token_or(CloseParen, "Expect ')' after parameters")?;
        self.expect_token_or(OpenBrace, "Expect '{' before {} body")?;
        let body = Box::new(self.block()?);
        match *body {
            ExprType::Block(value) => Ok(ExprType::Function(Function {
                name,
                params,
                body: value,
            })),
            _ => Err(Parser::error(self.peek(), "Expecting block")),
        }
    }

    fn block(&self) -> Result<ExprType, TullyError> {
        let mut statements = Vec::new();
        while !self.check(&CloseBrace) && !self.at_end() {
            statements.push(Box::new(self.statement()?))
        }
        self.expect_token_or(CloseBrace, "Expecting } after block")?;
        Ok(ExprType::Block(Block { statements }))
    }

    fn expression_statement(&self) -> Result<ExprType, TullyError> {
        let expr = self.expression()?;
        self.expect_token_or(SemiColon, "Expect ';' after expression")?;
        Ok(ExprType::ExpressionStmt(Expression {
            expression: Box::new(expr),
        }))
    }

    fn expression(&self) -> Result<ExprType, TullyError> {
        self.assignment()
    }

    fn assignment(&self) -> Result<ExprType, TullyError> {
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
                _ => Err(Parser::error(
                    self.peek(),
                    "Expecting variable in left side of assignment",
                )),
            };
        }
        return Ok(left);
    }

    fn equality(&self) -> Result<ExprType, TullyError> {
        let mut expr = self.comparator()?;
        while self.match_token(&[EqualEqual, BangEqual]) {
            let operator = self.previous().clone();
            let right = self.comparator()?;
            expr = ExprType::Binary(Binary {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            });
        }
        return Ok(expr);
    }

    fn comparator(&self) -> Result<ExprType, TullyError> {
        let mut expr = self.addition()?;
        while self.match_token(&[Greater, GreaterEqual, Lesser, LesserEqual]) {
            let operator = self.previous().clone();
            let right = self.addition()?;
            expr = ExprType::Binary(Binary {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            });
        }
        return Ok(expr);
    }

    fn addition(&self) -> Result<ExprType, TullyError> {
        let mut expr = self.multiply()?;
        while self.match_token(&[Plus, Minus]) {
            let operator = self.previous().clone();
            let right = self.multiply()?;
            expr = ExprType::Binary(Binary {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            });
        }
        return Ok(expr);
    }

    fn multiply(&self) -> Result<ExprType, TullyError> {
        let mut expr = self.unary()?;
        while self.match_token(&[Star, Slash]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = ExprType::Binary(Binary {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            });
        }
        return Ok(expr);
    }

    fn unary(&self) -> Result<ExprType, TullyError> {
        while self.match_token(&[Plus, Minus, Bang]) {
            let operator = self.previous().clone();
            let expression = self.unary()?;
            return Ok(ExprType::Unary(Unary {
                expression: Box::new(expression),
                operator,
            }));
        }
        self.call()
    }

    fn call(&self) -> Result<ExprType, TullyError> {
        let mut expr = self.term()?;
        loop {
            if self.match_token(&[OpenParen]) {
                let open_paren = self.previous().clone();
                let mut arguments = Vec::new();
                if !self.check(&CloseParen) {
                    loop {
                        if arguments.len() >= 255 {
                            return Err(Parser::error(
                                self.peek(),
                                "Can not have more than 255 arguments",
                            ));
                        }
                        arguments.push(Box::new(self.expression()?));
                        if self.match_token(&[COMMA]) {
                            continue;
                        }
                        break;
                    }
                }
                self.expect_token_or(CloseParen, "Expecting ')' after arguments")?;
                expr = ExprType::Call(Call {
                    paren: open_paren,
                    callee: Box::new(expr),
                    arguments,
                })
            } else {
                break;
            }
        }
        return Ok(expr);
    }

    fn term(&self) -> Result<ExprType, TullyError> {
        if self.match_token(&[TokenType::Number]) {
            let t = self.previous();
            let number: f64 = t.lexeme.parse().unwrap();
            return Ok(ExprType::Literal(Literal {
                value: LiteralValue::Float(number),
            }));
        }

        if self.match_token(&[TokenType::String]) {
            let t = self.previous();
            let string_value = &t.lexeme[1..t.lexeme.len() - 1];
            return Ok(ExprType::Literal(Literal {
                value: LiteralValue::String(String::from(string_value)),
            }));
        }

        if self.match_token(&[TokenType::True]) {
            return Ok(ExprType::Literal(Literal {
                value: LiteralValue::Boolean(true),
            }));
        }

        if self.match_token(&[TokenType::False]) {
            return Ok(ExprType::Literal(Literal {
                value: LiteralValue::Boolean(false),
            }));
        }

        if self.match_token(&[TokenType::Nil]) {
            return Ok(ExprType::Literal(Literal {
                value: LiteralValue::Nil,
            }));
        }

        if self.match_token(&[TokenType::Identifier]) {
            let t = self.previous();
            return Ok(ExprType::Variable(Variable { name: t.clone() }));
        }

        if self.match_token(&[TokenType::OpenParen]) {
            let group = ExprType::Group(Group {
                expression: Box::new(self.expression()?),
            });
            self.expect_token_or(TokenType::CloseParen, "Expecting ')'")?;
            return Ok(group);
        }

        Err(Parser::error(self.peek(), "Unexpected token {:?}"))
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

    fn expect_token_or(&self, tt: TokenType, message: &str) -> Result<&Token, TullyError> {
        if !self.match_token(&[tt]) {
            return Err(Parser::error(Some(self.previous()), message));
        }
        Ok(self.previous())
    }

    fn error(tt: Option<&Token>, message: &str) -> TullyError {
        TullyError::parser_error_message(tt, message)
    }
}
