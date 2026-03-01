use crate::lexer::token::*;
use crate::ast::vtype::VarType;
use crate::ast::expr::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Var { 
        name: Token, 
        vtype: VarType,
        initializer: Expr 
    },

    Fn { 
        name: Token, 
        params: Vec<Token>, 
        body: Vec<Stmt> 
    },

    For {
        item: Token,
        iterable: Expr,
        body: Vec<Stmt>,
    },

    While {
        condition: Expr,
        body: Vec<Stmt>,
    },

    If { 
        condition: Expr, 
        then_branch: Vec<Stmt>, 
        else_branch: Option<Box<Stmt>> 
    },

    Return { 
        keyword: Token, 
        value: Option<Expr> 
    },

    Break,
    Continue,

    Expression(Expr),
    Block(Vec<Stmt>),
}