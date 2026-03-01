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

impl Stmt {
    pub fn line(&self) -> usize {
        match self {
            Stmt::Var { name, .. } => name.line,
            Stmt::Fn { name, .. } => name.line,
            Stmt::For { item, .. } => item.line,
            Stmt::Return { keyword, .. } => keyword.line,
            Stmt::While { condition, .. } => 0,
            Stmt::Expression(_expr) => 0,
            Stmt::Block(stmts) => stmts.first().map(|s| s.line()).unwrap_or(0),
            _ => 0,
        }
    }
}