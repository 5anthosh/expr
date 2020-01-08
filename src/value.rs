#[derive(Debug)]
pub enum Value {
    Boolean(bool),
    Float(f64),
    String(String),
    Nil,
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
