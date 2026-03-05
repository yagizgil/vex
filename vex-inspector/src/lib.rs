use std::io::{self, stdout};
use ratatui::prelude::*;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use vex_core::smap;

pub mod app;
pub mod ui;
pub mod report;
pub mod gui;

pub use app::InspectorApp;

/// InspectorPhase tracks which part of the compiler we are currently inspecting.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InspectorPhase {
    Lexer,       // Converting text to tokens
    PreParser,   // Fixing tokens (newlines, brackets)
    Parser,      // Building the Syntax Tree (AST)
    Interpreter, // Running the code
}

impl InspectorApp {
    /// Start the TUI inspector application.
    /// This is the main entry point for the vex-inspector crate.
    pub fn run(file_id: usize) -> io::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        stdout().execute(event::EnableMouseCapture)?;
        
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        // Prepare data for initialization
        let (source, filename) = {
            let map = smap!();
            let file = map.get_file(file_id).expect("Inspector: File not found in SourceMap");
            (file.content.clone(), file.path.clone())
        };

        let mut app = InspectorApp::new(file_id, source, filename);

        // Main Event Loop
        while !app.should_quit {
            // Draw UI
            terminal.draw(|f| ui::render(f, &mut app))?;

            // Handle Events
            if event::poll(std::time::Duration::from_millis(16))? {
                match event::read()? {
                    Event::Key(key) => match key.code {
                        KeyCode::Char('q') => app.should_quit = true,
                        KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => app.should_quit = true,
                        
                        // Step: Space, N, or Enter
                        KeyCode::Char(' ') | KeyCode::Char('n') | KeyCode::Enter 
                            if !key.modifiers.contains(event::KeyModifiers::SHIFT) => app.step(),
                        
                        // Cycle Panels: Tab
                        KeyCode::Tab => {
                             app.focused_pane = (app.focused_pane + 1) % 4;
                        }
                        KeyCode::Char('S') | KeyCode::Char('s') => app.skip_phase(),
                        KeyCode::Enter if key.modifiers.contains(event::KeyModifiers::SHIFT) => app.skip_phase(),

                        // Export and Reload
                        KeyCode::Char('E') | KeyCode::Char('e') => {
                            let _ = report::export_all(&app);
                        }
                        KeyCode::Char('R') | KeyCode::Char('r') => {
                            let _ = app.reload();
                        }
                        KeyCode::Char('T') | KeyCode::Char('t') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                            let _ = report::export_tokens_csv(&app);
                        }
                        KeyCode::Char('A') | KeyCode::Char('a') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                            let _ = report::export_ast_json(&app);
                        }

                        // Navigation: Up/Down
                        KeyCode::Up => {
                            if app.focused_pane == 1 && !app.ast.is_empty() {
                                let idx = app.get_selected_ast_idx().unwrap_or(0);
                                if idx > 0 {
                                    app.select_ast(idx - 1);
                                }
                            } else if app.focused_pane == 2 {
                                app.detail_scroll = app.detail_scroll.saturating_sub(1);
                            } else {
                                let idx = app.selected_token_idx.unwrap_or(0);
                                if idx > 0 {
                                    app.select_token(idx - 1);
                                }
                            }
                        }
                        KeyCode::Down => {
                            if app.focused_pane == 1 && !app.ast.is_empty() {
                                let idx = app.get_selected_ast_idx().unwrap_or(0);
                                if idx + 1 < app.ast.len() {
                                    app.select_ast(idx + 1);
                                }
                            } else if app.focused_pane == 2 {
                                app.detail_scroll = app.detail_scroll.saturating_add(1);
                            } else {
                                let idx = app.selected_token_idx.unwrap_or(0);
                                if idx + 1 < app.current_tokens().len() {
                                    app.select_token(idx + 1);
                                }
                            }
                        }
                        KeyCode::PageUp => {
                            match app.focused_pane {
                                1 => {
                                    let idx = app.get_selected_ast_idx().unwrap_or(0);
                                    app.select_ast(idx.saturating_sub(10));
                                }
                                2 => { app.detail_scroll = app.detail_scroll.saturating_sub(10); }
                                3 => { app.code_scroll = app.code_scroll.saturating_sub(10); }
                                _ => {
                                    let idx = app.selected_token_idx.unwrap_or(0);
                                    app.select_token(idx.saturating_sub(10));
                                }
                            }
                        }
                        KeyCode::PageDown => {
                            match app.focused_pane {
                                1 => {
                                    let idx = app.get_selected_ast_idx().unwrap_or(0);
                                    let max = app.ast.len().saturating_sub(1);
                                    app.select_ast((idx + 10).min(max));
                                }
                                2 => { app.detail_scroll = app.detail_scroll.saturating_add(10); }
                                3 => { app.code_scroll = app.code_scroll.saturating_add(10); }
                                _ => {
                                    let idx = app.selected_token_idx.unwrap_or(0);
                                    let max = app.current_tokens().len().saturating_sub(1);
                                    app.select_token((idx + 10).min(max));
                                }
                            }
                        }
                        _ => {}
                    },
                    Event::Mouse(mouse) => match mouse.kind {
                        event::MouseEventKind::ScrollUp => {
                            match app.focused_pane {
                                1 => {
                                    let idx = app.get_selected_ast_idx().unwrap_or(0);
                                    if idx > 0 { app.select_ast(idx - 1); }
                                }
                                2 => { app.detail_scroll = app.detail_scroll.saturating_sub(3); }
                                3 => { app.code_scroll = app.code_scroll.saturating_sub(3); }
                                _ => {
                                    let idx = app.selected_token_idx.unwrap_or(0);
                                    if idx > 0 { app.select_token(idx - 1); }
                                }
                            }
                        }
                        event::MouseEventKind::ScrollDown => {
                            match app.focused_pane {
                                1 => {
                                    let idx = app.get_selected_ast_idx().unwrap_or(0);
                                    if idx + 1 < app.ast.len() { app.select_ast(idx + 1); }
                                }
                                2 => { app.detail_scroll = app.detail_scroll.saturating_add(3); }
                                3 => { app.code_scroll = app.code_scroll.saturating_add(3); }
                                _ => {
                                    let idx = app.selected_token_idx.unwrap_or(0);
                                    if idx + 1 < app.current_tokens().len() { app.select_token(idx + 1); }
                                }
                            }
                        }
                        event::MouseEventKind::Down(event::MouseButton::Left) => {
                            // Divider drag detection
                            let w = terminal.size()?.width;
                            let x = mouse.column;
                            
                            let mut current_pct = 0;
                            for i in 0..3 {
                                current_pct += app.panel_widths[i];
                                let div_x = (w as u32 * current_pct as u32 / 100) as u16;
                                if x.saturating_sub(1) <= div_x && x + 1 >= div_x {
                                    app.dragging_panel = Some(i);
                                    break;
                                }
                            }

                            if app.dragging_panel.is_none() {
                                app.handle_click(mouse.column, mouse.row);
                            }
                        }
                        event::MouseEventKind::Drag(event::MouseButton::Left) => {
                            if let Some(panel_idx) = app.dragging_panel {
                                let w = terminal.size()?.width;
                                let x = mouse.column;
                                let new_pct = (x as u32 * 100 / w as u32) as u16;
                                
                                // Calculate sum of previous panels
                                let mut prev_sum = 0;
                                for i in 0..panel_idx {
                                    prev_sum += app.panel_widths[i];
                                }
                                
                                if new_pct > prev_sum + 5 && new_pct < 95 {
                                    let diff = new_pct.saturating_sub(prev_sum);
                                    let old_width = app.panel_widths[panel_idx];
                                    let next_width = app.panel_widths[panel_idx + 1];
                                    
                                    if diff < old_width + next_width - 5 {
                                        let change = diff as i16 - old_width as i16;
                                        app.panel_widths[panel_idx] = diff;
                                        app.panel_widths[panel_idx + 1] = (next_width as i16 - change) as u16;
                                    }
                                }
                            }
                        }
                        event::MouseEventKind::Up(event::MouseButton::Left) => {
                            app.dragging_panel = None;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        // Cleanup Terminal
        stdout().execute(event::DisableMouseCapture)?;
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        
        Ok(())
    }

    /// Start the GUI inspector application.
    pub fn run_gui(file_id: usize) -> Result<(), String> {
        gui::GuiInspector::run(file_id)
    }
}
