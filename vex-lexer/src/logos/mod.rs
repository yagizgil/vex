use vex_core::token::{Token, TokenType};
use vex_core::span::Span;
use logos::Logos;

/// LogosToken enum defines the regex patterns and rules for the Vex Lexer.
/// Logos uses a finite state machine under the hood to evaluate all of these rules concurrently,
/// making the tokenization phase extremely fast.
#[derive(Logos, Debug, PartialEq, Clone)]
// Skip any spaces, tabs, or carriage returns (used in Windows \r\n). 
// We DO NOT skip newlines (\n) here because we need them to calculate Indent/Dedent behavior!
#[logos(skip r"[ \t\f\r]+")] 
// Skip single-line comments. Starts with '//' and stops at the end of the line.
#[regex(r"//[^\n]*", logos::skip, allow_greedy = true)]    
pub enum LogosToken {
    // Keywords
    #[token("var")] Var,
    #[token("fn")] Fn,
    #[token("return")] Return,
    #[token("if")] If,
    #[token("else")] Else,
    #[token("while")] While,
    #[token("for")] For,
    #[token("in")] In,
    
    // Primitive Type keywords
    #[token("int")] TInt,
    #[token("str")] TStr,
    #[token("float")] TFloat,
    #[token("bool")] TBool,
    
    // Booleans
    #[token("true")] True,
    #[token("false")] False,

    // Operators
    #[token("+")] Plus,
    #[token("++")] PlusPlus,
    #[token("-")] Minus,
    #[token("--")] MinusMinus,
    #[token("*")] Star,
    #[token("/")] Slash,
    #[token("=")] Equal,
    #[token("==")] EqualEqual,
    #[token("!=")] BangEqual,
    #[token(">")] Greater,
    #[token("<")] Less,
    #[token(">=")] GreaterEqual,
    #[token("<=")] LessEqual,
    #[token(";")] StatementEnd,
    #[token(":")] Colon,
    #[token(",")] Comma,
    #[token(".")] Dot,
    #[token("(")] LeftParen,
    #[token(")")] RightParen,
    #[token("{")] LeftBrace,
    #[token("}")] RightBrace,
    #[token("[")] LeftBracket,
    #[token("]")] RightBracket,

    // --- Literals ---
    // Matches any valid variable or function name (starts with an english letter or underscore)
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // Matches any integer or floating point numbers (e.g. 100 or 100.5)
    #[regex(r"[0-9]+(\.[0-9]+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),
    
    // Matches standard string literals wrapped in double quotes
    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    StringLiteral(String),

    // --- Structural Tokens ---
    // This is the core engine for our Indentation logic. 
    // It captures a "\n" immediately followed by spaces. We use the length - 1 
    // to extract the exact number of spaces after the newline to compute indentation levels.
    #[regex(r"\n[ \t]*", |lex| lex.slice().len() - 1)] 
    Newline(usize),
}

/// CursorCompat provides a compatibility layer for the Vex Inspector.
/// Since Logos consumes the string internally, we simulate a cursor position
/// manually to keep the TUI highlighting and state displays happy.
pub struct CursorCompat {
    pub pos: usize,
    pub line: usize,
    pub col: usize,
    content: String,
}

impl CursorCompat {
    /// Returns the character at the current simulated position.
    pub fn peek(&self) -> char {
        self.content.chars().nth(self.pos).unwrap_or('\0')
    }
}

/// The Lexer struct wraps the Logos implementation to seamlessly integrate it 
/// into the Vex Compiler pipeline. It handles indent/dedent conversion and line counting.
pub struct Lexer {
    pub file_id: usize,
    content: String,
    
    // Compatibility fields for the Inspector app
    pub cursor: CursorCompat,
    pub interpolation_stack: Vec<(char, bool, usize)>,
    
    // Internal Iterator containing the fully processed token stream
    tokens: std::vec::IntoIter<Token>,
}

impl Lexer {
    pub fn new(file_id: usize, content: String) -> Self {
        let mut lexer = Self {
            file_id,
            content: content.clone(),
            cursor: CursorCompat { pos: 0, line: 1, col: 1, content: content.clone() },
            interpolation_stack: Vec::new(),
            tokens: vec![].into_iter(),
        };
        lexer.tokens = lexer.run_logos().into_iter();
        lexer
    }
    
    /// Main Logos execution path. This function reads the entire file in one go,
    /// analyzes it with the Logos Regex engine, and handles indentation mathematics.
    fn run_logos(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut lex = LogosToken::lexer(&self.content);
        let mut indent_stack = vec![0];
        let mut current_line = 1;
        let mut last_col_offset = 0;

        while let Some(res) = lex.next() {
            let start = lex.span().start;
            let end = lex.span().end;
            let col = start - last_col_offset + 1;
            let span = Span::new(self.file_id, start, end, current_line, col);

            match res {
                Ok(LogosToken::Newline(spaces)) => {
                    // We found a newline. Push a standard Newline token first.
                    // The actual \n character is exactly at `start`.
                    let newline_span = Span::new(self.file_id, start, start + 1, current_line, col);
                    tokens.push(Token::new(TokenType::Newline, newline_span));
                    
                    // Update our manual line/column trackers to simulate 2D locations
                    current_line += 1;
                    // Start of the new line is exactly at `start + 1`. This fixes the column indices of following tokens.
                    last_col_offset = start + 1;
                    
                    // The Indent token span should only point to the space characters, not the newline.
                    let indent_span = Span::new(self.file_id, start + 1, end, current_line, 1);
                    
                    // Pythonic Indentation Algorithm
                    let current_indent = *indent_stack.last().unwrap();
                    if spaces > current_indent {
                        // User went inward (e.g. from 0 spaces to 4 spaces)
                        indent_stack.push(spaces);
                        tokens.push(Token::new(TokenType::Indent, indent_span.clone()));
                    } else if spaces < current_indent {
                        // User went backward (e.g. from 8 spaces to 0)
                        // We must emit a Dedent token for EVERY step back up the ladder.
                        while let Some(&last) = indent_stack.last() {
                            if last > spaces {
                                indent_stack.pop();
                                tokens.push(Token::new(TokenType::Dedent, indent_span.clone()));
                            } else {
                                break;
                            }
                        }
                    }
                }
                Ok(LogosToken::Identifier(_s)) => tokens.push(Token::new(TokenType::Identifier, span)),
                Ok(LogosToken::Number(n)) => tokens.push(Token::new(TokenType::NumberLiteral(n), span)),
                Ok(LogosToken::StringLiteral(s)) => tokens.push(Token::new(TokenType::StringLiteral(s), span)),
                Ok(LogosToken::Var) => tokens.push(Token::new(TokenType::Var, span)),
                Ok(LogosToken::Fn) => tokens.push(Token::new(TokenType::Fn, span)),
                Ok(LogosToken::If) => tokens.push(Token::new(TokenType::If, span)),
                Ok(LogosToken::Else) => tokens.push(Token::new(TokenType::Else, span)),
                Ok(LogosToken::While) => tokens.push(Token::new(TokenType::While, span)),
                Ok(LogosToken::For) => tokens.push(Token::new(TokenType::For, span)),
                Ok(LogosToken::In) => tokens.push(Token::new(TokenType::In, span)),
                Ok(LogosToken::Return) => tokens.push(Token::new(TokenType::Return, span)),
                Ok(LogosToken::Equal) => tokens.push(Token::new(TokenType::Equal, span)),
                Ok(LogosToken::EqualEqual) => tokens.push(Token::new(TokenType::EqualEqual, span)),
                Ok(LogosToken::BangEqual) => tokens.push(Token::new(TokenType::BangEqual, span)),
                Ok(LogosToken::Greater) => tokens.push(Token::new(TokenType::Greater, span)),
                Ok(LogosToken::Less) => tokens.push(Token::new(TokenType::Less, span)),
                Ok(LogosToken::GreaterEqual) => tokens.push(Token::new(TokenType::GreaterEqual, span)),
                Ok(LogosToken::LessEqual) => tokens.push(Token::new(TokenType::LessEqual, span)),
                Ok(LogosToken::Plus) => tokens.push(Token::new(TokenType::Plus, span)),
                Ok(LogosToken::PlusPlus) => tokens.push(Token::new(TokenType::PlusPlus, span)),
                Ok(LogosToken::Minus) => tokens.push(Token::new(TokenType::Minus, span)),
                Ok(LogosToken::MinusMinus) => tokens.push(Token::new(TokenType::MinusMinus, span)),
                Ok(LogosToken::Star) => tokens.push(Token::new(TokenType::Star, span)),
                Ok(LogosToken::Slash) => tokens.push(Token::new(TokenType::Slash, span)),
                Ok(LogosToken::StatementEnd) => tokens.push(Token::new(TokenType::StatementEnd, span)),
                Ok(LogosToken::Colon) => tokens.push(Token::new(TokenType::Colon, span)),
                Ok(LogosToken::Comma) => tokens.push(Token::new(TokenType::Comma, span)),
                Ok(LogosToken::Dot) => tokens.push(Token::new(TokenType::Dot, span)),
                Ok(LogosToken::LeftBrace) => tokens.push(Token::new(TokenType::LeftBrace, span)),
                Ok(LogosToken::RightBrace) => tokens.push(Token::new(TokenType::RightBrace, span)),
                Ok(LogosToken::LeftParen) => tokens.push(Token::new(TokenType::LeftParen, span)),
                Ok(LogosToken::RightParen) => tokens.push(Token::new(TokenType::RightParen, span)),
                Ok(LogosToken::LeftBracket) => tokens.push(Token::new(TokenType::LeftBracket, span)),
                Ok(LogosToken::RightBracket) => tokens.push(Token::new(TokenType::RightBracket, span)),
                Ok(LogosToken::TInt) => tokens.push(Token::new(TokenType::TInt, span)),
                Ok(LogosToken::TStr) => tokens.push(Token::new(TokenType::TStr, span)),
                Ok(LogosToken::TFloat) => tokens.push(Token::new(TokenType::TFloat, span)),
                Ok(LogosToken::TBool) => tokens.push(Token::new(TokenType::TBool, span)),
                Ok(LogosToken::True) => tokens.push(Token::new(TokenType::True, span)),
                Ok(LogosToken::False) => tokens.push(Token::new(TokenType::False, span)),
                
                // Unhandled matches map to unknown for now
                _ => tokens.push(Token::new(TokenType::Unknown, span)),
            }
        }
        
        // Close the entire hierarchy when we hit the End Of File (EOF).
        // If the file ends while we are deep inside multiple blocks, we must emit 
        // the remaining Dedent logic correctly.
        let end_span = Span::new(self.file_id, self.content.len(), self.content.len(), current_line, self.content.len() - last_col_offset + 1);
        while indent_stack.len() > 1 {
            indent_stack.pop();
            tokens.push(Token::new(TokenType::Dedent, end_span.clone()));
        }
        tokens.push(Token::new(TokenType::Eof, end_span));

        tokens
    }

    pub fn next_token(&mut self) -> Token {
        if let Some(token) = self.tokens.next() {
            self.cursor.pos = token.span.end;
            self.cursor.line = token.span.line;
            self.cursor.col = token.span.col + (token.span.end - token.span.start);
            token
        } else {
            let len = self.content.len();
            Token::new(TokenType::Eof, Span::new(self.file_id, len, len, self.cursor.line, self.cursor.col))
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut all = Vec::new();
        loop {
            let t = self.next_token();
            let is_eof = matches!(t.kind, TokenType::Eof);
            all.push(t);
            if is_eof { break; }
        }
        all
    }
}
