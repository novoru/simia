use crate::ast::{ Ast };
use crate::object::{ Object, new_error };

fn len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return new_error(format!("wrong number of arguments. got={}, want=1", args.len()));
    }

    match &args[0] {
        Object::String { value } => return Object::Integer { value: value.len() as i64 },
        Object::Array { elements } => return Object::Integer { value: elements.len() as i64 },
        _ => new_error(format!("argument to 'len' not supported, got {}", args[0].kind())),
    }
}

fn first(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return new_error(format!("wrong number of arguments. got={}, want=1", args.len()));
    }

    match &args[0] {
        Object::String { value } => {
            if value.len() == 0 {
                return Object::Null;
            }
            return Object::String { value: value[0..1].to_string() }
        },
        Object::Array { elements } => {
            if elements.len() == 0 {
                return Object::Null;
            }
            return elements[0].clone()
        },
        _ => new_error(format!("argument to 'first' not supported, got {}", args[0].kind())),
    }    
}

fn last(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return new_error(format!("wrong number of arguments. got={}, want=1", args.len()));
    }

    match &args[0] {
        Object::String { value } => {
            if value.len() == 0 {
                return Object::Null;
            }
            return Object::String { value: value[value.len()-1..].to_string() }
        },
        Object::Array { elements } => {
            if elements.len() == 0 {
                return Object::Null;
            }
            return elements[elements.len()-1].clone()
        },
        _ => new_error(format!("argument to 'last' not supported, got {}", args[0].kind())),
    }
}

fn rest(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return new_error(format!("wrong number of arguments. got={}, want=1", args.len()));
    }

    match &args[0] {
        Object::String { value } => {
            if value.len() == 0 {
                return Object::Null;
            }
            return Object::String { value: value[1..].to_string() }
        },
        Object::Array { elements } => {
            if elements.len() == 0 {
                return Object::Null;
            }
            return Object::Array { elements: elements[1..].to_vec() }
        },
        _ => new_error(format!("argument to 'rest' not supported, got {}", args[0].kind())),
    }
}

fn push(args: Vec<Object>) -> Object {

    if args.len() != 2 {
        return new_error(format!("wrong number of arguments. got={}, want=1", args.len()));
    }

    match &args[0] {
        Object::String { value: origin } => {
            match &args[1] {
                Object::String { value: str } => {
                    let mut tmp = (*origin).to_owned();
                    tmp.extend(str.to_string().chars());
                    return Object::String { value: tmp };
                    },
                _ => return new_error(format!("argument to 'push' not String. got={}", &args[1].kind())),
            }
        },
        Object::Array { elements } => {
            let mut tmp = elements.to_owned();
            tmp.push(args[1].clone());
            return Object::Array { elements: tmp };
        },
        _ => new_error(format!("argument to 'push' not supported, got {}", args[0].kind())),
    }
}

pub fn builtins(name: String) -> Object{
    match name.as_ref() {
        "len" => Object::Builtin { function: len },
        "first" => Object::Builtin { function: first },
        "last" => Object::Builtin { function: last },
        "rest" => Object::Builtin { function: rest },
        "push" => Object::Builtin { function: push },
        _     => Object::Null,
    }
}
