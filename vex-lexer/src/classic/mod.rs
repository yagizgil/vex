use crate::cursor::Cursor;
use vex_core::span::Span;
use vex_core::token::{Token, TokenType};
use vex_diagnostic::diag_emit;

pub mod cursor;
pub mod handlers;
pub mod keywords;

/// The Lexer is responsible for the lexical analysis phase of the Vex compiler.
/// It processes the raw source code string and transforms it into a sequence of Tokens.
/// 
/// The lexer maintains internal state structures to support advanced language features
/// such as Python-style block indentation and nested string interpolation boundaries.
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub struct Lexer {
    /// Unique identifier mapping to a registered SourceFile in the global Loader.
    pub file_id: usize,
    
    /// The Cursor maintains the structural position, line, and column offsets within the source array.
    pub cursor: Cursor,
    
    /// Stack used to resolve contextual scopes inside f-strings (`f"..."`).
    /// Vex allows arbitrary expression execution inside interpolations, which may
    /// themselves encompass other string definitions.
    ///
    /// Data layout: `(quote_character, is_triple_quoted, open_brace_depth)`
    pub interpolation_stack: Vec<(char, bool, usize)>,
    
    /// Stack representing the current hierarchical depth for Pythonic block scopes.
    /// Each integer reflects the cumulative whitespace offset (e.g., measured in characters).
    pub indent_stack: Vec<usize>,
    
    /// Queue for tokens generated asynchronously or retroactively within a single operation cycle.
    /// For instance, a return to block level 0 from level 3 will queue multiple `Dedent` tokens.
    pub pending_tokens: Vec<Token>,
}

impl Lexer {
    /// Instantiates a stateful lexical analyzer for the specified source file.
    pub fn new(file_id: usize, content: String) -> Self {
        Self {
            file_id,
            cursor: Cursor::new(content.chars().collect()),
            interpolation_stack: Vec::new(),
            indent_stack: vec![0],
            pending_tokens: Vec::new(),
        }
    }

    /// Primary iteration function. Retrieves the sequentially next lexical token from the source array.
    /// This method routes continuous execution through interpolation boundaries, whitespace collapsing,
    /// character classification, and semantic grouping primitives.
    pub fn next_token(&mut self) -> Token {
        // Yield tokens retroactively accumulated during branching routines (e.g., Dedent cascading).
        if !self.pending_tokens.is_empty() {
            return self.pending_tokens.remove(0);
        }

        // --- Contextual Override: Interpolation Scope Validation ---
        // Validate if the lexer is currently traversing literal text inside an interpolation boundary
        // rather than standard executable instructions. If brace depth reaches 0, standard expression
        // evaluation is suspended in favor of `FStringContent` capture.
        if let Some((_, _, brace_count)) = self.interpolation_stack.last() {
            if *brace_count == 0 {
                let start_pos = self.cursor.pos;
                let start_line = self.cursor.line;
                let start_col = self.cursor.col;

                let kind = self.fstring_content();
                let span = Span::new(self.file_id, start_pos, self.cursor.pos, start_line, start_col);
                return Token::new(kind, span);
            }
        }

        // --- Whitespace & Block Structure Resolution ---
        // Analyzes layout metrics, stripping comments and formatting spaces.
        // It concurrently yields structural `Indent`, `Dedent`, and `Newline` tokens 
        // to establish abstract logical blocks.
        let whitespace_tokens = self.skip_whitespace_and_comments();
        if !whitespace_tokens.is_empty() {
            self.pending_tokens.extend(whitespace_tokens);
            if !self.pending_tokens.is_empty() {
                return self.pending_tokens.remove(0);
            }
        }

        // Positional offsets must be synchronized post-layout analysis.
        let start_pos = self.cursor.pos;
        let start_line = self.cursor.line;
        let start_col = self.cursor.col;

        if self.cursor.is_at_end() {
            return Token::new(
                TokenType::Eof,
                Span::new(self.file_id, start_pos, start_pos + 1, start_line, start_col),
            );
        }

        // --- Core Token Identification Routing ---
        let ch = self.cursor.advance();

        let kind = match ch {
            // Structural characters
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => {
                if self.cursor.match_char('-') {
                    TokenType::MinusMinus
                } else {
                    TokenType::Minus
                }
            }
            '+' => {
                if self.cursor.match_char('+') {
                    TokenType::PlusPlus
                } else {
                    TokenType::Plus
                }
            }
            ';' => TokenType::StatementEnd,
            '*' => TokenType::Star,
            '/' => TokenType::Slash, // Inline comments and documentation blocks are resolved centrally via whitespace routines
            ':' => TokenType::Colon,
            '?' => TokenType::Question,
            
            // Scope demarcations and context resumption handling
            '{' => {
                // Adjusts the internal scope counter for nested logic blocks deployed within f-strings constraint.
                if let Some((_, _, count)) = self.interpolation_stack.last_mut() {
                    *count += 1;
                }
                TokenType::LeftBrace
            }
            '}' => {
                let mut is_close_interpolation = false;
                if let Some((_, _, count)) = self.interpolation_stack.last_mut() {
                    if *count > 0 {
                        *count -= 1;
                        // Hitting base depth inside an active substitution block flags the resumption of literal buffering
                        if *count == 0 {
                            is_close_interpolation = true;
                        }
                    }
                }
                if is_close_interpolation {
                    TokenType::CloseInterpolation
                } else {
                    TokenType::RightBrace
                }
            }

            // Stateful relational operators mapping
            '!' => {
                if self.cursor.match_char('=') {
                    TokenType::BangEqual 
                } else {
                    TokenType::Bang 
                }
            }
            '=' => {
                if self.cursor.match_char('=') {
                    TokenType::EqualEqual 
                } else {
                    TokenType::Equal 
                }
            }
            '<' => {
                if self.cursor.match_char('=') {
                    TokenType::LessEqual 
                } else {
                    TokenType::Less 
                }
            }
            '>' => {
                if self.cursor.match_char('=') {
                    TokenType::GreaterEqual 
                } else {
                    TokenType::Greater 
                }
            }

            // Primitive value abstractions and definition handling
            '"' | '\'' => {
                let is_triple = self.consume_triple(ch);
                self.string(ch, is_triple)
            }
            '`' => {
                // Backticks map inherently to raw string interpolation behavior in Vex infrastructure.
                let is_triple = self.consume_triple('`');
                self.fstring('`', is_triple)
            }
            'f' if self.cursor.peek() == '"' || self.cursor.peek() == '\'' || self.cursor.peek() == '`' => {
                // Formal f-prefix detection explicitly mapping the adjacent literal block to the interpolation routine.
                let quote = self.cursor.advance();
                let is_triple = self.consume_triple(quote);
                self.fstring(quote, is_triple)
            }
            '0'..='9' => self.number(ch),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(ch),

            // Operational fault handling for unsupported lexical definitions
            _ => {
                let span = Span::new(self.file_id, start_pos, self.cursor.pos, start_line, start_col);
                diag_emit!(Error, L001, format!("Unexpected character definition: '{}'", ch), span);
                TokenType::Unknown
            }
        };

        // Token instantiation correlating structural metadata and line matrix context
        let span = Span::new(
            self.file_id,
            start_pos,
            self.cursor.pos,
            start_line,
            start_col,
        );
        Token::new(kind, span)
    }

    /// Evaluates adjacent characters sequentially to classify potential triple-quote string markers (`"""`).
    fn consume_triple(&mut self, ch: char) -> bool {
        if self.cursor.peek() == ch && self.cursor.peek_next() == ch {
            self.cursor.advance(); 
            self.cursor.advance(); 
            true
        } else {
            false
        }
    }

    /// Processes the entire source text and yields all tokens until End-Of-File.
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            let is_eof = matches!(token.kind, TokenType::Eof);
            tokens.push(token);
            if is_eof { break; }
        }
        tokens
    }
}
