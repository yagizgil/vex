use crate::lexer::token::*;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
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


