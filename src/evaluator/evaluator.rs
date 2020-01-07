use crate::lexer::token::TokenType;
use crate::parser::{Binary, ExprType, Group, Literal, Parser, Unary, Visitor};
use crate::value::Value;

pub struct Evaluator {
    source: String,
}

impl Evaluator {
    pub fn new(source: &str) -> Evaluator {
        Evaluator {
            source: String::from(source),
        }
    }

    pub fn eval(&mut self) -> Value {
        
        let parser = Parser::new(self.source.clone());
        let ast = parser.parse();
        self.accept(ast)
    }

    fn accept(&mut self, expr: ExprType) -> Value {
        match expr {
            ExprType::Binary(bin) => self.visit_binary_operation(bin),
            ExprType::Literal(lit) => self.visit_literal(lit),
            ExprType::Unary(unary) => self.visit_unary(unary),
            ExprType::Group(group) => self.visit_group(group),
        }
    }
}

impl Visitor<Value> for Evaluator {
    fn visit_binary_operation(&mut self, expr: Binary) -> Value {
        let left = self.accept(*expr.left);
        let right = self.accept(*expr.right);
        let operation = expr.operator;
        let left_value = match left {
            Value::Float(left_val) => left_val,
            _ => panic!("Expecting float in left side of operation"),
        };
        let right_value = match right {
            Value::Float(right_value) => right_value,
            _ => panic!("Expecting number in right side of operation"),
        };
        match operation.tt {
            TokenType::PLUS => Value::Float(left_value + right_value),
            TokenType::MINUS => Value::Float(left_value - right_value),
            TokenType::SLASH => Value::Float(left_value / right_value),
            TokenType::STAR => Value::Float(left_value * right_value),
            _ => panic!("Unsupported binary operation"),
        }
    }

    fn visit_literal(&mut self, expr: Literal) -> Value {
        return expr.value;
    }

    fn visit_unary(&mut self, expr: Unary) -> Value {
        let value = self.accept(*expr.expression);
        let value = match value {
            Value::Float(value) => value,
            _ => panic!("Expecting number in unary operation"),
        };
        match expr.operator.tt {
            TokenType::PLUS => Value::Float(value),
            TokenType::MINUS => Value::Float(-value),
            _ => panic!("Unsupported unary operation"),
        }
    }

    fn visit_group(&mut self, expr: Group) -> Value {
        self.accept(*expr.expression)
    }
}
