pub mod ifstmt;
pub mod whilestmt;
pub mod forstmt;
pub mod matchstmt;
pub mod returnstmt;
pub mod breakstmt;
pub mod continuestmt;
pub mod blockstmt;
pub mod exprstmt;

pub use ifstmt::IfStmt;
pub use whilestmt::WhileStmt;
pub use forstmt::ForStmt;
pub use matchstmt::MatchStmt;
pub use returnstmt::ReturnStmt;
pub use breakstmt::BreakStmt;
pub use continuestmt::ContinueStmt;
pub use blockstmt::BlockStmt;
pub use exprstmt::ExprStmt;

pub enum Statement {
    If(IfStmt),
    While(WhileStmt),
    For(ForStmt),
    Match(MatchStmt),
    Return(ReturnStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    Block(BlockStmt),
    Expr(ExprStmt),
}
