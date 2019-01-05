use crate::ast::{ Ast };
use crate::env::{ Env };

#[derive(Debug, Clone)]
pub enum Object{
    Null,

    Integer {
        value: i64,
    },

    Boolean {
        value: bool,
    },

    ReturnValue {
        value: Box<Object>,
    },

    Error {
        msg: String,
    },

    Function {
        parameters: Vec<Box<Ast>>,
        body: Box<Ast>,
        env: Box<Env>,
    },

    String {
        value: String,
    },

    Builtin {
        function: fn(Vec<Object>) -> Object, 
    },
    
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Null              => format!("null"),
            Object::Integer { value } => format!("{}", value),
            Object::Boolean { value } => format!("{}", value),
            Object::ReturnValue { value } => format!("{}", value.inspect()),
            Object::Error { msg }       => format!("Error: {}", msg),
            Object::Function { parameters, body, ..} => {
                let mut string = String::new();
                string = format!("fn(");
                for (i, parameter) in parameters.iter().enumerate() {
                    if i == 0 {
                        string = format!("{}{}", string, parameter.to_string());
                    }
                    else {
                        string = format!("{}, {}", string, parameter.to_string());
                    }
                }
                return  format!("{}) {{{}}}", string, body.to_string());
            },
            Object::String { value } => value.to_string(),
            Object::Builtin { .. } => "builtin function".to_string(),
        }
    }

    pub fn kind(&self) -> String {
        match self {
            Object::Null           => "Null".to_string(),
            Object::Integer { .. } => "Integer".to_string(),
            Object::Boolean { .. } => "Boolean".to_string(),
            Object::ReturnValue { .. } => "ReturnValue".to_string(),
            Object::Error { .. }   => "Error".to_string(),
            Object::Function { .. } => "Function".to_string(),
            Object::String { .. } => "String".to_string(),
            Object::Builtin { .. } => "Builtin".to_string(),
        }
    }
}

pub fn new_error(msg: String) -> Object {
    Object::Error { msg: msg }
}
