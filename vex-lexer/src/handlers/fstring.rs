use crate::Lexer;
use vex_core::token::TokenType;

impl Lexer {
    /// Initializes the parsing state for an interpolated string (f-string).
    ///
    /// Vex supports string interpolation using the `f""`, `f''`, or backtick `` ` `` syntax.
    /// Multi-line f-strings using triple quotes (`f"""..."""`) are also supported.
    ///
    /// Example:
    /// ```vex
    /// var user "Admin"
    /// var msg f"Welcome, {user}!"
    /// ```
    ///
    /// This method does not eagerly parse the entire string. Instead, it records the 
    /// delimiter specification to the `interpolation_stack`. This lazy evaluation model 
    /// allows the lexer to suspend string tokenization and fall back to standard 
    /// expression tokenization when encountering `{}` boundaries.
    ///
    /// Arguments:
    /// - `quote`: The primary delimiter character ('"', '\'', or '`').
    /// - `is_triple`: Indicates if the sequence is a triple-quote multi-line block.
    pub(crate) fn fstring(&mut self, quote: char, is_triple: bool) -> TokenType {
        // Push the context frame. The `0` indicates the current depth of nested `{}` braces.
        self.interpolation_stack.push((quote, is_triple, 0));
        TokenType::FStringStart
    }

    /// Consumes the raw text segment of an f-string until an interpolation boundary or termination quote.
    ///
    /// The lexer toggles between `fstring_content` (for literal text) and the standard 
    /// tokenization routine (for injected expressions inside `{}`).
    /// 
    /// Example execution trace for input `f"Value: {x * 2}!"`:
    /// 1. `fstring` pushes context and emits `FStringStart`.
    /// 2. `fstring_content` accumulates "Value: " and emits `FStringContent`.
    /// 3. The `{` boundary is detected, emitting `OpenInterpolation`.
    /// 4. Standard lexer paths evaluate `x`, `*`, `2` and emit corresponding tokens.
    /// 5. The `}` boundary is resolved by the `next_token` core loop.
    /// 6. `fstring_content` resumes, accumulates "!", and emits `FStringContent`.
    /// 7. The terminating quote `"` is reached, emitting `FStringEnd`.
    pub(crate) fn fstring_content(&mut self) -> TokenType {
        let mut content = String::new();
        
        // Retrieve the active boundary context to determine termination criteria.
        let (quote, is_triple, _) = if let Some(last) = self.interpolation_stack.last() {
            *last
        } else {
            ('`', false, 0) // Fallback for invalid state
        };
        
        while !self.cursor.is_at_end() {
            let ch = self.cursor.peek();
            
            // Evaluates if the current character matches the expected termination quote.
            if ch == quote {
                if is_triple {
                    // Triple-quoted strings require strict sequence validation.
                    if self.cursor.peek_at(1) == quote && self.cursor.peek_at(2) == quote {
                        if content.is_empty() {
                            // Resolve the termination state if the buffer is empty.
                            self.cursor.advance();
                            self.cursor.advance();
                            self.cursor.advance();
                            self.interpolation_stack.pop();
                            return TokenType::FStringEnd;
                        }
                        // Suspend execution to yield the buffered content block.
                        // The termination sequence will be processed on the next invocation.
                        break;
                    }
                } else {
                    if content.is_empty() {
                        self.cursor.advance();
                        self.interpolation_stack.pop();
                        return TokenType::FStringEnd;
                    }
                    // Suspend execution to yield the current content slice.
                    break; 
                }
            }
            
            // Interpolation boundary detection.
            if ch == '{' {
                if content.is_empty() {
                    self.cursor.advance(); 
                    
                    // Increment the brace depth counter to accurately track nested structures,
                    // ensuring that `{ {x} }` correctly maps back to the string domain.
                    if let Some((_, _, count)) = self.interpolation_stack.last_mut() {
                        *count += 1;
                    }
                    return TokenType::OpenInterpolation; 
                }
                // Yield the accumulated text payload before delegating structural tokens.
                break; 
            }
            
            // Escape sequence resolution.
            if ch == '\\' {
                self.cursor.advance(); 
                match self.cursor.advance() {
                    'n' => content.push('\n'),
                    'r' => content.push('\r'),
                    't' => content.push('\t'),
                    '\\' => content.push('\\'),
                    // Escaping `{` and `}` bypasses the interpolation trigger mechanisms.
                    '{' => content.push('{'), 
                    '}' => content.push('}'), 
                    c if c == quote => content.push(quote), 
                    _ => {} // Unmapped sequences are discarded.
                }
            } else {
                content.push(self.cursor.advance());
            }
        }
        
        TokenType::FStringContent(content)
    }
}
