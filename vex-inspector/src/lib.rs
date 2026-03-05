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
                        
                        // Skip Phase: Tab, Shift+Enter, or S
                        KeyCode::Tab | KeyCode::Char('S') | KeyCode::Char('s') => app.skip_phase(),
                        KeyCode::Enter if key.modifiers.contains(event::KeyModifiers::SHIFT) => app.skip_phase(),

                        // Export Report: E
                        KeyCode::Char('E') | KeyCode::Char('e') => {
                            let _ = report::export_report(&app);
                        }

                        // Navigation: Up/Down
                        KeyCode::Up => {
                            if let Some(idx) = app.selected_token_idx {
                                if idx + 1 < app.current_tokens().len() {
                                    app.select_token(idx + 1);
                                }
                            }
                        }
                        KeyCode::Down => {
                            if let Some(idx) = app.selected_token_idx {
                                if idx > 0 {
                                    app.select_token(idx - 1);
                                }
                            }
                        }
                        _ => {}
                    },
                    Event::Mouse(mouse) => match mouse.kind {
                        event::MouseEventKind::ScrollUp => {
                            if let Some(idx) = app.selected_token_idx {
                                if idx + 1 < app.current_tokens().len() {
                                    app.select_token(idx + 1);
                                }
                            }
                        }
                        event::MouseEventKind::ScrollDown => {
                            if let Some(idx) = app.selected_token_idx {
                                if idx > 0 {
                                    app.select_token(idx - 1);
                                }
                            }
                        }
                        event::MouseEventKind::Down(event::MouseButton::Left) => {
                            app.handle_click(mouse.column, mouse.row);
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
}
