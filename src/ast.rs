use crate::token:: { TokenKind, Token };

#[derive(Debug, Clone)]
pub enum AST {
    PROGRAM {
        statements: Vec<Box<AST>>
    },

    EXPRESSION {
        token: Token,
    },

    IDENT {
        token: Token,
        value: String,
    },
    
    LET_STATEMENT {
        token: Token,
        ident: Box<AST>,
        value: Box<AST>,
    },

    RETURN_STATEMENT {
        token: Token,
        return_value: Box<AST>,
    },
    
    EXPRESSION_STATEMENT {
        token: Token,
        expression: Box<AST>,
    },

    INT_LITERAL {
        token: Token,
        value: i64,
    },

    PREFIX_EXPRESSION {
        token: Token,
        operator: String,
        right: Box<AST>,
    },

    INFIX_EXPRESSION {
        token: Token,
        left: Box<AST>,
        operator: String,
        right: Box<AST>,
    },

    BOOLEAN {
        token: Token,
        value: bool,
    },

    IF_EXPRESSION {
        token: Token,
        condition: Box<AST>,
        consequence: Box<AST>,
        alternative: Box<AST>,
    },

    BLOCK_STATEMENT {
        token: Token,
        statements: Vec<Box<AST>>,
    },

    FUNCTION_LITERAL {
        token: Token,
        parameters: Vec<Box<AST>>,  // AST::IDENT
        body: Box<AST>,             // AST::BLOCK_STATEMENT
    },

    CALL_EXPRESSION {
        token: Token,               // '(' token
        function: Box<AST>,         // AST::IDENT or AST::FUNCTION_LITERAL
        arguments: Vec<Box<AST>>,
    },
}

impl AST {
    pub fn to_string(&self) -> String {
        let mut string = String::new();
        match self {
            AST::PROGRAM { statements } => {
                for statement in statements {
                    string = format!("{}{}", string, statement.to_string() );
                }
            },
            AST::IDENT { value, .. } => {
                string = format!("{}", value);
            },
            AST::LET_STATEMENT {token, ident, value} => {
                string = format!("{} {} = {};",
                                 token.literal, ident.to_string(), value.to_string());
            },
            AST::RETURN_STATEMENT { token, return_value } => {
                string = format!("{} {};", token.literal, return_value.to_string());
            },
            AST::EXPRESSION_STATEMENT { expression,.. } => {
                string = format!("{}", expression.to_string());
            },
            AST::EXPRESSION { token } => {
                string = format!("{}", token.literal);
            },
            AST::INT_LITERAL { token, .. } => {
                string = format!("{}", token.literal);
            },
            AST::PREFIX_EXPRESSION { operator, right, ..} => {
                string = format!("({}{})", operator, right.to_string());
            },
            AST::INFIX_EXPRESSION { left, operator, right, .. } => {
                string = format!("({} {} {})", left.to_string(), operator, right.to_string());
            },
            AST::BOOLEAN { value, ..} => {
                string = format!("{}", value.to_string());
            },
            AST::IF_EXPRESSION { token, condition, consequence, alternative } => {
                string = format!("{}{} {}", token.literal, condition.to_string(), consequence.to_string());
                if  let AST::BLOCK_STATEMENT { ref token, ..} = **alternative {
                    match token.kind {
                        TokenKind::ILLEGAL => (),
                        _ => string = format!("{}else {}", string, alternative.to_string()),
                    }
                };
            },
            AST::BLOCK_STATEMENT { statements, .. } => {
                for statement in statements {
                    string = format!("{}{}", string, statement.to_string());
                }
            },
            AST::FUNCTION_LITERAL { token, parameters, body } => {
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
            AST::CALL_EXPRESSION { ref function, arguments, .. } => {
                match **function {
                    AST::IDENT            { ref value, .. } => string = format!("{}", value.to_string()),
                    AST::FUNCTION_LITERAL { .. }            => string = format!("{}", function.to_string()),
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
        }

        string
    }

    pub fn get_kind_literal(&self) -> String {
        match self {
            AST::PROGRAM              {..} => "PROGRAM".to_string(),
            AST::IDENT                {..} => "IDENT".to_string(),
            AST::LET_STATEMENT        {..} => "LET_STATEMENT".to_string(),
            AST::RETURN_STATEMENT     {..} => "RETURN_STATEMENT".to_string(),
            AST::EXPRESSION_STATEMENT {..} => "EXPRESSION_STATEMENT".to_string(),
            AST::EXPRESSION           {..} => "EXPRESSION".to_string(),
            AST::INT_LITERAL          {..} => "INT_LITERAL".to_string(),
            AST::PREFIX_EXPRESSION    {..} => "PREFIX_EXPRESSION".to_string(),
            AST::INFIX_EXPRESSION     {..} => "INFIX_EXPRESSION".to_string(),
            AST::BOOLEAN              {..} => "BOOLEAN".to_string(),
            AST::IF_EXPRESSION        {..} => "IF_EXPRESSION".to_string(),
            AST::BLOCK_STATEMENT      {..} => "BLOCK_STATEMENT".to_string(),
            AST::FUNCTION_LITERAL     {..} => "FUNCTION_LITERAL".to_string(),
            AST::CALL_EXPRESSION      {..} => "CALL_EXPRESSION".to_string(),
        }
    }
    
}

#[test]
fn test_ast_string() {
    let program = AST::PROGRAM {
        statements: vec![
            Box::new(
                AST::LET_STATEMENT {
                    token: Token {
                        kind: TokenKind::LET,
                        literal: "let".to_string()
                    },
                    ident: Box::new(
                        AST::IDENT {
                            token: Token {
                                kind: TokenKind::IDENT,
                                literal: "myVar".to_string()
                        },
                            value: "myVar".to_string()
                        }
                    ),
                    value: Box::new(
                        AST::EXPRESSION {
                            token: Token {
                                kind: TokenKind::IDENT,
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
