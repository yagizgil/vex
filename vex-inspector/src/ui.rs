use ratatui::{prelude::*, widgets::*};
use crate::app::InspectorApp;
use crate::InspectorPhase;

pub fn render(f: &mut Frame, app: &mut InspectorApp) {
    let v_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // Content
            Constraint::Length(7), // Internal State
            Constraint::Length(1), // Help
        ])
        .split(f.area());

    // --- Title ---
    let phase_color = match app.phase {
        InspectorPhase::Lexer => Color::Cyan,
        InspectorPhase::PreParser => Color::Yellow,
        InspectorPhase::Parser => Color::Magenta,
        InspectorPhase::Interpreter => Color::Green,
    };

    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(phase_color))
        .title(" VEX - COMPILER INSPECTOR ")
        .title_alignment(Alignment::Center);

    let has_errors = app.diagnostics.iter().any(|d| matches!(d.level, vex_diagnostic::diagnostic::DiagnosticLevel::Error));
    let status_text = if app.is_finished {
        if has_errors {
            Span::styled(" [ FAILED ] ", Style::default().fg(Color::Red).bold())
        } else {
            Span::styled(" [ FINISHED ] ", Style::default().fg(Color::Green).bold())
        }
    } else {
        Span::raw("")
    };

    let title_content = Paragraph::new(Line::from(vec![
        Span::styled(format!(" [ {:?} MODE ] ", app.phase).to_uppercase(), Style::default().bg(phase_color).fg(Color::Black).bold()),
        Span::raw(format!(" | File: {} ", app.filename)),
        status_text,
    ])).block(title_block).alignment(Alignment::Center);
    f.render_widget(title_content, v_layout[0]);

    // --- Content Split (Tokens | AST | Code) ---
    let h_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // Tokens
            Constraint::Percentage(30), // AST Nodes
            Constraint::Percentage(50), // Code
        ])
        .split(v_layout[1]);

    let highlight_colors = [
        Color::Rgb(60, 40, 40),
        Color::Rgb(40, 60, 40),
        Color::Rgb(40, 40, 60),
        Color::Rgb(60, 60, 40),
        Color::Rgb(40, 60, 60),
        Color::Rgb(60, 40, 60),
    ];

    let source_colors = [
        Color::Rgb(80, 50, 50),
        Color::Rgb(50, 80, 50),
        Color::Rgb(50, 50, 80),
        Color::Rgb(80, 80, 50),
        Color::Rgb(50, 80, 80),
        Color::Rgb(80, 50, 80),
    ];

    // PRE-CALCULATE EVERYTHING to free up 'app' borrow
    let (token_items, ast_items, current_token, selected_token_range) = {
        let tokens = app.current_tokens();
        let ast = app.current_ast();
        
        let current_token = app.selected_token_idx.and_then(|idx| tokens.get(idx).cloned());
        let selected_ast_idx = app.get_selected_ast_idx();
        let selected_token_range = selected_ast_idx.and_then(|idx| app.ast_token_ranges.get(idx)).cloned();

        let t_items: Vec<ListItem> = tokens.iter().enumerate().map(|(idx, t)| {
            let is_selected = Some(idx) == app.selected_token_idx;
            let (is_in_range, range_color) = selected_token_range.map_or((false, Color::Reset), |(s, e)| {
                let in_range = idx >= s && idx < e;
                let color = if in_range { highlight_colors[idx % highlight_colors.len()] } else { Color::Reset };
                (in_range, color)
            });
            
            let mut style = Style::default().fg(Color::Gray);
            if is_selected {
                style = style.bg(Color::Rgb(30, 30, 60)).fg(Color::Yellow).bold();
            } else if is_in_range {
                style = style.bg(range_color).fg(Color::White);
            }
            
            let lexeme = t.lexeme().replace("\n", "\\n");
            let token_lexeme = format!("'{}'", if lexeme.len() > 10 { format!("{}...", &lexeme[..7]) } else { lexeme });

            ListItem::new(Line::from(vec![
                Span::styled(format!("{:2} ", idx), Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{:<12} ", format!("{:?}", t.kind)), Style::default().fg(if is_selected { Color::Yellow } else if is_in_range { Color::Cyan } else { Color::DarkGray })),
                Span::styled(token_lexeme, Style::default().fg(Color::White)),
            ])).style(style)
        }).collect();

        let a_items: Vec<ListItem> = ast.iter().enumerate().map(|(idx, decl)| {
            let pretty_ast = format!("{:#?}", decl);
            let token_range = app.ast_token_ranges.get(idx).cloned().unwrap_or((0, 0));

            let mut lines = vec![
                Line::from(vec![
                    Span::styled(format!("#{} DECL ", idx), Style::default().fg(Color::Magenta).bold()),
                ])
            ];

            for line in pretty_ast.lines() {
                let mut spans = vec![Span::raw("  ")];
                // Very basic split to isolate potential lexemes for coloring
                let parts = line.split_inclusive(|c: char| !c.is_alphanumeric() && c != '"' && c != '\'');
                
                for part in parts {
                    let mut style = Style::default();
                    let trimmed = part.trim_matches(|c: char| !c.is_alphanumeric());
                    
                    if !trimmed.is_empty() {
                         for t_idx in token_range.0..token_range.1 {
                            if let Some(t) = tokens.get(t_idx) {
                                if t.lexeme() == trimmed || t.lexeme().trim_matches('"') == trimmed {
                                    style = style.bg(highlight_colors[t_idx % highlight_colors.len()]).fg(Color::White).bold();
                                    break;
                                }
                            }
                        }
                    }

                    if style.bg.is_none() {
                        if part.contains(':') { style = style.fg(Color::Yellow); }
                        else if part.contains('"') || part.contains('\'') { style = style.fg(Color::Cyan); }
                        else if part.chars().any(|c| c.is_numeric()) { style = style.fg(Color::Green); }
                    }

                    spans.push(Span::styled(part.to_string(), style));
                }
                lines.push(Line::from(spans));
            }

            ListItem::new(lines).style(Style::default().fg(Color::White))
        }).collect();

        (t_items, a_items, current_token, selected_token_range)
    };

    let tokens_list = List::new(token_items)
        .block(Block::default()
            .title(format!(" Tokens{} ", if app.focused_pane == 0 { " (Focused)" } else { "" }))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(if app.focused_pane == 0 { phase_color } else { Color::DarkGray })))
        .highlight_symbol(">> ");
    
    f.render_stateful_widget(tokens_list, h_layout[0], &mut app.list_state);

    let ast_list = List::new(ast_items)
        .block(Block::default()
            .title(format!(" AST Nodes (Pretty){} ", if app.focused_pane == 1 { " (Focused)" } else { "" }))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(if app.focused_pane == 1 { phase_color } else { Color::DarkGray })))
        .highlight_symbol(">> ");
    f.render_stateful_widget(ast_list, h_layout[1], &mut app.ast_list_state);

    // 3. Source Code View
    let mut code_lines = Vec::new();
    let current_token = current_token.as_ref();
    let cursor_pos = app.lexer.cursor.pos;
    let mut global_offset = 0;

    let mut range_spans = Vec::new();
    if let Some((start, end)) = selected_token_range {
        let tokens = app.current_tokens();
        for t_idx in start..end {
            if let Some(t) = tokens.get(t_idx) {
                range_spans.push((t.span.start, t.span.end, t_idx));
            }
        }
    }

    for (i, line_with_nl) in app.source.split_inclusive('\n').enumerate() {
        let line_num = i + 1;
        let segment_len = line_with_nl.len();
        let display_content = line_with_nl.trim_end_matches(['\n', '\r']);
        let display_len = display_content.len();

        let mut spans = vec![
            Span::styled(format!("{:3} | ", line_num), Style::default().fg(Color::DarkGray)),
        ];

        let mut char_idx = 0;
        let chars_vec: Vec<char> = display_content.chars().collect();
        while char_idx < display_len {
            let current_byte = global_offset + char_idx;
            let ch = chars_vec[char_idx];
            let mut style = Style::default();

            if let Some(token) = current_token {
                if current_byte >= token.span.start && current_byte < token.span.end {
                    style = style.bg(Color::Rgb(200, 160, 0)).fg(Color::Black).bold();
                }
            }
            
            // Range highlight for AST tokens
            for (s, e, t_idx) in &range_spans {
                if current_byte >= *s && current_byte < *e {
                    if current_token.map_or(true, |ct| ct.span.start != *s) {
                        style = style.bg(source_colors[*t_idx % source_colors.len()]).fg(Color::White);
                    }
                    break;
                }
            }
            
            for diag in &app.diagnostics {
                if current_byte >= diag.span.start && current_byte < diag.span.end {
                    let color = if matches!(diag.level, vex_diagnostic::diagnostic::DiagnosticLevel::Error) { Color::Red } else { Color::Yellow };
                    style = style.underline_color(color).underlined();
                    if style.bg.is_none() && matches!(diag.level, vex_diagnostic::diagnostic::DiagnosticLevel::Error) {
                        style = style.bg(Color::Rgb(80, 0, 0));
                    }
                }
            }

            if current_byte == cursor_pos {
                style = style.bg(Color::Green).fg(Color::Black).bold();
            }
            
            // If in Parser phase, highlight the token currently being looked at by the parser
            if let Some(parser) = &app.parser {
                let p_token = parser.peek();
                if current_byte >= p_token.span.start && current_byte < p_token.span.end {
                    style = style.bg(Color::Magenta).fg(Color::White).bold();
                }
            }

            spans.push(Span::styled(ch.to_string(), style));
            char_idx += 1;
        }

        // EOL Marker
        let eol_byte = global_offset + display_len;
        let mut eol_style = Style::default();
        if let Some(token) = current_token {
            if token.span.start >= eol_byte && token.span.start < (global_offset + segment_len) {
                eol_style = eol_style.bg(Color::Rgb(200, 160, 0)).fg(Color::Black).bold();
            }
        }
        
        for diag in &app.diagnostics {
            if diag.span.start >= eol_byte && diag.span.start < (global_offset + segment_len) {
                eol_style = eol_style.bg(Color::Rgb(80, 0, 0)).underlined();
            }
        }

        if cursor_pos == eol_byte {
            eol_style = eol_style.bg(Color::Green).fg(Color::Black).bold();
        }
        
        if eol_style != Style::default() {
            spans.push(Span::styled(" ", eol_style));
        }

        code_lines.push(Line::from(spans));
        global_offset += segment_len;
    }

    app.last_code_rect = h_layout[2];
    let code_view = Paragraph::new(code_lines)
        .block(Block::default().title(" Source Code ").borders(Borders::ALL).border_style(Style::default().fg(phase_color)))
        .scroll((app.code_scroll, 0));
    f.render_widget(code_view, h_layout[2]);

    // --- Bottom Layout (State | Diagnostics) ---
    let bottom_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(60),
        ])
        .split(v_layout[2]);

    // 1. Internal State
    let state_block = Block::default()
        .borders(Borders::ALL)
        .title(" INTERNAL STATE ")
        .border_style(Style::default().fg(Color::Green));
    
    let state_text = match app.phase {
        InspectorPhase::Lexer => {
            let stack_info = format!("{:?}", app.lexer.interpolation_stack);
            vec![
                Line::from(vec![
                    Span::styled(" CURSOR -> ", Style::default().fg(Color::Green).bold()),
                    Span::raw(format!("Pos: {}  Line: {}  Col: {}", app.lexer.cursor.pos, app.lexer.cursor.line, app.lexer.cursor.col)),
                ]),
                Line::from(vec![
                    Span::styled(" STACK  -> ", Style::default().fg(Color::Green).bold()),
                    Span::raw(stack_info),
                ]),
                Line::from(vec![
                    Span::styled(" PEEK   -> ", Style::default().fg(Color::Green).bold()),
                    Span::raw(format!("'{}'", app.lexer.cursor.peek().to_string().replace("\0", "EOF").replace("\n", "\\n"))),
                ]),
            ]
        }
        InspectorPhase::PreParser => {
             let mut lines = vec![
                Line::from(vec![
                    Span::styled(" STATUS -> ", Style::default().fg(Color::Yellow).bold()),
                    Span::raw("Refining token stream..."),
                ]),
                Line::from(vec![
                    Span::styled(" TOKENS -> ", Style::default().fg(Color::Yellow).bold()),
                    Span::raw(format!("In: {} | Out: {}", app.lexer_tokens.len(), app.refined_tokens.len())),
                ]),
            ];

            if let Some(pp) = &app.preparser {
                lines.push(Line::from(vec![
                    Span::styled(" BRK STS-> ", Style::default().fg(Color::Yellow).bold()),
                    Span::raw(format!("Stack Depth: {}", pp.bracket_stack.len())),
                ]));
                for log in pp.logs.iter().rev().take(3).rev() {
                    lines.push(Line::from(vec![
                        Span::styled(" LOG    -> ", Style::default().fg(Color::DarkGray)),
                        Span::raw(&log.message),
                    ]));
                }
            }
            lines
        }
        InspectorPhase::Parser => {
             let mut lines = vec![
                Line::from(vec![
                    Span::styled(" STATUS -> ", Style::default().fg(Color::Magenta).bold()),
                    Span::raw("Parsing declarations..."),
                ]),
            ];

            if let Some(parser) = &app.parser {
                lines.push(Line::from(vec![
                    Span::styled(" CURSOR -> ", Style::default().fg(Color::Magenta).bold()),
                    Span::raw(format!("Token Index: {}/{}", parser.idx, parser.tokens.len())),
                ]));
                lines.push(Line::from(vec![
                    Span::styled(" AST    -> ", Style::default().fg(Color::Magenta).bold()),
                    Span::raw(format!("Nodes Created: {}", app.ast.len())),
                ]));
                
                let next_t = parser.peek();
                lines.push(Line::from(vec![
                    Span::styled(" PEEK   -> ", Style::default().fg(Color::Magenta).bold()),
                    Span::styled(format!("{:?} ", next_t.kind), Style::default().fg(Color::Cyan)),
                    Span::raw(format!("'{}'", next_t.lexeme().replace("\n", "\\n"))),
                ]));
            }
            lines
        }
        _ => vec![Line::from(vec![
            Span::styled(" STATUS -> ", Style::default().fg(phase_color).bold()),
            Span::raw("Processing..."),
        ])],
    };
    f.render_widget(Paragraph::new(state_text).block(state_block), bottom_layout[0]);

    // 2. Diagnostics
    let diag_block = Block::default()
        .borders(Borders::ALL)
        .title(" DIAGNOSTICS ")
        .border_style(Style::default().fg(if has_errors { Color::Red } else { Color::Yellow }));
    
    let diag_items: Vec<Line> = app.diagnostics.iter().map(|d| {
        let (level_str, color) = match d.level {
            vex_diagnostic::diagnostic::DiagnosticLevel::Error => ("ERR", Color::Red),
            vex_diagnostic::diagnostic::DiagnosticLevel::Warning => ("WRN", Color::Yellow),
            vex_diagnostic::diagnostic::DiagnosticLevel::Note => ("NOT", Color::Blue),
            vex_diagnostic::diagnostic::DiagnosticLevel::Hint => ("HNT", Color::Cyan),
        };
        Line::from(vec![
            Span::styled(format!("[{}] ", level_str), Style::default().fg(color).bold()),
            Span::styled(format!("{}: ", d.code.as_str()), Style::default().fg(color)),
            Span::raw(&d.message),
            Span::styled(format!(" (L:{}, C:{})", d.span.line, d.span.col), Style::default().fg(Color::DarkGray)),
        ])
    }).collect();

    f.render_widget(Paragraph::new(diag_items).block(diag_block).wrap(Wrap { trim: true }), bottom_layout[1]);

    // Help
    let help_para = Paragraph::new(Line::from(vec![
        Span::styled(" Q:", Style::default().bold()), Span::raw(" Quit |"),
        Span::styled(" SPACE/ENT:", Style::default().bold()), Span::raw(" Step |"),
        Span::styled(" TAB/S:", Style::default().bold()), Span::raw(" Skip | "),
        Span::styled(" E:", Style::default().bold()), Span::raw(" Export | "),
        Span::styled(" UP/DOWN:", Style::default().bold()), Span::raw(" Nav"),
    ])).alignment(Alignment::Center);
    f.render_widget(help_para, v_layout[3]);
}
