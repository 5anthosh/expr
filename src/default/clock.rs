use crate::error::ExprError;
use crate::evaluator::{Callable, Evaluator};
use crate::value::Value;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Clock;
impl<'a> Callable<'a> for Clock {
    fn arity(&self) -> usize {
        return 0;
    }

    fn call(
        &self,
        _evaluator: &'a mut Evaluator<'a>,
        _arguments: Vec<Rc<Value<'a>>>,
    ) -> Result<Rc<Value<'a>>, ExprError> {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH);
        let since_the_epoch = match since_the_epoch {
            Ok(duration) => duration,
            Err(e) => {
                return Err(ExprError::RunTimeMessage(String::from(format!(
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
