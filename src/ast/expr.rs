use crate::lexer::token::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub enum LiteralValue {
    Number(f64),
    Str(String),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
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
    FString(LiteralValue),

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
    List {
        elements: Vec<Expr>,
    },
    Dict {
        entries: Vec<(Expr, Expr)>,
    },
}
