use crate::environment::Environment;
use crate::error::ExprError;
use crate::evaluator::callable::TullyCallable;
use crate::lexer::token::TokenType;
use crate::parser::{
    Assign, Binary, Block, Call, ExprType, Expression, Function, Group, IfStatement, Literal,
    Parser, Print, Unary, Var, Variable, Visitor, WhileStatement,
};
use crate::value::{Constants, Value};
use std::borrow::Borrow;
use std::rc::Rc;

pub struct Evaluator<'a> {
    pub constants: Constants<'a>,
    pub globals: Environment<'a>,
}

impl<'a> Evaluator<'a> {
    pub fn new() -> Evaluator<'a> {
        let mut env = Environment::new();
        env.set_default_functions();
        Evaluator {
            constants: Constants::new(),
            globals: env,
        }
    }

    pub fn eval(&'a mut self, source: String) {
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

    fn execute(&'a mut self, ast: &'a ExprType<'a>) -> Result<Rc<Value<'a>>, ExprError> {
        self.accept(ast)
    }

    pub fn accept(&'a mut self, expr: &'a ExprType<'a>) -> Result<Rc<Value<'a>>, ExprError> {
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
            ExprType::Function(function) => self.visit_function(function),
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

    pub fn execute_block(
        &'a mut self,
        statements: &'a Vec<Box<ExprType<'a>>>,
        new_block: bool,
    ) -> Result<(), ExprError> {
        if new_block {
            self.globals.new_env();
        }
        for statement in statements {
            self.execute(&*statement)?;
        }
        if new_block {
            self.globals.delete_recent();
        }
        Ok(())
    }
}

impl<'a> Visitor<'a, Result<Rc<Value<'a>>, ExprError>> for Evaluator<'a> {
    fn visit_binary_operation(&'a mut self, expr: &'a Binary<'a>) -> Result<Rc<Value<'a>>, ExprError> {
        let left = self.accept(&*expr.left)?;
        let right = self.accept(&*expr.right)?;
        let operation = &expr.operator;
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
                // Not reachable
                return Err(ExprError::RunTimeMessage(String::from(
                    "Unsupported binary operation",
                )));
            }
        }
    }

    fn visit_literal(&'a mut self, expr: &'a Literal<'a>) -> Result<Rc<Value<'a>>, ExprError> {
        return Ok(Rc::new(expr.value.borrow().clone()));
    }

    fn visit_unary(&'a mut self, expr: &'a Unary<'a>) -> Result<Rc<Value<'a>>, ExprError> {
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
                // Not reachable
                return Err(ExprError::RunTimeMessage(String::from(
                    "Unsupported unary operation",
                )));
            }
        }
    }

    fn visit_group(&'a mut self, expr: &'a Group<'a>) -> Result<Rc<Value<'a>>, ExprError> {
        self.accept(&*expr.expression)
    }

    fn visit_expression(&'a mut self, expr: &'a Expression<'a>) -> Result<Rc<Value<'a>>, ExprError> {
        let _ = self.accept(&*expr.expression)?;
        return Ok(Rc::clone(&self.constants.nil));
    }

    fn visit_print(&'a mut self, expr: &'a Print<'a>) -> Result<Rc<Value<'a>>, ExprError> {
        let value = self.accept(&*expr.expression)?;
        println!("{}", value.to_string());
        return Ok(Rc::clone(&self.constants.nil));
    }

    fn visit_variable(&'a mut self, expr: &'a Variable<'a>) -> Result<Rc<Value<'a>>, ExprError> {
        match self.globals.get(expr.name) {
            Some(value) => Ok(value),
            None => Err(ExprError::RunTimeMessage(format!(
                "Undefined variable {}",
                expr.name
            ))),
        }
    }

    fn visit_var(&'a mut self, expr: &'a Var<'a>) -> Result<Rc<Value<'a>>, ExprError> {
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

    fn visit_assign(&'a mut self, expr: &'a Assign<'a>) -> Result<Rc<Value<'a>>, ExprError> {
        let value = self.accept(&*expr.initializer)?;
        self.globals.assign(&expr.name, Rc::clone(&value))?;
        Ok(Rc::clone(&value))
    }

    fn visit_block(&'a mut self, expr: &'a Block<'a>) -> Result<Rc<Value<'a>>, ExprError> {
        self.execute_block(&expr.statements, true)?;
        Ok(Rc::clone(&self.constants.nil))
    }

    fn visit_if_statement(&'a mut self, expr: &'a IfStatement<'a>) -> Result<Rc<Value<'a>>, ExprError> {
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

    fn visit_while_statement(&'a mut self, expr: &'a WhileStatement<'a>) -> Result<Rc<Value<'a>>, ExprError> {
        while Evaluator::is_trusty(self.accept(&*expr.condition)?.borrow()) {
            self.accept(&*expr.body)?;
        }
        Ok(Rc::clone(&self.constants.nil))
    }

    fn visit_call(&'a mut self, expr: &'a Call<'a>) -> Result<Rc<Value<'a>>, ExprError> {
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

    fn visit_function(&'a mut self, expr: &'a Function<'a>) -> Result<Rc<Value<'a>>, ExprError> {
        let name = &expr.name.lexeme;
        let function = TullyCallable { declaration: expr };
        self.globals
            .define(name, Rc::new(Value::Function(Rc::new(function))));
        Ok(Rc::clone(&self.constants.nil))
    }
}
