#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenKind {
    Illegal,
    Eof,

    Identifier,      // identifier
    Integer,         // integer literal
    String,

    // operator
    Assign,     // '='
    Plus,       // '+'
    Minus,      // '-'
    Bang,       // '!'
    Asterisk,   // '*'
    Slash,      // '/'
    Lt,         // '<'
    Gt,         // '>'
    Eq,         // '=='
    NotEq,     // '!='

    // delimeter
    Comma,      // ','
    Semicolon,  // ';'

    Lparen,     // '('
    Rparen,     // ')'
    Lbrace,     // '{'
    Rbrace,     // '}'
    Lbracket,   // '['
    Rbracket,   // ']'

    // keyword
    Function,   // 'fn'
    Let,        // 'let'
    True ,      // 'true'
    False,      // 'false'
    If,         // 'if'
    Else,       // 'else'
    Return,     // 'return'
}

impl TokenKind {
    pub fn get_kind_literal(&self) -> String {
        match self {
            TokenKind::Illegal    => "Illegal".to_string(),
            TokenKind::Eof        => "Eof".to_string(),
            TokenKind::Identifier => "Identifier".to_string(),
            TokenKind::Integer    => "Integer".to_string(),
            TokenKind::Assign     => "Assign".to_string(),
            TokenKind::Plus       => "Plus".to_string(),
            TokenKind::Minus      => "Minus".to_string(),
            TokenKind::Bang       => "Bang".to_string(),
            TokenKind::Asterisk   => "Asterisk".to_string(),
            TokenKind::Slash      => "Slash".to_string(),
            TokenKind::Lt         => "Lt".to_string(),
            TokenKind::Gt         => "Gt".to_string(),
            TokenKind::Eq         => "Illegal".to_string(),
            TokenKind::NotEq      => "NotEq".to_string(),
            TokenKind::Comma      => "Comma".to_string(),
            TokenKind::Semicolon  => "Semicolon".to_string(),
            TokenKind::Lparen     => "Lparen".to_string(),
            TokenKind::Rparen     => "Rparen".to_string(),
            TokenKind::Lbrace     => "Lbrace".to_string(),
            TokenKind::Rbrace     => "Rbrace".to_string(),
            TokenKind::Function   => "Function".to_string(),
            TokenKind::Let        => "Let".to_string(),
            TokenKind::True       => "True".to_string(),
            TokenKind::False      => "False".to_string(),
            TokenKind::If         => "If".to_string(),
            TokenKind::Else       => "Else".to_string(),
            TokenKind::Return     => "Return".to_string(),
            TokenKind::String     => "String".to_string(),
            TokenKind::Lbracket   => "Lbracket".to_string(),
            TokenKind::Rbracket   => "Rbracket".to_string(),
        }
        
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

impl Token {
    pub fn get_kind_literal(&self) -> String{
        match self.kind {
            TokenKind::Illegal    => "Illegal".to_string(),
            TokenKind::Eof        => "Eof".to_string(),
            TokenKind::Identifier => "Identifier".to_string(),
            TokenKind::Integer    => "Integer".to_string(),
            TokenKind::Assign     => "Assign".to_string(),
            TokenKind::Plus       => "Plus".to_string(),
            TokenKind::Minus      => "Minus".to_string(),
            TokenKind::Bang       => "Bang".to_string(),
            TokenKind::Asterisk   => "Asterisk".to_string(),
            TokenKind::Slash      => "Slash".to_string(),
            TokenKind::Lt         => "Lt".to_string(),
            TokenKind::Gt         => "Gt".to_string(),
            TokenKind::Eq         => "Illegal".to_string(),
            TokenKind::NotEq      => "NotEq".to_string(),
            TokenKind::Comma      => "Comma".to_string(),
            TokenKind::Semicolon  => "Semicolon".to_string(),
            TokenKind::Lparen     => "Lparen".to_string(),
            TokenKind::Rparen     => "Rparen".to_string(),
            TokenKind::Lbrace     => "Lbrace".to_string(),
            TokenKind::Rbrace     => "Rbrace".to_string(),
            TokenKind::Function   => "Function".to_string(),
            TokenKind::Let        => "Let".to_string(),
            TokenKind::True       => "True".to_string(),
            TokenKind::False      => "False".to_string(),
            TokenKind::If         => "If".to_string(),
            TokenKind::Else       => "Else".to_string(),
            TokenKind::Return     => "Return".to_string(),
            TokenKind::String     => "String".to_string(),
            TokenKind::Lbracket   => "Lbracket".to_string(),
            TokenKind::Rbracket   => "Rbracket".to_string(),
        }
    }
}
