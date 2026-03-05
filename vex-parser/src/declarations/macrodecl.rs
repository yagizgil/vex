use crate::Parser;

#[derive(Debug)]
pub struct MacroDecl;

impl MacroDecl {
    pub fn parse(parser: &mut Parser) -> Option<Self> {
        None
    }
}
