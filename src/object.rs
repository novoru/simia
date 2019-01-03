pub enum Object{
    Integer {
        value: i64,
    },

    Boolean {
        value: bool,
    },

    Null,
    
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer { value } => format!("{}", value),
            Object::Boolean { value } => format!("{}", value),
            Object::Null              => format!("null"),
        }
    }

    pub fn kind(&self) -> String {
        match self {
            Object::Integer { .. } => "INTEGER".to_string(),
            Object::Boolean { .. } => "BOOLEAN".to_string(),
            Object::Null           => "NULL".to_string(),
        }
    }
}
