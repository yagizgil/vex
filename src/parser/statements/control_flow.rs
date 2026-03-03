use super::Statement;
use crate::ast::stmt::{Stmt, MatchCase};
use crate::lexer::token::TokenType;
use crate::parser::Parser;

pub struct IfStmt;
pub struct WhileStmt;
pub struct ReturnStmt;
pub struct ForStmt;
pub struct MatchStmt;

impl Statement for IfStmt {
    fn parse(p: &mut Parser) -> Stmt {
        let condition = p.assignment();

        p.consume(TokenType::Colon, "Expect ':' after condition.");
        p.consume(TokenType::Newline, "Expect newline after ':'.");
        p.consume(TokenType::Indent, "Expect indent after colon.");

        let then_branch = p.block();
        let mut else_branch = None;

        if p.match_token(&[TokenType::Elif]) {
            else_branch = Some(Box::new(IfStmt::parse(p)));
        } else if p.match_token(&[TokenType::Else]) {
            p.consume(TokenType::Colon, "Expect ':' after else.");
            p.consume(TokenType::Newline, "Expect newline after ':'.");
            p.consume(TokenType::Indent, "Expect indent after else.");

            else_branch = Some(Box::new(Stmt::Block(p.block())));
        }

        Stmt::If {
            condition,
            then_branch,
            else_branch,
        }
    }
}

impl Statement for WhileStmt {
    fn parse(p: &mut Parser) -> Stmt {
        let condition = p.assignment();

        p.consume(TokenType::Colon, "Expect ':' after while condition.");
        p.consume(TokenType::Newline, "Expect newline after ':'.");
        p.consume(TokenType::Indent, "Expect indent after while.");

        let body = p.block();

        Stmt::While { condition, body }
    }
}

impl Statement for ReturnStmt {
    fn parse(p: &mut Parser) -> Stmt {
        let keyword = p.previous();
        let mut value = None;
        if !p.check(&TokenType::Newline) && !p.check(&TokenType::Dedent) {
            value = Some(p.assignment());
        }
        p.consume_end_of_statement();
        Stmt::Return { keyword, value }
    }
}

impl Statement for ForStmt {
    fn parse(p: &mut Parser) -> Stmt {
        let item = p.consume(TokenType::Identifier, "Expect variable name after 'for'.");

        p.consume(TokenType::In, "Expect 'in' after loop variable.");

        let iterable = p.assignment();

        p.consume(TokenType::Colon, "Expect ':' after for loop header.");
        p.consume(TokenType::Newline, "Expect newline after ':'.");
        p.consume(TokenType::Indent, "Expect indent for loop body.");

        let body = p.block();

        Stmt::For {
            item,
            iterable,
            body,
        }
    }
}

impl Statement for MatchStmt {
    fn parse(p: &mut Parser) -> Stmt {
        let condition = p.assignment();

        p.consume(TokenType::Colon, "Expect ':' after match condition.");
        p.consume(TokenType::Newline, "Expect newline after ':'.");
        p.consume(TokenType::Indent, "Expect indent for match body.");

        let mut cases = Vec::new();

        while !p.check(&TokenType::Dedent) && !p.is_at_end() {
            let pattern = p.assignment();

            p.consume(TokenType::Colon, "Expect ':' after pattern.");
            p.consume(TokenType::Newline, "Expect newline after ':'.");
            p.consume(TokenType::Indent, "Expect indent for case body.");

            let body = p.block();

            cases.push(MatchCase { pattern, body });

            while p.match_token(&[TokenType::Newline]) {}
        }

        p.consume(TokenType::Dedent, "Expect dedent after match cases.");

        Stmt::Match { condition, cases }
    }
}
