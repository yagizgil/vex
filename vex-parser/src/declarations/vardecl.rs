use crate::Parser;
use vex_core::ast::{Stmt, Expr};
use vex_core::token::TokenType;

pub struct VarDecl;

impl VarDecl {
    pub fn parse(parser: &mut Parser) -> Option<Stmt> {
        let mut modifiers = Vec::new();

        // 1. Collect Modifiers (pub, static, priv)
        while parser.check(TokenType::Pub) || parser.check(TokenType::Static) || parser.check(TokenType::Priv) {
            modifiers.push(parser.advance());
        }

        // 2. Keyword check (var or const)
        let keyword = if parser.match_token(TokenType::Var) || parser.match_token(TokenType::Const) {
            parser.previous().clone()
        } else {
            return None;
        };

        // 3. Identifier (Name)
        let name = parser.expect(TokenType::Identifier, "Expected variable name after keyword")?;

        // 4. Optional Type check
        // Check if there's a colon or a known type keyword
        let vtype = if parser.match_token(TokenType::Colon) || parser.match_token(TokenType::Dot) {
            parser.parse_type_expr()
        } else {
            parser.parse_type_expr()
        };

        // 5. Initializer
        let mut initializer = None;
        if !parser.check(TokenType::StatementEnd) && !parser.check(TokenType::Newline) && !parser.check(TokenType::Eof) {
            // Optional '='
            parser.match_token(TokenType::Equal);
            initializer = parser.parse_expression(0);
        }

        // Consume trailing Newline/StatementEnd
        while parser.match_token(TokenType::Newline) || parser.match_token(TokenType::StatementEnd) {}

        Some(Stmt::VarDecl {
            modifiers,
            keyword,
            name,
            vtype,
            initializer,
        })
    }
}
