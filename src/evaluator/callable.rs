use crate::error::ExprError;
use crate::evaluator::Evaluator;
use crate::parser::Function;
use crate::value::Value;
use std::rc::Rc;

pub trait Callable<'a> {
    fn arity(&self) -> usize;
    fn call(
        &self,
        evaluator: &'a mut Evaluator<'a>,
        arguments: Vec<Rc<Value<'a>>>,
    ) -> Result<Rc<Value<'a>>, ExprError>;
    fn to_string(&self) -> String;
}

pub struct TullyCallable<'a> {
    declaration: &'a Function<'a>,
}

impl<'a> Callable<'a> for TullyCallable<'a> {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
    
    fn call(
        &self,
        evaluator: &'a mut Evaluator<'a>,
        arguments: Vec<Rc<Value<'a>>>,
    ) -> Result<Rc<Value<'a>>, ExprError> {
        evaluator.globals.new_env();
        for (i, param) in self.declaration.params.iter().enumerate() {
            evaluator
                .globals
                .define(&param.lexeme, Rc::clone(&arguments[i]));
            evaluator.execute_block(&self.declaration.body.statements, false)?;
        }
        evaluator.globals.delete_recent();
        return Ok(Rc::clone(&evaluator.constants.nil));
    }

    fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.name.lexeme)
    }
}
