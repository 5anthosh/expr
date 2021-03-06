use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::error::TullyError;
use crate::evaluator::callable::TullyCallable;
use crate::evaluator::Callable;
use crate::lexer::token::{Token, TokenType};
use crate::parser::{
    Assign, Binary, Block, Call, ExprType, Expression, Function, Group, IfStatement, Literal,
    Parser, Print, Return, Unary, Var, Variable, Visitor, WhileStatement,
};
use crate::value::{Constants, Value};
use crate::value::{LiteralValue, TullyFunction};

#[derive(Debug)]
pub struct Evaluator {
    pub constants: Constants,
    pub globals: Environment,
}

impl<'a> Evaluator {
    pub fn new() -> Evaluator {
        let mut env = Environment::new();
        env.set_default_functions();
        Evaluator {
            constants: Constants::new(),
            globals: env,
        }
    }

    pub fn eval(&mut self, source: &str) -> Result<(), TullyError> {
        let mut parser = Parser::new(String::from(source.trim()));
        let ast = parser.parse()?;
        // println!("{:?}", statements);
        for statement in ast {
            self.execute(&statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, ast: &ExprType) -> Result<Rc<Value>, TullyError> {
        self.accept(ast)
    }

    pub fn accept(&mut self, expr: &ExprType) -> Result<Rc<Value>, TullyError> {
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
            ExprType::Return(return_statement) => self.visit_return(return_statement),
        }
    }

    fn check_numbers(left: &Value, right: &Value) -> Result<(f64, f64), TullyError> {
        let left_value = match left {
            Value::Float(left_val) => left_val,
            _ => {
                return Err(TullyError::RunTimeMessage(String::from(
                    "Expecting float in left side of operation",
                )));
            }
        };
        let right_value = match right {
            Value::Float(right_value) => right_value,
            _ => {
                return Err(TullyError::RunTimeMessage(String::from(
                    "Expecting number in right side of operation",
                )));
            }
        };
        return Ok((left_value.to_owned(), right_value.to_owned()));
    }

    fn check_number(value: &Value) -> Result<f64, TullyError> {
        match value {
            Value::Float(value) => Ok(value.to_owned()),
            _ => Err(TullyError::RunTimeMessage(String::from(
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
        &mut self,
        statements: &Vec<Box<ExprType>>,
        new_block: bool,
    ) -> Result<(), TullyError> {
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

    pub fn error(token: &Token, message: &str) -> TullyError {
        TullyError::runtime_error_message(token, message)
    }
}

impl Visitor<Result<Rc<Value>, TullyError>> for Evaluator {
    fn visit_binary_operation(&mut self, expr: &Binary) -> Result<Rc<Value>, TullyError> {
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
                    _ => Err(Evaluator::error(
                        operation,
                        "Operators must be  strings or numbers for '+' ",
                    )),
                },
                Value::Float(value) => match right.borrow() {
                    Value::String(value2) => {
                        return Ok(Rc::new(Value::String(format!("{}{}", value, value2))));
                    }
                    Value::Float(value2) => Ok(Rc::new(Value::Float(value + value2))),
                    _ => Err(Evaluator::error(
                        operation,
                        "Operators must be  strings or numbers for '+' ",
                    )),
                },
                _ => Err(Evaluator::error(
                    operation,
                    "Operators must be  strings or numbers for '+' ",
                )),
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
                return Err(Evaluator::error(operation, "Unsupported binary operation"));
            }
        }
    }

    fn visit_literal(&mut self, expr: &Literal) -> Result<Rc<Value>, TullyError> {
        match &expr.value {
            LiteralValue::Float(value) => Ok(Rc::new(Value::Float(value.clone()))),
            LiteralValue::String(value) => Ok(Rc::new(Value::String(value.clone()))),
            LiteralValue::Boolean(value) => Ok(Rc::new(Value::Boolean(value.clone()))),
            LiteralValue::Nil => return Ok(Rc::clone(&self.constants.nil)),
        }
    }

    fn visit_unary(&mut self, expr: &Unary) -> Result<Rc<Value>, TullyError> {
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
                return Err(Evaluator::error(
                    &expr.operator,
                    "Unsupported unary operation",
                ));
            }
        }
    }

    fn visit_group(&mut self, expr: &Group) -> Result<Rc<Value>, TullyError> {
        self.accept(&*expr.expression)
    }

    fn visit_expression(&mut self, expr: &Expression) -> Result<Rc<Value>, TullyError> {
        let _ = self.accept(&*expr.expression)?;
        return Ok(Rc::clone(&self.constants.nil));
    }

    fn visit_print(&mut self, expr: &Print) -> Result<Rc<Value>, TullyError> {
        let value = self.accept(&*expr.expression)?;
        println!("{}", value.to_string());
        return Ok(Rc::clone(&self.constants.nil));
    }

    fn visit_variable(&mut self, expr: &Variable) -> Result<Rc<Value>, TullyError> {
        match self.globals.get(&expr.name.lexeme) {
            Some(value) => Ok(value),
            None => Err(Evaluator::error(
                &expr.name,
                &format!("Undefined variable {}", &expr.name.lexeme),
            )),
        }
    }

    fn visit_var(&mut self, expr: &Var) -> Result<Rc<Value>, TullyError> {
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

    fn visit_assign(&mut self, expr: &Assign) -> Result<Rc<Value>, TullyError> {
        let value = self.accept(&*expr.initializer)?;
        self.globals.assign(&expr.name, Rc::clone(&value))?;
        Ok(Rc::clone(&value))
    }

    fn visit_block(&mut self, expr: &Block) -> Result<Rc<Value>, TullyError> {
        self.execute_block(&expr.statements, true)?;
        Ok(Rc::clone(&self.constants.nil))
    }

    fn visit_if_statement(&mut self, expr: &IfStatement) -> Result<Rc<Value>, TullyError> {
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

    fn visit_while_statement(&mut self, expr: &WhileStatement) -> Result<Rc<Value>, TullyError> {
        while Evaluator::is_trusty(self.accept(&*expr.condition)?.borrow()) {
            self.accept(&*expr.body)?;
        }
        Ok(Rc::clone(&self.constants.nil))
    }

    fn visit_call(&mut self, expr: &Call) -> Result<Rc<Value>, TullyError> {
        let callee = self.accept(&*expr.callee)?;
        let mut arguments = Vec::new();
        for arg in &expr.arguments {
            arguments.push(self.accept(&*arg)?);
        }
        match &*callee {
            Value::Function(function) => {
                let func = function;
                match func {
                    TullyFunction::NFunction(nf) => {
                        //                        println!("Calling {:?}", nf.deref().borrow().to_string());
                        let nf: &RefCell<TullyCallable> = nf.borrow();
                        if nf.borrow().arity() != arguments.len() {
                            return Err(Evaluator::error(
                                &expr.paren,
                                &format!(
                                    "Expected {} args but got {}",
                                    nf.borrow().arity(),
                                    arguments.len()
                                ),
                            ));
                        }
                        nf.borrow().call(self, arguments)
                    }
                    TullyFunction::NativeFunction(nf) => {
                        if nf.arity() != arguments.len() {
                            return Err(Evaluator::error(
                                &expr.paren,
                                &format!(
                                    "Expected {} args but got {}",
                                    nf.arity(),
                                    arguments.len()
                                ),
                            ));
                        }
                        nf.call(self, arguments)
                    }
                }
            }
            _ => Err(Evaluator::error(&expr.paren, " Not a callable")),
        }
    }

    fn visit_function(&mut self, expr: &Function) -> Result<Rc<Value>, TullyError> {
        let name = &expr.name.lexeme;
        let function = TullyCallable {
            declaration: expr.clone(),
            closure: None,
        };
        self.globals.define(
            name,
            Rc::new(Value::Function(TullyFunction::NFunction(Rc::new(
                RefCell::new(function),
            )))),
        );
        Ok(Rc::clone(&self.constants.nil))
    }

    fn visit_return(&mut self, expr: &Return) -> Result<Rc<Value>, TullyError> {
        match &expr.value {
            Some(value) => {
                let value = self.accept(&*value)?;
                Err(TullyError::Return(value))
            }
            None => Err(TullyError::Return(Rc::clone(&self.constants.nil))),
        }
    }
}
