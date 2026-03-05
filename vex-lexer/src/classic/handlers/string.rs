use crate::Lexer;
use vex_core::token::TokenType;

impl Lexer {
    /// Analyzes string boundary elements dynamically allocating static block structures.
    ///
    /// Manages standard discrete line token sequences bounded centrally by isolated matching parameters.
    /// Implements continuous multi-line payload propagation via standard Triple-Quote protocols (`"""`).
    pub(crate) fn string(&mut self, quote: char, is_triple: bool) -> TokenType {
        let mut value = String::new();
        
        while !self.cursor.is_at_end() {
            if is_triple {
                // Resolution constraints targeting multi-line blocks requiring absolute triple validation structures.
                if self.cursor.peek() == quote && self.cursor.peek_next() == quote && self.cursor.peek_at(2) == quote {
                    self.cursor.advance();
                    self.cursor.advance();
                    self.cursor.advance();
                    return TokenType::StringLiteral(value);
                }
            } else {
                if self.cursor.peek() == quote {
                    self.cursor.advance();
                    return TokenType::StringLiteral(value);
                }
                
                // Unprocessed structural line feeds inherently invalidate inline text abstractions logic.
                if self.cursor.peek() == '\n' {
                    break;
                }
            }

            // Executes character escaping evaluation mechanisms to implement system protocol variables natively.
            if self.cursor.peek() == '\\' {
                self.cursor.advance(); 
                match self.cursor.advance() {
                    'n' => value.push('\n'),
                    'r' => value.push('\r'),
                    't' => value.push('\t'),
                    '\\' => value.push('\\'), 
                    c if c == quote => value.push(quote), 
                    _ => {} 
                }
            } else {
                value.push(self.cursor.advance());
            }
        }

        TokenType::StringLiteral(value)
    }
}
