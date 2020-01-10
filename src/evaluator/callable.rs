use crate::error::ExprError;
use crate::evaluator::Evaluator;
use crate::parser::Function;
use crate::value::Value;
use std::rc::Rc;
use std::result::Result::Err;

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(
        &self,
        evaluator: &mut Evaluator,
        arguments: Vec<Rc<Value>>,
    ) -> Result<Rc<Value>, ExprError>;
    fn to_string(&self) -> String;
}

pub struct TullyCallable {
    pub declaration: Function,
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
        evaluator.globals.new_env();
        for (i, param) in self.declaration.params.iter().enumerate() {
            evaluator
                .globals
                .define(&param.lexeme, Rc::clone(&arguments[i]));
        }
        let value = evaluator.execute_block(&self.declaration.body.statements, false);
        evaluator.globals.delete_recent();
        if let Err(err) = value {
            if let ExprError::Return(value) = err {
                return Ok(value);
            }
            return Err(err);
        }
        return Ok(Rc::clone(&evaluator.constants.nil));
    }

    fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.name.lexeme)
    }
}
