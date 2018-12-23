use crate::token:: { TokenKind, Token };

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum AST {
    PROGRAM { statements: Vec<AST> },

    EXPRESSION {
        token: Token,
    },
    
    LET_STATEMENT {
        token: Token,
        ident: Box<Identifier>,
        value: Box<AST>
    },

    RETURN_STATEMENT {
        token: Token,
        return_value: Box<AST>
    },
    
    EXPRESSION_STATEMENT {
        token: Token,
        expression: Box<AST>,
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
            AST::LET_STATEMENT {token, ident, value} => {
                string = format!("{} {} = {};",
                                 token.literal, ident.value, value.to_string());
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
            _ => string = "".to_string(),
        }

        string
    }
}

#[test]
fn test_ast_string() {
    let program = AST::PROGRAM {
        statements: vec![
            AST::LET_STATEMENT {
                token: Token {
                    kind: TokenKind::LET,
                    literal: "let".to_string()
                },
                ident: Box::new(
                    Identifier {
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
        ]
    };

    assert_eq!(program.to_string(), "let myVar = anotherVar;".to_string());
    
}
