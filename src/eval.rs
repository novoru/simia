use crate::ast::{ Ast };
use crate::lexier::{ Lexier };
use crate::object::{ Object };
use crate::parser::{ Parser };

pub fn eval(node: Ast) -> Option<Object> {
    match node {
        Ast::Program { statements, .. } => {
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
        Ast::IntegerLiteral { value, .. }  => return Some(Object::Integer{value: value}),
        Ast::Boolean { value, .. }      => return Some(Object::Boolean{value: value}),
        Ast::PrefixExpression { operator, right, .. } => {
            let right = match eval(*right){
                Some(value) => value,
                None        => Object::Null,
            };
            match eval_prefix_expression(operator, right) {
                Some(value) => return Some(value),
                None        => return None,
            }
        }
        _ => return None,
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
        Object::Boolean { .. } => Some(result),
        _                      => None,
    }
}

fn eval_prefix_expression(operator: String, right: Object) -> Option<Object> {
    match operator.as_ref() {
        "!" => return Some(eval_bang_operator_expression(right)),
        _   => return None,
    }
}

fn eval_bang_operator_expression(right: Object) -> Object {
    match right {
        Object::Boolean {value} => return Object::Boolean{value: !value},
        Object::Null            => return Object::Boolean{value: true},
        _                       => return Object::Boolean{value: false},
    }
}
