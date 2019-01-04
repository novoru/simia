use crate::object::{ Object };
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Env {
    store: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Env {
        Env { store: HashMap::new() }
    }

    pub fn get(&self, name: String) -> Object {
        match self.store.get(&name) {
            Some(value) => (*value).clone(),
            None         => Object::Null,
        }       
    }
    
    pub fn set(&mut self, name: String, val: Object) -> Object {
        self.store.insert(name.clone(), val.clone());

        val
    }
}
