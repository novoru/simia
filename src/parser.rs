use crate::ast:: { Ast };
use crate::lexier:: { Lexier };
use crate::token:: { TokenKind, Token};


#[derive(Debug, Clone)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

pub fn precedences (kind: TokenKind) -> Precedence {
    match kind {
        TokenKind::Eq       => Precedence::Equals,
        TokenKind::NotEq    => Precedence::Equals,
        TokenKind::Lt       => Precedence::LessGreater,
        TokenKind::Gt       => Precedence::LessGreater,
        TokenKind::Plus     => Precedence::Sum,
        TokenKind::Minus    => Precedence::Sum,
        TokenKind::Slash    => Precedence::Product,
        TokenKind::Asterisk => Precedence::Product,
        TokenKind::Lparen   => Precedence::Call,
        _                   => Precedence::Lowest
    }
}

#[derive(Debug, Clone)]
pub struct Parser {
    pub lexier: Lexier,
    cur_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(lexier: Lexier) -> Parser {
        let mut parser = Parser { lexier: lexier,
                                  cur_token:  Token {
                                      kind: TokenKind::Illegal,
                                      literal: "".to_string()},
                                  peek_token: Token {
                                      kind: TokenKind::Illegal,
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
    
    pub fn parse_program(&mut self) -> Option<Ast> {
        let mut program = Ast::Program { statements: Vec::new() };
        
        while self.cur_token.kind.clone() as u8 != TokenKind::Eof.clone() as u8 {
            let statement = match self.parse_statement() {
                Some(value) => Box::new(value),
                None        => {
                    self.next_token();
                    continue;
                }
            };
            
            if let Ast::Program { ref mut statements } = program {
                statements.push(statement);
            }

            self.next_token();
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Ast>{
        match self.cur_token.kind {
            TokenKind::Let    => {
                match self.parse_let_statement() {
                    Some(value) => Some(value),
                    None        => return None,
                }
            }
            TokenKind::Return => {
                match self.parse_return_statement() {
                    Some(value) => Some(value),
                    None        => return None,
                }
            }
            _ => {
                match self.parse_expression_statement() {
                    Some(value) => Some(value),
                    None        => return None,
                }
            }
        }
    }

    fn parse_let_statement(&mut self) -> Option<Ast>{
        let token = self.cur_token.clone();
        
        if !self.expect_peek(TokenKind::Identifier) {
            return None
        }
        
        let ident = Box::new(Ast::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });

        if !self.expect_peek(TokenKind::Assign) {
            return None
        }

        self.next_token();

        let mut value = match self.parse_expression(Precedence::Lowest) {
            Some(value) => Box::new(value),
            None        => return None,
        };
        
        if self.peek_token_is(TokenKind::Semicolon) {
            self.next_token();
        }

        Some( Ast::LetStatement {
            token: token,
            ident: ident,
            value: value,
        })
    }

    fn parse_return_statement(&mut self) -> Option<Ast> {
        let token = self.cur_token.clone();

        self.next_token();

        let return_value = match self.parse_expression(Precedence::Lowest) {
            Some(value) => Box::new(value),
            _           => return None,
        };
        
        if self.peek_token_is(TokenKind::Semicolon) {
            self.next_token();
        }

        Some( Ast::ReturnStatement {
            token: token,
            return_value: return_value,  
        })
    }
        

    fn parse_expression_statement(&mut self) -> Option<Ast> {
        let expression = match self.parse_expression(Precedence::Lowest) {
            Some(value) => Box::new(value),
            _           => return None,
        };

        let statement = Ast::ExpressionStatement {
            token: self.cur_token.clone(),
            expression: expression,
        };

        if self.peek_token_is(TokenKind::Semicolon) {
            self.next_token();
        }

        Some(statement)
    }

    fn parse_expression (&mut self, precedence: Precedence) -> Option<Ast> {
        let mut left_exp = Ast::Expression {
            token : Token {
                kind: TokenKind::Illegal,
                literal: "".to_string()
            }
        };
        
        match self.cur_token.kind {
            TokenKind::Identifier    {..}  => {
                left_exp = match self.parse_identifier() {
                    Some(value) => value,
                    None        => return None,
                };
            },
            TokenKind::Integer  {..}  => {
                left_exp = match self.parse_integer_literal() {
                    Some(value) => value,
                    None        => return None,
                };
            }
            TokenKind::Bang     {..} |
            TokenKind::Minus    {..}  => {
                left_exp = match self.parse_prefix_expression() {
                    Some(value) => value,
                    None        => return None,
                };
            }
            TokenKind::True     {..} |
            TokenKind::False    {..}  => {
                left_exp = match self.parse_boolean() {
                    Some(value) => value,
                    None        => return None,
                };
            }
            TokenKind::Lparen   {..}  => {
                left_exp = match self.parse_grouped_expression() {
                    Some(value) => value,
                    None        => return None,
                };
            }
            TokenKind::If       {..}  => {
                left_exp = match self.parse_if_expression() {
                    Some(value) => value,
                    None        => return None,
                };
            }
            TokenKind::Function {..}  => {
                left_exp = match self.parse_function_literal() {
                    Some(value) => value,
                    None        => return None,
                }
            }
            _ => {
                self.no_prefix_parse_fn_error(self.cur_token.kind);
                return None;
            }
        }
        
        while !self.peek_token_is(TokenKind::Semicolon) && (precedence.clone() as u8) < (self.peek_precedence() as u8) {
            match self.peek_token.kind {
                TokenKind::Plus     {..} |
                TokenKind::Minus    {..} |
                TokenKind::Slash    {..} |
                TokenKind::Asterisk {..} |
                TokenKind::Eq       {..} |
                TokenKind::NotEq    {..} |
                TokenKind::Lt       {..} |
                TokenKind::Gt       {..} => {
                    self.next_token();
                    left_exp = match self.parse_infix_expression(Box::new(left_exp)) {
                        Some(value) => value,
                        None        => return None,
                    };
                },
                TokenKind::Lparen   {..} => {
                    self.next_token();
                    left_exp = match self.parse_call_expression(Box::new(left_exp)) {
                        Some(value) => value,
                        None        => return None,
                    };
                },
                _ => {
                    return Some(left_exp);
                }
            }

        }

        Some(left_exp)
    }

    fn parse_identifier(&mut self) -> Option<Ast> {
        Some(Ast::Identifier { token: self.cur_token.clone(), value: self.cur_token.literal.clone() })
    }

    fn parse_integer_literal(&mut self) -> Option<Ast> {

        let value = match self.cur_token.literal.clone().parse::<i64>() {
            Ok(value) => value,
            Err(_)    => return None,
        };

        Some(Ast::IntegerLiteral {
            token: self.cur_token.clone(),
            value: value,
        })
    }

    fn parse_prefix_expression(&mut self) -> Option<Ast>{
        let token = self.cur_token.clone();
        let operator = self.cur_token.clone().literal;

        self.next_token();

        let right = match self.parse_expression(Precedence::Prefix) {
            Some(value) => Box::new(value),
            None        => return None,
        };
                
        Some(Ast::PrefixExpression {
            token: token,
            operator: operator,
            right: right,
        })
    }

    fn parse_infix_expression(&mut self, left: Box<Ast>) -> Option<Ast>{
        let mut expression = Ast::InfixExpression {
            token: self.cur_token.clone(),
            left: left.clone(),
            operator: self.cur_token.literal.clone(),
            right: Box::new(
                Ast::Expression {
                    token:
                    Token {
                        kind: TokenKind::Illegal,
                        literal: "".to_string(),
                    }
                }
            ),
        };

        let precedence = self.cur_precedence();
        self.next_token();

        if let Ast::InfixExpression { ref mut right, ..} = expression {
            *right = match self.parse_expression(precedence) {
                Some(value) => Box::new(value),
                _           => return None
            };
        }

        Some(expression)
        
    }

    fn parse_boolean(&mut self) -> Option<Ast> {
        Some(Ast::Boolean {
            token: self.cur_token.clone(),
            value: self.cur_token_is(TokenKind::True),
        })
    }

    fn parse_grouped_expression(&mut self) -> Option<Ast> {
        self.next_token();
        let expression = match self.parse_expression(Precedence::Lowest) {
            Some(value) => value,
            _           => return None,
        };

        if !self.expect_peek(TokenKind::Rparen) {
            return None;
        }

        Some(expression)
    }

    fn parse_if_expression(&mut self) -> Option<Ast> {
        let empty_expression = Ast::Expression {
            token:
            Token {
                kind: TokenKind::Illegal,
                literal: "".to_string(),
            }
        };
        let mut expression = Ast::IfExpression {
            token: self.cur_token.clone(),
            condition: Box::new(empty_expression.clone()),
            consequence: Box::new(empty_expression.clone()),
            alternative: Box::new(empty_expression.clone()),
        };

        if !self.expect_peek(TokenKind::Lparen) {
            return None;
        }

        self.next_token();

        if let Ast::IfExpression { ref mut condition, .. } = expression {
            *condition = match self.parse_expression(Precedence::Lowest) {
                Some(value) => Box::new(value),
                _           => return None,
            };
        };

        if !self.expect_peek(TokenKind::Rparen) {
            return None;
        }

        if !self.expect_peek(TokenKind::Lbrace) {
            return None;
        }

        if let Ast::IfExpression { ref mut consequence, .. } = expression {
            *consequence = match self.parse_block_statement() {
                Some(value) => Box::new(value),
                _           => return None,
            };
        };

        if self.peek_token_is(TokenKind::Else) {
            self.next_token();

            if !self.expect_peek(TokenKind::Lbrace) {
                return None;
            }

            if let Ast::IfExpression { ref mut alternative, .. } = expression {
                *alternative = match self.parse_block_statement() {
                    Some(value) => Box::new(value),
                    _           => return None,
                };
            }
        }

        Some(expression)
    }

    fn parse_block_statement(&mut self) -> Option<Ast> {
        let mut block = Ast::BlockStatement {
            token: self.cur_token.clone(),
            statements: Vec::new(),
        };

        self.next_token();

        while !self.cur_token_is(TokenKind::Rbrace) && !self.cur_token_is(TokenKind::Eof) {
            let mut statement = match self.parse_statement() {
                Some(value) => value,
                _           => return None,
            };
            
            match &statement {
                Ast    => {
                    if let Ast::BlockStatement { ref mut statements, ..} = block {
                        statements.push(Box::new(statement));
                    };
                },
                _ => return None,
            }
            
            self.next_token();
        }

        Some(block)
    }

    fn parse_function_literal(&mut self) -> Option<Ast> {
        let mut literal = Ast::FunctionLiteral {
            token: self.cur_token.clone(),
            parameters: Vec::new(),
            body: Box::new(
                Ast::Expression {
                token:
                Token {
                    kind: TokenKind::Illegal,
                    literal: "".to_string(),
                }
            }),
        };

        if !self.expect_peek(TokenKind::Lparen) {
            return None;
        }
        
        if let Ast::FunctionLiteral { ref mut parameters, .. } = literal {
            *parameters = match self.parse_function_parameters() {
                Some(value) => value,
                _           => return None,
            };
        }
        
        if !self.expect_peek(TokenKind::Lbrace) {
            return None;
        }

        if let Ast::FunctionLiteral { ref mut body, .. } = literal {
            *body = match self.parse_block_statement() {
                Some(value) => Box::new(value),
                _           => return None,
            };
        }

        Some(literal)
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Box<Ast>>> {
        let mut identifiers = Vec::new();

        if self.peek_token_is(TokenKind::Rparen) {
            self.next_token();
            return Some(identifiers);
        }

        self.next_token();

        let mut identifier = Ast::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        identifiers.push(Box::new(identifier));

        while self.peek_token_is(TokenKind::Comma) {
            self.next_token();
            self.next_token();

            identifier = Ast::Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            };

            identifiers.push(Box::new(identifier));
        }

        if !self.expect_peek(TokenKind::Rparen) {
            return None;
        }

        Some(identifiers)
    }

    fn parse_call_expression(&mut self, function: Box<Ast> ) -> Option<Ast> {
        let expression = Ast::CallExpression {
            token: self.cur_token.clone(),
            function:  function,
            arguments: self.parse_call_arguments(), 
        };
        
        Some(expression)
    }

    fn parse_call_arguments(&mut self) -> Vec<Box<Ast>> {
        let mut arguments = Vec::new();

        if self.peek_token_is(TokenKind::Rparen) {
            self.next_token();
            return arguments;
        }

        self.next_token();
        let mut argument = match self.parse_expression(Precedence::Lowest) {
            Some(value) => Box::new(value),
            _           => return Vec::new(),
        };
        
        arguments.push(argument);

        while self.peek_token_is(TokenKind::Comma) {
            self.next_token();
            self.next_token();

            argument = match self.parse_expression(Precedence::Lowest) {
                Some(value) => Box::new(value),
                _           => return Vec::new(),
            };
            
            arguments.push(argument);
        }

        if !self.expect_peek(TokenKind::Rparen) {
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

    fn cur_precedence(&mut self) -> Precedence {
        precedences(self.cur_token.kind)
    }
    
    fn peek_precedence(&mut self) -> Precedence {
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

    fn no_prefix_parse_fn_error(&mut self, kind: TokenKind) {
        let msg = format!("no prefix parse function for {} found", kind.get_kind_literal());
        self.errors.push(msg);
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
        Ast::Program { ref statements } if statements.len() == 3 => (),
        Ast::Program { ref statements } =>
            panic!("program does not contain 3 statements. got={}", statements.len()),
        _ => panic!("parse_program() returned None. ")
    }

    let expected_let_statements = [  ( "x".to_string(), 5),
                                        ( "y".to_string(), 10),
                                        ( "foobar".to_string(), 838383)
    ];

    for (i, expected) in expected_let_statements.iter().enumerate() {
        if let Ast::Program { ref mut statements } = program {
            if let  Ast::LetStatement { ref mut ident, .. } = *statements[i] {
                if let Ast::Identifier { ref value, .. } = **ident {
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
    //parser.check_parser_errors();

    match program {
        Ast::Program { ref statements } if statements.len() == 3 => (),
        Ast::Program { ref statements } =>
            panic!("program does not contain 3 statements. got={}", statements.len()),
        _ => panic!("parse_program() returned None. ")
    }
    
    let expected_return_value = [ (5),
                                   (10),
                                   (838383)
    ];

    for (i, _) in expected_return_value.iter().enumerate() {
        if let Ast::Program { ref statements } = program {
            if let Ast::ReturnStatement { ref token, .. } = *statements[i] {
                //: TODO check return value
                println!("token.literal:\t{}", token.literal);
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
        Ast::Program { ref statements } if statements.len() == 2 => (),
        Ast::Program { ref statements } =>
            panic!("program does not contain 2 statements. got={}", statements.len()),
        _ => panic!("parse_program() returned None. ")
    }
    
    let expected_value = [ 5,
                           12345
    ];

    for (i, expected) in expected_value.iter().enumerate() {
        if let Ast::Program { ref statements } = program {
            if let Ast::IntegerLiteral { ref value, .. } = *statements[i] {
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
            Ast::Program { ref statements } if statements.len() == 1 => (),
            Ast::Program { ref statements } =>
                panic!("program does not contain 1 statements. got={}", statements.len()),
            _ => panic!("parse_program() returned None. ")
        }
        
        if let Ast::Program { ref statements } = program {
            if let Ast::ExpressionStatement { ref expression, .. } = *statements[0] {
                if let Ast::PrefixExpression { ref operator, ref right, ..} = **expression {
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
        ("add(a + b + c * d / f + g)", "add((((a + b) + ((c * d) / f)) + g))"),
    ];

    for (_i, test) in tests.iter().enumerate() {

        let lexier = Lexier::new((*test.clone().0).to_string());
        let mut parser = Parser::new(lexier);

        let program = parser.parse_program().unwrap();
        parser.check_parser_errors();
        
        if let Ast::Program { .. } = program {
            assert_eq!(program.to_string(), (*test.clone().1).to_string());
        }        

    }
}
    


#[cfg(test)]
pub mod tests {
    use crate::ast:: { Ast };
    use crate::lexier:: { Lexier };
    use crate::token:: { TokenKind, Token};
    use crate::parser:: { Parser };
    
    #[derive(Clone)]
    enum Type {
        INT(i64),
        STRING(String),
        Boolean(bool),
    }

    fn test_boolean_literal(exp: Ast, expected: bool) -> bool {
        match exp {
            Ast::Boolean {..} => (),
            _ => {
                println!("exp not Ast::Boolean. got={}", exp.get_kind_literal());
                return false;
            }
        }

        if let Ast::Boolean { token, value,..} = exp {
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
    
    fn test_integer_literal(exp: Ast, expected: i64) ->bool {
        match exp {
            Ast::IntegerLiteral {..} => (),
            _        => {
                println!("exp not Ast::IntegerLiteral. got={}", exp.get_kind_literal());
                return false;
            }
        }

        if let Ast::IntegerLiteral { token, value,..} = exp {
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
    
    fn test_identifier(exp: Ast, expected: String) -> bool {
        match exp {
            Ast::Identifier {..} => (),
            _ => {
                println!("exp not Ast::Identifier. got={}", exp.get_kind_literal());
                return false;
            }
        }

        if let Ast::Identifier { token, value,..} = exp {
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

    fn test_literal_expression(exp: Ast, expected: Type) -> bool {
        match expected {
            Type::INT(value)     => test_integer_literal(exp, value),
            Type::STRING(value)  => test_identifier(exp, value),
            Type::Boolean(value) => test_boolean_literal(exp, value),
            _ => {
                println!("type of exp not handled.");
                return false;
            }
        }
    }

    fn test_infix_expression(exp: Ast, left: Type, operator: String, right: Type) -> bool {
        match exp {
            Ast::InfixExpression {..} => (),
            _  => {
                println!("exp not Ast::InfixExpression. got={}", exp.get_kind_literal());
                return false;
            }
        }

        if let Ast::InfixExpression { left: lval, operator: op, right: rval, ..} = exp {
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
        ("true == true;", Type::Boolean(true), "==", Type::Boolean(true)),
        ("true != false;", Type::Boolean(true), "!=", Type::Boolean(false)),
        ("false == false;", Type::Boolean(false), "==", Type::Boolean(false))
    ];

    for (_i, test) in tests.iter().enumerate() {

        let lexier = Lexier::new((*test.clone().0).to_string());
        let mut parser = Parser::new(lexier);

        let program = parser.parse_program().unwrap();
        parser.check_parser_errors();

        match program {
            Ast::Program { ref statements } if statements.len() == 1 => (),
            Ast::Program { ref statements } =>
                panic!("program does not contain 1 statements. got={}", statements.len()),
            _ => panic!("parse_program() returned None. ")
        }
        
        if let Ast::Program { ref statements } = program {
            if let Ast::ExpressionStatement { ref expression, .. } = *statements[0] {
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
            Ast::Program { ref statements } if statements.len() == 1 => (),
            Ast::Program { ref statements } =>
                panic!("program does not contain 1 statements. got={}", statements.len()),
            _ => panic!("parse_program() returned None. ")
        }

        if let Ast::Program { ref statements } = program {
            if let Ast::ExpressionStatement { ref expression, .. } = *statements[0] {
                match **expression {
                    Ast::IfExpression  { ref token, ref condition, ref consequence, ref alternative } => {
                        if !test_infix_expression(*condition.clone(), Type::STRING("x".to_string()), "<".to_string(), Type::STRING("y".to_string())) {
                            panic!("invalide condition");
                        }

                        if let Ast::BlockStatement { ref statements, .. } = **consequence {
                            if statements.len() != 1 {
                                panic!("consequence is not 1 statement. got={}", statements.len());
                            }
                        }

                    }
                    _ => panic!("expression not Ast::IfExpression."),
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
            Ast::Program { ref statements } if statements.len() == 1 => (),
            Ast::Program { ref statements } =>
                panic!("program does not contain 1 statements. got={}", statements.len()),
            _ => panic!("parse_program() returned None. ")
        }

        if let Ast::Program { ref statements } = program {
            if let Ast::ExpressionStatement { ref expression, .. } = *statements[0] {
                match **expression {
                    Ast::IfExpression  { ref token, ref condition, ref consequence, ref alternative } => {
                        if !test_infix_expression(*condition.clone(), Type::STRING("x".to_string()), "<".to_string(), Type::STRING("y".to_string())) {
                            panic!("invalide condition");
                        }

                        if let Ast::BlockStatement { ref statements, .. } = **consequence {
                            if statements.len() != 1 {
                                panic!("consequence is not 1 statement. got={}", statements.len());
                            }

                            if let Ast::ExpressionStatement { ref expression, .. } = *statements[0] {
                                if !test_identifier(*expression.clone(), "x".to_string()) {
                                    panic!("invalide consequence");
                                }
                            }
                            
                        }

                        if let Ast::BlockStatement { ref statements, .. } = **alternative {
                            if statements.len() != 1 {
                                panic!("alternative is not 1 statement. got={}", statements.len());
                            }

                            if let Ast::ExpressionStatement { ref expression, .. } = *statements[0] {
                                if !test_identifier(*expression.clone(), "y".to_string()) {
                                    panic!("invalide alternative");
                                }
                            }
                        }
                    }
                    _ => panic!("expression not Ast::IfExpression."),
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
            Ast::Program { ref statements } if statements.len() == 1 => (),
            Ast::Program { ref statements } =>
                panic!("program does not contain 1 statements. got={}", statements.len()),
            _ => panic!("parse_program() returned None. ")
        }

        if let Ast::Program { ref statements } = program {
            if let Ast::ExpressionStatement { ref expression, .. } = *statements[0] {
                match **expression {
                    Ast::FunctionLiteral { ref parameters, ref body, .. } => {
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
                        
                        if let Ast::BlockStatement { ref statements, .. } = **body {
                            if statements.len() != 1 {
                                panic!("body is not 1 statement. got={}", statements.len());
                            }
                            
                            if let Ast::ExpressionStatement { ref expression, .. } = *statements[0] {
                                if !test_infix_expression( *expression.clone(),
                                                            Type::STRING("x".to_string()),
                                                            "+".to_string(),
                                                            Type::STRING("y".to_string())) {
                                    panic!("invalide body");
                                }
                            }
                        }
                    }
                    _ => panic!("expression not Ast::FunctionLiteral."),
                }
            }
        }
    }
}
