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
                    token = Token {kind:TokenKind::EQ, literal: "==".to_string()};
                    self.read_char();
                }
                else {
                    token = Token {kind: TokenKind::ASSIGN, literal: self.ch.to_string()};
                }
            },
            '+' => token = Token {kind: TokenKind::PLUS, literal: self.ch.to_string()},
            '-' => token = Token {kind: TokenKind::MINUS, literal: self.ch.to_string()},
            '!' => {
                if self.peek_char() == '=' {
                    token = Token {kind: TokenKind::NOT_EQ, literal: "!=".to_string()};
                    self.read_char();
                }
                else {
                    token = Token {kind: TokenKind::BANG, literal: self.ch.to_string()};
                }
            },
            '*' => token = Token {kind: TokenKind::ASTERISK, literal: self.ch.to_string()},
            '/' => token = Token {kind: TokenKind::SLASH, literal: self.ch.to_string()},
            '<' => token = Token {kind: TokenKind::LT, literal: self.ch.to_string()},
            '>' => token = Token {kind: TokenKind::GT,  literal: self.ch.to_string()},
            '(' => token = Token {kind: TokenKind::LPAREN, literal: self.ch.to_string()},
            ')' => token = Token {kind: TokenKind::RPAREN, literal: self.ch.to_string()},
            '{' => token = Token {kind: TokenKind::LBRACE, literal: self.ch.to_string()},
            '}' => token = Token {kind: TokenKind::RBRACE, literal: self.ch.to_string()},
            ',' => token = Token {kind: TokenKind::COMMA, literal: self.ch.to_string()},
            ';' => token = Token {kind: TokenKind::SEMICOLON, literal: self.ch.to_string()},
            '\0' => token = Token {kind: TokenKind::EOF, literal: "".to_string()},
            'a'...'z' | 'A' ... 'Z' | '_' => {
                let ident = self.read_identifier();
                return self.lookup_ident(&ident)
            },
            '0' ... '9' => return Token {kind: TokenKind::INT, literal: self.read_integer()},
            _  => token = Token {kind: TokenKind::ILLEGAL, literal: self.ch.to_string()},
        }

        self.read_char();
        token
    }

    /// Check whether ident is keywords, and return the suitable token. 
    fn lookup_ident(&mut self, ident: &str) -> Token {
        match ident {
            "fn" => Token {kind: TokenKind::FUNCTION, literal: ident.to_string()},
            "let" => Token {kind: TokenKind::LET, literal: ident.to_string()},
            "true" => Token {kind: TokenKind::TRUE, literal: ident.to_string()},
            "false" => Token {kind: TokenKind::FALSE, literal: ident.to_string()},
            "if" => Token {kind: TokenKind::IF, literal: ident.to_string()},
            "else" => Token {kind: TokenKind::ELSE, literal: ident.to_string()},
            "return" => Token {kind: TokenKind::RETURN, literal: ident.to_string()},
            _ => Token {kind: TokenKind::IDENT, literal: ident.to_string()}
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
".to_string();

    let tests = [ Token { kind: TokenKind::IDENT, literal: "foo".to_string() },
                  Token { kind: TokenKind::SEMICOLON, literal: ";".to_string() },
                  
                  Token { kind: TokenKind::IDENT, literal: "bar".to_string() },
                  Token { kind: TokenKind::SEMICOLON, literal: ";".to_string() },
                  
                  Token { kind: TokenKind::LET, literal: "let".to_string() },
                  Token { kind: TokenKind::IDENT, literal: "five".to_string() },
                  Token { kind: TokenKind::ASSIGN, literal: "=".to_string() },
                  Token { kind: TokenKind::INT, literal: "5".to_string() },
                  Token { kind: TokenKind::SEMICOLON, literal: ";".to_string() },
                  
                  Token { kind: TokenKind::LET, literal: "let".to_string() },
                  Token { kind: TokenKind::IDENT, literal: "ten".to_string() },
                  Token { kind: TokenKind::ASSIGN, literal: "=".to_string() },
                  Token { kind: TokenKind::INT, literal: "10".to_string() },
                  Token { kind: TokenKind::SEMICOLON, literal: ";".to_string() },

                  Token { kind: TokenKind::LET, literal: "let".to_string() },
                  Token { kind: TokenKind::IDENT, literal: "add".to_string() },
                  Token { kind: TokenKind::ASSIGN, literal: "=".to_string() },
                  Token { kind: TokenKind::FUNCTION, literal: "fn".to_string() },
                  Token { kind: TokenKind::LPAREN, literal: "(".to_string() },
                  Token { kind: TokenKind::IDENT, literal: "x".to_string() },
                  Token { kind: TokenKind::COMMA, literal: ",".to_string() },
                  Token { kind: TokenKind::IDENT, literal: "y".to_string() },
                  Token { kind: TokenKind::RPAREN, literal: ")".to_string() },
                  Token { kind: TokenKind::LBRACE, literal: "{".to_string() },

                  Token { kind: TokenKind::IDENT, literal: "x".to_string() },
                  Token { kind: TokenKind::PLUS, literal: "+".to_string() },
                  Token { kind: TokenKind::IDENT, literal: "y".to_string() },
                  Token { kind: TokenKind::SEMICOLON, literal: ";".to_string() },

                  Token { kind: TokenKind::RBRACE, literal: "}".to_string() },
                  Token { kind: TokenKind::SEMICOLON, literal: ";".to_string() },
                  
                  Token { kind: TokenKind::LET, literal: "let".to_string() },
                  Token { kind: TokenKind::IDENT, literal: "result".to_string() },
                  Token { kind: TokenKind::ASSIGN, literal: "=".to_string() },
                  Token { kind: TokenKind::IDENT, literal: "add".to_string() },
                  Token { kind: TokenKind::LPAREN, literal: "(".to_string() },
                  Token { kind: TokenKind::IDENT, literal: "five".to_string() },
                  Token { kind: TokenKind::COMMA, literal: ",".to_string() },
                  Token { kind: TokenKind::IDENT, literal: "ten".to_string() },
                  Token { kind: TokenKind::RPAREN, literal: ")".to_string() },
                  Token { kind: TokenKind::SEMICOLON, literal: ";".to_string() },
                  
                  Token { kind: TokenKind::BANG, literal: "!".to_string() },
                  Token { kind: TokenKind::MINUS, literal: "-".to_string() },
                  Token { kind: TokenKind::SLASH, literal: "/".to_string() },
                  Token { kind: TokenKind::ASTERISK, literal: "*".to_string() },
                  Token { kind: TokenKind::INT, literal: "5".to_string() },
                  Token { kind: TokenKind::SEMICOLON, literal: ";".to_string() },

                  Token { kind: TokenKind::INT, literal: "5".to_string() },
                  Token { kind: TokenKind::LT, literal: "<".to_string() },
                  Token { kind: TokenKind::INT, literal: "10".to_string() },
                  Token { kind: TokenKind::GT, literal: ">".to_string() },
                  Token { kind: TokenKind::INT, literal: "5".to_string() },
                  Token { kind: TokenKind::SEMICOLON, literal: ";".to_string() },

                  Token { kind: TokenKind::IF, literal: "if".to_string() },
                  Token { kind: TokenKind::LPAREN, literal: "(".to_string() },
                  Token { kind: TokenKind::INT, literal: "5".to_string() },
                  Token { kind: TokenKind::LT, literal: "<".to_string() },
                  Token { kind: TokenKind::INT, literal: "10".to_string() },
                  Token { kind: TokenKind::RPAREN, literal: ")".to_string() },
                  Token { kind: TokenKind::LBRACE, literal: "{".to_string() },

                  Token { kind: TokenKind::RETURN, literal: "return".to_string() },
                  Token { kind: TokenKind::TRUE, literal: "true".to_string() },
                  Token { kind: TokenKind::SEMICOLON, literal: ";".to_string() },

                  Token { kind: TokenKind::RBRACE, literal: "}".to_string() },
                  Token { kind: TokenKind::ELSE, literal: "else".to_string() },
                  Token { kind: TokenKind::LBRACE, literal: "{".to_string() },
                  
                  Token { kind: TokenKind::RETURN, literal: "return".to_string() },
                  Token { kind: TokenKind::FALSE, literal: "false".to_string() },
                  Token { kind: TokenKind::SEMICOLON, literal: ";".to_string() },
                  
                  Token { kind: TokenKind::RBRACE, literal: "}".to_string() },

                  Token { kind: TokenKind::INT, literal: "10".to_string() },
                  Token { kind: TokenKind::EQ, literal: "==".to_string() },
                  Token { kind: TokenKind::INT, literal: "10".to_string() },
                  Token { kind: TokenKind::SEMICOLON, literal: ";".to_string() },

                  Token { kind: TokenKind::INT, literal: "10".to_string() },
                  Token { kind: TokenKind::NOT_EQ, literal: "!=".to_string() },
                  Token { kind: TokenKind::INT, literal: "9".to_string() },
                  Token { kind: TokenKind::SEMICOLON, literal: ";".to_string() },
                  
    ];
    
    let mut lexier = Lexier::new(input);
    let mut token: Token;
    
    for test in tests.iter() {
        token = lexier.next_token();
        //println!("input:\t{{ kind: {}, literal: {} }}", token.get_kind(), token.literal);
        //println!("test:\t{{ kind: {}, literal: {} }}", test.get_kind(), test.literal);
        if token.get_kind() != test.get_kind() {
            panic!("token.kind not equal test.kind.");
        }
    }
    
}
