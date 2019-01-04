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
    
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Null              => format!("null"),
            Object::Integer { value } => format!("{}", value),
            Object::Boolean { value } => format!("{}", value),
            Object::ReturnValue { value } => format!("{}", value.inspect()),
            Object::Error {msg}       => format!("Error: {}", msg),
        }
    }

    pub fn kind(&self) -> String {
        match self {
            Object::Null           => "Null".to_string(),
            Object::Integer { .. } => "Integer".to_string(),
            Object::Boolean { .. } => "Boolean".to_string(),
            Object::ReturnValue { .. } => "ReturnValue".to_string(),
            Object::Error { .. }   => "Error".to_string(),
        }
    }
}

pub fn new_error(msg: String) -> Object {
    Object::Error { msg: msg }
}
