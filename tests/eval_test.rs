extern crate simia;
use simia::ast::{ Ast };
use simia::env::*;
use simia::eval::{ eval };
use simia::lexier::{ Lexier };
use simia::object::{ Object };
use simia::parser::{ Parser };

fn test_eval(input: String) -> Object {
    let lexier = Lexier::new(input);
    let mut parser = Parser::new(lexier);
    let program = parser.parse_program().unwrap();
    let mut env = Env::new();

    eval(program, &mut env).unwrap()
}

fn test_integer_object(obj: Object, expected: i64) -> bool {
    match obj {
        Object::Integer { value } => {
            if value != expected {
                eprintln!("object has wrong value. got={}, want={}", value, expected);
                return false;
            }
            return true;
        },
        _                         => {
            eprintln!("object is not Integer. got={}", obj.kind());
            return false;
        },
    }
}

fn test_boolean_object(obj: Object, expected: bool) -> bool {
    match obj {
        Object::Boolean { value } => {
            if value != expected {
                eprintln!("object has wrong value. got={}, want={}", value, expected);
                return false;
            }
            return true;
        },
        _                         => {
            eprintln!("object is not Boolean. got={}", obj.kind());
            return false;
        },
    }
}

fn test_null_object(obj: Object) -> bool {
    match obj {
        Object::Null => return true,
        _ => return false,
    }
}

#[test]
fn test_eval_integer_expression() {
    let tests = [("5", 5),
                 ("10", 10),
                 ("-5", -5),
                 ("-10", -10),
                 ("1 + 2", 3),
                 ("2 - 1", 1),
                 ("-1 + 5 + -4", 0),
                 ("2 * 3", 6),
                 ("10 / 5", 2),
                 ("(1 + 2) * 3", 9),
                 ("3 * (1 + 2)", 9),
                 ("(1 + 2 * 3) * 4 / 7", 4)
    ];

    for test in &tests {
        let evaluated = test_eval(test.0.to_string());
        if !test_integer_object(evaluated, test.1) {
            panic!();
        }
    }
    
}

#[test]
fn test_eval_boolean_expression() {
    let tests = [("true", true),
                 ("false", false),
                 ("1 < 2", true),
                 ("1 > 2", false),
                 ("1 < 1", false),
                 ("1 > 1", false),
                 ("1 == 1", true),
                 ("1 != 1", false),
                 ("1 == 2", false),
                 ("1 != 2", true),
                 ("true == true", true),
                 ("false == false", true),
                 ("true == false", false),
                 ("true != false", true),
                 ("false != true", true),
                 ("(1 < 2) == true", true),
                 ("(1 < 2) == false", false),
                 ("(1 > 2) == true", false),
                 ("(1 > 2) == false", true)
    ];
    
    for test in &tests {
        let evaluated = test_eval(test.0.to_string());
        if !test_boolean_object(evaluated, test.1) {
            panic!();
        }
    }
}

#[test]
fn test_bang_operator() {
    let tests = [("!true", false),
                 ("!false", true),
                 ("!5", false),
                 ("!!true", true),
                 ("!!false", false),
                 ("!!5", true)
    ];

    for test in &tests {
        let evaluated = test_eval(test.0.to_string());
        if !test_boolean_object(evaluated, test.1) {
            panic!();
        }
    }
}

#[test]
fn test_if_expression() {
    let tests = [("if (true) { 10 }", 10),
                 ("if (1) { 10 }", 10),
                 ("if (1 < 2) { 10 }", 10),
                 ("if (1 > 2) { 10 } else { 20 }", 20),
                 ("if (1 < 2) { 10 } else { 20 }", 10)
    ];

    for test in &tests {
        let evaluated = test_eval(test.0.to_string());
        if !test_integer_object(evaluated, test.1) {
            panic!();
        }
    }
}

#[test]
fn test_null_expression() {
    let tests = ["if (false) { 10 }",
                 "if (1 > 2) { 10 }",
                 "[1, 2, 3][3]",
                 "[1, 2, 3][-1]"
    ];

    for test in &tests {
        let evaluated = test_eval(test.to_string());
        if !test_null_object(evaluated) {
            panic!();
        }
    }
}

#[test]
fn test_return_statements() {
    let tests = [("return 10;", 10),
                 ("return 10; 9;", 10),
                 ("return 2 * 5; 9;", 10),
                 ("9; return 2 * 5; 9;", 10),
                 ("\
if (10 > 1) {\
   if(10 > 1) {\
      return 10;\
   }\
   return 1;\
}\
", 10)
    ];

    for test in &tests {
        let evaluated = test_eval(test.0.to_string());
        if !test_integer_object(evaluated, test.1) {
            panic!();
        }
    }    
}

#[test]
fn test_error_handling() {
    let tests = [("5 + true", "type mismatch: Integer + Boolean"),
                 ("5 + true; 5;", "type mismatch: Integer + Boolean"),
                 ("-true", "unknown operator: -Boolean"),
                 ("true + false", "unknown operator: Boolean + Boolean"),
                 ("5; true + false; 5;", "unknown operator: Boolean + Boolean"),
                 ("if(10 > 1) { true + false; }", "unknown operator: Boolean + Boolean"),
                 ("\
if (10 > 1) {\
   if (10 > 1) {\
      return true + false;\
   }\
   return 1;\
   }", "unknown operator: Boolean + Boolean"),
                 ("\"Hello\" - \"World\"", "unknown operator: String - String")
    ];

    for test in &tests {
        let evaluated = test_eval(test.0.to_string());

        match evaluated {
            Object::Error { msg } => {
                if msg != test.1.to_string() {
                    panic!("wrond error message. expected={}, got={}", test.1.to_string(), msg);
                }
            },
            _ => eprintln!("no error object returned. got={}", evaluated.kind()),
        }
    }    
}

#[test]
fn test_let_statements() {
    let tests = [("let a = 5; a;", 5),
                 ("let a = 5 * 5; a;", 25),
                 ("let a = 5; let b = a; b;", 5),
                 ("let a = 5; let b = a; let c = a + b + 5; c;", 15)
    ];

    for test in &tests {
        let evaluated = test_eval(test.0.to_string());
        if !test_integer_object(evaluated, test.1) {
            panic!();
        }
    }        
}

#[test]
fn test_function_object() {
    let input = "fn(x) { x + 2;}";

    let evaluated = test_eval(input.to_string());

    match evaluated {
        Object::Function { parameters, body, .. } => {
            if parameters.len() != 1 {
                panic!("function has wrong  parameters. got={}", parameters.len());
            }

            if let Ast::Identifier { value, .. } =  *parameters[0].clone() {
                if value != "x".to_string() {
                    panic!("parameter is not 'x'. got={}", value);
                }
            }

            let expected_body = "(x + 2)";
                        
            if (*body).to_string() != expected_body.to_string() {
                panic!("body is not {}. got={}", (*body).to_string(), expected_body);
            }
        }
        _ => panic!("object is not function. got={}", evaluated.kind()),
    }
}

#[test]
fn test_function_application() {
    let tests = [("let identity = fn(x) { x;}; identity(5);", 5),
                 ("let identity = fn(x) { return x;}; identity(5);", 5),
                 ("let double = fn(x) { x * 2;}; double(5);", 10),
                 ("let add = fn(x, y) { return x + y;}; add(5, 5)", 10),
                 ("let add = fn(x, y) { return x + y;}; add(5 + 5, add(5, 5));", 20),
                 ("fn(x) { x;}(5)", 5)
    ];
    for test in &tests {
        let evaluated = test_eval(test.0.to_string());
        if !test_integer_object(evaluated, test.1) {
            panic!();
        }
    }        
}

#[test]
fn test_closures() {
    let input = "\
let newAddr = fn(x) {\
   fn(y) { x + y };\
}\
\
let addTwo = newAddr(2);
addTwo(2);
";

    if !test_integer_object(test_eval(input.to_string()), 4) {
        panic!("");
    }
    
}

#[test]
fn test_string_literal() {
    let input = "\"Hello World\"".to_string();
    let evaluated = test_eval(input);

    match evaluated {
        Object::String { value } => {
            if value != "Hello World" {
                panic!("String has wrong value. got={}", value);
            }
        }
        _ => panic!("object is not String. got={}", evaluated.kind()),
    }
}

#[test]
fn test_string_concatenation() {
    let input = "\"Hello\" + \" \" + \"World\"".to_string();
    let evaluated = test_eval(input);

    match evaluated {
        Object::String { value } => {
            if value != "Hello World" {
                panic!("String has wrong value. got={}", value);
            }
        }
        _ => panic!("object is not String. got={}", evaluated.kind()),
    }    
}

#[test]
fn test_builtin_functions() {
    enum Type{
        Integer(i64),
        String(String),
    };
    
    let tests = [("len(\"\")", Type::Integer(0)),
                 ("len(1)", Type::String("argument to 'len' not supported, got Integer".to_string())),
                 ("len(\"\", \"\")", Type::String("wrong number of arguments. got=2, want=1".to_string()))
    ];

    for test in &tests {
        let evaluated = test_eval(test.0.to_string());
        match &test.1 {
            Type::Integer(value) => {
                if !test_integer_object(evaluated, *value) {
                    panic!("");
                }
            },
            Type::String(value)  => {
                match evaluated {
                    Object::Error { msg } => {
                        if msg != *value {
                            panic!("msg is not '{}', got='{}'", *value, msg);
                        }
                    },
                    _ => panic!("object is not Error. got={}", evaluated.kind()),
                }
            },
        }
    }        
}

#[test]
fn test_array_literals() {
    let input = "[1, 2 * 2, 3 + 3]".to_string();

    let evaluated = test_eval(input);

    match evaluated {
        Object::Array { elements } => {
            if elements.len() != 3 {
                panic!("array has wrong num of elements. got={}", elements.len());
            }

            test_integer_object(elements[0].clone(), 1);
            test_integer_object(elements[1].clone(), 4);
            test_integer_object(elements[2].clone(), 6);
        },
        _ => panic!("object is not Array. got={}", evaluated.kind()),
    }
}

#[test]
fn test_array_index_expressions() {
    let tests = [("[1, 2, 3][0]", 1),
                 ("[1, 2, 3][1]", 2),
                 ("[1, 2, 3][2]", 3),
                 ("let i = 0; [1][i]", 1),
                 ("[1, 2, 3][1 + 1]", 3),
                 ("let myArray = [1, 2, 3]; myArray[2];", 3),
                 ("let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];", 6),
                 ("let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]", 2)
    ];

    for test in &tests {
        let evaluated = test_eval(test.0.to_string());
        if !test_integer_object(evaluated, test.1) {
            panic!();
        }
    }        
}
