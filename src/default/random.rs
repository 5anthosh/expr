use crate::error::TullyError;
use crate::evaluator::{Callable, Evaluator};
use crate::value::Value;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::rc::Rc;

pub struct RandomAlphaNumeric;

impl Callable for RandomAlphaNumeric {
    fn arity(&self) -> usize {
        return 0;
    }

    fn call(
        &self,
        _evaluator: &mut Evaluator,
        _arguments: Vec<Rc<Value>>,
    ) -> Result<Rc<Value>, TullyError> {
        let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
        return Ok(Rc::new(Value::String(rand_string)));
    }

    fn to_string(&self) -> String {
        String::from("<native fn random_alphanumeric>")
    }
}

pub struct Random;

impl Callable for Random {
    fn arity(&self) -> usize {
        return 0;
    }

    fn call(
        &self,
        _evaluator: &mut Evaluator,
        _arguments: Vec<Rc<Value>>,
    ) -> Result<Rc<Value>, TullyError> {
        return Ok(Rc::new(Value::Float(rand::thread_rng().gen::<f64>())));
    }

    fn to_string(&self) -> String {
        String::from("<native fn random>")
    }
}
