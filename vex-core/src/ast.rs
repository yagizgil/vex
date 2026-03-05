use crate::token::Token;

// TYPE SYSTEM

/// Represents types in the language (e.g., int, List<str>, fn(int) -> bool)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub enum TypeExpr {
    Simple(Token),                                 // e.g., int, str
    Generic(Token, Vec<TypeExpr>),                 // e.g., List<int>, Map<str, any>
    Function(Vec<TypeExpr>, Box<TypeExpr>),        // e.g., fn(int, str) -> bool
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub struct Parameter {
    pub name: Token,
    pub var_type: Option<TypeExpr>, // Optional for type inference (e.g., fn(x) vs fn(x: int))
}


// EXPRESSIONS

#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub enum LiteralValue {
    Number(f64),
    Str(String),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub enum Expr {
    // --- Primitives & Variables ---
    Literal(LiteralValue),
    Variable { name: Token },
    
    // --- Data Structures ---
    List { elements: Vec<Expr> },
    Dict { entries: Vec<(Expr, Expr)> },
    FString(Vec<Expr>), // Interpolated strings (e.g., f"Hello {name}")

    // --- Object-Oriented & Structs ---
    /// Object property access (e.g., car.speed or car.?speed)
    Get { 
        object: Box<Expr>, 
        name: Token,
        is_safe: bool
    },
    /// Object instantiation (e.g., User { name: "John", age: 30 })
    StructInit { 
        name: Token, 
        fields: Vec<(Token, Expr)> 
    },

    // --- Memory Access & Mutation ---
    /// Array/Dict indexing (e.g., list[0] or dict["key"] or dict.?/dyn_key)
    Index { 
        object: Box<Expr>, 
        index: Box<Expr>, 
        closing_bracket: Token, // Kept for exact Span/error location
        is_safe: bool
    },
    /// Assignment to variables, properties or indexes (e.g., x = 5, car.speed = 100, list[0] = 1)
    Assign { 
        target: Box<Expr>, // Can be Variable, Get, or Index expression
        operator: Token,   // =, +=, -=, etc.
        value: Box<Expr> 
    },

    // --- Operations ---
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Unary { operator: Token, right: Box<Expr> },
    /// Logical operations with short-circuiting (e.g., a && b, a || b)
    Logical { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping(Box<Expr>),

    // --- Functions & Control Flow ---
    /// Function execution (e.g., math.add(2, 3))
    Call { 
        callee: Box<Expr>, 
        arguments: Vec<Expr>,
        closing_paren: Token // For error tracking
    },
    /// Anonymous functions / Lambdas (e.g., fn(x) { return x * 2 })
    Closure { 
        params: Vec<Parameter>, 
        rtype: Option<TypeExpr>, 
        body: Box<Stmt> 
    },
    /// Asynchronous wait (e.g., await fetch_data())
    Await { 
        keyword: Token, 
        value: Box<Expr> 
    },
}

// STATEMENTS

#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub struct MatchCase {
    pub pattern: Expr,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub enum Stmt {
    // --- Declarations ---
    /// Variable declaration (e.g., pub var x: int = 5; or stat const y = 10;)
    VarDecl { 
        modifiers: Vec<Token>, // e.g., 'pub', 'static', 'priv'
        keyword: Token,        // 'var' or 'const'
        name: Token, 
        vtype: Option<TypeExpr>,
        initializer: Option<Expr> 
    },
    /// Function declaration (e.g., pub async fn process(data: str) -> bool { ... })
    FnDecl { 
        modifiers: Vec<Token>, // e.g., 'pub', 'static'
        name: Token, 
        params: Vec<Parameter>, 
        rtype: Option<TypeExpr>, 
        body: Vec<Stmt>,
        is_async: bool 
    },
    /// Structure/Class declaration (e.g., pub struct Queue[T] { items: list })
    StructDecl { 
        modifiers: Vec<Token>, // e.g., 'pub'
        name: Token,
        type_params: Vec<Token>, // e.g., 'T', 'K', 'V'
        fields: Vec<(Token, TypeExpr)> 
    },
    /// Enum declaration (e.g., pub enum Result[T, E] { Ok(any), Err(str) })
    EnumDecl {
        modifiers: Vec<Token>, // e.g., 'pub'
        name: Token,
        type_params: Vec<Token>, // e.g., 'T', 'E'
        variants: Vec<Token>, // Enum elements (e.g., Red, Green, Blue)
    },
    /// Implementation block for structs (e.g., impl User { pub fn grow() { ... } })
    ImplDecl { 
        target: TypeExpr, 
        methods: Vec<Stmt> // Should strictly contain FnDecl statements
    },
    /// Macro declaration for code generation (e.g., macro loop(n) { ... })
    MacroDecl {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    /// Standalone definitions (e.g., define PI 3.14)
    DefineDecl {
        name: Token,
        value: Expr,
    },

    // --- Control Flow ---
    If { 
        condition: Expr, 
        then_branch: Vec<Stmt>, 
        // Handles "elif" chains: Vec<(Condition, Body)>
        elifs: Vec<(Expr, Vec<Stmt>)>, 
        else_branch: Option<Vec<Stmt>> 
    },
    While { condition: Expr, body: Vec<Stmt> },
    /// For loop supporting multiple destructuring items (e.g., for key, value in dict)
    For { items: Vec<Token>, iterable: Expr, body: Vec<Stmt> },
    Match { condition: Expr, cases: Vec<MatchCase> },

    // --- Loop & Execution Modifiers ---
    Return { keyword: Token, value: Option<Expr> },
    Break { keyword: Token },
    Continue { keyword: Token },

    // --- Module System ---
    /// Importing from other files/modules (e.g., import math.utils)
    Import { keyword: Token, path: Vec<Token> },

    // --- Wrappers ---
    /// A standalone expression acting as a statement (e.g., print("Hello");)
    Expression(Expr),
    /// A localized scope block { ... }
    Block(Vec<Stmt>),
}