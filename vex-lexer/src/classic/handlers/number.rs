use crate::Lexer;
use vex_core::token::TokenType;

impl Lexer {
    /// Tokenizes numeric scalar literals comprising integral boundaries and decimal precision sequences.
    ///
    /// The parser evaluates continuous buffer boundaries targeting standardized `f64` structures for 
    /// subsequent semantic analysis. It supports unified resolution sequences for generic digit mapping 
    /// parameters correlating directly to arithmetic execution contexts.
    pub(crate) fn number(&mut self, first_digit: char) -> TokenType {
        let mut number_str = String::from(first_digit);
        
        // Sequence acquisition boundary constraints strictly evaluating digit subsets.
        while self.cursor.peek().is_ascii_digit() {
            number_str.push(self.cursor.advance());
        }

        // Decimal precision logic branch execution boundary tests.
        if self.cursor.peek() == '.' && self.cursor.peek_next().is_ascii_digit() {
            number_str.push(self.cursor.advance()); 
            
            while self.cursor.peek().is_ascii_digit() {
                number_str.push(self.cursor.advance());
            }
        }

        // Applies native memory representation bindings validating structural constraints logic sets.
        let val: f64 = number_str.parse().unwrap_or(0.0);
        TokenType::NumberLiteral(val)
    }
}
