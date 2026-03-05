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

    // --- Content Split (Tokens | Code) ---
    let h_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Percentage(65),
        ])
        .split(v_layout[1]);

    // 1. History List (Tokens)
    let tokens = app.current_tokens();
    let token_items: Vec<ListItem> = tokens.iter().enumerate().rev().map(|(idx, t)| {
        let is_selected = Some(idx) == app.selected_token_idx;
        let style = if is_selected {
            Style::default().bg(Color::Rgb(50, 50, 80)).fg(Color::Yellow).bold()
        } else {
            Style::default().fg(Color::Gray)
        };
        
        let lexeme = t.lexeme().replace("\n", "\\n");
        let token_lexeme = format!("'{}'", if lexeme.len() > 15 { format!("{}...", &lexeme[..12]) } else { lexeme });

        ListItem::new(Line::from(vec![
            Span::styled(format!("{:2} | ", idx), Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{:<15} ", format!("{:?}", t.kind)), Style::default().fg(if is_selected { Color::Yellow } else { Color::Cyan })),
            Span::styled(token_lexeme, Style::default().fg(Color::White)),
        ])).style(style)
    }).collect();

    let tokens_list = List::new(token_items)
        .block(Block::default().title(format!(" {} History ", match app.phase {
            InspectorPhase::Lexer => "Token",
            InspectorPhase::PreParser => "Refined Token",
            _ => "AST",
        })).borders(Borders::ALL).border_style(Style::default().fg(phase_color)))
        .highlight_symbol(">> ");
    
    let current_token_data = app.selected_token_idx.and_then(|idx| tokens.get(idx).cloned());
    f.render_stateful_widget(tokens_list, h_layout[0], &mut app.list_state);

    // 2. Source Code View
    let mut code_lines = Vec::new();
    let current_token = current_token_data.as_ref();
    let cursor_pos = app.lexer.cursor.pos;
    let mut global_offset = 0;

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

    app.last_code_rect = h_layout[1];
    let code_view = Paragraph::new(code_lines)
        .block(Block::default().title(" Source Code ").borders(Borders::ALL).border_style(Style::default().fg(phase_color)))
        .scroll((app.code_scroll, 0));
    f.render_widget(code_view, h_layout[1]);

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
