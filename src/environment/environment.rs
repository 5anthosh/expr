use crate::default::Clock;
use crate::error::ExprError;
use crate::value::{TullyFunction, Value};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Environment {
    global: HashMap<String, Rc<Value>>,
    pub locals: Option<Vec<Rc<RefCell<HashMap<String, Rc<Value>>>>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            global: HashMap::new(),
            locals: None,
        }
    }

    pub fn define(&mut self, name: &String, value: Rc<Value>) {
        let env = &mut self.global;
        env.insert(name.clone(), value);
    }

    pub fn get(&self, name: &String) -> Option<Rc<Value>> {
        match self.global.get(name) {
            Some(v) => return Some(Rc::clone(v)),
            None => (),
        };
        match &self.locals {
            Some(v) => {
                for env in v {
                    match env.borrow().get(name) {
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
            None => None,
        }
    }

    pub fn assign(&mut self, name: &String, value: Rc<Value>) -> Result<(), ExprError> {
        if self.global.contains_key(name) {
            self.global.insert(name.clone(), Rc::clone(&value));
            return Ok(());
        }
        if let Some(values) = &mut self.locals {
            for env in values {
                if env.borrow().contains_key(name) {
                    env.borrow_mut().insert(name.clone(), Rc::clone(&value));
                    return Ok(());
                }
            }
        }
        return Err(ExprError::RunTimeMessage(format!(
            "Undefined variable {}",
            name
        )));
    }

    pub fn new_env(&mut self) {
        if let Some(value) = &mut self.locals {
            value.insert(0, Rc::new(RefCell::new(HashMap::new())));
            return;
        }
        self.locals = Some(vec![Rc::new(RefCell::new(HashMap::new()))]);
    }

    pub fn import_env(&mut self, environments: &Vec<Rc<RefCell<HashMap<String, Rc<Value>>>>>) {
        if let None = self.locals {
            self.locals = Some(vec![]);
        }
        if let Some(locals) = & mut self.locals {
            for env in environments {
                locals.push(Rc::clone(env))
            }
        }
    }

    pub fn delete_recent(&mut self) -> Option<Rc<RefCell<HashMap<String, Rc<Value>>>>> {
        if let Some(value) = &mut self.locals {
            let deleted_env = Some(value.remove(0));
            if value.len() == 1 {
                self.locals = None;
            }
            return deleted_env;
        }
        return None;
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
