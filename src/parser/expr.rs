use crate::lexer::token::Token;
use crate::value::Value;

pub trait Expr {
    fn accept<V>(self, visitor: impl Visitor<V>) -> V;
}

pub trait Visitor<T> {
    fn visit_binary_operation(&mut self, expr: &Binary) -> T;
    fn visit_literal(&mut self, expr: &Literal) -> T;
    fn visit_unary(&mut self, expr: &Unary) -> T;
    fn visit_group(&mut self, expr: &Group) -> T;
    fn visit_expression(&mut self, expr: &Expression) -> T;
    fn visit_print(&mut self, expr: &Print) -> T;
    fn visit_variable(&mut self, expr: &Variable) -> T;
    fn visit_var(&mut self, expr: &Var) -> T;
    fn visit_assign(&mut self, expr: &Assign) -> T;
    fn visit_block(&mut self, expr: &Block) -> T;
    fn visit_if_statement(&mut self, expr: &IfStatement) -> T;
    fn visit_while_statement(&mut self, expr: &WhileStatement) -> T;
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
    Variable(Variable),
    Var(Var<'a>),
    Assign(Assign<'a>),
    Block(Block<'a>),
    IfStatement(IfStatement<'a>),
    WhileStatement(WhileStatement<'a>),
}

impl<'a> Expr for Binary<'a> {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_binary_operation(&self);
    }
}
#[derive(Debug)]
pub struct Literal {
    pub value: Value,
}

impl<'a> Expr for Literal {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_literal(&self);
    }
}

#[derive(Debug)]
pub struct Unary<'a> {
    pub expression: Box<ExprType<'a>>,
    pub operator: &'a Token,
}

impl<'a> Expr for Unary<'a> {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_unary(&self);
    }
}

#[derive(Debug)]
pub struct Group<'a> {
    pub expression: Box<ExprType<'a>>,
}

impl<'a> Expr for Group<'a> {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_group(&self);
    }
}

#[derive(Debug)]
pub struct Expression<'a> {
    pub expression: Box<ExprType<'a>>,
}

impl<'a> Expr for Expression<'a> {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_expression(&self);
    }
}

#[derive(Debug)]
pub struct Print<'a> {
    pub expression: Box<ExprType<'a>>,
}

impl<'a> Expr for Print<'a> {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_print(&self);
    }
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
}

impl Expr for Variable {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_variable(&self);
    }
}

#[derive(Debug)]
pub struct Var<'a> {
    pub name: String,
    pub initializer: Option<Box<ExprType<'a>>>,
}

impl<'a> Expr for Var<'a> {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_var(&self);
    }
}

#[derive(Debug)]
pub struct Assign<'a> {
    pub name: String,
    pub initializer: Box<ExprType<'a>>,
}

impl<'a> Expr for Assign<'a> {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_assign(&self);
    }
}

#[derive(Debug)]
pub struct Block<'a> {
    pub statements: Vec<Box<ExprType<'a>>>,
}

impl<'a> Expr for Block<'a> {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_block(&self);
    }
}

#[derive(Debug)]
pub struct IfStatement<'a> {
    pub condition: Box<ExprType<'a>>,
    pub then_branch: Box<ExprType<'a>>,
    pub else_branch: Option<Box<ExprType<'a>>>,
}

impl<'a> Expr for IfStatement<'a> {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_if_statement(&self);
    }
}

#[derive(Debug)]
pub struct WhileStatement<'a> {
    pub condition: Box<ExprType<'a>>,
    pub body: Box<ExprType<'a>>,
}

impl<'a> Expr for WhileStatement<'a> {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_while_statement(&self);
    }
}
