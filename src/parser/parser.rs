use crate::ast::{expr::Expr, expr::LiteralValue, stmt::Stmt, vtype::VarType};
use crate::lexer::token::{Token, TokenType};
use crate::utils::logger::error::ErrorCode;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if self.match_token(&[TokenType::Newline]) {
                continue;
            }
            statements.push(self.declaration());
        }
        statements
    }

    fn declaration(&mut self) -> Stmt {
        while self.match_token(&[TokenType::Newline]) {}

        match self.peek().token_type {
            TokenType::Fn => {
                self.advance();
                self.fn_declaration()
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
                self.var_declaration()
            }
            TokenType::Return => {
                self.advance();
                self.return_statement()
            }
            TokenType::If => {
                self.advance();
                self.if_statement()
            }
            TokenType::While => {
                self.advance();
                self.while_statement()
            }
            _ => self.statement(),
        }
    }

    fn fn_declaration(&mut self) -> Stmt {
        let name = self.consume(TokenType::Identifier, "Expect function name.");
        let mut params = Vec::new();
        while !self.check(&TokenType::Colon) && !self.is_at_end() {
            params.push(self.consume(TokenType::Identifier, "Expect parameter name."));
        }
        self.consume(TokenType::Colon, "Expect ':' after parameters.");
        self.consume(TokenType::Newline, "Expect newline after ':'.");
        self.consume(TokenType::Indent, "Expect indentation for function body.");
        let body = self.block();
        Stmt::Fn { name, params, body }
    }

    fn var_declaration(&mut self) -> Stmt {
        let mut _vtype = VarType::from(&self.peek().token_type);

        let name = self.consume(TokenType::Identifier, "Expect variable name.");
        self.match_token(&[TokenType::Equal]);
        let initializer = self.expression();
        self.consume_end_of_statement();

        Stmt::Var {
            name,
            vtype: _vtype,
            initializer,
        }
    }

    fn while_statement(&mut self) -> Stmt {
        let condition = self.expression();
        self.consume(TokenType::Colon, "Expect ':' after while condition.");
        self.consume(TokenType::Newline, "Expect newline after ':'.");
        self.consume(TokenType::Indent, "Expect indent after while.");
        let body = self.block();
        Stmt::While { condition, body }
    }

    fn if_statement(&mut self) -> Stmt {
        let condition = self.expression();
        self.consume(TokenType::Colon, "Expect ':' after condition.");
        self.consume(TokenType::Newline, "Expect newline after ':'.");
        self.consume(TokenType::Indent, "Expect indent after colon.");
        let then_branch = self.block();
        let mut else_branch = None;

        if self.match_token(&[TokenType::Elif]) {
            else_branch = Some(Box::new(self.if_statement()));
        } else if self.match_token(&[TokenType::Else]) {
            self.consume(TokenType::Colon, "Expect ':' after else.");
            self.consume(TokenType::Newline, "Expect newline after ':'.");
            self.consume(TokenType::Indent, "Expect indent after else.");
            else_branch = Some(Box::new(Stmt::Block(self.block())));
        }

        Stmt::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    fn statement(&mut self) -> Stmt {
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
                        let value = self.expression();
                        self.consume_end_of_statement();
                        return Stmt::Expression(Expr::Assign {
                            name,
                            value: Box::new(value),
                        });
                    }
                }
            }
        }

        self.expression_statement()
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        while !self.check(&TokenType::Dedent) && !self.is_at_end() {
            stmts.push(self.declaration());
        }

        if self.check(&TokenType::Dedent) {
            self.advance();
        }

        stmts
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume_end_of_statement();
        Stmt::Expression(expr)
    }

    fn expression(&mut self) -> Expr {
        self.assignment()
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.comparison();

        if self.match_token(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment();

            if let Expr::Variable(name) = expr {
                return Expr::Assign {
                    name,
                    value: Box::new(value),
                };
            }
            // panic!("Line {}: Invalid assignment target.", equals.line);
            self._panic(ErrorCode::Unknown, Some(format!("Line {}: Invalid assignment target.", equals.line)));
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
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

    fn term(&mut self) -> Expr {
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

    fn factor(&mut self) -> Expr {
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

    fn unary(&mut self) -> Expr {
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

    fn call(&mut self) -> Expr {
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

    fn is_argument_start(&self) -> bool {
        let t = &self.peek().token_type;
        matches!(
            t,
            TokenType::Identifier
                | TokenType::NumberLiteral(_)
                | TokenType::StringLiteral(_)
                | TokenType::LeftParen
                | TokenType::True
                | TokenType::False
                | TokenType::Null
        )
    }

    fn is_binary_operator(&self) -> bool {
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

    fn peek_next_is(&self, t_type: &TokenType) -> bool {
        if self.current + 1 >= self.tokens.len() {
            return false;
        }
        std::mem::discriminant(&self.tokens[self.current + 1].token_type)
            == std::mem::discriminant(t_type)
    }
    fn primary(&mut self) -> Expr {
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
                Expr::Variable(self.previous())
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression();
                self.consume(TokenType::RightParen, "Expect ')' after expression.");
                Expr::Grouping(Box::new(expr))
            }
            _ => {
                // panic!(
                //     "Line {}: Expect expression, found {:?}",
                //     self.peek().line,
                //     self.peek().token_type
                // );

                self._panic(ErrorCode::Unknown, Some(format!("{:?}", self.peek().token_type)));
            }
        }
    }

    fn return_statement(&mut self) -> Stmt {
        let keyword = self.previous();
        let mut value = None;
        if !self.check(&TokenType::Newline) && !self.check(&TokenType::Dedent) {
            value = Some(self.expression());
        }
        self.consume_end_of_statement();
        Stmt::Return { keyword, value }
    }

    fn consume_end_of_statement(&mut self) {
        while self.match_token(&[TokenType::Newline]) {}
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn match_number(&mut self) -> Option<Token> {
        if let TokenType::NumberLiteral(_) = self.peek().token_type {
            return Some(self.advance());
        }
        None
    }

    fn match_string(&mut self) -> Option<Token> {
        if let TokenType::StringLiteral(_) = self.peek().token_type {
            return Some(self.advance());
        }
        None
    }

    fn check(&self, t_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(t_type)
    }
    #[inline]
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
     #[inline]
    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }
     #[inline]
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    #[inline]
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn consume(&mut self, t_type: TokenType, message: &str) -> Token {
        if self.check(&t_type) {
            return self.advance();
        }
        // panic!("Error (Line {}): {}", self.peek().line, message);
        self._panic(ErrorCode::Unknown, Some(message.to_string()));
    }
}


impl Parser  {
    fn _rerr(&mut self, _err: ErrorCode, detail: Option<String>) {
        match detail {
            Some(d) => vex_pars_err!(self.peek().line, _err, d),
            None => vex_pars_err!(self.peek().line, _err),
        }
    }

    fn _panic(&mut self, _err: ErrorCode, detail: Option<String>) -> ! {
        self._rerr(_err, detail);
        crate::utils::logger::error::Reporter::display();
        std::process::exit(1);
    }
}