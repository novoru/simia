use crate::ast::{ Ast };
use crate::object::{ Object, new_error };

fn len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return new_error(format!("wrong number of arguments. got={}, want=1", args.len()));
    }

    match &args[0] {
        Object::String { value } => return Object::Integer { value: value.len() as i64 },
        _ => new_error(format!("argument to 'len' not supported, got {}", args[0].kind())),
    }
}

pub fn builtins(name: String) -> Object{
    match name.as_ref() {
        "len" => Object::Builtin { function: len },
        _     => Object::Null,
    }
}
