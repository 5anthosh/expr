use crate::lexer::token::Token;
use crate::value::Value;

pub trait Expr {
    fn accept<V>(self, visitor: impl Visitor<V>) -> V;
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

pub trait Visitor<T> {
    fn visit_binary_operation(&mut self, expr: Binary) -> T;
    fn visit_literal(&mut self, expr: Literal) -> T;
}
