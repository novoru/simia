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
        TokenKind::LPAREN   => PRECEDENCE::CALL,
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
            TokenKind::TRUE     {..} |
            TokenKind::FALSE    {..}  => left_exp = self.parse_boolean().unwrap(),
            TokenKind::LPAREN   {..}  => left_exp = self.parse_grouped_expression().unwrap(),
            TokenKind::IF       {..}  => left_exp = self.parse_if_expression().unwrap(),
            TokenKind::FUNCTION {..}  => left_exp = self.parse_function_literal().unwrap(),
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
                TokenKind::LPAREN   {..} => {
                    self.next_token();
                    left_exp = self.parse_call_expression(Box::new(left_exp)).unwrap();
                }
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

    fn parse_boolean(&mut self) -> Option<AST> {
        Some(AST::BOOLEAN {
            token: self.cur_token.clone(),
            value: self.cur_token_is(TokenKind::TRUE),
        })
    }

    fn parse_grouped_expression(&mut self) -> Option<AST> {
        self.next_token();
        let expression = self.parse_expression(PRECEDENCE::LOWEST).unwrap();

        if !self.expect_peek(TokenKind::RPAREN) {
            return None;
        }

        Some(expression)
    }

    fn parse_if_expression(&mut self) -> Option<AST> {
        let empty_expression = AST::EXPRESSION {
            token:
            Token {
                kind: TokenKind::ILLEGAL,
                literal: "".to_string(),
            }
        };
        let mut expression = AST::IF_EXPRESSION {
            token: self.cur_token.clone(),
            condition: Box::new(empty_expression.clone()),
            consequence: Box::new(empty_expression.clone()),
            alternative: Box::new(empty_expression.clone()),
        };

        if !self.expect_peek(TokenKind::LPAREN) {
            return None;
        }

        self.next_token();

        if let AST::IF_EXPRESSION { ref mut condition, .. } = expression {
            *condition = Box::new(self.parse_expression(PRECEDENCE::LOWEST).unwrap());
        };

        if !self.expect_peek(TokenKind::RPAREN) {
            return None;
        }

        if !self.expect_peek(TokenKind::LBRACE) {
            return None;
        }

        if let AST::IF_EXPRESSION { ref mut consequence, .. } = expression {
            *consequence = Box::new(self.parse_block_statement().unwrap());
        };

        if self.peek_token_is(TokenKind::ELSE) {
            self.next_token();

            if !self.expect_peek(TokenKind::LBRACE) {
                return None;
            }

            if let AST::IF_EXPRESSION { ref mut alternative, .. } = expression {
                *alternative = Box::new(self.parse_block_statement().unwrap());
            }
        }

        Some(expression)
    }

    fn parse_block_statement(&mut self) -> Option<AST> {
        let mut block = AST::BLOCK_STATEMENT {
            token: self.cur_token.clone(),
            statements: Vec::new(),
        };

        self.next_token();

        while !self.cur_token_is(TokenKind::RBRACE) && !self.cur_token_is(TokenKind::EOF) {
            let mut statement = self.parse_statement().unwrap();
            
            match &statement {
                AST    => {
                    if let AST::BLOCK_STATEMENT { ref mut statements, ..} = block {
                        statements.push(Box::new(statement));
                    };
                },
                _ => return None,
            }
            
            self.next_token();
        }

        Some(block)
    }

    fn parse_function_literal(&mut self) -> Option<AST> {
        let mut literal = AST::FUNCTION_LITERAL {
            token: self.cur_token.clone(),
            parameters: Vec::new(),
            body: Box::new(
                AST::EXPRESSION {
                token:
                Token {
                    kind: TokenKind::ILLEGAL,
                    literal: "".to_string(),
                }
            }),
        };

        if !self.expect_peek(TokenKind::LPAREN) {
            return None;
        }
        
        if let AST::FUNCTION_LITERAL { ref mut parameters, .. } = literal {
            *parameters = self.parse_function_parameters().unwrap();
        }
        
        if !self.expect_peek(TokenKind::LBRACE) {
            return None;
        }

        if let AST::FUNCTION_LITERAL { ref mut body, .. } = literal {
            *body = Box::new(self.parse_block_statement().unwrap());
        }

        Some(literal)
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Box<AST>>> {
        let mut identifiers = Vec::new();

        if self.peek_token_is(TokenKind::RPAREN) {
            self.next_token();
            return Some(identifiers);
        }

        self.next_token();

        let mut identifier = AST::IDENT {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        identifiers.push(Box::new(identifier));

        while self.peek_token_is(TokenKind::COMMA) {
            self.next_token();
            self.next_token();

            identifier = AST::IDENT {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            };

            identifiers.push(Box::new(identifier));
        }

        if !self.expect_peek(TokenKind::RPAREN) {
            return None;
        }

        Some(identifiers)
    }

    fn parse_call_expression(&mut self, function: Box<AST> ) -> Option<AST> {
        let expression = AST::CALL_EXPRESSION {
            token: self.cur_token.clone(),
            function:  function,
            arguments: self.parse_call_arguments(), 
        };
        
        Some(expression)
    }

    fn parse_call_arguments(&mut self) -> Vec<Box<AST>> {
        let mut arguments = Vec::new();

        if self.peek_token_is(TokenKind::RPAREN) {
            self.next_token();
            return arguments;
        }

        self.next_token();
        arguments.push(Box::new(self.parse_expression(PRECEDENCE::LOWEST).unwrap()));

        while self.peek_token_is(TokenKind::COMMA) {
            self.next_token();
            self.next_token();
            arguments.push(Box::new(self.parse_expression(PRECEDENCE::LOWEST).unwrap()));
        }

        if !self.expect_peek(TokenKind::RPAREN) {
            return Vec::new();
        }

        arguments
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
fn test_integer_literal_expression() {
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
        ("3 + 4 * 5 == 3 * 1 + 4 * 5", "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
        ("true", "true"),
        ("false", "false"),
        ("3 > 5 == false", "((3 > 5) == false)"),
        ("3 < 5 == true", "((3 < 5) == true)"),
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2", "((5 + 5) * 2)"),
        ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("!(true == true)", "(!(true == true))"),
        ("add(a)", "add(a)"),
        ("add(a + b * c)", "add((a + (b * c)))"),
        ("add(a + b * c) + d", "(add((a + (b * c))) + d)"),
        ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
        ("add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))", "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))"),
        ("add(a + b + c * d / f + g)", "add((((a + b) + ((c * d) / f)) + g))")
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
    


#[cfg(test)]
pub mod tests {
    use crate::ast:: { AST };
    use crate::lexier:: { Lexier };
    use crate::token:: { TokenKind, Token};
    use crate::parser:: { Parser };
    
    #[derive(Clone)]
    enum Type {
        INT(i64),
        STRING(String),
        BOOLEAN(bool),
    }

    fn test_boolean_literal(exp: AST, expected: bool) -> bool {
        match exp {
            AST::BOOLEAN {..} => (),
            _ => {
                println!("exp not AST::BOOLEAN. got={}", exp.get_kind_literal());
                return false;
            }
        }

        if let AST::BOOLEAN { token, value,..} = exp {
            if value != expected {
                println!("exp.value not {}. got={}", expected, value);
                return false;
            }

            if token.literal != expected.to_string() {
                println!("exp.token.literal not {}. got={}", expected, token.literal);
                return false;
            }
            
        }

        
        true
    }
    
    fn test_integer_literal(exp: AST, expected: i64) ->bool {
        match exp {
            AST::INT_LITERAL {..} => (),
            _        => {
                println!("exp not AST::INT_LITERAL. got={}", exp.get_kind_literal());
                return false;
            }
        }

        if let AST::INT_LITERAL { token, value,..} = exp {
            if value != expected {
                println!("exp.value not {}. got={}", expected, value);
                return false;
            }

            if token.literal != expected.to_string() {
                println!("exp.token.literal not {}. got={}", expected, token.literal);
                return false;
            }
            
        }

        true


    }
    
    fn test_identifier(exp: AST, expected: String) -> bool {
        match exp {
            AST::IDENT {..} => (),
            _ => {
                println!("exp not AST::IDENT. got={}", exp.get_kind_literal());
                return false;
            }
        }

        if let AST::IDENT { token, value,..} = exp {
            if value != expected {
                println!("exp.value not {}. got={}", expected, value);
                return false;
            }

            if token.literal != expected {
                println!("exp.token.literal not {}. got={}", expected, token.literal);
                return false;
            }
            
        }

        true
    }

    fn test_literal_expression(exp: AST, expected: Type) -> bool {
        match expected {
            Type::INT(value)     => test_integer_literal(exp, value),
            Type::STRING(value)  => test_identifier(exp, value),
            Type::BOOLEAN(value) => test_boolean_literal(exp, value),
            _ => {
                println!("type of exp not handled.");
                return false;
            }
        }
    }

    fn test_infix_expression(exp: AST, left: Type, operator: String, right: Type) -> bool {
        match exp {
            AST::INFIX_EXPRESSION {..} => (),
            _  => {
                println!("exp not AST::INFIX_EXPRESSION. got={}", exp.get_kind_literal());
                return false;
            }
        }

        if let AST::INFIX_EXPRESSION { left: lval, operator: op, right: rval, ..} = exp {
            if !test_literal_expression(*lval, left) {
                return false;
            }
            
            if op != operator {
                println!("operator is not {}. got={}", operator, op);
                return false;
            }
            
            if !test_literal_expression(*rval, right) {
                return false;
            }
        }
        
        true
    }

    #[test]
    fn test_parsing_infix_expression() {
    let tests = [
        ("5 + 5;", Type::INT(5), "+", Type::INT(5)),
        ("5 - 5;", Type::INT(5), "-", Type::INT(5)),
        ("5 * 5;", Type::INT(5), "*", Type::INT(5)),
        ("5 / 5;", Type::INT(5), "/", Type::INT(5)),
        ("5 > 5;", Type::INT(5), ">", Type::INT(5)),
        ("5 < 5;", Type::INT(5), "<", Type::INT(5)),
        ("5 == 5;", Type::INT(5), "==", Type::INT(5)),
        ("5 != 5;", Type::INT(5), "!=", Type::INT(5)),
        ("true == true;", Type::BOOLEAN(true), "==", Type::BOOLEAN(true)),
        ("true != false;", Type::BOOLEAN(true), "!=", Type::BOOLEAN(false)),
        ("false == false;", Type::BOOLEAN(false), "==", Type::BOOLEAN(false))
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
                if !test_infix_expression(*expression.clone(), test.1.clone(), test.2.to_string(), test.3.clone()) {
                    panic!("");
                }
            }
        }        
    }    
    }

    #[test]
    fn test_if_expression() {
        let input = "if (x < y) { x }".to_string();
        
        let lexier = Lexier::new(input);
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
                match **expression {
                    AST::IF_EXPRESSION  { ref token, ref condition, ref consequence, ref alternative } => {
                        if !test_infix_expression(*condition.clone(), Type::STRING("x".to_string()), "<".to_string(), Type::STRING("y".to_string())) {
                            panic!("invalide condition");
                        }

                        if let AST::BLOCK_STATEMENT { ref statements, .. } = **consequence {
                            if statements.len() != 1 {
                                panic!("consequence is not 1 statement. got={}", statements.len());
                            }
                        }

                    }
                    _ => panic!("expression not AST::IF_EXPRESSION."),
                }
                
            }
        }
        
    }

    #[test]
    fn test_if_else_expression() {
        let input = "if (x < y) { x } else { y }".to_string();
        
        let lexier = Lexier::new(input);
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
                match **expression {
                    AST::IF_EXPRESSION  { ref token, ref condition, ref consequence, ref alternative } => {
                        if !test_infix_expression(*condition.clone(), Type::STRING("x".to_string()), "<".to_string(), Type::STRING("y".to_string())) {
                            panic!("invalide condition");
                        }

                        if let AST::BLOCK_STATEMENT { ref statements, .. } = **consequence {
                            if statements.len() != 1 {
                                panic!("consequence is not 1 statement. got={}", statements.len());
                            }

                            if let AST::EXPRESSION_STATEMENT { ref expression, .. } = *statements[0] {
                                if !test_identifier(*expression.clone(), "x".to_string()) {
                                    panic!("invalide consequence");
                                }
                            }
                            
                        }

                        if let AST::BLOCK_STATEMENT { ref statements, .. } = **alternative {
                            if statements.len() != 1 {
                                panic!("alternative is not 1 statement. got={}", statements.len());
                            }

                            if let AST::EXPRESSION_STATEMENT { ref expression, .. } = *statements[0] {
                                if !test_identifier(*expression.clone(), "y".to_string()) {
                                    panic!("invalide alternative");
                                }
                            }
                        }
                    }
                    _ => panic!("expression not AST::IF_EXPRESSION."),
                }
                
            }
        }
    }

    #[test]
    fn test_function_literal_parsing() {
        let input = "fn (x, y) { x + y; }".to_string();
        
        let lexier = Lexier::new(input);
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
                match **expression {
                    AST::FUNCTION_LITERAL { ref parameters, ref body, .. } => {
                        for (i, parameter) in parameters.iter().enumerate() {
                            if i == 0 {
                                if !test_identifier(*parameter.clone(), "x".to_string()) {
                                    panic!("invalide parameter.");
                                }
                            }
                            if i == 1 {
                                if !test_identifier(*parameter.clone(), "y".to_string()) {
                                    panic!("invalide parameter.");
                                }
                            }
                        }
                        
                        if let AST::BLOCK_STATEMENT { ref statements, .. } = **body {
                            if statements.len() != 1 {
                                panic!("body is not 1 statement. got={}", statements.len());
                            }
                            
                            if let AST::EXPRESSION_STATEMENT { ref expression, .. } = *statements[0] {
                                if !test_infix_expression( *expression.clone(),
                                                            Type::STRING("x".to_string()),
                                                            "+".to_string(),
                                                            Type::STRING("y".to_string())) {
                                    panic!("invalide body");
                                }
                            }
                        }
                    }
                    _ => panic!("expression not AST::FUNCTION_LITERAL."),
                }
            }
        }
    }
}
