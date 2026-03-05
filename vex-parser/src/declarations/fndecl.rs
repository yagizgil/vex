use crate::Parser;
use vex_core::trace_fn;

#[derive(Debug)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub struct FnDecl;

impl FnDecl {
    pub fn parse(parser: &mut Parser) -> Option<Self> {
        trace_fn!("FnDecl::parse", "at={:?}", parser.peek().lexeme());
        None
    }
}
