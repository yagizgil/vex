use crate::Parser;
use vex_core::trace_fn;

#[derive(Debug)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub struct StructDecl;

impl StructDecl {
    pub fn parse(parser: &mut Parser) -> Option<Self> {
        trace_fn!("StructDecl::parse", "at={:?}", parser.peek().lexeme());
        None
    }
}
