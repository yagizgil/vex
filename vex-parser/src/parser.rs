use crate::preparser::PreParser;
use vex_core::token::{Token, TokenType};
use vex_core::ast::{Expr, Stmt, TypeExpr};
use vex_diagnostic::diag_emit;
use vex_diagnostic::error_codes::DiagnosticCode;
use crate::declarations::{
    Declaration, EnumDecl, FnDecl, StructDecl, VarDecl, 
    ImplDecl, MacroDecl, DefineDecl, ImportDecl
};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut preparser = PreParser::new(tokens);
        let refined = preparser.process();
        Self {
            tokens: refined,
            idx: 0,
        }
    }

    pub fn peek_at(&self, offset: usize) -> &Token {
        self.tokens
            .get(self.idx + offset)
            .unwrap_or_else(|| self.tokens.last().unwrap())
    }

    pub fn peek(&self) -> &Token {
        self.peek_at(0)
    }

    pub fn check(&self, token_type: TokenType) -> bool {
        self.peek().kind == token_type
    }

    pub fn match_token(&mut self, token_type: TokenType) -> bool {
        self.check(token_type).then(|| self.advance()).is_some()
    }

    pub fn advance(&mut self) -> Token {
        let t = self.peek().clone();
        if !self.is_at_end() {
            self.idx += 1;
        }
        t
    }

    pub fn previous(&self) -> &Token {
        &self.tokens[self.idx.saturating_sub(1)]
    }

    pub fn is_at_end(&self) -> bool {
        self.peek().kind == TokenType::Eof
    }

    pub fn parse(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::new();
        while let Some(decl) = self.next_declaration() {
            declarations.push(decl);
        }
        declarations
    }

    pub fn next_declaration(&mut self) -> Option<Declaration> {
        while !self.is_at_end() {
            // Newline and StatementEnd tokens at the root level can be safely skipped
            if self.match_token(TokenType::Newline) || self.match_token(TokenType::StatementEnd) {
                continue;
            }

            let prev_idx = self.idx;
            if let Some(decl) = self.parse_declaration() {
                return Some(decl);
            } else {
                // If we consumed tokens but didn't return a declaration, an error likely happened
                if self.idx > prev_idx {
                    return None; // Stop for inspector
                }
                self.advance();
                return None; // Stop at each step
            }
        }
        None
    }

    pub fn parse_declaration(&mut self) -> Option<Declaration> {
        match self.peek().kind {
            TokenType::Var | TokenType::Const => VarDecl::parse(self).map(Declaration::Var),
            TokenType::Fn => FnDecl::parse(self).map(Declaration::Fn),
            TokenType::Struct => StructDecl::parse(self).map(Declaration::Struct),
            TokenType::Enum => EnumDecl::parse(self).map(Declaration::Enum),
            TokenType::Impl => ImplDecl::parse(self).map(Declaration::Impl),
            TokenType::Macro => MacroDecl::parse(self).map(Declaration::Macro),
            TokenType::Define => DefineDecl::parse(self).map(Declaration::Define),
            TokenType::Import => ImportDecl::parse(self).map(Declaration::Import),
            // Modifiers can precede declarations
            TokenType::Pub | TokenType::Static | TokenType::Priv | TokenType::Async => {
                // Determine the actual declaration after skipping modifiers (this logic will be implemented in the components themselves, but we can fast-forward or peek properly).
                // Or maybe the components will consume the modifiers. For now, since peek() gives modifier, we should let the components try parsing if they start with a modifier.
                // A better approach is peeking ahead or letting a unified declaration consumer take over.
                // For now, let's keep it simple. If we see `pub`, it could be `Fn`, `Struct`, `Var`, `Enum`, etc.
                // We'll peek ahead to find the actual declaration core keyword.
                let mut lookahead = 1;
                let mut token_type = self.peek_at(lookahead).kind.clone();
                while matches!(token_type, TokenType::Pub | TokenType::Static | TokenType::Priv | TokenType::Async) {
                    lookahead += 1;
                    token_type = self.peek_at(lookahead).kind.clone();
                }
                match token_type {
                    TokenType::Var | TokenType::Const => VarDecl::parse(self).map(Declaration::Var),
                    TokenType::Fn => FnDecl::parse(self).map(Declaration::Fn),
                    TokenType::Struct => StructDecl::parse(self).map(Declaration::Struct),
                    TokenType::Enum => EnumDecl::parse(self).map(Declaration::Enum),
                    TokenType::Impl => ImplDecl::parse(self).map(Declaration::Impl),
                    _ => None,
                }
            },
            _ => None,
        }
    }

    // --- Pratt Parser Precedences ---
    const PREC_NONE: u8 = 0;
    const PREC_ASSIGNMENT: u8 = 1;
    const PREC_OR: u8 = 2;
    const PREC_AND: u8 = 3;
    const PREC_EQUALITY: u8 = 4;
    const PREC_COMPARISON: u8 = 5;
    const PREC_TERM: u8 = 6;
    const PREC_FACTOR: u8 = 7;
    const PREC_UNARY: u8 = 8;
    const PREC_CALL: u8 = 9;
    const PREC_PRIMARY: u8 = 10;

    pub fn get_precedence(kind: &TokenType) -> u8 {
        match kind {
            TokenType::Equal | TokenType::PlusPlus | TokenType::MinusMinus => Self::PREC_ASSIGNMENT,
            TokenType::Or => Self::PREC_OR,
            TokenType::And => Self::PREC_AND,
            TokenType::EqualEqual | TokenType::BangEqual => Self::PREC_EQUALITY,
            TokenType::Less | TokenType::LessEqual | TokenType::Greater | TokenType::GreaterEqual => Self::PREC_COMPARISON,
            TokenType::Plus | TokenType::Minus => Self::PREC_TERM,
            TokenType::Star | TokenType::Slash => Self::PREC_FACTOR,
            TokenType::LeftParen | TokenType::Dot | TokenType::LeftBracket => Self::PREC_CALL,
            // In Vex, any token that can start an expression acts as an infix for Function Calls
            k if Self::has_prefix_rule(k) => Self::PREC_CALL,
            _ => Self::PREC_NONE,
        }
    }

    // --- Parser Utilities ---

    /// Expects a token of a certain type, consuming it if matched, and returning it.
    /// Emits a diagnostic error if it doesn't match.
    pub fn expect(&mut self, kind: TokenType, err_msg: &str) -> Option<Token> {
        if self.check(kind) {
            Some(self.advance())
        } else {
            let token = self.peek().clone();
            diag_emit!(Error, P001, err_msg.to_string(), token.span);
            None
        }
    }

    /// Same as match_token, but reads better in some contexts
    pub fn consume(&mut self, kind: TokenType) -> bool {
        self.match_token(kind)
    }

    pub fn is_next(&self, kind: TokenType) -> bool {
        self.peek_at(1).kind == kind
    }

    /// Parses a type expression (e.g., int, str, CustomClass, List[str], fn(int) -> bool)
    pub fn parse_type_expr(&mut self) -> Option<TypeExpr> {
        // Function Reference (Callback type) hint: If match_token(Fn)
        if self.match_token(TokenType::Fn) {
            let mut params = Vec::new();
            self.expect(TokenType::LeftParen, "Expected '(' after 'fn' in type expression");
            
            while !self.check(TokenType::RightParen) && !self.is_at_end() {
                if let Some(t) = self.parse_type_expr() {
                    params.push(t);
                }
                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
            self.expect(TokenType::RightParen, "Expected ')' after function type parameters");
            
            // In Vex, the return arrow is strictly "Minus + Greater" (->)
            self.expect(TokenType::Minus, "Expected '->' for return type");
            self.expect(TokenType::Greater, "Expected '>' for return type arrow '->'");
            
            if let Some(ret) = self.parse_type_expr() {
                return Some(TypeExpr::Function(params, Box::new(ret)));
            }
            return None;
        }

        // Generic or Simple
        if self.match_token(TokenType::Identifier) 
            || self.match_token(TokenType::TInt) 
            || self.match_token(TokenType::TStr) 
            || self.match_token(TokenType::TFloat) 
            || self.match_token(TokenType::TBool) 
            || self.match_token(TokenType::TAny)
        {
            let name = self.previous().clone();

            // Generic check (e.g., List[str])
            if self.match_token(TokenType::LeftBracket) {
                let mut generics = Vec::new();
                while !self.check(TokenType::RightBracket) && !self.is_at_end() {
                    if let Some(t) = self.parse_type_expr() {
                        generics.push(t);
                    }
                    if !self.match_token(TokenType::Comma) {
                        break;
                    }
                }
                self.expect(TokenType::RightBracket, "Expected ']' after generic type list");
                return Some(TypeExpr::Generic(name, generics));
            }

            return Some(TypeExpr::Simple(name));
        }

        None
    }

    /// Parses an expression with precedence (Pratt parsing)
    pub fn parse_expression(&mut self, precedence: u8) -> Option<Expr> {
        let mut left = self.parse_prefix()?;

        while !self.is_at_end() && precedence < Self::get_precedence(&self.peek().kind) {
            left = self.parse_infix(left)?;
        }

        Some(left)
    }

    fn parse_prefix(&mut self) -> Option<Expr> {
        let token = self.advance();
        match token.kind {
            TokenType::NumberLiteral(n) => Some(Expr::Literal(vex_core::ast::LiteralValue::Number(n))),
            TokenType::StringLiteral(s) => Some(Expr::Literal(vex_core::ast::LiteralValue::Str(s))),
            TokenType::True => Some(Expr::Literal(vex_core::ast::LiteralValue::Bool(true))),
            TokenType::False => Some(Expr::Literal(vex_core::ast::LiteralValue::Bool(false))),
            TokenType::Null => Some(Expr::Literal(vex_core::ast::LiteralValue::Null)),
            TokenType::Identifier => Some(Expr::Variable { name: token }),
            
            TokenType::LeftParen => {
                let inner = self.parse_expression(0)?;
                self.expect(TokenType::RightParen, "Expected ')' after grouped expression");
                Some(Expr::Grouping(Box::new(inner)))
            }

            TokenType::Bang | TokenType::Minus => {
                let right = self.parse_expression(Self::PREC_UNARY)?;
                Some(Expr::Unary { operator: token, right: Box::new(right) })
            }
            
            // F-String, List, Dict, Await, Closure etc. will go here
            _ => {
                diag_emit!(Error, P002, "Expected expression".to_string(), token.span);
                None
            }
        }
    }

    fn parse_infix(&mut self, left: Expr) -> Option<Expr> {
        let token = self.peek().clone();
        let precedence = Self::get_precedence(&token.kind);
        
        match token.kind {
            TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash |
            TokenType::EqualEqual | TokenType::BangEqual | 
            TokenType::Less | TokenType::LessEqual | TokenType::Greater | TokenType::GreaterEqual => {
                self.advance();
                let right = self.parse_expression(precedence)?;
                Some(Expr::Binary { left: Box::new(left), operator: token, right: Box::new(right) })
            }

            TokenType::And | TokenType::Or => {
                self.advance();
                let right = self.parse_expression(precedence)?;
                Some(Expr::Logical { left: Box::new(left), operator: token, right: Box::new(right) })
            }

            TokenType::Equal => {
                self.advance();
                let value = self.parse_expression(Self::PREC_ASSIGNMENT)?;
                Some(Expr::Assign { target: Box::new(left), operator: token, value: Box::new(value) })
            }

            // Vex Function Call (No parentheses)
            _ if Self::has_prefix_rule(&token.kind) => {
                let mut arguments = Vec::new();
                // Collect arguments as long as they look like expressions and are on the same line
                while !self.is_at_end() 
                    && Self::has_prefix_rule(&self.peek().kind) 
                    && !self.check(TokenType::Newline) 
                    && !self.check(TokenType::StatementEnd)
                    && !self.check(TokenType::Indent)
                    && !self.check(TokenType::Dedent)
                {
                    // Use a precedence high enough to not gobble following operators 
                    // that might belong to a surrounding expression
                    if let Some(arg) = self.parse_expression(Self::PREC_CALL) {
                        arguments.push(arg);
                    } else {
                        break;
                    }
                }
                Some(Expr::Call { callee: Box::new(left), arguments, closing_paren: token })
            }

            // Dot access, Indexing etc. will go here
            _ => Some(left)
        }
    }

    pub fn has_prefix_rule(kind: &TokenType) -> bool {
        matches!(kind,
            TokenType::NumberLiteral(_) | TokenType::StringLiteral(_) | 
            TokenType::True | TokenType::False | TokenType::Null |
            TokenType::Identifier | TokenType::LeftParen | TokenType::LeftBracket |
            TokenType::LeftBrace | TokenType::Bang | TokenType::Minus |
            TokenType::Fn | TokenType::FStringStart | TokenType::Await
        )
    }

    /// Parses a general statement (if, while, for, expr stmt, etc.)
    pub fn parse_statement(&mut self) -> Option<Stmt> {
        // Skip leading whitespace-like tokens
        while self.match_token(TokenType::Newline) || self.match_token(TokenType::StatementEnd) {}

        if self.is_at_end() { return None; }

        match self.peek().kind {
            TokenType::If => None, // TODO: IfStmt::parse(self)
            TokenType::While => None, // TODO: WhileStmt::parse(self)
            TokenType::For => None, // TODO: ForStmt::parse(self)
            TokenType::Match => None, // TODO: MatchStmt::parse(self)
            TokenType::Return => None, // TODO: ReturnStmt::parse(self)
            TokenType::Break => None, // TODO: BreakStmt::parse(self)
            TokenType::Continue => None, // TODO: ContinueStmt::parse(self)
            TokenType::LeftBrace => Some(Stmt::Block(self.parse_block())),
            _ => {
                // Default to expression statement
                // Some(Stmt::Expression(self.parse_expression(0)?))
                None
            }
        }
    }

    /// Parses a block of statements { ... } or indented block
    pub fn parse_block(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        // Support both { } and Indent/Dedent
        let is_braced = self.match_token(TokenType::LeftBrace);
        let stop_token = if is_braced { TokenType::RightBrace } else { TokenType::Dedent };

        if !is_braced {
            self.expect(TokenType::Indent, "Expected indentation for block");
        }

        while !self.check(stop_token.clone()) && !self.is_at_end() {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            } else {
                // If we couldn't parse a statement, advance to avoid infinite loop
                self.advance();
            }
        }

        self.expect(stop_token, "Expected end of block");
        statements
    }
}
