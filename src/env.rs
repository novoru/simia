use crate::object::{ Object };
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Env {
    store: HashMap<String, Object>,
    outer: Option<Box<Env>>,
}

impl Env {
    pub fn new_enclosed_env(outer: Box<Env>) -> Env {
        let mut env = Env::new();
        env.outer = Some(outer);

        env
    }
    
    pub fn new() -> Env {
        Env { store: HashMap::new(), outer: None }
    }

    pub fn get(&self, name: String) -> Object {
        match self.store.get(&name) {
            Some(value) => (*value).clone(),
            None        => {
                match &self.outer {
                    Some(outer) => match outer.store.get(&name) {
                        Some(value) => (*value).clone(),
                        None        => Object::Null,
                    }
                    None        => Object::Null,
                }
            },
        }       
    }
    
    pub fn set(&mut self, name: String, val: Object) -> Object {
        self.store.insert(name.clone(), val.clone());

        val
    }
}
