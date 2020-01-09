use crate::environment::Environment;
use crate::error::ExprError;
use crate::lexer::token::TokenType;
use crate::parser::{
    Assign, Binary, Block, Call, ExprType, Expression, Group, IfStatement, Literal, Parser, Print,
    Unary, Var, Variable, Visitor, WhileStatement,
};
use crate::value::{Constants, Value};
use std::borrow::Borrow;
use std::rc::Rc;

pub struct Evaluator {
    pub constants: Constants,
    globals: Environment,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        let mut env = Environment::new();
        env.set_default_functions();
        Evaluator {
            constants: Constants::new(),
            globals: env,
        }
    }

    pub fn eval(&mut self, source: String) {
        let mut parser = Parser::new(String::from(source.trim()));
        let ast = parser.parse();
        match ast {
            Ok(statements) => {
                for statement in statements {
                    let value = self.execute(&statement);
                    let value = match value {
                        Ok(value) => value,
                        Err(e) => {
                            eprintln!("{}", e.to_string());
                            return;
                        }
                    };
                    match value.borrow() {
                        Value::Float(val) => println!("{}", val),
                        Value::String(string_value) => println!("{}", string_value),
                        Value::Boolean(boolean_value) => println!("{}", boolean_value),
                        Value::Nil => (),
                        Value::Function(_) => println!("Function"),
                    };
                }
            }
            Err(e) => eprintln!("{}", e.to_string()),
        }
    }

    fn execute(&mut self, ast: &ExprType) -> Result<Rc<Value>, ExprError> {
        self.accept(ast)
    }

    fn accept(&mut self, expr: &ExprType) -> Result<Rc<Value>, ExprError> {
        match expr {
            ExprType::Binary(bin) => self.visit_binary_operation(bin),
            ExprType::Literal(lit) => self.visit_literal(lit),
            ExprType::Unary(unary) => self.visit_unary(unary),
            ExprType::Group(group) => self.visit_group(group),
            ExprType::Print(print) => self.visit_print(print),
            ExprType::ExpressionStmt(expression) => self.visit_expression(expression),
            ExprType::Variable(variable) => self.visit_variable(variable),
            ExprType::Var(var) => self.visit_var(var),
            ExprType::Assign(assign) => self.visit_assign(assign),
            ExprType::Block(block) => self.visit_block(block),
            ExprType::IfStatement(if_statement) => self.visit_if_statement(if_statement),
            ExprType::WhileStatement(while_statement) => {
                self.visit_while_statement(while_statement)
            }
            ExprType::Call(call) => self.visit_call(call),
        }
    }

    fn check_numbers(left: &Value, right: &Value) -> Result<(f64, f64), ExprError> {
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
        return Ok((left_value.to_owned(), right_value.to_owned()));
    }

    fn check_number(value: &Value) -> Result<f64, ExprError> {
        match value {
            Value::Float(value) => Ok(value.to_owned()),
            _ => Err(ExprError::RunTimeMessage(String::from(
                "Expecting number in unary operation",
            ))),
        }
    }

    fn is_trusty(obj: &Value) -> bool {
        match obj {
            Value::Nil => false,
            Value::Boolean(value) => value.to_owned(),
            _ => true,
        }
    }

    fn execute_block(&mut self, statements: &Vec<Box<ExprType>>) -> Result<(), ExprError> {
        self.globals.new_env();
        for statement in statements {
            self.execute(&*statement)?;
        }
        self.globals.delete_recent();
        Ok(())
    }
}

impl Visitor<Result<Rc<Value>, ExprError>> for Evaluator {
    fn visit_binary_operation(&mut self, expr: &Binary) -> Result<Rc<Value>, ExprError> {
        let left = self.accept(&*expr.left)?;
        let right = self.accept(&*expr.right)?;
        let operation = expr.operator;
        match operation.tt {
            TokenType::Plus => match left.borrow() {
                Value::String(value) => match right.borrow() {
                    Value::String(value2) => Ok(Rc::new(Value::String(value.to_owned() + value2))),
                    Value::Float(value2) => {
                        Ok(Rc::new(Value::String(format!("{}{}", value, value2))))
                    }
                    _ => Err(ExprError::RunTimeMessage(String::from(
                        "Operators must be  strings or numbers for '+' ",
                    ))),
                },
                Value::Float(value) => match right.borrow() {
                    Value::String(value2) => {
                        return Ok(Rc::new(Value::String(format!("{}{}", value, value2))))
                    }
                    Value::Float(value2) => Ok(Rc::new(Value::Float(value + value2))),
                    _ => Err(ExprError::RunTimeMessage(String::from(
                        "Operators must be  strings or numbers for '+' ",
                    ))),
                },
                _ => Err(ExprError::RunTimeMessage(String::from(
                    "Operators must be  strings or numbers for '+' ",
                ))),
            },
            TokenType::Minus => {
                let (left_value, right_value) =
                    Evaluator::check_numbers(left.borrow(), right.borrow())?;
                Ok(Rc::new(Value::Float(left_value - right_value)))
            }
            TokenType::Slash => {
                let (left_value, right_value) =
                    Evaluator::check_numbers(left.borrow(), right.borrow())?;
                Ok(Rc::new(Value::Float(left_value / right_value)))
            }
            TokenType::Star => {
                let (left_value, right_value) =
                    Evaluator::check_numbers(left.borrow(), right.borrow())?;
                Ok(Rc::new(Value::Float(left_value * right_value)))
            }
            TokenType::Greater => {
                let (left_value, right_value) =
                    Evaluator::check_numbers(left.borrow(), right.borrow())?;
                Ok(Rc::new(Value::Boolean(left_value > right_value)))
            }
            TokenType::GreaterEqual => {
                let (left_value, right_value) =
                    Evaluator::check_numbers(left.borrow(), right.borrow())?;
                Ok(Rc::new(Value::Boolean(left_value >= right_value)))
            }
            TokenType::Lesser => {
                let (left_value, right_value) =
                    Evaluator::check_numbers(left.borrow(), right.borrow())?;
                Ok(Rc::new(Value::Boolean(left_value < right_value)))
            }
            TokenType::LesserEqual => {
                let (left_value, right_value) =
                    Evaluator::check_numbers(left.borrow(), right.borrow())?;
                Ok(Rc::new(Value::Boolean(left_value <= right_value)))
            }
            TokenType::EqualEqual => Ok(Rc::new(Value::Boolean(left.equals(&right)))),
            TokenType::BangEqual => Ok(Rc::new(Value::Boolean(!left.equals(&right)))),
            _ => {
                return Err(ExprError::RunTimeMessage(String::from(
                    "Unsupported binary operation",
                )));
            }
        }
    }

    fn visit_literal(&mut self, expr: &Literal) -> Result<Rc<Value>, ExprError> {
        return Ok(Rc::new(expr.value.clone()));
    }

    fn visit_unary(&mut self, expr: &Unary) -> Result<Rc<Value>, ExprError> {
        let value = self.accept(&*expr.expression)?;

        match expr.operator.tt {
            TokenType::Plus => {
                let value = Evaluator::check_number(value.borrow())?;
                Ok(Rc::new(Value::Float(value)))
            }
            TokenType::Minus => {
                let value = Evaluator::check_number(value.borrow())?;
                Ok(Rc::new(Value::Float(-value)))
            }
            TokenType::Bang => {
                let value = Evaluator::is_trusty(value.borrow());
                Ok(Rc::new(Value::Boolean(!value)))
            }
            _ => {
                return Err(ExprError::RunTimeMessage(String::from(
                    "Unsupported unary operation",
                )));
            }
        }
    }

    fn visit_group(&mut self, expr: &Group) -> Result<Rc<Value>, ExprError> {
        self.accept(&*expr.expression)
    }

    fn visit_expression(&mut self, expr: &Expression) -> Result<Rc<Value>, ExprError> {
        let _ = self.accept(&*expr.expression)?;
        return Ok(Rc::clone(&self.constants.nil));
    }

    fn visit_print(&mut self, expr: &Print) -> Result<Rc<Value>, ExprError> {
        let value = self.accept(&*expr.expression)?;
        println!("{}", value.to_string());
        return Ok(Rc::clone(&self.constants.nil));
    }

    fn visit_variable(&mut self, expr: &Variable) -> Result<Rc<Value>, ExprError> {
        match self.globals.get(&expr.name) {
            Some(value) => Ok(value),
            None => Err(ExprError::RunTimeMessage(format!(
                "Undefined variable {}",
                expr.name
            ))),
        }
    }

    fn visit_var(&mut self, expr: &Var) -> Result<Rc<Value>, ExprError> {
        match &expr.initializer {
            Some(value) => {
                let value = self.accept(&*value)?;
                self.globals.define(&expr.name, value);
            }
            None => self
                .globals
                .define(&expr.name, Rc::clone(&self.constants.nil)),
        }
        Ok(Rc::clone(&self.constants.nil))
    }

    fn visit_assign(&mut self, expr: &Assign) -> Result<Rc<Value>, ExprError> {
        let value = self.accept(&*expr.initializer)?;
        self.globals.assign(&expr.name, Rc::clone(&value))?;
        Ok(Rc::clone(&value))
    }

    fn visit_block(&mut self, expr: &Block) -> Result<Rc<Value>, ExprError> {
        self.execute_block(&expr.statements)?;
        Ok(Rc::clone(&self.constants.nil))
    }

    fn visit_if_statement(&mut self, expr: &IfStatement) -> Result<Rc<Value>, ExprError> {
        let condition = self.accept(&*expr.condition)?;
        if Evaluator::is_trusty(condition.borrow()) {
            self.accept(&*expr.then_branch)?;
            return Ok(Rc::clone(&self.constants.nil));
        }
        match &expr.else_branch {
            Some(value) => {
                self.accept(&*value)?;
            }
            None => (),
        };
        Ok(Rc::clone(&self.constants.nil))
    }

    fn visit_while_statement(&mut self, expr: &WhileStatement) -> Result<Rc<Value>, ExprError> {
        while Evaluator::is_trusty(self.accept(&*expr.condition)?.borrow()) {
            self.accept(&*expr.body)?;
        }
        Ok(Rc::clone(&self.constants.nil))
    }

    fn visit_call(&mut self, expr: &Call) -> Result<Rc<Value>, ExprError> {
        let callee = self.accept(&*expr.callee)?;
        let mut arguments = Vec::new();
        for arg in &expr.arguments {
            arguments.push(self.accept(&*arg)?);
        }
        match &*callee {
            Value::Function(function) => {
                let func = function;
                if func.arity() != arguments.len() {
                    return Err(ExprError::RunTimeMessage(String::from(format!(
                        "Expected {} args but got {}",
                        func.arity(),
                        arguments.len()
                    ))));
                }
                func.call(self, arguments)
            }
            _ => Err(ExprError::RunTimeMessage(String::from(" Not a callable"))),
        }
    }
}
