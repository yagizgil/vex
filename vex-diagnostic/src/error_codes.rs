
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub enum DiagnosticCode {
    // --- Lexer Errors ---
    L001, // Unexpected character
    L002, // Unterminated string
    L003, // Invalid number literal
    L004, // Unterminated f-string expression
    L005, // Indentation error

    // --- Parser Errors ---
    P001, // Unexpected token
    P002, // Expected expression
    P003, // Expected statement
    P004, // Missing semicolon
    P005, // Missing closing parenthesis/bracket/brace
    P006, // Invalid assignment target

    // --- Pre-parser Errors ---
    PR001, // Circular dependency detected
    PR002, // Module not found
    PR003, // Invalid import

    // --- Semantic Errors ---
    S001, // Variable already defined
    S002, // Variable not found
    S003, // Type mismatch
    S004, // Function already defined
    S005, // Function not found
    S006, // Immutable variable reassignment
    S007, // Break/Continue outside of loop

    // --- Interpreter / Runtime Errors ---
    R001, // Division by zero
    R002, // Out of bounds access
    R003, // Invalid operation
    R004, // Stack overflow
}

impl DiagnosticCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::L001 => "L001",
            Self::L002 => "L002",
            Self::L003 => "L003",
            Self::L004 => "L004",
            Self::L005 => "L005",
            
            Self::P001 => "P001",
            Self::P002 => "P002",
            Self::P003 => "P003",
            Self::P004 => "P004",
            Self::P005 => "P005",
            Self::P006 => "P006",
            
            Self::PR001 => "PR001",
            Self::PR002 => "PR002",
            Self::PR003 => "PR003",

            Self::S001 => "S001",
            Self::S002 => "S002",
            Self::S003 => "S003",
            Self::S004 => "S004",
            Self::S005 => "S005",
            Self::S006 => "S006",
            Self::S007 => "S007",

            Self::R001 => "R001",
            Self::R002 => "R002",
            Self::R003 => "R003",
            Self::R004 => "R004",
        }
    }
}
