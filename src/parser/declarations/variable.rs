use super::Declaration;
use crate::ast::{stmt::Stmt, vtype::VarType, expr::Expr};
use crate::lexer::token::TokenType;
use crate::parser::Parser;

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
            p.assignment()
        } else {
            p.assignment()
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

    pub fn list(p: &mut Parser) -> Expr {
        let mut elements = Vec::new();
        while !p.check(&TokenType::RightBracket) && !p.is_at_end() {
            elements.push(p.assignment());
            if !p.match_token(&[TokenType::Comma]) {
                break;
            }
        }
        p.consume(TokenType::RightBracket, "Expect ']' after list items.");
        Expr::List { elements }
    }

    pub fn dict(p: &mut Parser) -> Expr {
        let mut entries = Vec::new();
        while !p.check(&TokenType::RightBrace) && !p.is_at_end() {
            let key = p.assignment();
            p.consume(TokenType::Colon, "Expect ':' after dictionary key.");
            let value = p.assignment();
            entries.push((key, value));
            if !p.match_token(&[TokenType::Comma]) {
                break;
            }
        }
        p.consume(TokenType::RightBrace, "Expect '}' after dictionary items.");
        Expr::Dict { entries }
    }
}
