use std::cell::RefCell;

#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    Unknown = 000,
    UnexpectedChar = 100,
    UnterminatedString = 101,
    Indentation = 102,
    ExprExpected = 200,
    VarUndefined = 300,
}

impl ErrorCode {
    pub fn message(&self) -> &str {
        match self {
            ErrorCode::Unknown => "Unknown error!",
            ErrorCode::UnexpectedChar => "Unexpected character encountered.",
            ErrorCode::UnterminatedString => "Unterminated string literal.",
            ErrorCode::Indentation => "Indentation error.",
            ErrorCode::ExprExpected => "Expression expected.",
            ErrorCode::VarUndefined => "Undefined variable reference.",
            
        }
    }
}

pub enum VexError {
    General(usize, ErrorCode, Option<String>),
    Lexer(usize, ErrorCode, Option<String>),
    Parser(usize, ErrorCode, Option<String>),
    Engine(usize, ErrorCode, Option<String>),
}

pub struct Reporter {
    pub errors: Vec<VexError>,
}

thread_local! {
    pub static REPORTER: RefCell<Reporter> = RefCell::new(Reporter { errors: Vec::new() });
}

impl Reporter {
    pub fn add(err: VexError) {
        REPORTER.with(|r| r.borrow_mut().errors.push(err));
    }

    pub fn display() {
        REPORTER.with(|r| {
            for err in &r.borrow().errors {
                let (prefix, color, line, code, detail, label) = match err {
                    VexError::General(l, c, d) => ("General", "\x1b[31m", l, c, d, "E"),
                    VexError::Lexer(l, c, d) => ("Lexical", "\x1b[31m", l, c, d, "L"),
                    VexError::Parser(l, c, d) => ("Syntax", "\x1b[33m", l, c, d, "P"),
                    VexError::Engine(l, c, d) => ("Engine", "\x1b[35m", l, c, d, "R"),
                };

                let detail_str = detail.as_ref()
                    .map(|s| format!(" -> \x1b[1m{}\x1b[0m", s))
                    .unwrap_or_default();

                eprintln!("{}[{} {}{:03} @ L:{}]\x1b[0m {}{}", 
                    color, prefix, label, *code as u32, line, code.message(), detail_str);
            }
        });
    }

    pub fn has_errors() -> bool {
        REPORTER.with(|r| !r.borrow().errors.is_empty())
    }
}