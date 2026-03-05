use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub enum TokenType {
    // --- Single-Character Symbols ---
    Dot,          // .
    Minus,        // -
    Colon,        // :
    Comma,        // ,
    Plus,         // +
    PlusPlus,     // ++
    Star,         // *
    Slash,        // /
    Equal,        // =
    MinusMinus,   // --
    Bang,         // !
    SemiColon,    // ;
    Question,     // ? (Error Propagation / Try operator)
    
    // --- Compound Access Operators ---
    DoubleColon,    // ::
    DynamicDot,     // ./
    SafeDot,        // .?
    SafeDynamicDot, // .?/

    // --- Brackets & Parentheses ---
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]

    // --- Comparison & Logical Operators ---
    Greater,      // >
    Less,         // <
    GreaterEqual, // >=
    LessEqual,    // <=
    EqualEqual,   // ==
    BangEqual,    // !=
    And,          // Logical AND keyword or operator
    Or,           // Logical OR keyword or operator

    // --- Literals & Identifiers ---
    Identifier,            // Variable, function, or class names
    StringLiteral(String), // Standard string text (e.g., "hello")
    NumberLiteral(f64),    // Numeric values (both integers and floats)

    // --- Keywords (Declarations & Control Flow) ---
    Var,          // 'var' keyword for variable declaration
    Fn,           // 'fn' keyword for function declaration
    Return,       // 'return' keyword
    If,           // 'if' keyword
    Elif,         // 'elif' keyword
    Else,         // 'else' keyword
    For,          // 'for' keyword
    In,           // 'in' keyword
    Match,        // 'match' keyword
    Const,        // 'const' keyword for constants
    Struct,       // 'struct' keyword for data structures
    Enum,         // 'enum' keyword for sum types (variants/states)
    List,         // 'list' keyword
    Dict,         // 'dict' keyword
    Impl,         // 'impl' keyword for implementations
    Import,       // 'import' keyword for modules
    Define,       // 'define' keyword
    Macro,        // 'macro' keyword
    Self_,        // 'self' keyword (appended with _ to avoid Rust keyword clash)
    
    // --- Access ---
    Pub,
    Static,
    Priv,

    // --- Async & Error Handling ---
    Async,        // 'async' keyword
    Await,        // 'await' keyword
    Ok,           // 'Ok' keyword for success states
    Err,          // 'Err' keyword for error states
    Try,          // 'try' keyword

    // --- Booleans & Null ---
    True,         // 'true' boolean literal
    False,        // 'false' boolean literal
    Null,         // 'null' value literal

    // --- Loop Controls ---
    Break,        // 'break' keyword to exit a loop
    Continue,     // 'continue' keyword to skip iteration
    While,        // 'while' loop keyword

    // --- Type Annotations ---
    TStr,         // String type annotation
    TInt,         // Integer type annotation
    TFloat,       // Float type annotation
    TBool,        // Boolean type annotation
    TList,        // List type annotation
    TDict,        // Dictionary type annotation
    TAny,         // Any type annotation

   // --- F-String ---
    FStringStart,           // Opening of an f-string: f" or f'
    FStringContent(String), // Literal text content within an f-string
    OpenInterpolation,      // Start of an embedded expression: {
    CloseInterpolation,     // End of an embedded expression: }
    FStringEnd,             // Closing of an f-string: " or '

    // --- Conversion ---
    As,           // 'as' keyword for casting
    To,           // 'to' keyword (alternatively for - or conversion)

    // --- Structural & Whitespace ---
    Indent,       // Increase in indentation level
    Dedent,       // Decrease in indentation level
    Newline,      // End of a line (\n)
    StatementEnd, // Statement separator (virtual or real)
    Eof,          // End of file (EOF) marker
    Unknown,      // For lexing errors / illegal characters
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub struct Token {
    pub kind: TokenType,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenType, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn lexeme(&self) -> String {
        let map = smap!();
        let file = map.get_file(self.span.file_id).expect("File missing");
        let start = self.span.start.min(file.content.len());
        let end = self.span.end.min(file.content.len());
        file.content[start..end].to_string()
    }
}