use crate::Parser;
use vex_core::ast::{Expr, Stmt};
use vex_core::token::{Token, TokenType};
use vex_core::trace_fn;

pub struct VarDecl;

impl VarDecl {
    pub fn parse(parser: &mut Parser) -> Option<Stmt> {
        trace_fn!("VarDecl::parse", "at={:?}", parser.peek().lexeme());
        let mut modifiers = Vec::new();

        // 1. Collect Modifiers (pub, static, priv, async)
        while matches!(
            parser.peek().kind,
            TokenType::Pub | TokenType::Static | TokenType::Priv | TokenType::Async
        ) {
            modifiers.push(parser.advance());
        }

        // 2. Keyword check (var or const)
        let keyword = if parser.match_token(TokenType::Var) || parser.match_token(TokenType::Const)
        {
            parser.previous().clone()
        } else {
            // Missing var/const: only valid if we already have modifiers.
            if !modifiers.is_empty() {
                // Determine what follows to see if it's potentially a VarDecl
                let p_kind = &parser.peek().kind;
                let next_kind = &parser.peek_at(1).kind;

                let is_var_decl_looking = 
                    // type ident ...
                    (is_primitive_type_kind(p_kind) && matches!(next_kind, TokenType::Identifier)) ||
                    // ident ident ...
                    (matches!(p_kind, TokenType::Identifier) && matches!(next_kind, TokenType::Identifier)) ||
                    // ident . type ...
                    (matches!(p_kind, TokenType::Identifier) && matches!(next_kind, TokenType::Dot | TokenType::Colon));

                if is_var_decl_looking {
                    Token::new(TokenType::Var, modifiers.last().unwrap().span.clone())
                } else {
                    return None;
                }
            } else {
                return None;
            }
        };

        // 3. Optional Type BEFORE name (e.g., var int mil)
        // Check: type ident ...
        let mut vtype = None;
        if is_primitive_type(parser.peek()) && matches!(parser.peek_at(1).kind, TokenType::Identifier) {
            trace_fn!("VarDecl::parse [type_before_name]");
            vtype = parser.parse_type_expr();
        }

        // 4. Identifier (Name)
        let name = parser.expect(
            TokenType::Identifier,
            "Expected variable name",
        )?;

        // 5. Optional Type AFTER name (e.g., var limit:int or var limit.int)
        if vtype.is_none() {
            if parser.match_token(TokenType::Colon) || parser.match_token(TokenType::Dot) {
                trace_fn!("VarDecl::parse [type_after_colon/dot]");
                vtype = parser.parse_type_expr();
            } else if is_primitive_type(parser.peek()) {
                trace_fn!("VarDecl::parse [type_after_name_no_sep]");
                vtype = parser.parse_type_expr();
            }
        }

        // 6. Initializer
        let mut initializer = None;
        // Skip optional '=' if present
        parser.match_token(TokenType::Equal);

        if !parser.check(TokenType::StatementEnd)
            && !parser.check(TokenType::Newline)
            && !parser.check(TokenType::Eof)
        {
            trace_fn!("VarDecl::parse [initializer_found]");
            initializer = parser.parse_expression(0);
        }

        // Consume trailing whitespace/separators
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

fn is_primitive_type(token: &Token) -> bool {
    is_primitive_type_kind(&token.kind)
}

fn is_primitive_type_kind(kind: &TokenType) -> bool {
    matches!(
        kind,
        TokenType::TInt
            | TokenType::TStr
            | TokenType::TFloat
            | TokenType::TBool
            | TokenType::TAny
            | TokenType::TList
            | TokenType::TDict
            | TokenType::Fn
    )
}
