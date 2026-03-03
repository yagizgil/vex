pub mod variable;
pub mod function;
pub mod import;

use crate::parser::Parser;
use crate::ast::stmt::Stmt;

pub trait Declaration {
    fn parse(parser: &mut Parser) -> Stmt;
}

pub use function::FunctionDecl;
pub use variable::VariableDecl;
pub use import::ImportDecl;