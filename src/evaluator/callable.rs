use crate::error::ExprError;
use crate::evaluator::Evaluator;
use crate::parser::Function;
use crate::value::{TullyFunction, Value};
use std::borrow::{Borrow};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(
        &self,
        evaluator: &mut Evaluator,
        arguments: Vec<Rc<Value>>,
    ) -> Result<Rc<Value>, ExprError>;
    fn to_string(&self) -> String;
}

#[derive(Clone)]
pub struct TullyCallable {
    pub declaration: Function,
    pub closure: Option<Vec<Rc<RefCell<HashMap<String, Rc<Value>>>>>>,
}

impl TullyCallable {

    pub fn add_closure(&mut self, closure: Rc<RefCell<HashMap<String, Rc<Value>>>>) {
        if let Some(closures) = & mut self.closure {
            closures.push(closure);
            return;
        }
        self.closure = Some(vec![closure]);
    }

    pub fn add_closure_others(&mut self, closures: &Vec<Rc<RefCell<HashMap<String, Rc<Value>>>>>) {
        if let None = &self.closure {
            self.closure = Some(vec![]);
        }
        if let Some(closures_2) = &mut self.closure {
            for closure in closures {
                closures_2.push(Rc::clone(closure));
            }
        }
    }
}

impl Callable for TullyCallable {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn call(
        &self,
        evaluator: &mut Evaluator,
        arguments: Vec<Rc<Value>>,
    ) -> Result<Rc<Value>, ExprError> {
        let mut n = 0;
        if let Some(value) = &self.closure {
            n = value.len();
            evaluator.globals.import_env(value);
        }
        evaluator.globals.new_env();

        print!("{:?}", evaluator.globals.locals);
        for (i, param) in self.declaration.params.iter().enumerate() {
            evaluator
                .globals
                .define(&param.lexeme, Rc::clone(&arguments[i]));
        }

        let value = evaluator.execute_block(&self.declaration.body.statements, false);
        if let Some(env) = evaluator.globals.delete_recent() {
            for _i in 0..n {
                evaluator.globals.delete_recent();
            }

            if let Err(err) = value {
                if let ExprError::Return(value) = err {
                    if let Value::Function(tf) = value.borrow() {
                        if let TullyFunction::NFunction(tc) = tf {
                            println!("print");
                            println!("{:?}", env);
                            let t = tc.clone();
                            if let Some(closures) = &self.closure {
                                t.borrow_mut().add_closure_others(closures);
                            }
                            return Ok(Rc::new(Value::Function(TullyFunction::NFunction(t))));
                        }
                    }
                    return Ok(value);
                }
                return Err(err);
            }
        }
        return Ok(Rc::clone(&evaluator.constants.nil));
    }

    fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.name.lexeme)
    }
}
