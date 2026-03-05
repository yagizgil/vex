use vex_core::token::TokenType;

pub fn get_keyword(s: &str) -> Option<TokenType> {
    match s {
        // Declarations & Control Flow
        "var" => Some(TokenType::Var),
        "fn" => Some(TokenType::Fn),
        "return" => Some(TokenType::Return),
        "ret" => Some(TokenType::Return),
        "if" => Some(TokenType::If),
        "elif" => Some(TokenType::Elif),
        "else" => Some(TokenType::Else),
        "for" => Some(TokenType::For),
        "in" => Some(TokenType::In),
        "match" => Some(TokenType::Match),
        "const" => Some(TokenType::Const),
        "struct" => Some(TokenType::Struct),
        "enum" => Some(TokenType::Enum),
        "impl" => Some(TokenType::Impl),
        "import" => Some(TokenType::Import),
        "define" => Some(TokenType::Define),
        "def" => Some(TokenType::Define),
        "macro" => Some(TokenType::Macro),
        "self" => Some(TokenType::Self_),
        
        // Access Modifiers
        "pub" => Some(TokenType::Pub),
        "static" => Some(TokenType::Static),
        "priv" => Some(TokenType::Priv),
        
        // Async & Error Handling
        "async" => Some(TokenType::Async),
        "await" => Some(TokenType::Await),
        "Ok" => Some(TokenType::Ok),
        "Err" => Some(TokenType::Err),
        "try" => Some(TokenType::Try),
        
        // Booleans & Null
        "true" => Some(TokenType::True),
        "false" => Some(TokenType::False),
        "null" => Some(TokenType::Null),
        
        // Loop Controls
        "break" => Some(TokenType::Break),
        "continue" => Some(TokenType::Continue),
        "while" => Some(TokenType::While),
        
        // Type Annotations
        "string" => Some(TokenType::TStr),
        "str" => Some(TokenType::TStr),
        "int" => Some(TokenType::TInt),
        "float" => Some(TokenType::TFloat),
        "bool" => Some(TokenType::TBool),
        "list" => Some(TokenType::TList),
        "List" => Some(TokenType::TList),
        "dict" => Some(TokenType::TDict),
        "Dict" => Some(TokenType::TDict),
        "any" => Some(TokenType::TAny),
        
        // Logical Operators
        "and" => Some(TokenType::And),
        "or" => Some(TokenType::Or),
        
        // Conversion
        "as" => Some(TokenType::As),
        "to" => Some(TokenType::To),
        
        _ => None,
    }
}