use crate::lexer::token::*;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    Str(String),
    Bool(bool),
    Null,
}

#[derive(Serialize, Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },

    Unary {
        operator: Token,
        right: Box<Expr>,
    },

    Literal(LiteralValue),

    Variable {
        name: Token,
        index: Option<(usize, usize)>,
    },

    Assign {
        name: Token,
        value: Box<Expr>,
        index: Option<(usize, usize)>,
    },

    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },

    Grouping(Box<Expr>),
}
