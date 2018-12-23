use crate::token:: { Token };

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum AST {
    PROGRAM { statements: Vec<AST> },

    EXPRESSION { literal: String },
    
    LET_STATEMENT {
        token: Token,
        name: String,
        ident: Box<Identifier>,
        value: Box<AST>
    },
    
}
