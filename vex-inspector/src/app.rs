use ratatui::widgets::ListState;
use ratatui::layout::Rect;
use vex_core::token::{Token, TokenType};
use vex_lexer::Lexer;
use vex_diagnostic::diagnostic::Diagnostic;
use vex_parser::preparser::PreParser;
use vex_parser::Parser;
use vex_parser::declarations::Declaration;
use crate::InspectorPhase;

/// InspectorApp is the main state for the TUI debugger.
pub struct InspectorApp {
    pub phase: InspectorPhase,
    pub file_id: usize,
    pub filename: String,
    
    // Core compiler components stored here to be stepped manually
    pub lexer: Lexer,
    pub lexer_tokens: Vec<Token>,   // Tokens found in Lexer phase
    pub preparser: Option<PreParser>,
    pub refined_tokens: Vec<Token>, // Tokens after PreParser cleanup
    pub parser: Option<Parser>,
    pub ast: Vec<Declaration>,      // Abstract Syntax Tree nodes
    pub source: String,             // Original source code text
    
    pub selected_token_idx: Option<usize>,
    pub list_state: ListState,
    pub ast_list_state: ListState,
    pub focused_pane: usize,        // 0: Tokens, 1: AST
    pub ast_token_ranges: Vec<(usize, usize)>, // Maps AST index to its (start_token, end_token) range
    pub diagnostics: Vec<Diagnostic>, // Errors and warnings found so far
    
    pub should_quit: bool,
    pub is_finished: bool, // Set to true if an error stops the process

    // UI state for scrolling and rendering
    pub code_scroll: u16,
    pub last_code_rect: Rect,
}

impl InspectorApp {
    pub fn new(file_id: usize, source: String, filename: String) -> Self {
        use vex_diagnostic::diag;

        // Reset global diagnostics for a fresh run
        {
            let mut d = diag!(write);
            d.reset();
        }

        let lexer = Lexer::new(file_id, source.clone());
        
        Self {
            phase: InspectorPhase::Lexer,
            file_id,
            filename,
            lexer,
            lexer_tokens: Vec::new(),
            preparser: None,
            refined_tokens: Vec::new(),
            parser: None,
            ast: Vec::new(),
            source,
            selected_token_idx: None,
            list_state: ListState::default(),
            ast_list_state: ListState::default(),
            focused_pane:0,
            ast_token_ranges: Vec::new(),
            diagnostics: Vec::new(),
            should_quit: false,
            is_finished: false,
            code_scroll: 0,
            last_code_rect: Rect::default(),
        }
    }

    pub fn current_tokens(&self) -> &Vec<Token> {
        if self.phase == InspectorPhase::Lexer {
            &self.lexer_tokens
        } else {
            &self.refined_tokens
        }
    }

    pub fn current_ast(&self) -> &Vec<Declaration> {
        &self.ast
    }

    /// Move the compiler forward by exactly one small step.
    pub fn step(&mut self) {
        if self.is_finished { return; }
        use vex_diagnostic::diag;

        match self.phase {
            InspectorPhase::Lexer => {
                let reached_eof = self.lexer_tokens.last().map_or(false, |t| matches!(t.kind, TokenType::Eof));
                
                if reached_eof {
                    self.phase = InspectorPhase::PreParser;
                } else {
                    let token = self.lexer.next_token();
                    self.lexer_tokens.push(token);
                    let idx = self.lexer_tokens.len() - 1;
                    self.select_token(idx);
                }
            }
            InspectorPhase::PreParser => {
                if let Some(pp) = &mut self.preparser {
                    if pp.step() {
                        self.refined_tokens = pp.refined_tokens.clone();
                        let idx = self.refined_tokens.len().saturating_sub(1);
                        self.select_token(idx);
                    } else if pp.cursor >= pp.tokens.len() && pp.bracket_stack.is_empty() {
                        // Phase is finished, move to Parser on NEXT step
                        self.phase = InspectorPhase::Parser;
                        self.selected_token_idx = None;
                        self.list_state.select(None);
                    }
                } else {
                    self.preparser = Some(PreParser::new(self.lexer_tokens.clone()));
                    self.selected_token_idx = None;
                    self.list_state.select(None);
                }
            }
            InspectorPhase::Parser => {
                if let Some(parser) = &mut self.parser {
                    let start_token_idx = parser.idx;
                    if let Some(decl) = parser.next_declaration() {
                        self.ast.push(decl);
                        self.ast_token_ranges.push((start_token_idx, parser.idx));
                        let idx = self.ast.len() - 1;
                        self.select_ast(idx);
                        self.focused_pane = 1; // Focus AST when a new node is added
                    } else if parser.is_at_end() {
                        // EOF reached
                        self.is_finished = true;
                    }
                } else {
                    self.parser = Some(Parser::new(self.refined_tokens.clone()));
                    self.selected_token_idx = None;
                    self.focused_pane = 0;
                    return;
                }
            }
            _ => {}
        }

        // Sync diagnostics
        {
            let diag_handler = diag!();
            self.diagnostics = diag_handler.get_diagnostics().to_vec();
        }

        // (Diagnostic sync kept)
        if self.diagnostics.iter().any(|d| matches!(d.level, vex_diagnostic::diagnostic::DiagnosticLevel::Error)) {
            // We don't set is_finished = true anymore, just let the UI show FAILED
            // This allows the user to still navigate and see what happened.
            if let Some(first_err) = self.diagnostics.iter().find(|d| matches!(d.level, vex_diagnostic::diagnostic::DiagnosticLevel::Error)) {
                let line = first_err.span.line as u16;
                if line > 5 {
                    self.code_scroll = line - 5;
                } else {
                    self.code_scroll = 0;
                }
            }
        }
    }

    pub fn select_ast(&mut self, idx: usize) {
        if idx < self.ast.len() {
            self.ast_list_state.select(Some(idx));
            
            // Sync with tokens and code (using ONLY the first token for scrolling)
            if let Some(&(start, _)) = self.ast_token_ranges.get(idx) {
                self.select_token(start);
            }
        }
    }

    pub fn get_selected_ast_idx(&self) -> Option<usize> {
        self.ast_list_state.selected()
    }

    pub fn select_token(&mut self, idx: usize) {
        let tokens_len = self.current_tokens().len();
        if idx < tokens_len {
            self.selected_token_idx = Some(idx);
            self.list_state.select(Some(idx));

            let tokens = self.current_tokens();
            if let Some(t) = tokens.get(idx) {
                let line = t.span.line as u16;
                if line > 5 {
                    self.code_scroll = line - 5;
                } else {
                    self.code_scroll = 0;
                }
            }
        }
    }

    pub fn skip_phase(&mut self) {
        let current_phase = self.phase;
        while self.phase == current_phase && !self.is_finished {
            self.step();
        }
    }

    pub fn handle_click(&mut self, x: u16, y: u16) {
        let rect = self.last_code_rect;
        if x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height {
            let rel_y = (y - rect.y).saturating_sub(1) + self.code_scroll; 
            let rel_x = (x - rect.x).saturating_sub(6); 

            let lines: Vec<&str> = self.source.split_inclusive('\n').collect();
            if let Some(_line_content) = lines.get(rel_y as usize) {
                let mut offset = 0;
                for i in 0..rel_y as usize {
                    offset += lines[i].len();
                }
                
                let target_offset = offset + rel_x as usize;

                let tokens = self.current_tokens();
                for (idx, token) in tokens.iter().enumerate() {
                    if target_offset >= token.span.start && target_offset < token.span.end {
                        self.select_token(idx);
                        break;
                    }
                }
            }
        }
    }
}
