use crate::Lexer;
use vex_core::token::TokenType;

impl Lexer {
    /// Tokenizes functional alphanumeric reference sequences.
    ///
    /// Identifiers resolve abstract symbol specifications correlating to variable assignments,
    /// definitions, and structured protocol references. Lexical constraints require compliance 
    /// with standard alphanumeric prefixes including underscores (`_`).
    /// 
    /// Values parsed undergo logical mapping against predefined framework directives 
    /// managed centrally in `get_keyword()`.
    pub(crate) fn identifier(&mut self, first_char: char) -> TokenType {
        let mut id_str = String::from(first_char);
        
        // Accumulate contiguous permitted values within structural token bounds constraints.
        while self.cursor.peek().is_alphanumeric() || self.cursor.peek() == '_' {
            id_str.push(self.cursor.advance());
        }

        // Evaluate mapping tables for potential operational keyword semantics overlay mappings.
        if let Some(kind) = crate::keywords::get_keyword(&id_str) {
            kind
        } else {
            TokenType::Identifier 
        }
    }
}