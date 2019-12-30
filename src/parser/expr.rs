use crate::lexer::token::Token;
use crate::value::Value;

pub trait Expr {
    fn accept<V>(self, visitor: impl Visitor<V>) -> V;
}

pub struct Binary<'a, T: Expr> {
    left: T,
    right: T,
    operator: &'a Token,
}

impl<'a, T: Expr> Expr for Binary<'a, T> {
    fn accept<V>(self, visitor: impl Visitor<V>) -> V {
        return visitor.visit_binary_operation(self);
    }
}

pub struct Literal<'a> {
    value: &'a Value,
}

impl<'a> Expr for Literal<'a> {
    fn accept<V>(self, visitor: impl Visitor<V>) -> V {
        return visitor.visit_literal(self);
    }
}

trait Visitor<T> {
    fn visit_binary_operation(&self, expr: impl Expr) -> T;
    fn visit_literal(&self, expr: impl Expr) -> T;
}
