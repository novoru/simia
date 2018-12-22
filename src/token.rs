#[derive(PartialEq)]
pub enum Token {
    ILLEGAL { literal: String},
    EOF  { literal: String},

    IDENT { literal: String},      // identifier
    INT { literal: String},        // integer literal

    // operator
    ASSIGN { literal: String},     // '='
    PLUS { literal: String},       // '+'
    MINUS { literal: String},      // '-'
    BANG { literal: String},      // '!'
    ASTERISK { literal: String},   // '*'
    SLASH { literal: String},      // '/'
    LT { literal: String},         // '<'
    GT { literal: String},         // '>'

    // delimeter
    COMMA { literal: String},      // ','
    SEMICOLON { literal: String},  // ';'

    LPAREN { literal: String},     // '('
    RPAREN { literal: String},     // ')'
    LBRACE { literal: String},     // '{'
    RBRACE { literal: String},     // '}'

    // keyword
    FUNCTION { literal: String},   // 'fn'
    LET { literal: String},        // 'let'
    TRUE { literal: String},       // 'true'
    FALSE { literal: String},      // 'false'
    IF { literal: String},         // 'if'
    ELSE { literal: String},       // 'else'
    RETURN { literal:String },     // 'return'
}


