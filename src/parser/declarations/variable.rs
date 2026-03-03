use crate::ast::{stmt::Stmt, vtype::VarType};
use crate::lexer::token::TokenType;
use crate::parser::Parser;
use super::Declaration;

pub struct VariableDecl;

impl Declaration for VariableDecl {
    fn parse(p: &mut Parser) -> Stmt {
        let mut _vtype = VarType::Any;
        if Self::is_type_token(p.peek().token_type.clone()) {
            _vtype = VarType::from(&p.peek().token_type);
            p.advance();
        }

        let name = p.consume(TokenType::Identifier, "Expect variable name.");

        let initializer = if p.match_token(&[TokenType::Equal]) {
            p.expression()
        } else {
            p.expression()
        };

        p.consume_end_of_statement();

        // ------ Inspector Record ------
        inspect!(
            "Parser",
            &vec![name.clone()],
            &vec![Stmt::Var {
                name: name.clone(),
                vtype: _vtype.clone(),
                initializer: initializer.clone(),
            }],
            "Variable declared: {} with type {:?}",
            name.lexeme,
            _vtype
        );
        // ------ Inspector Record ------

        Stmt::Var {
            name,
            vtype: _vtype,
            initializer,
        }
    }
}

impl VariableDecl {
    fn is_type_token(t: TokenType) -> bool {
        matches!(
            t,
            TokenType::TInt
                | TokenType::TStr
                | TokenType::TFloat
                | TokenType::TBool
                | TokenType::TList
                | TokenType::TDict
        )
    }
}