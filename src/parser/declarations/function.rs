use crate::ast::{stmt::Stmt, vtype::Parameter, vtype::VarType};
use crate::lexer::token::{TokenType};
use crate::parser::Parser;

use super::Declaration;

pub struct FunctionDecl;

impl Declaration for FunctionDecl {
    fn parse(p: &mut Parser) -> Stmt {
        let name = p.consume(TokenType::Identifier, "Expect function name.");

        // ------ Inspector Record ------
        inspect!(
            "Parser",
            &[name.clone()],
            &vec![],
            "fn_declaration Identifier."
        );
        // ------ Inspector Record ------

        let mut params = Vec::new();
        while (!p.check(&TokenType::Colon) && !p.check(&TokenType::Minus))
            && !p.is_at_end()
        {
            let p_name = p.consume(TokenType::Identifier, "Expect parameter name.");
            let mut p_type = VarType::Any;

            if p.match_token(&[TokenType::Dot]) {
                let t_type = p.peek().token_type.clone();
                p_type = VarType::from(&p.consume(t_type, "Expect parameter type.").token_type);
            }

            params.push(Parameter {
                name: p_name,
                var_type: p_type,
            });
        }

        let mut rtype = VarType::Any;
        if p.match_token(&[TokenType::Minus]) {
            let t_type = p.peek().token_type.clone();
            rtype = VarType::from(&p.consume(t_type, "Expect function return type.").token_type);
        }

        p.consume(TokenType::Colon, "Expect ':' after parameters.");
        p.consume(TokenType::Newline, "Expect newline after ':'.");
        p.consume(TokenType::Indent, "Expect indentation for function body.");
        
        let body = p.block();
        
        Stmt::Fn {
            name,
            params,
            body,
            rtype,
        }
    }
}