use crate::token::{ TokenKind, Token };

/// Lexical Analyzer
#[derive(Clone)]
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
    let mut lexier = Lexier::new(input);

    loop {
        let token = lexier.next_token();
        match token.kind {
            TokenKind::IDENT => println!("IDENT: {}", token.literal),
            TokenKind::INT => println!("INT: {}", token.literal),
            TokenKind::ASSIGN => println!("ASSIGN: {}", token.literal),
            TokenKind::PLUS => println!("PLUS: {}", token.literal),
            TokenKind::MINUS => println!("MINUS: {}", token.literal),
            TokenKind::BANG => println!("BANG: {}", token.literal),
            TokenKind::ASTERISK => println!("ASTERISK: {}", token.literal),
            TokenKind::SLASH => println!("SLASH: {}", token.literal),
            TokenKind::LT => println!("LT: {}", token.literal),
            TokenKind::GT => println!("GT: {}", token.literal),
            TokenKind::EQ => println!("EQ: {}", token.literal),
            TokenKind::NOT_EQ => println!("NOT_EQ: {}", token.literal),
            TokenKind::COMMA => println!("COMMA: {}", token.literal),
            TokenKind::SEMICOLON => println!("SEMICOLON: {}", token.literal),
            TokenKind::LPAREN => println!("LPAREN: {}", token.literal),
            TokenKind::RPAREN => println!("RPAREN: {}", token.literal),
            TokenKind::LBRACE => println!("LBRACE: {}", token.literal),
            TokenKind::RBRACE => println!("RBRACE: {}", token.literal),
            TokenKind::FUNCTION => println!("FUNCTION: {}", token.literal),
            TokenKind::LET => println!("LET: {}", token.literal),
            TokenKind::TRUE => println!("TRUE: {}", token.literal),
            TokenKind::FALSE => println!("FALSE: {}", token.literal),
            TokenKind::IF => println!("IF: {}", token.literal),
            TokenKind::ELSE => println!("ELSE: {}", token.literal),
            TokenKind::RETURN => println!("RETURN: {}", token.literal),
            TokenKind::EOF => {
                println!("EOF: {}", token.literal);
                break;
            }
            _ => {
                println!("{}", token.literal);
                assert!(false);
            }
        };
    }

    
}
