use crate::error::ExprError;
use crate::evaluator::Evaluator;
use crate::value::Value;
use std::rc::Rc;

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(
        &self,
        evaluator: &Evaluator,
        arguments: Vec<Rc<Value>>,
    ) -> Result<Rc<Value>, ExprError>;
}
