use crate::ast:: { AST };
use crate::lexier:: { Lexier };
use crate::token:: { TokenKind, Token};
use crate::util::*;
use std::collections::HashMap;


pub enum PRECEDENCE {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PERFIX,
    CALL,
}

#[derive(Debug, Clone)]
pub struct Parser {
    pub lexier: Lexier,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexier: Lexier) -> Parser {
        let mut parser = Parser { lexier: lexier,
                                  cur_token:  Token {
                                      kind: TokenKind::ILLEGAL,
                                      literal: "".to_string()},
                                  peek_token: Token {
                                      kind: TokenKind::ILLEGAL,
                                      literal: "".to_string()},
                                  errors: Vec::new(),
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexier.next_token();
    }
    
    pub fn parse_program(&mut self) -> Option<AST> {
        let mut program = AST::PROGRAM { statements: Vec::new() };
        
        while self.cur_token.kind.clone() as u8 != TokenKind::EOF.clone() as u8 {
            let statement = self.parse_statement();

            if let None = statement {
                ;
            }
            else {
                if let AST::PROGRAM { ref mut statements } = program {
                    statements.push(statement.unwrap());
                }
            }

            self.next_token();
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Box<AST>>{
        match self.cur_token.kind {
            TokenKind::LET => self.parse_let_statement(),
            TokenKind::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement()
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<AST>>{

        if !self.expect_peek(TokenKind::IDENT) {
            return None
        }
        
        let ident = Box::new(AST::IDENT {
            token: self.cur_token.clone(),
            value: self.cur_token.clone().literal,
        });

        if !self.expect_peek(TokenKind::ASSIGN) {
            return None
        }

        // TODO: change statement value to valid expression
        let value = Box::new(AST::EXPRESSION {
            token: Token { kind: TokenKind::ILLEGAL, literal: "".to_string() }
        });

        while !self.cur_token_is(TokenKind::SEMICOLON) {
            self.next_token();
        }

        Some( Box::new(AST::LET_STATEMENT {
            token: self.cur_token.clone(),
            ident: ident,
            value: value,
        }))
    }

    fn parse_return_statement(&mut self) -> Option<Box<AST>> {
        // TODO: change return value to valid expression
        let ret = Box::new(AST::RETURN_STATEMENT {
            token: self.cur_token.clone(),
            return_value:  Box::new(AST::EXPRESSION {
                token: Token {
                    kind: TokenKind::ILLEGAL,
                    literal: "".to_string(),
                }
            })
            
        });
        
        self.next_token();

        while !self.cur_token_is(TokenKind::SEMICOLON) {
            self.next_token();
        }

        Some(ret)
    }

    fn parse_expression_statement(&mut self) -> Option<Box<AST>> {
        let expression = self.parse_expression(PRECEDENCE::LOWEST);

        let statement = Box::new(AST::EXPRESSION_STATEMENT {
            token: self.cur_token.clone(),
            expression: expression.unwrap(),
        });

        if self.peek_token_is(TokenKind::SEMICOLON) {
            self.next_token();
        }

        Some(statement)
    }

    fn parse_expression (&mut self, prec: PRECEDENCE) -> Option<Box<AST>> {
        let mut left_exp = Box::new(AST::EXPRESSION {
            token : Token {
                kind: TokenKind::ILLEGAL,
                literal: "".to_string()
            }
        });
        match self.cur_token.kind {
            TokenKind::IDENT {..}  => left_exp = self.parse_identifier().unwrap(),
            _ => (),
        }
        
        Some(left_exp)
    }

    fn parse_identifier(&mut self) -> Option<Box<AST>> {
        Some(Box::new(AST::IDENT { token: self.cur_token.clone(), value: self.cur_token.literal.clone() }))
    }
    
    fn cur_token_is(&mut self, kind: TokenKind) -> bool {
        self.cur_token.kind.clone() as u8  == kind as u8
    }
    
    fn peek_token_is(&mut self, kind: TokenKind) -> bool {
        self.peek_token.kind.clone() as u8 == kind as u8
    }
    
    fn expect_peek(&mut self, kind: TokenKind) -> bool {
        if self.peek_token_is(kind) {
            self.next_token();
            return true;
        }
        
        self.peek_error(kind);

        false
    }

    fn peek_error(&mut self, kind: TokenKind) {
        let msg = format!("expeceted next token to be {}, got {} instead",
                          kind.get_kind_literal(), self.peek_token.get_kind_literal() );

        self.errors.push(msg);
    }

    
    pub fn check_parser_errors(&mut self) {
        if self.errors.len() == 0 {
            return ();
        }

        println!("parser has {} errors", self.errors.len());
        
        for msg in self.errors.clone() {
            println!("parser error: {}", msg);
        }
        
        panic!();
    }
    
}

// --- test code ---

#[test]
fn test_let_statements() {
    let input = "\
let x = 5;\
let y = 10;\
let foobar = 838383;\
".to_string();

    let mut lexier = Lexier::new(input);
    let mut parser = Parser::new(lexier);

    let mut program = parser.parse_program().unwrap();
    parser.check_parser_errors();

    match program {
        AST::PROGRAM { ref statements } if statements.len() == 3 => (),
        AST::PROGRAM { ref statements } =>
            panic!("program does not contain 3 statements. got={}", statements.len()),
        _ => panic!("parse_program() returned None. ")
    }

    let expected_let_statements = [  ( "x".to_string(), 5),
                                     ( "y".to_string(), 10),
                                     ( "foobar".to_string(), 838383)
    ];

    for (i, expected) in expected_let_statements.iter().enumerate() {
        if let AST::PROGRAM { ref mut statements } = program {
            if let  AST::LET_STATEMENT { ref mut ident, .. } = unbox(statements[i].clone()) {
                if let AST::IDENT { ref value, .. } = unbox(ident.clone()) {
                    // TODO: check value bound in identifier
                    assert_eq!(*value, expected.0);
                }
            }
        }    
    }
}

#[test]
fn test_return_statement() {
    let input = "\
return  5;\
return 10;\
return 993322;\
".to_string();

    let mut lexier = Lexier::new(input);
    let mut parser = Parser::new(lexier);

    let mut program = parser.parse_program().unwrap();
    parser.check_parser_errors();

    match program {
        AST::PROGRAM { ref statements } if statements.len() == 3 => (),
        AST::PROGRAM { ref statements } =>
            panic!("program does not contain 3 statements. got={}", statements.len()),
        _ => panic!("parse_program() returned None. ")
    }
    
    let expected_return_value = [ (5),
                                  (10),
                                  (838383)
    ];

    for (i, expected) in expected_return_value.iter().enumerate() {
        if let AST::PROGRAM { ref statements } = program {
            if let AST::RETURN_STATEMENT { ref token, .. } = unbox(statements[i].clone()) {
                //: TODO check return value
                assert_eq!(token.literal, "return");
            }
        }
    }        
}
