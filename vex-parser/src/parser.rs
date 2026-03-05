use crate::preparser::PreParser;
use vex_core::token::{Token, TokenType};

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
        self.idx += (t.kind != TokenType::Eof) as usize;
        t
    }

    pub fn is_at_end(&self) -> bool {
        self.peek().kind == TokenType::Eof
    }
}
