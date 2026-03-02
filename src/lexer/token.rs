use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum TokenType {
    Dot, Minus, Colon, Comma, Plus, Star, Slash, Equal, Bang, SemiColon,
    LeftParen, RightParen,   // ( )
    LeftBrace, RightBrace,   // { }
    LeftBracket, RightBracket, // [ ]

    Greater, Less, GreaterEqual, LessEqual, 
    EqualEqual, BangEqual,
    And, Or, 

    Identifier,
    StringLiteral(String),
    NumberLiteral(f64),

    Var, Fn, Return, If, Elif,  Else, For, In, Match, Const,
    
    Struct, Impl, Import, Define, Macro, Self_,
    
    Async, Await,
    Ok, Err, Try,
    
    True, False, Null,

    TStr, TInt, TFloat, TBool, TList, TDict, TAny,

    Indent,
    Dedent,
    Newline,
    Eof,

    Break, Continue, While
}

#[derive(Serialize, Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Self { token_type, lexeme, line }
    }
}