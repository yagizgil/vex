use crate::ast::stmt::Stmt;
use crate::lexer::token::TokenType;
use crate::parser::Parser;
use super::Statement;

pub struct IfStmt;
pub struct WhileStmt;
pub struct ReturnStmt;

impl Statement for IfStmt {
    fn parse(p: &mut Parser) -> Stmt {
        let condition = p.expression();
        
        p.consume(TokenType::Colon, "Expect ':' after condition.");
        p.consume(TokenType::Newline, "Expect newline after ':'.");
        p.consume(TokenType::Indent, "Expect indent after colon.");
        
        let then_branch = p.block();
        let mut else_branch = None;

        if p.match_token(&[TokenType::Elif]) {
            else_branch = Some(Box::new(IfStmt::parse(p)));
        }
        else if p.match_token(&[TokenType::Else]) {
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
        let condition = p.expression();
        
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
            value = Some(p.expression());
        }
        p.consume_end_of_statement();
        Stmt::Return { keyword, value }
    }
}