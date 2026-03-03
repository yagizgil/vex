use crate::ast::{expr::Expr, expr::LiteralValue, stmt::Stmt};
use crate::lexer::token::{Token, TokenType};
use crate::utils::logger::error::ErrorCode;

use crate::parser::declarations::*;
use crate::parser::statements::*;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
    #[cfg(feature = "inspector")]
    pub _r: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            #[cfg(feature = "inspector")]
            _r: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if self.match_token(&[TokenType::Newline]) {
                continue;
            }

            #[cfg(feature = "inspector")]
            {
                self._r = self.current;
            }

            let stmt = self.declaration();

            // ------ Inspector Record ------
            inspect!("Parser", &self.token_range(), &vec![stmt.clone()], "ok.");
            // ------ Inspector Record ------

            statements.push(stmt);
        }
        statements
    }

    #[cfg(feature = "inspector")]
    pub(crate) fn token_range(&mut self) -> Vec<Token> {
        if self._r < self.current {
            self.tokens[self._r..self.current].to_vec()
        } else {
            Vec::new()
        }
    }

    pub(crate) fn declaration(&mut self) -> Stmt {
        while self.match_token(&[TokenType::Newline]) {}

        let peek_type = self.peek().token_type.clone();

        // ------ Inspector Record ------
        inspect!(
            "Parser",
            &vec![self.peek().clone()],
            &vec![],
            "({:?})",
            peek_type
        );
        // ------ Inspector Record ------

        match peek_type {
            TokenType::Fn => {
                self.advance();
                FunctionDecl::parse(self)
            }
            TokenType::Var
            | TokenType::TInt
            | TokenType::TStr
            | TokenType::TFloat
            | TokenType::TBool
            | TokenType::TList
            | TokenType::TDict => {
                if self.peek().token_type == TokenType::Var {
                    self.advance();
                }
                VariableDecl::parse(self)
            }
            TokenType::Return => {
                self.advance();
                ReturnStmt::parse(self)
            }
            TokenType::If => {
                self.advance();
                IfStmt::parse(self)
            }
            TokenType::While => {
                self.advance();
                WhileStmt::parse(self)
            }
            TokenType::For => {
                self.advance();
                ForStmt::parse(self)
            }
            TokenType::Match => {
                self.advance();
                MatchStmt::parse(self)
            }
            TokenType::Import => {
                self.advance();
                ImportDecl::parse(self)
            }
            _ => self.statement(),
        }
    }

    pub(crate) fn block(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        while !self.check(&TokenType::Dedent) && !self.is_at_end() {
            stmts.push(self.declaration());
        }

        if self.check(&TokenType::Dedent) {
            self.advance();
        }

        stmts
    }

    pub(crate) fn assignment(&mut self) -> Expr {
        let expr = self.comparison();

        if self.match_token(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment();

            if let Expr::Variable { name, .. } = expr {
                return Expr::Assign {
                    name,
                    value: Box::new(value),
                    index: None,
                };
            }
            // panic!("Line {}: Invalid assignment target.", equals.line);
            vex_pars_panic!(
                self.peek().line,
                ErrorCode::Unknown,
                Some(format!("Line {}: Invalid assignment target.", equals.line))
            );
        }
        expr
    }

    pub(crate) fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::EqualEqual,
            TokenType::BangEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    pub(crate) fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    pub(crate) fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_token(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    pub(crate) fn unary(&mut self) -> Expr {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }
        self.call()
    }

    pub(crate) fn call(&mut self) -> Expr {
        let mut expr = self.primary();

        loop {
            if !self.is_at_end()
                && !self.check(&TokenType::Newline)
                && !self.check(&TokenType::Dedent)
                && !self.is_binary_operator()
                && !self.check(&TokenType::Equal)
                && !self.check(&TokenType::Colon)
                && !self.check(&TokenType::RightParen)
            {
                let mut arguments = Vec::new();
                while !self.is_at_end() && self.is_argument_start() {
                    arguments.push(self.primary());
                }

                if !arguments.is_empty() {
                    expr = Expr::Call {
                        callee: Box::new(expr),
                        arguments,
                    };
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        expr
    }

    pub(crate) fn is_argument_start(&self) -> bool {
        let t = &self.peek().token_type;
        matches!(
            t,
            TokenType::Identifier
                | TokenType::NumberLiteral(_)
                | TokenType::StringLiteral(_)
                | TokenType::LeftParen
                | TokenType::LeftBracket
                | TokenType::LeftBrace
                | TokenType::True
                | TokenType::False
                | TokenType::Null
        )
    }

    pub(crate) fn is_binary_operator(&self) -> bool {
        let t = &self.peek().token_type;
        matches!(
            t,
            TokenType::Plus
                | TokenType::Minus
                | TokenType::Star
                | TokenType::Slash
                | TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual
                | TokenType::EqualEqual
                | TokenType::BangEqual
        )
    }

    pub(crate) fn _peek_next_is(&self, t_type: &TokenType) -> bool {
        if self.current + 1 >= self.tokens.len() {
            return false;
        }
        std::mem::discriminant(&self.tokens[self.current + 1].token_type)
            == std::mem::discriminant(t_type)
    }

    pub(crate) fn primary(&mut self) -> Expr {
        while self.match_token(&[TokenType::Newline]) {}

        match &self.peek().token_type {
            TokenType::False => {
                self.advance();
                Expr::Literal(LiteralValue::Bool(false))
            }
            TokenType::True => {
                self.advance();
                Expr::Literal(LiteralValue::Bool(true))
            }
            TokenType::Null => {
                self.advance();
                Expr::Literal(LiteralValue::Null)
            }
            TokenType::NumberLiteral(val) => {
                let value = *val;
                self.advance();
                Expr::Literal(LiteralValue::Number(value))
            }
            TokenType::StringLiteral(val) => {
                let value = val.clone();
                self.advance();
                Expr::Literal(LiteralValue::Str(value))
            }
            TokenType::Identifier => {
                self.advance();
                Expr::Variable {
                    name: self.previous(),
                    index: None,
                }
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.assignment();
                self.consume(TokenType::RightParen, "Expect ')' after expression.");
                Expr::Grouping(Box::new(expr))
            }
            TokenType::LeftBracket => {
                self.advance();
                self.list()
            }
            TokenType::LeftBrace => {
                self.advance();
                self.dict()
            }
            _ => {
                // panic!(
                //     "Line {}: Expect expression, found {:?}",
                //     self.peek().line,
                //     self.peek().token_type
                // );

                vex_pars_panic!(
                    self.peek().line,
                    ErrorCode::Unknown,
                    Some(format!("{:?}", self.peek().token_type))
                );
            }
        }
    }


    pub(crate) fn consume_end_of_statement(&mut self) {
        while self.match_token(&[TokenType::Newline]) {}
    }

    pub(crate) fn list(&mut self) -> Expr {
        let mut elements = Vec::new();
        while !self.check(&TokenType::RightBracket) && !self.is_at_end() {
            elements.push(self.assignment());
            if !self.match_token(&[TokenType::Comma]) {
                break;
            }
        }
        self.consume(TokenType::RightBracket, "Expect ']' after list items.");
        Expr::List { elements }
    }

    pub(crate) fn dict(&mut self) -> Expr {
        let mut entries = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let key = self.assignment();
            self.consume(TokenType::Colon, "Expect ':' after dictionary key.");
            let value = self.assignment();
            entries.push((key, value));
            if !self.match_token(&[TokenType::Comma]) {
                break;
            }
        }
        self.consume(TokenType::RightBrace, "Expect '}' after dictionary items.");
        Expr::Dict { entries }
    }

    pub(crate) fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    pub(crate) fn _match_number(&mut self) -> Option<Token> {
        if let TokenType::NumberLiteral(_) = self.peek().token_type {
            return Some(self.advance());
        }
        None
    }

    pub(crate) fn _match_string(&mut self) -> Option<Token> {
        if let TokenType::StringLiteral(_) = self.peek().token_type {
            return Some(self.advance());
        }
        None
    }

    pub(crate) fn check(&self, t_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        let r = std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(t_type);

        // ------ Inspector Record ------
        inspect!(
            "Parser",
            &[self.peek().clone()],
            &vec![],
            "Matching peek({:?}) against expected({:?}) -> Result: {}",
            self.peek().token_type,
            t_type,
            r
        );
        // ------ Inspector Record ------

        r
    }

    #[inline]
    pub(crate) fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    #[inline]
    pub(crate) fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    #[inline]
    pub(crate) fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    #[inline]
    pub(crate) fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    pub(crate) fn consume(&mut self, t_type: TokenType, message: &str) -> Token {
        if self.check(&t_type) {
            return self.advance();
        }
        // panic!("Error (Line {}): {}", self.peek().line, message);

        // ------ Inspector Record ------
        inspect!("Parser", &[self.peek().clone()], &vec![], "before panic.");
        // ------ Inspector Record ------

        vex_pars_panic!(
            self.peek().line,
            ErrorCode::Unknown,
            Some(message.to_string())
        );
    }
}
