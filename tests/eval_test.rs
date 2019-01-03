extern crate simia;
use simia::eval::{ eval };
use simia::lexier::{ Lexier };
use simia::object::{ Object };
use simia::parser::{ Parser };

fn test_eval(input: String) -> Object {
    let lexier = Lexier::new(input);
    let mut parser = Parser::new(lexier);
    let program = parser.parse_program().unwrap();

    eval(program).unwrap()
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

#[test]
fn test_eval_integer_expression() {
    let tests = [("5", 5),
                 ("10", 10)
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
                 ("false", false)
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
