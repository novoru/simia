use crate::ast::{ Ast };
use crate::lexier::{ Lexier };
use crate::object::{ Object };
use crate::parser::{ Parser };

pub fn eval(node: Ast) -> Option<Object> {
    match node {
        Ast::Program             { statements, .. } => {
            match eval_statements(statements) {
                Some(value) => return Some(value),
                None        => return None,
            }
        },
        Ast::ExpressionStatement { expression, .. } => {
            match eval(*expression) {
                Some(value) => return Some(value),
                None        => return None,
            }
        },
        Ast::IntegerLiteral      { value, .. }      => return Some(Object::Integer{value: value}),
        _                                           => return None,
    }
}

fn eval_statements(statements: Vec<Box<Ast>>) -> Option<Object> {
    let mut result = Object::Null;
    
    for statement in statements {
        result = match eval(*statement) {
            Some(value) => value,
            None        => return None,
        }
    }

    match result {
        Object::Integer { .. } => Some(result),
        _                      => None,
    }
}
