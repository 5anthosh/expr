use crate::error::ExprError;
use crate::lexer::token::TokenType;
use crate::parser::{Binary, ExprType, Expression, Group, Literal, Parser, Print, Unary, Visitor};
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

    pub fn eval(&mut self) -> Result<Value, ExprError> {
        let mut parser = Parser::new(self.source.clone());
        let ast = parser.parse()?;
        // println!("{:?}", ast);
        self.accept(ast)
    }

    fn accept(&mut self, expr: ExprType) -> Result<Value, ExprError> {
        match expr {
            ExprType::Binary(bin) => self.visit_binary_operation(bin),
            ExprType::Literal(lit) => self.visit_literal(lit),
            ExprType::Unary(unary) => self.visit_unary(unary),
            ExprType::Group(group) => self.visit_group(group),
            ExprType::Print(print) => self.visit_print(print),
            ExprType::ExpressionStmt(expression) => self.visit_expression(expression),
        }
    }

    fn check_numbers(left: Value, right: Value) -> Result<(f64, f64), ExprError> {
        let left_value = match left {
            Value::Float(left_val) => left_val,
            _ => {
                return Err(ExprError::RunTimeMessage(String::from(
                    "Expecting float in left side of operation",
                )));
            }
        };
        let right_value = match right {
            Value::Float(right_value) => right_value,
            _ => {
                return Err(ExprError::RunTimeMessage(String::from(
                    "Expecting number in right side of operation",
                )));
            }
        };
        return Ok((left_value, right_value));
    }

    fn check_number(value: Value) -> Result<f64, ExprError> {
        match value {
            Value::Float(value) => Ok(value),
            _ => Err(ExprError::RunTimeMessage(String::from(
                "Expecting number in unary operation",
            ))),
        }
    }

    fn is_trusty(obj: Value) -> bool {
        match obj {
            Value::Nil => false,
            Value::Boolean(value) => value,
            _ => true,
        }
    }
}

impl<'a> Visitor<Result<Value, ExprError>> for Evaluator {
    fn visit_binary_operation(&mut self, expr: Binary) -> Result<Value, ExprError> {
        let left = self.accept(*expr.left)?;
        let right = self.accept(*expr.right)?;
        let operation = expr.operator;
        match operation.tt {
            TokenType::Plus => match left {
                Value::String(value) => match right {
                    Value::String(value2) => Ok(Value::String(value + &value2)),
                    Value::Float(value2) => Ok(Value::String(format!("{}{}", value, value2))),
                    _ => Err(ExprError::RunTimeMessage(String::from(
                        "Operators must be  strings or numbers for '+' ",
                    ))),
                },
                Value::Float(value) => match right {
                    Value::String(value2) => {
                        return Ok(Value::String(format!("{}{}", value, value2)))
                    }
                    Value::Float(value2) => Ok(Value::Float(value + value2)),
                    _ => Err(ExprError::RunTimeMessage(String::from(
                        "Operators must be  strings or numbers for '+' ",
                    ))),
                },
                _ => Err(ExprError::RunTimeMessage(String::from(
                    "Operators must be  strings or numbers for '+' ",
                ))),
            },
            TokenType::Minus => {
                let (left_value, right_value) = Evaluator::check_numbers(left, right)?;
                Ok(Value::Float(left_value - right_value))
            }
            TokenType::Slash => {
                let (left_value, right_value) = Evaluator::check_numbers(left, right)?;
                Ok(Value::Float(left_value / right_value))
            }
            TokenType::Star => {
                let (left_value, right_value) = Evaluator::check_numbers(left, right)?;
                Ok(Value::Float(left_value * right_value))
            }
            TokenType::Greater => {
                let (left_value, right_value) = Evaluator::check_numbers(left, right)?;
                Ok(Value::Boolean(left_value > right_value))
            }
            TokenType::GreaterEqual => {
                let (left_value, right_value) = Evaluator::check_numbers(left, right)?;
                Ok(Value::Boolean(left_value >= right_value))
            }
            TokenType::Lesser => {
                let (left_value, right_value) = Evaluator::check_numbers(left, right)?;
                Ok(Value::Boolean(left_value < right_value))
            }
            TokenType::LesserEqual => {
                let (left_value, right_value) = Evaluator::check_numbers(left, right)?;
                Ok(Value::Boolean(left_value <= right_value))
            }
            TokenType::EqualEqual => Ok(Value::Boolean(left.equals(&right))),
            TokenType::BangEqual => Ok(Value::Boolean(!left.equals(&right))),
            _ => {
                return Err(ExprError::RunTimeMessage(String::from(
                    "Unsupported binary operation",
                )));
            }
        }
    }

    fn visit_literal(&mut self, expr: Literal) -> Result<Value, ExprError> {
        return Ok(expr.value);
    }

    fn visit_unary(&mut self, expr: Unary) -> Result<Value, ExprError> {
        let value = self.accept(*expr.expression)?;

        match expr.operator.tt {
            TokenType::Plus => {
                let value = Evaluator::check_number(value)?;
                Ok(Value::Float(value))
            }
            TokenType::Minus => {
                let value = Evaluator::check_number(value)?;
                Ok(Value::Float(-value))
            }
            TokenType::Bang => {
                let value = Evaluator::is_trusty(value);
                Ok(Value::Boolean(!value))
            }
            _ => {
                return Err(ExprError::RunTimeMessage(String::from(
                    "Unsupported unary operation",
                )));
            }
        }
    }

    fn visit_group(&mut self, expr: Group) -> Result<Value, ExprError> {
        self.accept(*expr.expression)
    }

    fn visit_expression(&mut self, expr: Expression) -> Result<Value, ExprError> {
        let _ = self.accept(*expr.expression)?;
        return Ok(Value::Nil);
    }

    fn visit_print(&mut self, expr: Print) -> Result<Value, ExprError> {
        let value = self.accept(*expr.expression)?;
        println!("{}", value.to_string());
        return Ok(Value::Nil);
    }
}
