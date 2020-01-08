use crate::value::Value;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Rc<Value>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: String, value: Rc<Value>) {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: &String) -> Option<Rc<Value>> {
        match self.values.get(name) {
            Some(v) => Some(Rc::clone(v)),
            None => None,
        }
    }
    pub fn contains(&self, name: &String) -> bool {
        self.values.contains_key(name)
    }
}
