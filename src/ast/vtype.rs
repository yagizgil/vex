use crate::lexer::token::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub enum VarType {
    Any,
    Int,
    Float,
    Str,
    Bool,
    List,
    Dict,
}


impl From<&TokenType> for VarType {
    fn from(token: &TokenType) -> Self {
        match token {
            TokenType::TInt => VarType::Int,
            TokenType::TStr => VarType::Str,
            TokenType::TFloat => VarType::Float,
            TokenType::TBool => VarType::Bool,
            TokenType::TList => VarType::List,
            TokenType::TDict => VarType::Dict,
            _ => VarType::Any,
        }
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub struct Parameter {
    pub name: Token,
    pub var_type: VarType,
}