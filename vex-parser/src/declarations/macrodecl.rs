use crate::Parser;

#[derive(Debug)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub struct MacroDecl;

impl MacroDecl {
    pub fn parse(parser: &mut Parser) -> Option<Self> {
        None
    }
}
