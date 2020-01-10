use crate::default::Clock;
use crate::error::ExprError;
use crate::value::Value;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Environment<'a> {
    environments: Vec<HashMap<String, Rc<Value<'a>>>>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Environment<'a> {
        Environment {
            environments: vec![HashMap::new()],
        }
    }

    pub fn define(&mut self, name: &String, value: Rc<Value<'a>>) {
        let env = &mut self.environments[0];
        env.insert(name.clone(), value);
    }

    pub fn get(&self, name: &String) -> Option<Rc<Value<'a>>> {
        for env in &self.environments {
            match env.get(name) {
                Some(v) => {
                    return Some(Rc::clone(v));
                }
                None => {
                    continue;
                }
            }
        }
        None
    }

    pub fn assign(&mut self, name: &String, value: Rc<Value<'a>>) -> Result<(), ExprError> {
        for env in self.environments.iter_mut() {
            if env.contains_key(name) {
                env.insert(name.clone(), Rc::clone(&value));
                return Ok(());
            }
        }
        return Err(ExprError::RunTimeMessage(format!(
            "Undefined variable {}",
            name
        )));
    }

    pub fn new_env(&mut self) {
        self.environments.insert(0, HashMap::new());
    }

    pub fn delete_recent(&mut self) {
        if self.environments.len() > 1 {
            self.environments.remove(0);
        }
    }

    pub fn set_default_functions(&mut self) {
        self.define(
            &String::from("clock"),
            Rc::new(Value::Function(Rc::new(Clock))),
        )
    }
}
