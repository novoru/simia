use crate::token:: { TokenKind, Token };

#[derive(Debug, Clone)]
pub enum AST {
    PROGRAM { statements: Vec<Box<AST>> },

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
            AST::IDENT { token, value } => {
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
            AST::INT_LITERAL { token,.. } => {
                string = format!("{}", token.literal);
            },
            AST::PREFIX_EXPRESSION { operator, right, ..} => {
                string = format!("({}{})", operator, right.to_string());
            },
            _ => string = "".to_string(),
        }

        string
    }

    pub fn get_kind_literal(&self) -> String {
        match self {
            AST::PROGRAM {..} => "PROGRAM".to_string(),
            AST::IDENT {..} => "IDENT".to_string(),
            AST::LET_STATEMENT {..} =>  "LET_STATEMENT".to_string(),
            AST::RETURN_STATEMENT {..} => "RETURN_STATEMENT".to_string(),
            AST::EXPRESSION_STATEMENT {..} => "EXPRESSION_STATEMENT".to_string(),
            AST::EXPRESSION {..} => "EXPRESSION".to_string(),
            AST::INT_LITERAL {..} => "INT_LITERAL".to_string(),
            AST::PREFIX_EXPRESSION {..} => "PREFIX_EXPRESSION".to_string()
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
