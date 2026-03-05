use crate::Parser;

#[derive(Debug)]
pub struct DefineDecl;

impl DefineDecl {
    pub fn parse(parser: &mut Parser) -> Option<Self> {
        None
    }
}
