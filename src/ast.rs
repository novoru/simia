use crate::token:: { TokenKind, Token };

#[derive(Debug, Clone)]
pub enum Ast {
    Program {
        statements: Vec<Box<Ast>>
    },

    Expression {
        token: Token,
    },

    Identifier {
        token: Token,
        value: String,
    },
    
    LetStatement {
        token: Token,
        ident: Box<Ast>,
        value: Box<Ast>,
    },

    ReturnStatement {
        token: Token,
        return_value: Box<Ast>,
    },
    
    ExpressionStatement {
        token: Token,
        expression: Box<Ast>,
    },

    IntegerLiteral {
        token: Token,
        value: i64,
    },

    PrefixExpression {
        token: Token,
        operator: String,
        right: Box<Ast>,
    },

    InfixExpression {
        token: Token,
        left: Box<Ast>,
        operator: String,
        right: Box<Ast>,
    },

    Boolean {
        token: Token,
        value: bool,
    },

    IfExpression {
        token: Token,
        condition: Box<Ast>,
        consequence: Box<Ast>,
        alternative: Box<Ast>,
    },

    BlockStatement {
        token: Token,
        statements: Vec<Box<Ast>>,
    },

    FunctionLiteral {
        token: Token,
        parameters: Vec<Box<Ast>>,  // Ast::Identifier
        body: Box<Ast>,             // Ast::BlockStatement
    },

    CallExpression {
        token: Token,               // '(' token
        function: Box<Ast>,         // Ast::Identifier or Ast::FunctionLiteral
        arguments: Vec<Box<Ast>>,
    },

    StringLiteral {
        token: Token,
        value: String,
    },

    ArrayLiteral {
        token: Token,
        elements: Vec<Box<Ast>>,
    },

    IndexExpression {
        token: Token,
        left: Box<Ast>,
        index: Box<Ast>,
    }
}

impl Ast {
    pub fn to_string(&self) -> String {
        let mut string = String::new();
        match self {
            Ast::Program { statements } => {
                for statement in statements {
                    string = format!("{}{}", string, statement.to_string() );
                }
            },
            Ast::Identifier { value, .. } => {
                string = format!("{}", value);
            },
            Ast::LetStatement {token, ident, value} => {
                string = format!("{} {} = {};",
                                 token.literal, ident.to_string(), value.to_string());
            },
            Ast::ReturnStatement { token, return_value } => {
                string = format!("{} {};", token.literal, return_value.to_string());
            },
            Ast::ExpressionStatement { expression,.. } => {
                string = format!("{}", expression.to_string());
            },
            Ast::Expression { token } => {
                string = format!("{}", token.literal);
            },
            Ast::IntegerLiteral { token, .. } => {
                string = format!("{}", token.literal);
            },
            Ast::PrefixExpression { operator, right, ..} => {
                string = format!("({}{})", operator, right.to_string());
            },
            Ast::InfixExpression { left, operator, right, .. } => {
                string = format!("({} {} {})", left.to_string(), operator, right.to_string());
            },
            Ast::Boolean { value, ..} => {
                string = format!("{}", value.to_string());
            },
            Ast::IfExpression { token, condition, consequence, alternative } => {
                string = format!("{}({}) {{ {} }}", token.literal, condition.to_string(), consequence.to_string());
                if  let Ast::BlockStatement { ref token, ..} = **alternative {
                    match token.kind {
                        TokenKind::Illegal => (),
                        _ => string = format!("{}else {{ {} }}", string, alternative.to_string()),
                    }
                };
            },
            Ast::BlockStatement { statements, .. } => {
                for statement in statements {
                    string = format!("{}{}", string, statement.to_string());
                }
            },
            Ast::FunctionLiteral { token, parameters, body } => {
                string = format!("{}(", token.literal);
                for (i, parameter ) in parameters.iter().enumerate() {
                    if i == 0 {
                        string = format!("{}{}", string, parameter.to_string());
                    }
                    else {
                        string = format!("{}, {}", string, parameter.to_string());
                    }
                }
                string = format!("{}) {{{}}}", string, body.to_string());
            }
            Ast::CallExpression { ref function, arguments, .. } => {
                match **function {
                    Ast::Identifier            { ref value, .. } => string = format!("{}", value.to_string()),
                    Ast::FunctionLiteral { .. }            => string = format!("{}", function.to_string()),
                    _ => (),
                }
                string = format!("{}(", string);

                for (i, argument) in arguments.iter().enumerate() {
                    if i == 0 {
                        string = format!("{}{}", string, argument.to_string());
                    }
                    else {
                        string = format!("{}, {}", string, argument.to_string());
                    }
                }
                string = format!("{})", string);
            }
            Ast::StringLiteral { value, .. } => string = value.to_string(),
            Ast::ArrayLiteral  { elements, .. } => {
                string = format!("[");
                for (i, element) in elements.iter().enumerate() {
                    if i == 0 {
                        string = format!("{}{}", string, element.to_string());
                    }
                    else {
                        string = format!("{}, {}", string, element.to_string());
                    }
                }
                string = format!("{}]", string);
            },
            Ast::IndexExpression { left, index, .. } => string = format!("({}[{}])", left.to_string(), index.to_string()),
        }

        string
    }

    pub fn get_kind_literal(&self) -> String {
        match self {
            Ast::Program              {..} => "Program".to_string(),
            Ast::Identifier           {..} => "Identifier".to_string(),
            Ast::LetStatement         {..} => "LetStatement".to_string(),
            Ast::ReturnStatement      {..} => "ReturnStatement".to_string(),
            Ast::ExpressionStatement  {..} => "ExpressionStatement".to_string(),
            Ast::Expression           {..} => "Expression".to_string(),
            Ast::IntegerLiteral       {..} => "IntegerLiteral".to_string(),
            Ast::PrefixExpression     {..} => "PrefixExpression".to_string(),
            Ast::InfixExpression      {..} => "InfixExpression".to_string(),
            Ast::Boolean              {..} => "Boolean".to_string(),
            Ast::IfExpression         {..} => "IfExpression".to_string(),
            Ast::BlockStatement       {..} => "BlockStatement".to_string(),
            Ast::FunctionLiteral      {..} => "FunctionLiteral".to_string(),
            Ast::CallExpression       {..} => "CallExpression".to_string(),
            Ast::StringLiteral        {..} => "StringLiteral".to_string(),
            Ast::ArrayLiteral         {..} => "ArrayLiteral".to_string(),
            Ast::IndexExpression      {..} => "IndexExpression".to_string(),
        }
    }
    
}

#[test]
fn test_ast_string() {
    let program = Ast::Program {
        statements: vec![
            Box::new(
                Ast::LetStatement {
                    token: Token {
                        kind: TokenKind::Let,
                        literal: "let".to_string()
                    },
                    ident: Box::new(
                        Ast::Identifier {
                            token: Token {
                                kind: TokenKind::Identifier,
                                literal: "myVar".to_string()
                        },
                            value: "myVar".to_string()
                        }
                    ),
                    value: Box::new(
                        Ast::Expression {
                            token: Token {
                                kind: TokenKind::Identifier,
                                literal: "anotherVar".to_string()
                            }
                        }
                    )
                }
            )
        ]
    };

    assert_eq!(program.to_string(), "let myVar = anotherVar;".to_string());
    
}
