use vex_core::token::{Token, TokenType};
use vex_diagnostic::diag_emit;

/// The PreParser processes the raw token stream before the main Parser.
/// It handles complex things like converting Newlines into StatementEnd tokens 
/// and ensuring brackets are properly closed.
pub struct PreParser {
    pub tokens: Vec<Token>,
    pub cursor: usize,
    pub refined_tokens: Vec<Token>,
    pub bracket_stack: Vec<Token>, 
    pub logs: Vec<PreParserLog>,
}

#[derive(Debug, Clone)]
pub struct PreParserLog {
    pub message: String,
}

impl PreParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            cursor: 0,
            refined_tokens: Vec::new(),
            bracket_stack: Vec::new(),
            logs: Vec::new(),
        }
    }

    /// Run the whole PreParser process.
    pub fn process(&mut self) -> Vec<Token> {
        while self.cursor < self.tokens.len() {
            self.step();
        }
        self.refined_tokens.clone()
    }

    /// Process one token at a time and transform the stream.
    /// Returns true if it processed a token, false if at the end.
    pub fn step(&mut self) -> bool {
        if self.cursor >= self.tokens.len() { return false; }

        let token = self.tokens[self.cursor].clone();
        self.cursor += 1;

        match token.kind {
            TokenType::LeftParen | TokenType::LeftBracket | TokenType::LeftBrace | TokenType::OpenInterpolation => {
                // Tracking opening brackets
                self.bracket_stack.push(token.clone());
                self.refined_tokens.push(token);
            }
            TokenType::RightParen | TokenType::RightBracket | TokenType::RightBrace | TokenType::CloseInterpolation => {
                // Check if the closing bracket matches the last opening one
                if let Some(opening) = self.bracket_stack.pop() {
                    let matches = match (&opening.kind, &token.kind) {
                        (TokenType::LeftParen, TokenType::RightParen) => true,
                        (TokenType::LeftBracket, TokenType::RightBracket) => true,
                        (TokenType::LeftBrace, TokenType::RightBrace) => true,
                        (TokenType::OpenInterpolation, TokenType::CloseInterpolation) => true,
                        _ => false,
                    };

                    if !matches {
                        self.error(token.clone(), "Mismatched closing bracket.");
                    }
                } else {
                    self.error(token.clone(), "Unexpected closing bracket.");
                }
                self.refined_tokens.push(token);
            }
            TokenType::Newline => {
                // In Vex, a Newline usually means the end of a statement.
                // But we skip it if it's inside brackets or redundant.
                let last_is_boundary = self.refined_tokens.last().map_or(true, |t| 
                    matches!(t.kind, TokenType::StatementEnd | TokenType::Indent | TokenType::Dedent | TokenType::Newline)
                );
                
                if !self.bracket_stack.is_empty() {
                    self.log("Skipping newline inside brackets".to_string());
                } else if last_is_boundary {
                    self.log("Skipping redundant/empty line marker".to_string());
                } else {
                    self.log("Converting Newline to StatementEnd".to_string());
                    // Transform Newline into StatementEnd (logical ;) for the parser
                    let mut semi = token.clone();
                    semi.kind = TokenType::StatementEnd;
                    self.refined_tokens.push(semi);
                }
            }
            TokenType::StatementEnd => {
                // Keep explicit semicolons (StatementEnd), but skip repeated ones.
                let last_is_boundary = self.refined_tokens.last().map_or(true, |t| 
                    matches!(t.kind, TokenType::StatementEnd | TokenType::Indent | TokenType::Dedent)
                );
                if last_is_boundary {
                    self.log("Skipping redundant statement end".to_string());
                } else {
                    self.refined_tokens.push(token);
                }
            }
            TokenType::Indent => {
                // Measure of indentation (significant boundary in Vex)
                this_token_is_significant_boundary(self, token);
            }
            TokenType::Dedent => {
                // Decrease in indentation also usually signals the end of a statement.
                let last_not_semi = self.refined_tokens.last().map_or(false, |t| !matches!(t.kind, TokenType::StatementEnd));
                if last_not_semi && self.bracket_stack.is_empty() {
                     let mut semi = token.clone();
                     semi.kind = TokenType::StatementEnd;
                     self.refined_tokens.push(semi);
                }
                self.refined_tokens.push(token);
            }
            TokenType::Minus => {
                // If we see '-' followed by '>', it represents an arrow '->'.
                // To simplify parser logic, we unify '->' and '-' into just a single '-' token 
                // since they are both valid ways to define function return types.
                if self.cursor < self.tokens.len() && self.tokens[self.cursor].kind == TokenType::Greater {
                    self.cursor += 1; // Eat the '>'
                    self.log("Unified '->' into '-' for return type simplification".to_string());
                }
                self.refined_tokens.push(token);
            }
            TokenType::Eof => {
                // At the end of the file, check if any brackets were left open.
                for unclosed in &self.bracket_stack {
                    self.error(unclosed.clone(), "Unclosed bracket.");
                }

                // Ensure the last statement is properly terminated.
                let last_not_semi = self.refined_tokens.last().map_or(false, |t| !matches!(t.kind, TokenType::StatementEnd | TokenType::Dedent | TokenType::Indent));
                if last_not_semi && self.bracket_stack.is_empty() {
                    let mut semi = token.clone();
                    semi.kind = TokenType::StatementEnd;
                    self.refined_tokens.push(semi);
                }
                self.log("Finalizing token stream (EOF)".to_string());
                self.refined_tokens.push(token);
            }
            TokenType::Colon => {
                let mut is_unified = false;
                if self.cursor < self.tokens.len() && self.tokens[self.cursor].kind == TokenType::Colon {
                    self.cursor += 1;
                    self.log("Unified '::' into DoubleColon".to_string());
                    let mut unified_token = token.clone();
                    unified_token.kind = TokenType::DoubleColon;
                    self.refined_tokens.push(unified_token);
                    is_unified = true;
                }
                if !is_unified {
                    self.refined_tokens.push(token);
                }
            }
            TokenType::Dot => {
                let mut unified_kind = None;
                
                if self.cursor < self.tokens.len() {
                    match self.tokens[self.cursor].kind {
                        TokenType::Slash => {
                            self.cursor += 1;
                            self.log("Unified './' into DynamicDot".to_string());
                            unified_kind = Some(TokenType::DynamicDot);
                        }
                        TokenType::Question => {
                            self.cursor += 1;
                            if self.cursor < self.tokens.len() && self.tokens[self.cursor].kind == TokenType::Slash {
                                self.cursor += 1;
                                self.log("Unified '.?/' into SafeDynamicDot".to_string());
                                unified_kind = Some(TokenType::SafeDynamicDot);
                            } else {
                                self.log("Unified '.?' into SafeDot".to_string());
                                unified_kind = Some(TokenType::SafeDot);
                            }
                        }
                        _ => {}
                    }
                }
                
                if let Some(kind) = unified_kind {
                    let mut unified_token = token.clone();
                    unified_token.kind = kind;
                    self.refined_tokens.push(unified_token);
                } else {
                    self.refined_tokens.push(token);
                }
            }
            _ => {
                // Keep all other tokens as they are (Identifiers, Numbers, etc.)
                self.refined_tokens.push(token);
            }
        }
        true
    }

    fn error(&self, token: Token, msg: &str) {
        diag_emit!(Error, P005, msg.to_string(), token.span);
    }

    fn log(&mut self, msg: String) {
        self.logs.push(PreParserLog { message: msg });
    }

    pub fn current_tokens(&self) -> &Vec<Token> {
        &self.refined_tokens
    }
}

fn this_token_is_significant_boundary(pre: &mut PreParser, token: Token) {
    pre.refined_tokens.push(token);
}
