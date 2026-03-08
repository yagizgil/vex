use crate::Parser;

#[derive(Debug)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub struct ImplDecl;

impl ImplDecl {
    pub fn parse(_parser: &mut Parser) -> Option<Self> {
        None
    }
}
