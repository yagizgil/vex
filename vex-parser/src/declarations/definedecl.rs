use crate::Parser;

#[derive(Debug)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub struct DefineDecl;

impl DefineDecl {
    pub fn parse(parser: &mut Parser) -> Option<Self> {
        None
    }
}
