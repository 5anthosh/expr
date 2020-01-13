use crate::default::Clock;
use crate::error::ExprError;
use crate::value::{TullyFunction, Value};
use std::collections::HashMap;
//use std::backtrace::Backtrace;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment {
    scopes: Vec<Rc<RefCell<HashMap<String, Rc<Value>>>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            scopes: vec![Rc::new(RefCell::new(HashMap::new()))],
        }
    }

    pub fn define(&mut self, name: &String, value: Rc<Value>) {
        let env = &mut self.scopes[0];
        let mut env = env.borrow_mut();
        env.insert(name.clone(), value);
    }

    pub fn get(&self, name: &String) -> Option<Rc<Value>> {
        for scope in &self.scopes {
            let value = scope.borrow();
            let value = value.get(name);
            match value {
                Some(value) => return Some(Rc::clone(value)),
                None => {
                    continue;
                }
            }
        }
        return None;
    }

    pub fn assign(&mut self, name: &String, value: Rc<Value>) -> Result<(), ExprError> {
        for scope in &self.scopes {
            if scope.deref().borrow().contains_key(name) {
                scope
                    .deref()
                    .borrow_mut()
                    .insert(name.clone(), Rc::clone(&value));
                return Ok(());
            }
        }
        return Err(ExprError::RunTimeMessage(format!(
            "Undefined variable {}",
            name
        )));
    }

    pub fn new_env(&mut self) {
        // println!("creating new");
        self.scopes.insert(0, Rc::new(RefCell::new(HashMap::new())));
    }

    pub fn import_env(&mut self, environments: &Vec<Rc<RefCell<HashMap<String, Rc<Value>>>>>) {
        for env in environments {
            self.scopes.insert(0, Rc::clone(env))
        }
    }

    pub fn delete_recent(&mut self) -> Option<Rc<RefCell<HashMap<String, Rc<Value>>>>> {
        // println!("deleting env , before deleteing {:?}", self.scopes);
        Some(self.scopes.remove(0))
    }

    pub fn set_default_functions(&mut self) {
        self.define(
            &String::from("clock"),
            Rc::new(Value::Function(TullyFunction::NativeFunction(Rc::new(
                Clock,
            )))),
        )
    }
}
