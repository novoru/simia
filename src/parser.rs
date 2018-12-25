use crate::ast:: { AST };
use crate::lexier:: { Lexier };
use crate::token:: { TokenKind, Token};


#[derive(Debug, Clone)]
pub enum PRECEDENCE {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

pub fn precedences (kind: TokenKind) -> PRECEDENCE {
    match kind {
        TokenKind::EQ       => PRECEDENCE::EQUALS,
        TokenKind::NOT_EQ   => PRECEDENCE::EQUALS,
        TokenKind::LT       => PRECEDENCE::LESSGREATER,
        TokenKind::GT       => PRECEDENCE::LESSGREATER,
        TokenKind::PLUS     => PRECEDENCE::SUM,
        TokenKind::MINUS    => PRECEDENCE::SUM,
        TokenKind::SLASH    => PRECEDENCE::PRODUCT,
        TokenKind::ASTERISK => PRECEDENCE::PRODUCT,
        _                   => PRECEDENCE::LOWEST
    }
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
            let statement = Box::new(self.parse_statement());

            if let None = *statement {
                ;
            }
            else {
                if let AST::PROGRAM { ref mut statements } = program {
                    statements.push(Box::new(statement.unwrap()));
                }
            }

            self.next_token();
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<AST>{
        match self.cur_token.kind {
            TokenKind::LET    => Some(self.parse_let_statement().unwrap()),
            TokenKind::RETURN => Some(self.parse_return_statement().unwrap()),
            _ => Some(self.parse_expression_statement().unwrap())
        }
    }

    fn parse_let_statement(&mut self) -> Option<AST>{

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

        Some( AST::LET_STATEMENT {
            token: self.cur_token.clone(),
            ident: ident,
            value: value,
        })
    }

    fn parse_return_statement(&mut self) -> Option<AST> {
        // TODO: change return value to valid expression
        let ret = AST::RETURN_STATEMENT {
            token: self.cur_token.clone(),
            return_value:  Box::new(AST::EXPRESSION {
                token: Token {
                    kind: TokenKind::ILLEGAL,
                    literal: "".to_string(),
                }
            })
        };
        self.next_token();
        
        while !self.cur_token_is(TokenKind::SEMICOLON) {
            self.next_token();
        }

        Some(ret)
    }

    fn parse_expression_statement(&mut self) -> Option<AST> {
        let expression = self.parse_expression(PRECEDENCE::LOWEST);

        let statement = AST::EXPRESSION_STATEMENT {
            token: self.cur_token.clone(),
            expression: Box::new(expression.unwrap()),
        };

        if self.peek_token_is(TokenKind::SEMICOLON) {
            self.next_token();
        }

        Some(statement)
    }

    fn parse_expression (&mut self, precedence: PRECEDENCE) -> Option<AST> {
        let mut left_exp = AST::EXPRESSION {
            token : Token {
                kind: TokenKind::ILLEGAL,
                literal: "".to_string()
            }
        };
        
        match self.cur_token.kind {
            TokenKind::IDENT    {..}  => left_exp = self.parse_identifier().unwrap(),
            TokenKind::INT      {..}  => left_exp = self.parse_integer_literal().unwrap(),
            TokenKind::BANG     {..} |
            TokenKind::MINUS    {..}  => left_exp = self.parse_prefix_expression().unwrap(),
            _ => (),
        }

        while !self.peek_token_is(TokenKind::SEMICOLON) && (precedence.clone() as u8) < (self.peek_precedence() as u8) {
            match self.peek_token.kind {
                TokenKind::PLUS     {..} |
                TokenKind::MINUS    {..} |
                TokenKind::SLASH    {..} |
                TokenKind::ASTERISK {..} |
                TokenKind::EQ       {..} |
                TokenKind::NOT_EQ   {..} |
                TokenKind::LT       {..} |
                TokenKind::GT       {..} => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(Box::new(left_exp)).unwrap();
                },
                _ => return Some(left_exp),
                
            }

        }
        
        Some(left_exp)
    }

    fn parse_identifier(&mut self) -> Option<AST> {
        Some(AST::IDENT { token: self.cur_token.clone(), value: self.cur_token.literal.clone() })
    }

    fn parse_integer_literal(&mut self) -> Option<AST> {
        Some(AST::INT_LITERAL {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone().parse::<i64>().unwrap(),
        })
    }

    fn parse_prefix_expression(&mut self) -> Option<AST>{
        let mut expression = AST::PREFIX_EXPRESSION {
            token: self.cur_token.clone(),
            operator: self.cur_token.clone().literal,
            right: Box::new(
                AST::EXPRESSION {
                    token:
                    Token {
                        kind: TokenKind::ILLEGAL,
                        literal: "".to_string(),
                    }
                }
            ),
        };

        self.next_token();

        if let AST::PREFIX_EXPRESSION { ref mut right, ..} = expression {
            *right = Box::new(self.parse_expression(PRECEDENCE::PREFIX).unwrap())
        }
        
        Some(expression)
    }

    fn parse_infix_expression(&mut self, left: Box<AST>) -> Option<AST>{
        let mut expression = AST::INFIX_EXPRESSION {
            token: self.cur_token.clone(),
            left: left.clone(),
            operator: self.cur_token.literal.clone(),
            right: Box::new(
                AST::EXPRESSION {
                    token:
                    Token {
                        kind: TokenKind::ILLEGAL,
                        literal: "".to_string(),
                    }
                }
            ),
        };

        let precedence = self.cur_precedence();
        self.next_token();
        if let AST::INFIX_EXPRESSION { ref mut right, ..} = expression {
            *right = Box::new(self.parse_expression(precedence).unwrap());
        }

        Some(expression)
        
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

    fn cur_precedence(&mut self) -> PRECEDENCE {
        precedences(self.cur_token.kind)
    }
    
    fn peek_precedence(&mut self) -> PRECEDENCE {
        precedences(self.peek_token.kind)
    }
    
    fn peek_error(&mut self, kind: TokenKind) {
        let msg = format!("expeceted next token to be {}, got {} instead",
                          kind.get_kind_literal(), self.peek_token.get_kind_literal() );

        self.errors.push(msg);
    }

    fn parse_error(&mut self) {
        let msg = format!("there is no matching pattern for {} found", self.cur_token.get_kind_literal());
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

    let lexier = Lexier::new(input);
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
            if let  AST::LET_STATEMENT { ref mut ident, .. } = *statements[i] {
                if let AST::IDENT { ref value, .. } = **ident {
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

    let lexier = Lexier::new(input);
    let mut parser = Parser::new(lexier);

    let program = parser.parse_program().unwrap();
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

    for (i, _) in expected_return_value.iter().enumerate() {
        if let AST::PROGRAM { ref statements } = program {
            if let AST::RETURN_STATEMENT { ref token, .. } = *statements[i] {
                //: TODO check return value
                assert_eq!(token.literal, "return");
            }
        }
    }        
}

#[test]
fn test_integer_literal() {
    let input = "5;\
12345;\
".to_string();

    let lexier = Lexier::new(input);
    let mut parser = Parser::new(lexier);

    let program = parser.parse_program().unwrap();
    parser.check_parser_errors();

    match program {
        AST::PROGRAM { ref statements } if statements.len() == 2 => (),
        AST::PROGRAM { ref statements } =>
            panic!("program does not contain 2 statements. got={}", statements.len()),
        _ => panic!("parse_program() returned None. ")
    }
    
    let expected_value = [ 5,
                          12345
    ];

    for (i, expected) in expected_value.iter().enumerate() {
        if let AST::PROGRAM { ref statements } = program {
            if let AST::INT_LITERAL { ref value, .. } = *statements[i] {
                assert_eq!(value, expected);
            }
        }
    }        
}

#[test]
fn test_prefix_expression() {
    let tests = [
        ("!5".to_string(), "!", 5),
        ("-15".to_string(), "-", 15)
    ];

    for (_i, test) in tests.iter().enumerate() {

        let lexier = Lexier::new(test.clone().0);
        let mut parser = Parser::new(lexier);

        let program = parser.parse_program().unwrap();
        parser.check_parser_errors();

        match program {
            AST::PROGRAM { ref statements } if statements.len() == 1 => (),
            AST::PROGRAM { ref statements } =>
                panic!("program does not contain 1 statements. got={}", statements.len()),
            _ => panic!("parse_program() returned None. ")
        }
               
        if let AST::PROGRAM { ref statements } = program {
            if let AST::EXPRESSION_STATEMENT { ref expression, .. } = *statements[0] {
                if let AST::PREFIX_EXPRESSION { ref operator, ref right, ..} = **expression {
                    assert_eq!(operator, test.1);
                    assert_eq!(right.to_string(), test.2.to_string());
                }
            }
        }        

    }
}

#[test]
fn test_infix_expression() {
    let tests = [
        ("5 + 5;", 5, "+", 5),
        ("5 - 5;", 5, "-", 5),
        ("5 * 5;", 5, "*", 5),
        ("5 / 5;", 5, "/", 5),
        ("5 > 5;", 5, ">", 5),
        ("5 < 5;", 5, "<", 5),
        ("5 == 5;", 5, "==", 5),
        ("5 != 5;", 5, "!=", 5)
    ];

    for (_i, test) in tests.iter().enumerate() {

        let lexier = Lexier::new((*test.clone().0).to_string());
        let mut parser = Parser::new(lexier);

        let program = parser.parse_program().unwrap();
        parser.check_parser_errors();

        match program {
            AST::PROGRAM { ref statements } if statements.len() == 1 => (),
            AST::PROGRAM { ref statements } =>
                panic!("program does not contain 1 statements. got={}", statements.len()),
            _ => panic!("parse_program() returned None. ")
        }
               
        if let AST::PROGRAM { ref statements } = program {
            if let AST::EXPRESSION_STATEMENT { ref expression, .. } = *statements[0] {
                if let AST::INFIX_EXPRESSION { ref left, ref operator, ref right, ..} = **expression {
                    assert_eq!(left.to_string(), test.1.to_string());
                    assert_eq!(*operator, test.2.to_string());
                    assert_eq!(right.to_string(), test.3.to_string());
                }
            }
        }        

    }
}

#[test]
fn test_operator_precedence_parsing() {
    let tests = [
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        ("3 + 4 * 5 == 3 * 1 + 4 * 5", "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))")
    ];

    for (_i, test) in tests.iter().enumerate() {

        let lexier = Lexier::new((*test.clone().0).to_string());
        let mut parser = Parser::new(lexier);

        let program = parser.parse_program().unwrap();
        parser.check_parser_errors();
               
        if let AST::PROGRAM { .. } = program {
            assert_eq!(program.to_string(), (*test.clone().1).to_string());
        }        

    }
}
