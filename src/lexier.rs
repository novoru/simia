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
        match self.ch {
            '=' => token = Token::ASSIGN    { literal: self.ch.to_string() },
            '+' => token = Token::PLUS      { literal: self.ch.to_string() },
            '(' => token = Token::LPAREN    { literal: self.ch.to_string() },
            ')' => token = Token::RPAREN    { literal: self.ch.to_string() },
            '{' => token = Token::LBRACE    { literal: self.ch.to_string() },
            '}' => token = Token::RBRACE    { literal: self.ch.to_string() },
            ',' => token = Token::COMMA     { literal: self.ch.to_string() },
            ';' => token = Token::SEMICOLON { literal: self.ch.to_string() },
            '\0' => token = Token::EOF { literal: "".to_string() },
            _  => token = Token::ILLEGAL    { literal: self.ch.to_string() },
        }

        self.read_char();
        token
    }

    pub fn read_char(&mut self) {
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
    
}


#[test]
fn test_next_token() {
    let input = "=+(){},;".to_string();

    let tests = [ Token::ASSIGN    { literal: "=".to_string() },
                  Token::PLUS      { literal: "+".to_string() },
                  Token::LPAREN    { literal: "(".to_string() },
                  Token::RPAREN    { literal: ")".to_string() },
                  Token::LBRACE    { literal: "{".to_string() },
                  Token::RBRACE    { literal: "}".to_string() },
                  Token::COMMA     { literal: ",".to_string() },
                  Token::SEMICOLON { literal: ";".to_string() },
                  Token::EOF       { literal: "".to_string() }
    ];

    let mut lexier = Lexier::new(input);
    
    loop {
        match lexier.next_token() {
            Token::ASSIGN { literal } => assert!(true),
            Token::PLUS { literal }   => assert!(true),
            Token::LPAREN { literal } => assert!(true),
            Token::RPAREN { literal } => assert!(true),
            Token::LBRACE { literal } => assert!(true),
            Token::RBRACE { literal } => assert!(true),
            Token::COMMA { literal }  => assert!(true),
            Token::SEMICOLON { literal } => assert!(true),
            Token::EOF { literal } => { assert!(true); break; },
            _ => assert!(false)
        };
    }
    
}
