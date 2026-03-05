pub mod vardecl;
pub mod fndecl;
pub mod enumdecl;
pub mod structdecl;
pub mod impldecl;
pub mod macrodecl;
pub mod definedecl;
pub mod importdecl;

pub use vardecl::VarDecl;
pub use fndecl::FnDecl;
pub use enumdecl::EnumDecl;
pub use structdecl::StructDecl;
pub use impldecl::ImplDecl;
pub use macrodecl::MacroDecl;
pub use definedecl::DefineDecl;
pub use importdecl::ImportDecl;

use vex_core::ast::Stmt;

#[derive(Debug)]
pub enum Declaration {
    Var(Stmt),
    Fn(FnDecl),
    Struct(StructDecl),
    Enum(EnumDecl),
    Impl(ImplDecl),
    Macro(MacroDecl),
    Define(DefineDecl),
    Import(ImportDecl),
}