use crate::lexier::Lexier;
use crate::token::{ TokenKind, Token };
use std::io::{ self, Write, stdin };


pub fn start() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                let mut lexier = Lexier::new(input);
                loop {
                    let token = lexier.next_token();
                    match token.kind {
                        TokenKind::EOF => break,
                        _ => println!("{{ kind: {}, literal: {} }}", token.get_kind(), token.literal)
                    }
                }
            }
            Err(error) => println!("error: {}", error)
        }
    }
}
