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
        },
        Ast::InfixExpression { left, operator, right, token } => {
            let left = match eval(*left){
                Some(value) => value,
                None        => Object::Null,
            };
            let right = match eval(*right){
                Some(value) => value,
                None        => Object::Null,
            };
            return Some(eval_infix_expression(operator, left, right));
        },
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
        Object::Integer { .. } |
        Object::Boolean { .. } |
        Object::Null           => Some(result),
        _                      => None,
    }
}

fn eval_prefix_expression(operator: String, right: Object) -> Option<Object> {
    match operator.as_ref() {
        "!" => return Some(eval_bang_operator_expression(right)),
        "-" => return Some(eval_minus_operator_expression(right)),
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

fn eval_minus_operator_expression(right: Object) -> Object {
    match right {
        Object::Integer { value } => return Object::Integer{value: -value},
        _                         => return Object::Null,
    }
}

fn eval_infix_expression(operator: String, left: Object, right: Object) -> Object {
    if left.kind() == "Integer".to_string() && right.kind() == "Integer".to_string() {
        return eval_integer_infix_expression(operator, left, right);
    }
    if operator == "==".to_string() {
        match left {
            Object::Boolean { value: lvalue } => {
                match right {
                    Object::Boolean { value: rvalue } => {
                        return Object::Boolean { value: lvalue==rvalue};
                    },
                    _ => return Object::Null
                }
            },
            _ => return Object::Null,
        }
    }
    else if operator == "!=".to_string() {
        match left {
            Object::Boolean { value: lvalue } => {
                match right {
                    Object::Boolean { value: rvalue } => {
                        return Object::Boolean { value: lvalue!=rvalue};
                    },
                    _ => return Object::Null
                }
            },
            _ => return Object::Null,
        }
    }
    Object::Null
}

fn eval_integer_infix_expression(operator: String, left: Object, right: Object) -> Object {
    match operator.as_ref() {
        "+" => {
            if let Object::Integer { value: lvalue } = left {
                if let Object::Integer { value: rvalue } = right {
                    return Object::Integer { value: lvalue + rvalue};
                };
            };
            return Object::Null;
        },
        "-" => {
            if let Object::Integer { value: lvalue } = left {
                if let Object::Integer { value: rvalue } = right {
                    return Object::Integer { value: lvalue - rvalue};
                };
            };
            return Object::Null;
        },
        "*" => {
            if let Object::Integer { value: lvalue } = left {
                if let Object::Integer { value: rvalue } = right {
                    return Object::Integer { value: lvalue * rvalue};
                };
            };
            return Object::Null;
        },
        "/" => {
            if let Object::Integer { value: lvalue } = left {
                if let Object::Integer { value: rvalue } = right {
                    return Object::Integer { value: lvalue / rvalue};
                };
            };
            return Object::Null;
        },
        "<" => {
            if let Object::Integer { value: lvalue } = left {
                if let Object::Integer { value: rvalue } = right {
                    return Object::Boolean { value: lvalue < rvalue};
                };
            };
            return Object::Null;
        },
        ">" => {
            if let Object::Integer { value: lvalue } = left {
                if let Object::Integer { value: rvalue } = right {
                    return Object::Boolean { value: lvalue > rvalue};
                };
            };
            return Object::Null;
        },
        "==" => {
            if let Object::Integer { value: lvalue } = left {
                if let Object::Integer { value: rvalue } = right {
                    return Object::Boolean { value: lvalue == rvalue};
                };
            };
            return Object::Null;
        },
        "!=" => {
            if let Object::Integer { value: lvalue } = left {
                if let Object::Integer { value: rvalue } = right {
                    return Object::Boolean { value: lvalue != rvalue};
                };
            };
            return Object::Null;
        },
        _  => return Object::Null,
    }
}
