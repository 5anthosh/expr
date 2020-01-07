use crate::lexer::token::Token;
use crate::value::Value;

pub trait Expr {
    fn accept<V>(self, visitor: impl Visitor<V>) -> V;
}

pub struct Binary<'a> {
    left: Box<ExprType<'a>>,
    right: Box<ExprType<'a>>,
    operator: &'a Token,
}

pub enum ExprType<'a> {
    Binary(Binary<'a>),
    Literal(Literal<'a>),
}
impl<'a> Expr for Binary<'a> {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_binary_operation(self);
    }
}

pub struct Literal<'a> {
    value: &'a Value,
}

impl<'a> Expr for Literal<'a> {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_literal(self);
    }
}

trait Visitor<T> {
    fn visit_binary_operation(&mut self, expr: impl Expr) -> T;
    fn visit_literal(&mut self, expr: impl Expr) -> T;
}
