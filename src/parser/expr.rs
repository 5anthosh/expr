use crate::lexer::token::Token;
use crate::value::LiteralValue;

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
    fn visit_call(&mut self, expr: &Call) -> T;
    fn visit_function(&mut self, expr: &Function) -> T;
    fn visit_return(&mut self, expr: &Return) -> T;
}

#[derive(Clone, Debug)]
pub struct Binary {
    pub left: Box<ExprType>,
    pub right: Box<ExprType>,
    pub operator: Token,
}

#[derive(Clone, Debug)]
pub enum ExprType {
    Binary(Binary),
    Literal(Literal),
    Unary(Unary),
    Group(Group),
    ExpressionStmt(Expression),
    Print(Print),
    Variable(Variable),
    Var(Var),
    Assign(Assign),
    Block(Block),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    Call(Call),
    Function(Function),
    Return(Return),
}

impl Expr for Binary {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_binary_operation(&self);
    }
}

#[derive(Clone, Debug)]
pub struct Literal {
    pub value: LiteralValue,
}

impl Expr for Literal {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_literal(&self);
    }
}

#[derive(Clone, Debug)]
pub struct Unary {
    pub expression: Box<ExprType>,
    pub operator: Token,
}

impl Expr for Unary {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_unary(&self);
    }
}

#[derive(Clone, Debug)]
pub struct Group {
    pub expression: Box<ExprType>,
}

impl Expr for Group {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_group(&self);
    }
}

#[derive(Clone, Debug)]
pub struct Expression {
    pub expression: Box<ExprType>,
}

impl Expr for Expression {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_expression(&self);
    }
}

#[derive(Clone, Debug)]
pub struct Print {
    pub expression: Box<ExprType>,
}

impl Expr for Print {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_print(&self);
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
}

impl Expr for Variable {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_variable(&self);
    }
}

#[derive(Clone, Debug)]
pub struct Var {
    pub name: String,
    pub initializer: Option<Box<ExprType>>,
}

impl Expr for Var {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_var(&self);
    }
}

#[derive(Clone, Debug)]
pub struct Assign {
    pub name: String,
    pub initializer: Box<ExprType>,
}

impl Expr for Assign {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_assign(&self);
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    pub statements: Vec<Box<ExprType>>,
}

impl Expr for Block {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_block(&self);
    }
}

#[derive(Clone, Debug)]
pub struct IfStatement {
    pub condition: Box<ExprType>,
    pub then_branch: Box<ExprType>,
    pub else_branch: Option<Box<ExprType>>,
}

impl Expr for IfStatement {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_if_statement(&self);
    }
}

#[derive(Clone, Debug)]
pub struct WhileStatement {
    pub condition: Box<ExprType>,
    pub body: Box<ExprType>,
}

impl Expr for WhileStatement {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_while_statement(&self);
    }
}

#[derive(Clone, Debug)]
pub struct Call {
    pub callee: Box<ExprType>,
    pub arguments: Vec<Box<ExprType>>,
}

impl Expr for Call {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_call(&self);
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Block,
}

impl Expr for Function {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_function(&self);
    }
}

#[derive(Clone, Debug)]
pub struct Return {
    pub keyword: Token,
    pub value: Option<Box<ExprType>>,
}

impl Expr for Return {
    fn accept<V>(self, mut visitor: impl Visitor<V>) -> V {
        return visitor.visit_return(&self);
    }
}
