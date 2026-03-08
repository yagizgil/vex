pub mod definedecl;
pub mod enumdecl;
pub mod fndecl;
pub mod impldecl;
pub mod importdecl;
pub mod macrodecl;
pub mod structdecl;
pub mod vardecl;

pub use crate::Parser;
pub use definedecl::DefineDecl;
pub use enumdecl::EnumDecl;
pub use fndecl::FnDecl;
pub use impldecl::ImplDecl;
pub use importdecl::ImportDecl;
pub use macrodecl::MacroDecl;
pub use structdecl::StructDecl;
pub use vardecl::VarDecl;
use vex_core::trace_fn;

use vex_core::ast::Stmt;

#[derive(Debug)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub enum Declaration {
    Var(Stmt),
    Fn(Stmt),
    Struct(StructDecl),
    Enum(EnumDecl),
    Impl(ImplDecl),
    Macro(MacroDecl),
    Define(DefineDecl),
    Import(ImportDecl),
}
