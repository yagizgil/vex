use crate::ast::stmt::Stmt;
use crate::lexer::token::TokenType;
use crate::parser::Parser;
use super::Declaration;

pub struct ImportDecl;

impl Declaration for ImportDecl {
    fn parse(p: &mut Parser) -> Stmt {
        let mut _imports = Vec::new();

        _imports.push(Self::consume_path(p));

        loop {
            if p.match_token(&[TokenType::Comma]) {
                p.consume_end_of_statement();
                _imports.push(Self::consume_path(p));
                continue;
            }

            if p.match_token(&[TokenType::Newline]) {
                if p.check(&TokenType::Indent) {
                    p.advance(); 

                    while !p.check(&TokenType::Dedent) && !p.is_at_end() {
                        p.consume_end_of_statement();
                        
                        if p.check(&TokenType::Identifier) {
                            _imports.push(Self::consume_path(p));
                        }

                        if p.match_token(&[TokenType::Comma]) {
                            continue;
                        }

                        if !p.check(&TokenType::Dedent) {
                            p.match_token(&[TokenType::Newline]);
                        }
                    }
                    p.consume(TokenType::Dedent, "Expect dedent after import list.");
                    break;
                } else {
                    break;
                }
            }
            break;
        }

        Stmt::Import(_imports)
    }
}

impl ImportDecl {
    fn consume_path(p: &mut Parser) -> String {
        let mut path = p
            .consume(TokenType::Identifier, "Expect identifier for import path.")
            .lexeme
            .clone();

        while p.match_token(&[TokenType::Dot]) {
            let part = p.consume(TokenType::Identifier, "Expect identifier after '.'.");
            path.push('.');
            path.push_str(&part.lexeme);
        }
        path
    }
}