#[derive(Debug, Clone)]
pub enum TokenKind {
    ILLEGAL,
    EOF,

    IDENT,      // identifier
    INT,        // integer literal

    // operator
    ASSIGN,     // '='
    PLUS,       // '+'
    MINUS,      // '-'
    BANG,       // '!'
    ASTERISK,   // '*'
    SLASH,      // '/'
    LT,         // '<'
    GT,         // '>'
    EQ,         // '=='
    NOT_EQ,     // '!='

    // delimeter
    COMMA,      // ','
    SEMICOLON,  // ';'

    LPAREN,     // '('
    RPAREN,     // ')'
    LBRACE,     // '{'
    RBRACE,     // '}'

    // keyword
    FUNCTION,   // 'fn'
    LET,        // 'let'
    TRUE ,      // 'true'
    FALSE,      // 'false'
    IF,         // 'if'
    ELSE,       // 'else'
    RETURN,     // 'return'
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

impl Token {
    pub fn get_kind(&self) -> String{
        match self.kind {
            TokenKind::ILLEGAL => "ILLEGAL".to_string(),
            TokenKind::EOF => "EOF".to_string(),
            TokenKind::IDENT => "IDENT".to_string(),
            TokenKind::INT => "INT".to_string(),
            TokenKind::ASSIGN => "ASSIGN".to_string(),
            TokenKind::PLUS => "PLUS".to_string(),
            TokenKind::MINUS => "MINUS".to_string(),
            TokenKind::BANG => "BANG".to_string(),
            TokenKind::ASTERISK => "ASTERISK".to_string(),
            TokenKind::SLASH => "SLASH".to_string(),
            TokenKind::LT => "LT".to_string(),
            TokenKind::GT => "GT".to_string(),
            TokenKind::EQ => "ILLEGAL".to_string(),
            TokenKind::NOT_EQ => "NOT_EQ".to_string(),
            TokenKind::COMMA => "COMMA".to_string(),
            TokenKind::SEMICOLON => "SEMICOLON".to_string(),
            TokenKind::LPAREN => "LPAREN".to_string(),
            TokenKind::RPAREN => "RPAREN".to_string(),
            TokenKind::LBRACE => "LBRACE".to_string(),
            TokenKind::RBRACE => "RBRACE".to_string(),
            TokenKind::FUNCTION => "FUNCTION".to_string(),
            TokenKind::LET => "LET".to_string(),
            TokenKind::TRUE => "TRUE".to_string(),
            TokenKind::FALSE => "FALSE".to_string(),
            TokenKind::IF => "IF".to_string(),
            TokenKind::ELSE => "ELSE".to_string(),
            TokenKind::RETURN => "RETURN".to_string()
        }
    }
}
