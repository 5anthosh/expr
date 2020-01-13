use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::TullyError;
use crate::evaluator::{Callable, Evaluator};
use crate::value::Value;

pub struct Clock;
impl Callable for Clock {
    fn arity(&self) -> usize {
        return 0;
    }

    fn call(
        &self,
        _evaluator: &mut Evaluator,
        _arguments: Vec<Rc<Value>>,
    ) -> Result<Rc<Value>, TullyError> {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH);
        let since_the_epoch = match since_the_epoch {
            Ok(duration) => duration,
            Err(e) => {
                return Err(TullyError::RunTimeMessage(String::from(format!(
                    "{}",
                    e.to_string()
                ))))
            }
        };
        Ok(Rc::new(Value::Float(since_the_epoch.as_secs_f64())))
    }

    fn to_string(&self) -> String {
        String::from("<native fn clock>")
    }
}
