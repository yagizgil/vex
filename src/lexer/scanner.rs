use crate::lexer::token::{Token, TokenType};
use crate::utils::logger::error::ErrorCode;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,

    indent_stack: Vec<usize>,
    is_at_line_start: bool,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            indent_stack: vec![0],
            is_at_line_start: true,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            line: self.line,
        });

        self.tokens.clone()
    }

    #[inline]
    fn scan_token(&mut self) {
        if self.is_at_line_start {
            self.handle_indentation();
        }

        if self.is_at_end() {
            return;
        }

        let c = self.advance();
        match c {
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            ':' => self.add_token(TokenType::Colon),
            ';' => self.add_token(TokenType::SemiColon),
            ',' => self.add_token(TokenType::Comma),
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            '+' => self.add_token(TokenType::Plus),
            '*' => self.add_token(TokenType::Star),
            '/' => self.add_token(TokenType::Slash),

            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                }
            }
            '<' => {
                let t = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(t);
            }
            '>' => {
                let t = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(t);
            }

            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
                self.is_at_line_start = true;
                self.add_token(TokenType::Newline);
            }

            '"' | '\'' | '`' => self.string(c),

            '#' => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            }

            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    self._rerr(ErrorCode::UnexpectedChar,Some(c.to_string()));
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();

        let t_type = match text.as_str() {
            "var" => TokenType::Var,
            "const" => TokenType::Const,
            "fn" => TokenType::Fn,
            "if" => TokenType::If,
            "elif" => TokenType::Elif,
            "else" => TokenType::Else,
            "for" => TokenType::For,
            "in" => TokenType::In,
            "while" => TokenType::While,
            "return" => TokenType::Return,
            "ret" => TokenType::Return,
            "match" => TokenType::Match,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "con" => TokenType::Continue,

            "struct" => TokenType::Struct,
            "impl" => TokenType::Impl,
            "use" => TokenType::Use,
            "define" => TokenType::Define,
            "def" => TokenType::Define,
            "macro" => TokenType::Macro,
            "self" => TokenType::Self_,
            "async" => TokenType::Async,
            "await" => TokenType::Await,

            "and" => TokenType::And,
            "or" => TokenType::Or,
            "not" => TokenType::Bang,

            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,

            "ok" => TokenType::Ok,
            "err" => TokenType::Err,
            "try" => TokenType::Try,

            "str" => TokenType::TStr,
            "int" => TokenType::TInt,
            "float" => TokenType::TFloat,
            "bool" => TokenType::TBool,
            "list" => TokenType::TList,
            "dict" => TokenType::TDict,
            "any" => TokenType::TAny,

            _ => TokenType::Identifier,
        };

        self.add_token(t_type);
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        let value: f64 = text.parse().unwrap_or(0.0);
        self.add_token(TokenType::NumberLiteral(value));
    }

    fn string(&mut self, delimiter: char) {
        while self.peek() != delimiter && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            if self.peek() == '\\' && self.peek_next() == delimiter {
                self.advance();
            }

            self.advance();
        }

        if self.is_at_end() {
            self._rerr(ErrorCode::UnterminatedString, None);
            return;
        }

        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();

        self.add_token(TokenType::StringLiteral(value));
    }

    fn handle_indentation(&mut self) {
        let mut spaces = 0;

        while !self.is_at_end() && (self.peek() == ' ' || self.peek() == '\t') {
            if self.peek() == ' ' {
                spaces += 1;
            } else {
                spaces += 4;
            }
            self.advance();
        }

        if self.peek() == '\n' || self.peek() == '#' || self.is_at_end() {
            self.is_at_line_start = false;
            return;
        }

        let last_indent = *self.indent_stack.last().unwrap();

        if spaces > last_indent {
            self.indent_stack.push(spaces);
            self.add_token(TokenType::Indent);
        } else if spaces < last_indent {
            while spaces < *self.indent_stack.last().unwrap() {
                self.indent_stack.pop();
                self.add_token(TokenType::Dedent);
            }

            if spaces != *self.indent_stack.last().unwrap() {
                self._rerr(ErrorCode::Indentation, None);
            }
        }

        self.is_at_line_start = false;
        self.start = self.current;
    }

    #[inline]
    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    #[inline]
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    #[inline]
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, t_type: TokenType) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token {
            token_type: t_type,
            lexeme: text,
            line: self.line,
        });
    }
}

impl Scanner {
    fn _rerr(&mut self, _err: ErrorCode, detail: Option<String>) {
        match detail {
            Some(d) => vex_lex_err!(self.line, _err, d),
            None => vex_lex_err!(self.line, _err),
        }
    }
}
