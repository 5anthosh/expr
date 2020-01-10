use crate::lexer::token::Token;
use crate::value::Value;

pub trait Expr<'a> {
    fn accept<V>(&'a mut self, visitor: impl Visitor<'a, V> + 'a) -> V;
}

pub trait Visitor<'a, T> {
    fn visit_binary_operation(&'a mut self, expr: &'a Binary<'a>) -> T;
    fn visit_literal(&'a mut self, expr: &'a Literal<'a>) -> T;
    fn visit_unary(&'a mut self, expr: &'a Unary<'a>) -> T;
    fn visit_group(&'a mut self, expr: &'a Group<'a>) -> T;
    fn visit_expression(&'a mut self, expr: &'a Expression<'a>) -> T;
    fn visit_print(&'a mut self, expr: &'a Print<'a>) -> T;
    fn visit_variable(&'a mut self, expr: &'a Variable<'a>) -> T;
    fn visit_var(&'a mut self, expr: &'a Var<'a>) -> T;
    fn visit_assign(&'a mut self, expr: &'a Assign<'a>) -> T;
    fn visit_block(&'a mut self, expr: &'a Block<'a>) -> T;
    fn visit_if_statement(&'a mut self, expr: &'a IfStatement<'a>) -> T;
    fn visit_while_statement(&'a mut self, expr: &'a WhileStatement<'a>) -> T;
    fn visit_call(&'a mut self, expr: &'a Call<'a>) -> T;
    fn visit_function(&'a mut self, expr: &'a Function<'a>) -> T;
}

pub struct Binary<'a> {
    pub left: Box<ExprType<'a>>,
    pub right: Box<ExprType<'a>>,
    pub operator: Token,
}

pub enum ExprType<'a> {
    Binary(Binary<'a>),
    Literal(Literal<'a>),
    Unary(Unary<'a>),
    Group(Group<'a>),
    ExpressionStmt(Expression<'a>),
    Print(Print<'a>),
    Variable(Variable<'a>),
    Var(Var<'a>),
    Assign(Assign<'a>),
    Block(Block<'a>),
    IfStatement(IfStatement<'a>),
    WhileStatement(WhileStatement<'a>),
    Call(Call<'a>),
    Function(Function<'a>),
}

impl<'a> Expr<'a> for Binary<'a> {
    fn accept<V>(&'a mut self, mut visitor: impl Visitor<'a, V> + 'a) -> V {
        return visitor.visit_binary_operation(self);
    }
}

pub struct Literal<'a> {
    pub value: Value<'a>,
}

impl<'a> Expr<'a> for Literal<'a> {
    fn accept<V>(&'a mut self, mut visitor: impl Visitor<'a, V> + 'a) -> V {
        return visitor.visit_literal(self);
    }
}

pub struct Unary<'a> {
    pub expression: Box<ExprType<'a>>,
    pub operator: Token,
}

impl<'a> Expr<'a> for Unary<'a> {
    fn accept<V>(&'a mut self, mut visitor: impl Visitor<'a, V> + 'a) -> V {
        return visitor.visit_unary(&self);
    }
}

pub struct Group<'a> {
    pub expression: Box<ExprType<'a>>,
}

impl<'a> Expr<'a> for Group<'a> {
    fn accept<V>(&'a mut self, mut visitor: impl Visitor<'a, V> + 'a) -> V {
        return visitor.visit_group(&self);
    }
}

pub struct Expression<'a> {
    pub expression: Box<ExprType<'a>>,
}

impl<'a> Expr<'a> for Expression<'a> {
    fn accept<V>(&'a mut self, mut visitor: impl Visitor<'a, V> + 'a) -> V {
        return visitor.visit_expression(&self);
    }
}

pub struct Print<'a>{
    pub expression: Box<ExprType<'a>>,
}

impl<'a> Expr<'a> for Print<'a> {
    fn accept<V>(&'a mut self, mut visitor: impl Visitor<'a, V> + 'a) -> V {
        return visitor.visit_print(&self);
    }
}

#[derive(Debug)]
pub struct Variable<'a> {
    pub name: &'a String,
}

impl<'a> Expr<'a> for Variable<'a> {
    fn accept<V>(&'a mut self, mut visitor: impl Visitor<'a, V> + 'a) -> V {
        return visitor.visit_variable(&self);
    }
}

pub struct Var<'a> {
    pub name: String,
    pub initializer: Option<Box<ExprType<'a>>>,
}

impl<'a> Expr<'a> for Var<'a> {
    fn accept<V>(&'a mut self, mut visitor: impl Visitor<'a, V> + 'a) -> V {
        return visitor.visit_var(&self);
    }
}

pub struct Assign<'a> {
    pub name: String,
    pub initializer: Box<ExprType<'a>>,
}

impl<'a> Expr<'a> for Assign<'a> {
    fn accept<V>(&'a mut self, mut visitor: impl Visitor<'a, V> + 'a) -> V {
        return visitor.visit_assign(&self);
    }
}

pub struct Block<'a> {
    pub statements: Vec<Box<ExprType<'a>>>,
}

impl<'a> Expr<'a> for Block<'a> {
    fn accept<V>(&'a mut self, mut visitor: impl Visitor<'a, V> + 'a) -> V {
        return visitor.visit_block(&self);
    }
}

pub struct IfStatement<'a> {
    pub condition: Box<ExprType<'a>>,
    pub then_branch: Box<ExprType<'a>>,
    pub else_branch: Option<Box<ExprType<'a>>>,
}

impl<'a> Expr<'a> for IfStatement<'a> {
    fn accept<V>(&'a mut self, mut visitor: impl Visitor<'a, V> + 'a) -> V {
        return visitor.visit_if_statement(&self);
    }
}

pub struct WhileStatement<'a> {
    pub condition: Box<ExprType<'a>>,
    pub body: Box<ExprType<'a>>,
}

impl<'a> Expr<'a> for WhileStatement<'a> {
    fn accept<V>(&'a mut self, mut visitor: impl Visitor<'a, V> + 'a) -> V {
        return visitor.visit_while_statement(&self);
    }
}

pub struct Call<'a> {
    pub callee: Box<ExprType<'a>>,
    pub arguments: Vec<Box<ExprType<'a>>>,
}

impl<'a> Expr<'a> for Call<'a> {
    fn accept<V>(&'a mut self, mut visitor: impl Visitor<'a ,V> + 'a) -> V {
        return visitor.visit_call(&self);
    }
}

pub struct Function<'a> {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Block<'a>,
}

impl<'a> Expr<'a> for Function<'a> {
    fn accept<V>(&'a mut self, mut visitor: impl Visitor<'a, V> + 'a) -> V {
        return visitor.visit_function(&self);
    }
}
