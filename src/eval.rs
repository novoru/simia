use crate::ast::{ Ast };
use crate::builtins::{ builtins };
use crate::env::*;
use crate::lexier::{ Lexier };
use crate::object::{ Object, new_error };
use crate::parser::{ Parser };
use crate::token::{ TokenKind };

pub fn eval(node: Ast, env: &mut Env) -> Option<Object> {
    match node {
        Ast::Program { .. } => return eval_program(node, env),
        Ast::ExpressionStatement { expression, .. } => {
            match eval(*expression, env) {
                Some(value) => return Some(value),
                None        => return None,
            }
        },
        Ast::IntegerLiteral { value, .. }  => return Some(Object::Integer{value: value}),
        Ast::Boolean { value, .. }      => return Some(Object::Boolean{value: value}),
        Ast::PrefixExpression { operator, right, .. } => {
            let right = match eval(*right, env){
                Some(value) => value,
                None        => return Some(new_error("prefix expression has no right hand side.".to_string())),
            };
            
            if is_error(&right) {
                return Some(right);
            }
            
            match eval_prefix_expression(operator, right) {
                Some(value) => return Some(value),
                None        => return None,
            }
        },
        Ast::InfixExpression { left, operator, right, .. } => {
            let left = match eval(*left, env){
                Some(value) => value,
                None        => return Some(new_error("infix expression has no left hand side.".to_string())),
            };

            if is_error(&left) {
                return Some(left);
            }
            
            let right = match eval(*right, env){
                Some(value) => value,
                None        => return Some(new_error("infix expression has no right hand side.".to_string())),
            };

            if is_error(&right) {
                return Some(right);
            }
            
            return Some(eval_infix_expression(operator, left, right));
        },
        Ast::BlockStatement { .. } => return eval_block_statement(node, env),
        Ast::IfExpression { .. } => return eval_if_expression(node, env),
        Ast::ReturnStatement { return_value, .. } => {
            let val = match eval(*return_value, env){
                Some(value) => Box::new(value),
                None        => Box::new(Object::Null),
            };

            if is_error(&*val) {
                return Some(*val);
            }
            
            return Some(Object::ReturnValue { value: val });
        },
        Ast::LetStatement { ident, value, .. } => {
            let val = match eval(*value, env){
                Some(value) => value,
                None        => Object::Null,
            };

            if is_error(&val) {
                return Some(val);
            }
            if let Ast::Identifier { value, ..} = *ident {
                return Some(env.set(value, val));
            }
            
            return None;

        },
        Ast::Identifier { value, .. } => return eval_identifier(value, env),
        Ast::FunctionLiteral { parameters, body, .. } => {
            return Some(Object::Function{
                parameters: parameters,
                body: body,
                env: Box::new(env.clone())
            });
        },
        Ast::CallExpression { function, arguments, ..} => {
            let func = match eval(*function, env) {
                Some(value) => value,
                None        => Object::Null,
            };

            if is_error(&func) {
                return Some(func);
            }

            let args = eval_expressions(arguments, env);

            if args.len() == 1 && is_error(&args[0]) {
                return Some(args[0].clone());
            }

            Some(apply_function(func, args))
        },
        Ast::StringLiteral { value, .. } => return Some(Object::String { value: value }),
        Ast::ArrayLiteral { elements, token } => {
            let elems = eval_expressions(elements, env);
            if elems.len() == 1 && is_error(&elems[0]) {
                return Some(elems[0].clone());
            }

            return Some(Object::Array { elements: elems})
        },
        Ast::IndexExpression { left, index, .. } => {
            let l = match eval(*left, env) {
                Some(value) => value,
                None => Object::Null,
            };

            if is_error(&l) {
                return Some(l);
            }

            let i = match eval(*index, env) {
                Some(value) => value,
                None => Object::Null,
            };

            if is_error(&i) {
                return Some(i);
            }

            return eval_index_expression(l, i)
        },
        _ => return None,
    }
}

fn eval_statements(statements: Vec<Box<Ast>>, env: &mut Env) -> Option<Object> {
    let mut result = Object::Null;
    
    for statement in statements {
        result = match eval(*statement, env) {
            Some(value) => {
                match value {
                    Object::ReturnValue { value: ret_value } => return Some(*ret_value),
                    _ =>  value,
                }
            },
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
        _                         => return new_error(format!("unknown operator: -{}", right.kind())),
    }
}

fn eval_infix_expression(operator: String, left: Object, right: Object) -> Object {
    if left.kind() == "Integer".to_string() && right.kind() == "Integer".to_string() {
        return eval_integer_infix_expression(operator, left, right);
    }
    else if left.kind() == "String".to_string() && right.kind() == "String".to_string() {
        return eval_string_infix_expression(operator, left, right);
    }
    else if left.kind() != right.kind() {
        return new_error(format!("type mismatch: {} {} {}", left.kind(), operator, right.kind()));
    }
    if operator == "==".to_string() {
        match left {
            Object::Boolean { value: lvalue } => {
                match right {
                    Object::Boolean { value: rvalue } => {
                        return Object::Boolean { value: lvalue==rvalue};
                    },
                    _ => return new_error(format!("type mismatch: {} {} {}", left.kind(), operator, right.kind())),
                }
            },
            _ => return new_error(format!("type mismatch: {} {} {}", left.kind(), operator, right.kind())),
        }
    }
    else if operator == "!=".to_string() {
        match left {
            Object::Boolean { value: lvalue } => {
                match right {
                    Object::Boolean { value: rvalue } => {
                        return Object::Boolean { value: lvalue!=rvalue};
                    },
                    _ => return new_error(format!("type mismatch: {} {} {}", left.kind(), operator, right.kind())),
                }
            },
            _ => return new_error(format!("type mismatch: {} {} {}", left.kind(), operator, right.kind())),
        }
    }
    new_error(format!("unknown operator: {} {} {}", left.kind(), operator, right.kind()))
}

fn eval_integer_infix_expression(operator: String, left: Object, right: Object) -> Object {
    match operator.as_ref() {
        "+" => {
            if let Object::Integer { value: lvalue } = left {
                if let Object::Integer { value: rvalue } = right {
                    return Object::Integer { value: lvalue + rvalue};
                };
            };
            return new_error(format!("type mismatch: {} {} {}", left.kind(), operator, right.kind()));
        },
        "-" => {
            if let Object::Integer { value: lvalue } = left {
                if let Object::Integer { value: rvalue } = right {
                    return Object::Integer { value: lvalue - rvalue};
                };
            };
            return new_error(format!("type mismatch: {} {} {}", left.kind(), operator, right.kind()));
        },
        "*" => {
            if let Object::Integer { value: lvalue } = left {
                if let Object::Integer { value: rvalue } = right {
                    return Object::Integer { value: lvalue * rvalue};
                };
            };
            return new_error(format!("type mismatch: {} {} {}", left.kind(), operator, right.kind()));
        },
        "/" => {
            if let Object::Integer { value: lvalue } = left {
                if let Object::Integer { value: rvalue } = right {
                    return Object::Integer { value: lvalue / rvalue};
                };
            };
            return new_error(format!("type mismatch: {} {} {}", left.kind(), operator, right.kind()));
        },
        "<" => {
            if let Object::Integer { value: lvalue } = left {
                if let Object::Integer { value: rvalue } = right {
                    return Object::Boolean { value: lvalue < rvalue};
                };
            };
            return new_error(format!("type mismatch: {} {} {}", left.kind(), operator, right.kind()));
        },
        ">" => {
            if let Object::Integer { value: lvalue } = left {
                if let Object::Integer { value: rvalue } = right {
                    return Object::Boolean { value: lvalue > rvalue};
                };
            };
            return new_error(format!("type mismatch: {} {} {}", left.kind(), operator, right.kind()));
        },
        "==" => {
            if let Object::Integer { value: lvalue } = left {
                if let Object::Integer { value: rvalue } = right {
                    return Object::Boolean { value: lvalue == rvalue};
                };
            };
            return new_error(format!("type mismatch: {} {} {}", left.kind(), operator, right.kind()));
        },
        "!=" => {
            if let Object::Integer { value: lvalue } = left {
                if let Object::Integer { value: rvalue } = right {
                    return Object::Boolean { value: lvalue != rvalue};
                };
            };
            return new_error(format!("type mismatch: {} {} {}", left.kind(), operator, right.kind()));
        },
        _  => return new_error(format!("unknown operator: {} {} {}", left.kind(), operator, right.kind())),
    }
}

fn eval_string_infix_expression(operator: String, left: Object, right: Object) -> Object{
    match operator.as_ref() {
        "+" => {
            if let Object::String { value: lvalue } = left.clone() {
                if let Object::String { value: rvalue } = right.clone() {
                    return Object::String { value: lvalue + &rvalue };
                };
            };
            return new_error(format!("type mismatch: {} {} {}", left.kind(), operator, right.kind()));
        },
        "==" => {
            if let Object::String { value: lvalue } = left.clone() {
                if let Object::String { value: rvalue } = right.clone() {
                    return Object::Boolean { value: lvalue == rvalue };
                };
            };
            return new_error(format!("type mismatch: {} {} {}", left.kind(), operator, right.kind()));
        },
        "!=" => {
            if let Object::String { value: lvalue } = left.clone() {
                if let Object::String { value: rvalue } = right.clone() {
                    return Object::Boolean { value: lvalue != rvalue };
                };
            };
            return new_error(format!("type mismatch: {} {} {}", left.kind(), operator, right.kind()));
        },
        _ => new_error(format!("unknown operator: {} {} {}", left.kind(), operator, right.kind())),
    }
}

fn eval_if_expression(node: Ast, env: &mut Env) -> Option<Object> {
    match node {
        Ast::IfExpression { condition, consequence, alternative, .. } => {
            let condition = match eval(*condition, env) {
                Some(value) => value,
                None        => return Some(Object::Null),
            };

            if is_error(&condition) {
                return Some(condition);
            }
            
            if is_truthy(condition) {
                return eval(*consequence, env);
            }
            
            else {
                match *alternative {
                    Ast::Expression { ref token, .. } => {
                        if token.get_kind_literal() == "Illegal".to_string()  {
                            return Some(Object::Null);
                        }
                        else {
                            return eval(*alternative, env);
                        }
                    },
                    _ => return eval(*alternative, env),
                }
            }
        },
        _ => return Some(Object::Null),
    }
}

fn is_truthy(obj: Object) -> bool {
    match obj {
        Object::Null => return false,
        Object::Boolean { value } => return value,
        _ => return true,
    }
}

fn eval_program(program: Ast, env: &mut Env) -> Option<Object> {
    let mut result = Object::Null;
    
    if let Ast::Program { statements, .. } = program {
        for statement in statements {
            result = match eval(*statement, env) {
                Some(value) => value,
                None        => Object::Null,
            };
            match  result {
                Object::ReturnValue { value } => return Some(*value),
                Object::Error { .. }          => return Some(result),
                _ => (),
            };
        }
    }

    Some(result)
}

fn eval_block_statement(block: Ast, env: &mut Env) -> Option<Object> {
    let mut result = Object::Null;

    if let Ast::BlockStatement { statements, .. } = block {
        for statement in statements {
            result = match eval(*statement, env){
                Some(value) => value,
                None        => Object::Null,
            };
            if result.kind() == "ReturnValue".to_string() || result.kind() == "Error" {
                return Some(result);
            }
        }
    }
    
    Some(result)
}

fn eval_identifier(value: String, env: &mut Env) -> Option<Object> {
    let val = env.get(value.clone());
    
    match val {
        Object::Null => (),
        _ => return Some(val),
    }

    if let Object::Builtin { function } = builtins(value.clone()) {
        return Some(builtins(value.clone()));
    }
    
    Some(new_error(format!("identifier not found: {}", value)))
}

fn eval_expressions(exps: Vec<Box<Ast>>, env: &mut Env) -> Vec<Object>{
    let mut result = Vec::new();

    for exp in exps {
        let evaluated = match eval(*exp, env){
            Some(value) => value,
            None        => Object::Null,
        };

        if is_error(&evaluated) {
            return vec![evaluated];
        }

        result.push(evaluated);
    }

    result
}

fn eval_index_expression(left: Object, index: Object) -> Option<Object> {
    if left.kind() == "Array".to_string() && index.kind() == "Integer" {
        return Some(eval_array_index_expression(left, index));
    }
    Some(new_error(format!("index operator not supported: {}", left.kind())))
}

fn eval_array_index_expression(array: Object, index: Object) -> Object  {
    let idx = match index {
        Object::Integer { value } => value as usize,
        _ => return Object::Null,
    };

    let max = match array {
        Object::Array { ref elements } => (&elements.len() - 1),
        _ => return Object::Null,
    };

    if idx < 0 || idx > max {
        return Object::Null;
    }

    match array {
        Object::Array { ref elements } => elements[idx].clone(),
        _ => Object::Null,
    }
}

fn apply_function(func: Object, args: Vec<Object>) -> Object {
    match func {
        Object::Function { .. } => {
            let mut extend_env = extend_function_env(func.clone(), args);
            let evaluated = match func {
                Object::Function { body, ..} => match eval(*body, &mut extend_env) {
                    Some(value) => value,
                    None        => return Object::Null,
                },
                _                            => return Object::Null,
            };
            
            return unwrap_return_value(evaluated);            
        },
        Object::Builtin { function } => return function(args),
        _                       => return new_error(format!("not a function: {}", func.kind())),
    }

}

fn extend_function_env(func: Object, args: Vec<Object>) -> Env {
    let mut env = match func.clone() {
        Object::Function { env, .. } => Env::new_enclosed_env(env),
        _                            => return Env::new(),
    };

    if let Object::Function { parameters, ..} = func {
        for (i, parameter) in parameters.iter().enumerate() {
            if let Ast::Identifier { value, .. } = (**parameter).clone() {
                env.set(value, args[i].clone());
            }
        }
    }

    env
}

fn unwrap_return_value(obj: Object) -> Object {
    if let Object::ReturnValue { value } = obj {
        return *value;
    }

    obj
}

fn is_error(obj: &Object) -> bool {
    match obj {
        Object::Null => (),
        _            => return obj.kind() == "Error".to_string(),
    }
    false
}
