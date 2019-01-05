use crate::token::{ TokenKind, Token };

/// Lexical Analyzer
#[derive(Debug, Clone)]
pub struct Lexier {
    input: String, 
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexier {
    pub fn new(input: String) -> Lexier {
        let mut lexier: Lexier;
        lexier = Lexier { input: input,
                          position: 0,
                          read_position: 0,
                          ch: ' ',
        };

        lexier.read_char();

        lexier
    }

    /// Tokenize input string 
    pub fn next_token(&mut self) -> Token {
        let token: Token;
        self.skip();
        
        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    token = Token {kind:TokenKind::Eq, literal: "==".to_string()};
                    self.read_char();
                }
                else {
                    token = Token {kind: TokenKind::Assign, literal: self.ch.to_string()};
                }
            },
            '+' => token = Token {kind: TokenKind::Plus, literal: self.ch.to_string()},
            '-' => token = Token {kind: TokenKind::Minus, literal: self.ch.to_string()},
            '!' => {
                if self.peek_char() == '=' {
                    token = Token {kind: TokenKind::NotEq, literal: "!=".to_string()};
                    self.read_char();
                }
                else {
                    token = Token {kind: TokenKind::Bang, literal: self.ch.to_string()};
                }
            },
            '*' => token = Token {kind: TokenKind::Asterisk, literal: self.ch.to_string()},
            '/' => token = Token {kind: TokenKind::Slash, literal: self.ch.to_string()},
            '<' => token = Token {kind: TokenKind::Lt, literal: self.ch.to_string()},
            '>' => token = Token {kind: TokenKind::Gt,  literal: self.ch.to_string()},
            '(' => token = Token {kind: TokenKind::Lparen, literal: self.ch.to_string()},
            ')' => token = Token {kind: TokenKind::Rparen, literal: self.ch.to_string()},
            '{' => token = Token {kind: TokenKind::Lbrace, literal: self.ch.to_string()},
            '}' => token = Token {kind: TokenKind::Rbrace, literal: self.ch.to_string()},
            ',' => token = Token {kind: TokenKind::Comma, literal: self.ch.to_string()},
            ';' => token = Token {kind: TokenKind::Semicolon, literal: self.ch.to_string()},
            '\0' => token = Token {kind: TokenKind::Eof, literal: "".to_string()},
            'a'...'z' | 'A' ... 'Z' | '_' => {
                let ident = self.read_identifier();
                return self.lookup_ident(&ident)
            },
            '0' ... '9' => return Token {kind: TokenKind::Integer, literal: self.read_integer()},
            '"' => token = Token {kind: TokenKind::String, literal: self.read_string()},
            _  => token = Token {kind: TokenKind::Illegal, literal: self.ch.to_string()},
        }

        self.read_char();
        token
    }

    /// Check whether ident is keywords, and return the suitable token. 
    fn lookup_ident(&mut self, ident: &str) -> Token {
        match ident {
            "fn" => Token {kind: TokenKind::Function, literal: ident.to_string()},
            "let" => Token {kind: TokenKind::Let, literal: ident.to_string()},
            "true" => Token {kind: TokenKind::True, literal: ident.to_string()},
            "false" => Token {kind: TokenKind::False, literal: ident.to_string()},
            "if" => Token {kind: TokenKind::If, literal: ident.to_string()},
            "else" => Token {kind: TokenKind::Else, literal: ident.to_string()},
            "return" => Token {kind: TokenKind::Return, literal: ident.to_string()},
            _ => Token {kind: TokenKind::Identifier, literal: ident.to_string()}
        }
    }

    /// Increment current position 
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        }
        else {
            self.ch = self.input.chars()
                .skip(self.read_position).next().unwrap()
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Read current character as identifier
    fn read_identifier(&mut self) -> String {
        let mut identifier = "".to_string();
        while self.ch.is_alphabetic() || self.ch == '_' {
            identifier.push(self.ch);
            self.read_char();
        }
 
        identifier
    }

    /// Read current character as integer
    fn read_integer(&mut self) -> String {
        let mut integer = "".to_string();
        while self.ch.is_digit(10) {
            integer.push(self.ch);
            self.read_char();
        }

        integer
    }

    /// Read string literal
    fn read_string(&mut self) -> String {
        let position = self.position + 1;
        let mut string = String::new();

        self.read_char();
        
        loop {
            if self.ch == '"' || self.ch == '\0' {
                break;
            }
            string.push(self.ch);
            self.read_char();
        }

        string
    }
    
    /// Read peek character
    fn peek_char(&mut self) -> char{
        if self.read_position >= self.input.len() {
            return '\0'
        }
        self.input.chars().nth(self.read_position).unwrap()
    }
    
    /// Skip meaningless character (e.x. whitespace)
    fn skip(&mut self) {
        if self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
    
}


#[test]
fn test_next_token() {
    let input = "\
foo;\
bar;\
let five = 5;\
let ten = 10;\
\
let add = fn(x, y) {\
x + y;\
};\
let result = add(five, ten);\
!-/*5 ;\
5 < 10 > 5;\
if( 5 < 10 ) {\
return true;\
} else {\
return false;\
}\
10 == 10;\
10 != 9;\
\"foobar\"\
\"foo bar\"\
\"\"\
".to_string();

    let tests = [ Token { kind: TokenKind::Identifier, literal: "foo".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },
                  
                  Token { kind: TokenKind::Identifier, literal: "bar".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },
                  
                  Token { kind: TokenKind::Let, literal: "let".to_string() },
                  Token { kind: TokenKind::Identifier, literal: "five".to_string() },
                  Token { kind: TokenKind::Assign, literal: "=".to_string() },
                  Token { kind: TokenKind::Integer, literal: "5".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },
                  
                  Token { kind: TokenKind::Let, literal: "let".to_string() },
                  Token { kind: TokenKind::Identifier, literal: "ten".to_string() },
                  Token { kind: TokenKind::Assign, literal: "=".to_string() },
                  Token { kind: TokenKind::Integer, literal: "10".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

                  Token { kind: TokenKind::Let, literal: "let".to_string() },
                  Token { kind: TokenKind::Identifier, literal: "add".to_string() },
                  Token { kind: TokenKind::Assign, literal: "=".to_string() },
                  Token { kind: TokenKind::Function, literal: "fn".to_string() },
                  Token { kind: TokenKind::Lparen, literal: "(".to_string() },
                  Token { kind: TokenKind::Identifier, literal: "x".to_string() },
                  Token { kind: TokenKind::Comma, literal: ",".to_string() },
                  Token { kind: TokenKind::Identifier, literal: "y".to_string() },
                  Token { kind: TokenKind::Rparen, literal: ")".to_string() },
                  Token { kind: TokenKind::Lbrace, literal: "{".to_string() },

                  Token { kind: TokenKind::Identifier, literal: "x".to_string() },
                  Token { kind: TokenKind::Plus, literal: "+".to_string() },
                  Token { kind: TokenKind::Identifier, literal: "y".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

                  Token { kind: TokenKind::Rbrace, literal: "}".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },
                  
                  Token { kind: TokenKind::Let, literal: "let".to_string() },
                  Token { kind: TokenKind::Identifier, literal: "result".to_string() },
                  Token { kind: TokenKind::Assign, literal: "=".to_string() },
                  Token { kind: TokenKind::Identifier, literal: "add".to_string() },
                  Token { kind: TokenKind::Lparen, literal: "(".to_string() },
                  Token { kind: TokenKind::Identifier, literal: "five".to_string() },
                  Token { kind: TokenKind::Comma, literal: ",".to_string() },
                  Token { kind: TokenKind::Identifier, literal: "ten".to_string() },
                  Token { kind: TokenKind::Rparen, literal: ")".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },
                  
                  Token { kind: TokenKind::Bang, literal: "!".to_string() },
                  Token { kind: TokenKind::Minus, literal: "-".to_string() },
                  Token { kind: TokenKind::Slash, literal: "/".to_string() },
                  Token { kind: TokenKind::Asterisk, literal: "*".to_string() },
                  Token { kind: TokenKind::Integer, literal: "5".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

                  Token { kind: TokenKind::Integer, literal: "5".to_string() },
                  Token { kind: TokenKind::Lt, literal: "<".to_string() },
                  Token { kind: TokenKind::Integer, literal: "10".to_string() },
                  Token { kind: TokenKind::Gt, literal: ">".to_string() },
                  Token { kind: TokenKind::Integer, literal: "5".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

                  Token { kind: TokenKind::If, literal: "if".to_string() },
                  Token { kind: TokenKind::Lparen, literal: "(".to_string() },
                  Token { kind: TokenKind::Integer, literal: "5".to_string() },
                  Token { kind: TokenKind::Lt, literal: "<".to_string() },
                  Token { kind: TokenKind::Integer, literal: "10".to_string() },
                  Token { kind: TokenKind::Rparen, literal: ")".to_string() },
                  Token { kind: TokenKind::Lbrace, literal: "{".to_string() },

                  Token { kind: TokenKind::Return, literal: "return".to_string() },
                  Token { kind: TokenKind::True, literal: "true".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

                  Token { kind: TokenKind::Rbrace, literal: "}".to_string() },
                  Token { kind: TokenKind::Else, literal: "else".to_string() },
                  Token { kind: TokenKind::Lbrace, literal: "{".to_string() },
                  
                  Token { kind: TokenKind::Return, literal: "return".to_string() },
                  Token { kind: TokenKind::False, literal: "false".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },
                  
                  Token { kind: TokenKind::Rbrace, literal: "}".to_string() },

                  Token { kind: TokenKind::Integer, literal: "10".to_string() },
                  Token { kind: TokenKind::Eq, literal: "==".to_string() },
                  Token { kind: TokenKind::Integer, literal: "10".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

                  Token { kind: TokenKind::Integer, literal: "10".to_string() },
                  Token { kind: TokenKind::NotEq, literal: "!=".to_string() },
                  Token { kind: TokenKind::Integer, literal: "9".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

                  Token { kind: TokenKind::String, literal: "foobar".to_string() },
                  Token { kind: TokenKind::String, literal: "foo bar".to_string() },
                  Token { kind: TokenKind::String, literal: "".to_string() },
                  Token { kind: TokenKind::Eof, literal: "".to_string() }
                  
    ];
    
    let mut lexier = Lexier::new(input);
    let mut token: Token;
    
    for test in tests.iter() {
        token = lexier.next_token();
        //println!("input:\t{{ kind: {}, literal: {} }}", token.get_kind_literal(), token.literal);
        //println!("test:\t{{ kind: {}, literal: {} }}", test.get_kind_literal(), test.literal);
        if token.get_kind_literal() != test.get_kind_literal() {
            panic!("token.kind not {}. got={}", test.get_kind_literal(), token.get_kind_literal());
        }

        if token.literal != test.literal {
            panic!("token.literal not {}. got={}", test.literal, token.literal);
        }
    }
    
}
