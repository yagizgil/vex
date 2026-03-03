pub mod control_flow;

use crate::ast::stmt::Stmt;
use crate::ast::expr::Expr;
use crate::lexer::token::TokenType;
use crate::parser::Parser;

use crate::utils::logger::error::ErrorCode;

pub trait Statement {
    fn parse(parser: &mut Parser) -> Stmt;
}

impl Parser {
    pub fn statement(&mut self) -> Stmt {
        if self.match_token(&[TokenType::Indent]) {
            return Stmt::Block(self.block());
        }

        if let TokenType::Identifier = self.peek().token_type {
            if self.peek().lexeme != "print" {
                if self.current + 1 < self.tokens.len() {
                    let next = &self.tokens[self.current + 1];
                    if !matches!(
                        next.token_type,
                        TokenType::Equal
                            | TokenType::Plus
                            | TokenType::Minus
                            | TokenType::Star
                            | TokenType::Slash
                            | TokenType::EqualEqual
                            | TokenType::BangEqual
                            | TokenType::Less
                            | TokenType::LessEqual
                            | TokenType::Greater
                            | TokenType::GreaterEqual
                            | TokenType::Newline
                            | TokenType::Eof
                            | TokenType::RightParen
                            | TokenType::RightBracket
                            | TokenType::Colon
                            | TokenType::Comma
                    ) {
                        let name = self.consume(TokenType::Identifier, "Expect variable name.");
                        let value = self.assignment();
                        self.consume_end_of_statement();
                        return Stmt::Expression(Expr::Assign {
                            name,
                            value: Box::new(value),
                            index: None,
                        });
                    } else {
                        vex_pars_panic!(
                            self.peek().line,
                            ErrorCode::Unknown,
                            Some("statement".to_string())
                        );
                    }
                }
            }
        }

        self.expression_statement()
    }

    pub fn expression_statement(&mut self) -> Stmt {
        let expr = self.assignment();
        self.consume_end_of_statement();
        Stmt::Expression(expr)
    }
}

pub use control_flow::*;
