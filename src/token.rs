pub enum Token {
    ILLEGAL { literal: String},
    EOF  { literal: String},

    IDENT { literal: String},      // identifier
    INT { literal: String},        // integer literal

    // operator
    ASSIGN { literal: String},     // '='
    PLUS { literal: String},       // '+'

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
}


