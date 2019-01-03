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
    
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Null              => format!("null"),
            Object::Integer { value } => format!("{}", value),
            Object::Boolean { value } => format!("{}", value),
            Object::ReturnValue { value } => format!("{}", value.inspect()),
        }
    }

    pub fn kind(&self) -> String {
        match self {
            Object::Null           => "Null".to_string(),
            Object::Integer { .. } => "Integer".to_string(),
            Object::Boolean { .. } => "Boolean".to_string(),
            Object::ReturnValue { .. } => "ReturnValue".to_string(),
        }
    }
}
