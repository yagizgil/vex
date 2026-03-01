
use crate::lexer::token::*;

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    Str(String),
    Bool(bool),
    Null,
}


#[derive(Debug, Clone)]
pub enum Expr {
    Binary { 
        left: Box<Expr>, 
        operator: Token, 
        right: Box<Expr> 
    },
    
    Unary { 
        operator: Token, 
        right: Box<Expr> 
    },

    Literal(LiteralValue),

    Variable(Token),

    Assign { 
        name: Token, 
        value: Box<Expr> 
    },

    Call { 
        callee: Box<Expr>,
        arguments: Vec<Expr> 
    },

    Grouping(Box<Expr>),
}
