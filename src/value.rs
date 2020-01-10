use crate::evaluator::Callable;
use std::rc::Rc;

#[derive(Clone)]
pub enum Value<'a> {
    Boolean(bool),
    Float(f64),
    String(String),
    Function(Rc<dyn Callable<'a> + 'a>),
    Nil,
}

pub struct Constants<'a> {
    pub nil: Rc<Value<'a>>,
}

impl<'a> Constants<'a> {
    pub fn new() -> Constants<'a> {
        Constants {
            nil: Rc::new(Value::Nil),
        }
    }
}

impl<'a> Value<'a> {
    pub fn equals(&self, another: &Value) -> bool {
        return match self {
            Value::Nil => match another {
                Value::Nil => true,
                _ => false,
            },
            Value::Boolean(value) => match another {
                Value::Boolean(value2) => value == value2,
                _ => false,
            },
            Value::String(value) => match another {
                Value::String(value2) => value.eq(value2),
                _ => false,
            },
            Value::Float(value) => match another {
                Value::Float(value2) => value == value2,
                _ => false,
            },
            Value::Function(_) => match another {
                Value::Function(_) => true,
                _ => false,
            },
        };
    }
}

impl<'a> ToString for Value<'a> {
    fn to_string(&self) -> String {
        match self {
            Value::Nil => String::from("nil"),
            Value::Float(value) => format!("{}", value),
            Value::Boolean(value) => format!("{}", value),
            Value::String(value) => value.to_string(),
            Value::Function(func) => func.to_string(),
        }
    }
}
