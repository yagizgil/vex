use crate::Parser;
use vex_core::ast::{Parameter, Stmt};
use vex_core::token::TokenType;
use vex_core::trace_fn;

#[derive(Debug)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub struct FnDecl;

impl FnDecl {
    pub fn parse(parser: &mut Parser) -> Option<Stmt> {
        trace_fn!("FnDecl::parse", "at={:?}", parser.peek().lexeme());
        let mut modifiers = Vec::new();
        let mut is_async = false;

        // 1. Collect Modifiers (pub, static, priv, async)
        while matches!(
            parser.peek().kind,
            TokenType::Pub | TokenType::Static | TokenType::Priv | TokenType::Async
        ) {
            let t = parser.advance();
            if t.kind == TokenType::Async {
                is_async = true;
            }
            modifiers.push(t);
        }

        // 2. Consume 'fn'
        parser.expect(TokenType::Fn, "Expected 'fn' keyword")?;

        // 3. Function Name
        let name = parser
            .expect(TokenType::Identifier, "Expected function name")?
            .clone();

        // 4. Parameters
        let mut params = Vec::new();
        while !parser.check(TokenType::Minus)
            && !parser.check(TokenType::Colon)
            && !parser.check(TokenType::StatementEnd)
            && !parser.check(TokenType::Newline)
            && !parser.is_at_end()
        {
            let p_name = parser
                .expect(TokenType::Identifier, "Expected parameter name")?
                .clone();
            let mut var_type = None;

            // Support 'id.int' or 'id:int' or 'id int'
            if parser.match_token(TokenType::Dot) || parser.match_token(TokenType::Colon) {
                var_type = parser.parse_type_expr();
            } else if parser.is_type_start() {
                var_type = parser.parse_type_expr();
            }

            params.push(Parameter {
                name: p_name,
                var_type,
            });

            // Optional comma
            parser.match_token(TokenType::Comma);
        }

        // 5. Return Type
        let mut rtype = None;
        if parser.match_token(TokenType::Minus) {
            rtype = parser.parse_type_expr();
        }

        // 6. End of signature
        parser.match_token(TokenType::Colon);

        parser.expect(TokenType::StatementEnd, "");

        // 7. Body
        let mut body = Vec::new();
        if parser.check(TokenType::Indent) {
            body = parser.parse_block();
            parser.expect(TokenType::Dedent, "");
        }

        Some(Stmt::FnDecl {
            modifiers,
            name,
            params,
            rtype,
            body,
            is_async,
        })
    }
}
