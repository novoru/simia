use crate::token::{ Token };


pub struct Lexier {
    input: String, 
    position: u32,
    read_position: u32,
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

    pub fn next_token(&mut self) -> Token {
        let token: Token;
        self.skip();
        
        match self.ch {
            '=' => token = Token::ASSIGN    { literal: self.ch.to_string() },
            '+' => token = Token::PLUS      { literal: self.ch.to_string() },
            '-' => token = Token::MINUS     { literal: self.ch.to_string() },
            '!' => token = Token::BANG      { literal: self.ch.to_string() },
            '*' => token = Token::ASTERISK  { literal: self.ch.to_string() },
            '/' => token = Token::SLASH     { literal: self.ch.to_string() },
            '<' => token = Token::LT        { literal: self.ch.to_string() },
            '>' => token = Token::GT        { literal: self.ch.to_string() },
            '(' => token = Token::LPAREN    { literal: self.ch.to_string() },
            ')' => token = Token::RPAREN    { literal: self.ch.to_string() },
            '{' => token = Token::LBRACE    { literal: self.ch.to_string() },
            '}' => token = Token::RBRACE    { literal: self.ch.to_string() },
            ',' => token = Token::COMMA     { literal: self.ch.to_string() },
            ';' => token = Token::SEMICOLON { literal: self.ch.to_string() },
            '\0' => token = Token::EOF { literal: "".to_string() },
            'a'...'z' | 'A' ... 'Z' | '_' => return Token::IDENT { literal: self.read_identifier() },
            '0' ... '9' => return Token::INT { literal: self.read_integer() },
            _  => token = Token::ILLEGAL    { literal: self.ch.to_string() },
        }

        self.read_char();
        token
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() as u32 {
            self.ch = '\0';
        }
        else {
            self.ch = self.input.chars()
                .skip(self.read_position as usize).next().unwrap()
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = "".to_string();
        while self.ch.is_alphabetic() || self.ch == '_' {
            identifier.push(self.ch);
            self.read_char();
        }
 
        identifier
    }

    fn read_integer(&mut self) -> String {
        let mut integer = "".to_string();
        while self.ch.is_digit(10) {
            integer.push(self.ch);
            self.read_char();
        }

        integer
    }

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
".to_string();
    let mut lexier = Lexier::new(input);

    loop {
        match lexier.next_token() {
            Token::ASSIGN { ref literal } if literal == "=" => println!("ASSIGN: {}", literal ),
            Token::PLUS { ref literal }  if literal == "+"  => println!("PLUS: {}", literal ),
            Token::MINUS { ref literal }  if literal == "-"  => println!("MINUS: {}", literal ),
            Token::BANG { ref literal }  if literal == "!"  => println!("BANG: {}", literal ),
            Token::ASTERISK { ref literal }  if literal == "*"  => println!("ASTERISK: {}", literal ),
            Token::SLASH { ref literal }  if literal == "/"  => println!("SLASH: {}", literal ),
            Token::LT { ref literal }  if literal == "<"  => println!("LT: {}", literal ),
            Token::GT { ref literal }  if literal == ">"  => println!("GT: {}", literal ),
            Token::LPAREN { ref literal } if literal == "(" => println!("LPAREN: {}", literal ),
            Token::RPAREN { ref literal } if literal == ")" => println!("RPAREN: {}", literal ),
            Token::LBRACE { ref literal } if literal == "{" => println!("LBRACE: {}", literal),
            Token::RBRACE { ref literal } if literal == "}" => println!("RBRACE: {}", literal),
            Token::COMMA { ref literal }  if literal == "," => println!("COMMA: {}", literal),
            Token::SEMICOLON { ref literal } if literal == ";" => println!("SEMICOLON: {}", literal),
            Token::EOF { ref literal } if literal == "" => { println!("EOF: {}", literal ); break; },
            Token::IDENT {ref literal } => println!("IDENT: {}", literal),
            Token::INT { ref literal } => println!("INT: {}", literal),
            _ => assert!(false)
        };
    }

    
}
