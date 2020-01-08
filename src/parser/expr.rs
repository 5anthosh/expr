use crate::lexer::token::Token;
use crate::value::Value;

pub trait Expr {
    fn accept<V>(self, visitor: impl Visitor<V>) -> V;
}

pub trait Visitor<T> {
    fn visit_binary_operation(&mut self, expr: Binary) -> T;
    fn visit_literal(&mut self, expr: Literal) -> T;
    fn visit_unary(&mut self, expr: Unary) -> T;
    fn visit_group(&mut self, expr: Group) -> T;
    fn visit_expression(&mut self, expr: Expression) -> T;
    fn visit_print(&mut self, expr: Print) -> T;
}

#[derive(Debug)]
pub struct Binary<'a> {
    pub left: Box<ExprType<'a>>,
    pub right: Box<ExprType<'a>>,
    pub operator: &'a Token,
}

#[derive(Debug)]
pub enum ExprType<'a> {
    Binary(Binary<'a>),
    Literal(Literal),
    Unary(Unary<'a>),
    Group(Group<'a>),
    ExpressionStmt(Expression<'a>),
    Print(Print<'a>),
}

impl<'a> Expr for Binary<'a> {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_binary_operation(self);
    }
}
#[derive(Debug)]
pub struct Literal {
    pub value: Value,
}

impl<'a> Expr for Literal {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_literal(self);
    }
}

#[derive(Debug)]
pub struct Unary<'a> {
    pub expression: Box<ExprType<'a>>,
    pub operator: &'a Token,
}

impl<'a> Expr for Unary<'a> {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_unary(self);
    }
}

#[derive(Debug)]
pub struct Group<'a> {
    pub expression: Box<ExprType<'a>>,
}

impl<'a> Expr for Group<'a> {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_group(self);
    }
}

#[derive(Debug)]
pub struct Expression<'a> {
    pub expression: Box<ExprType<'a>>,
}

impl<'a> Expr for Expression<'a> {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_expression(self);
    }
}

#[derive(Debug)]
pub struct Print<'a> {
    pub expression: Box<ExprType<'a>>,
}

impl<'a> Expr for Print<'a> {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_print(self);
    }
}
