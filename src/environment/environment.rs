use std::collections::HashMap;
use crate::value::Value;

pub struct Enviroment {
    values:HashMap<String, Value>,
}

impl Enviroment {
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: &String) ->Option<&Value> {
        self.values.get(name)
    }
}