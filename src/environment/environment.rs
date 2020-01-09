use crate::error::ExprError;
use crate::value::Value;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment {
    environments: Vec<HashMap<String, Rc<Value>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            environments: vec![HashMap::new()],
        }
    }

    pub fn define(&mut self, name: String, value: Rc<Value>) {
        let env = &mut self.environments[0];
        env.insert(name, value);
    }

    pub fn get(&self, name: &String) -> Option<Rc<Value>> {
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

    pub fn assign(&mut self, name: &String, value: Rc<Value>) -> Result<(), ExprError> {
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
}
