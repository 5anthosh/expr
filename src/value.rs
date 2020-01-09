use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Float(f64),
    String(String),
    Nil,
}

pub struct Constants {
    pub nil: Rc<Value>,
}

impl Constants {
    pub fn new() -> Constants {
        Constants {
            nil: Rc::new(Value::Nil),
        }
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
        }
    }
}
