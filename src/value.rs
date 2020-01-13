use crate::evaluator::{Callable, TullyCallable};
use std::cell::RefCell;
use std::fmt::{self, Debug, Formatter};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone)]
pub enum Value {
    Boolean(bool),
    Float(f64),
    String(String),
    Function(TullyFunction),
    Nil,
}

#[derive(Clone)]
pub enum TullyFunction {
    NFunction(Rc<RefCell<TullyCallable>>),
    NativeFunction(Rc<dyn Callable>),
}

impl TullyFunction {
    pub fn to_string(&self) -> String {
        match self {
            TullyFunction::NFunction(tc) => tc.deref().borrow().to_string(),
            TullyFunction::NativeFunction(nf) => nf.to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum LiteralValue {
    Boolean(bool),
    Float(f64),
    String(String),
    Nil,
}

#[derive(Debug)]
pub struct Constants {
    pub nil: Rc<Value>,
}

impl<'a> Constants {
    pub fn new() -> Constants {
        Constants {
            nil: Rc::new(Value::Nil),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Value {
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

impl ToString for Value {
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
