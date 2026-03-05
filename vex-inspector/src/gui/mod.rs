use crate::InspectorApp;
use eframe::egui;
use vex_core::smap;
use vex_core::token::TokenType;
use crate::InspectorPhase;
use vex_core::trace::TraceNode;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ViewMode {
    Lexer,
    PreParser,
    Parser,
    Trace,
    Full,
}

pub struct GuiInspector {
    app: InspectorApp,
    highlight_colors: Vec<egui::Color32>,
    scroll_to_selection: bool,
    view_mode: ViewMode,
}

impl GuiInspector {
    pub fn new(file_id: usize) -> Self {
        let (source, filename) = {
            let map = smap!();
            let file = map.get_file(file_id).expect("Inspector: File not found in SourceMap");
            (file.content.clone(), file.path.clone())
        };

        let highlight_colors = vec![
            egui::Color32::from_rgb(80, 40, 40),
            egui::Color32::from_rgb(40, 80, 40),
            egui::Color32::from_rgb(40, 40, 80),
            egui::Color32::from_rgb(80, 80, 40),
            egui::Color32::from_rgb(40, 80, 80),
            egui::Color32::from_rgb(80, 40, 80),
        ];

        Self {
            app: InspectorApp::new(file_id, source, filename),
            highlight_colors,
            scroll_to_selection: false,
            view_mode: ViewMode::Lexer,
        }
    }

    pub fn run(file_id: usize) -> Result<(), String> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1550.0, 950.0])
                .with_title("VEX - Compiler GUI Inspector"),
            ..Default::default()
        };

        eframe::run_native(
            "VEX Inspector",
            options,
            Box::new(|_cc| {
                Ok(Box::new(Self::new(file_id)))
            }),
        ).map_err(|e| e.to_string())
    }

    fn get_token_color(&self, kind: &TokenType) -> egui::Color32 {
        match kind {
            TokenType::Var | TokenType::Const | TokenType::Pub | TokenType::Priv | TokenType::Static | TokenType::Async => egui::Color32::from_rgb(200, 100, 255),
            TokenType::Fn | TokenType::Struct | TokenType::Enum | TokenType::Impl => egui::Color32::from_rgb(100, 150, 255),
            TokenType::TInt | TokenType::TStr | TokenType::TFloat | TokenType::TBool | TokenType::TAny | TokenType::TList | TokenType::TDict => egui::Color32::from_rgb(255, 180, 100),
            TokenType::Identifier => egui::Color32::from_rgb(255, 255, 255),
            TokenType::NumberLiteral(_) => egui::Color32::from_rgb(150, 255, 150),
            TokenType::StringLiteral(_) => egui::Color32::from_rgb(150, 220, 255),
            _ => egui::Color32::from_rgb(150, 150, 150),
        }
    }

    fn render_token_list(&self, ui: &mut egui::Ui, tokens: &Vec<vex_core::token::Token>, title: &str, primary_sel: Option<usize>, other_sel: Option<(usize, &Vec<vex_core::token::Token>)>, ast_range: Option<(usize, usize, usize)>, should_scroll: bool) -> Option<usize> {
        let mut clicked = None;
        ui.heading(title);
        ui.separator();
        
        let mut linked_idx = None;
        if let Some((o_idx, o_tokens)) = other_sel {
            if let Some(target) = o_tokens.get(o_idx) {
                for (idx, t) in tokens.iter().enumerate() {
                    if t.span.start == target.span.start && t.span.end == target.span.end {
                        linked_idx = Some(idx);
                        break;
                    }
                }
            }
        }

        egui::ScrollArea::vertical()
            .id_salt(title)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for (idx, token) in tokens.iter().enumerate() {
                    let is_primary = Some(idx) == primary_sel;
                    let is_linked = Some(idx) == linked_idx;
                    
                    let (in_range, color_idx) = match ast_range {
                        Some((s, e, c)) => (idx >= s && idx < e, c),
                        None => (false, 0),
                    };

                    let color = self.get_token_color(&token.kind);
                    let label = egui::RichText::new(format!("{:2} {:<12} '{}'", idx, format!("{:?}", token.kind), token.lexeme().replace("\n", "\\n")))
                        .color(if is_primary || is_linked { egui::Color32::BLACK } else { color });
                    
                    let mut bg = egui::Color32::TRANSPARENT;
                    if is_primary {
                        bg = egui::Color32::from_rgb(200, 160, 0);
                    } else if is_linked {
                        bg = egui::Color32::from_rgb(100, 180, 255);
                    } else if in_range {
                        bg = self.highlight_colors[color_idx % self.highlight_colors.len()];
                    }

                    let response = ui.selectable_label(is_primary, label.background_color(bg));

                    if response.clicked() {
                        clicked = Some(idx);
                    }
                    if (is_primary || is_linked) && should_scroll {
                        response.scroll_to_me(None);
                    }
                }
            });
        clicked
    }
}

impl eframe::App for GuiInspector {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        // GLOBAL INPUT
        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.app.step();
            self.scroll_to_selection = true;
        }

        // TOP PANEL
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("VEX COMPILER INSPECTOR").strong().color(egui::Color32::from_rgb(255, 100, 255)));
                ui.separator();
                
                ui.selectable_value(&mut self.view_mode, ViewMode::Lexer, "1. Lexer");
                ui.selectable_value(&mut self.view_mode, ViewMode::PreParser, "2. PreParser");
                ui.selectable_value(&mut self.view_mode, ViewMode::Parser, "3. Parser");
                ui.selectable_value(&mut self.view_mode, ViewMode::Trace, "4. Execution Trace");
                ui.selectable_value(&mut self.view_mode, ViewMode::Full, "⚡ Full View");
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⏭ Skip Phase").clicked() { self.app.skip_phase(); self.scroll_to_selection = true; }
                    if ui.button("Step (Space)").clicked() { self.app.step(); self.scroll_to_selection = true; }
                    if ui.button("🔄 Reload (R)").clicked() || ctx.input(|i| i.key_pressed(egui::Key::R)) { let _ = self.app.reload(); self.scroll_to_selection = true; }
                    if ui.button("🎯 Focus (F)").clicked() || ctx.input(|i| i.key_pressed(egui::Key::F)) { self.scroll_to_selection = true; }
                });
            });
        });

        // BOTTOM STATUS BAR
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!(" PH: {:?} ", self.app.phase)).background_color(egui::Color32::from_rgb(100, 50, 150)).color(egui::Color32::WHITE).strong());
                ui.separator();
                if let Some(idx) = self.app.selected_token_idx {
                    let tokens = self.app.current_tokens();
                    if let Some(t) = tokens.get(idx) {
                        ui.label(format!("Token: {} (L: {}, P: {})", idx, t.span.line, t.span.start));
                    }
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Traces: {} | Diags: {}", self.app.traces.len(), self.app.diagnostics.len()));
                });
            });
        });

        // SHARED UI DATA
        let mut source_clicked_token_idx = None;
        let selected_token_idx = self.app.selected_token_idx;
        let ast_idx = self.app.get_selected_ast_idx();
        let ast_range_info = ast_idx.and_then(|idx| {
            self.app.ast_token_ranges.get(idx).map(|(s, e)| (*s, *e, idx))
        });
        let should_scroll = self.scroll_to_selection;

        // PANELS
        let mut ast_clicked_idx = None;
        match self.view_mode {
            ViewMode::Lexer => {
                egui::SidePanel::left("p1").resizable(true).default_width(300.0).show(ctx, |ui| {
                    if let Some(idx) = self.render_token_list(ui, &self.app.lexer_tokens, "Lexer Tokens", selected_token_idx, None, None, should_scroll) {
                        source_clicked_token_idx = Some(idx);
                    }
                });
                egui::CentralPanel::default().show(ctx, |ui| render_source_code(ui, &self.app, &mut source_clicked_token_idx, &self.highlight_colors, should_scroll));
            }
            ViewMode::PreParser => {
                egui::SidePanel::left("p1").resizable(true).default_width(220.0).show(ctx, |ui| {
                    let other = selected_token_idx.map(|idx| (idx, &self.app.refined_tokens));
                    self.render_token_list(ui, &self.app.lexer_tokens, "Lexer Tokens", None, other, None, should_scroll);
                });
                egui::SidePanel::left("p2").resizable(true).default_width(220.0).show(ctx, |ui| {
                    let other = selected_token_idx.map(|idx| (idx, &self.app.lexer_tokens));
                    if let Some(idx) = self.render_token_list(ui, &self.app.refined_tokens, "Refined Tokens", selected_token_idx, other, None, should_scroll) {
                        source_clicked_token_idx = Some(idx);
                    }
                });
                egui::CentralPanel::default().show(ctx, |ui| render_source_code(ui, &self.app, &mut source_clicked_token_idx, &self.highlight_colors, should_scroll));
            }
            ViewMode::Parser => {
                egui::SidePanel::left("p1").resizable(true).default_width(200.0).show(ctx, |ui| {
                    if let Some(idx) = self.render_token_list(ui, &self.app.refined_tokens, "Refined Tokens", selected_token_idx, None, ast_range_info, should_scroll) {
                        source_clicked_token_idx = Some(idx);
                    }
                });
                egui::SidePanel::left("p2").resizable(true).default_width(180.0).show(ctx, |ui| {
                    if let Some(idx) = render_ast_list(ui, &mut self.app, should_scroll) {
                        ast_clicked_idx = Some(idx);
                    }
                });
                egui::SidePanel::right("p3").resizable(true).default_width(400.0).show(ctx, |ui| render_source_code(ui, &self.app, &mut source_clicked_token_idx, &self.highlight_colors, should_scroll));
                egui::CentralPanel::default().show(ctx, |ui| render_ast_content(ui, &self.app, &self.highlight_colors, should_scroll));
            }
            ViewMode::Trace => {
                egui::SidePanel::left("p1").resizable(true).default_width(300.0).show(ctx, |ui| {
                    ui.heading("Execution History");
                    ui.separator();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (idx, node) in self.app.traces.iter().enumerate() {
                            if ui.selectable_label(false, format!("{}. {}", idx, node.name)).clicked() {
                                // Potentially jump back or highlight
                            }
                        }
                    });
                });
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Step Trace (Call Stack)");
                    ui.separator();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for node in &self.app.last_step_traces {
                            render_trace_node(ui, node);
                        }
                    });
                });
            }
            ViewMode::Full => {
                egui::SidePanel::left("p1").resizable(true).default_width(150.0).show(ctx, |ui| {
                    let other = selected_token_idx.map(|idx| (idx, &self.app.refined_tokens));
                    self.render_token_list(ui, &self.app.lexer_tokens, "Lexer", None, other, None, should_scroll);
                });
                egui::SidePanel::left("p2").resizable(true).default_width(150.0).show(ctx, |ui| {
                    let other = selected_token_idx.map(|idx| (idx, &self.app.lexer_tokens));
                    if let Some(idx) = self.render_token_list(ui, &self.app.refined_tokens, "Refined", selected_token_idx, other, ast_range_info, should_scroll) {
                        source_clicked_token_idx = Some(idx);
                    }
                });
                egui::SidePanel::left("p3").resizable(true).default_width(150.0).show(ctx, |ui| {
                    if let Some(idx) = render_ast_list(ui, &mut self.app, should_scroll) {
                        ast_clicked_idx = Some(idx);
                    }
                });
                egui::SidePanel::right("p4").resizable(true).default_width(300.0).show(ctx, |ui| {
                    ui.heading("Execution Trace");
                    ui.separator();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for node in &self.app.last_step_traces {
                            render_trace_node(ui, node);
                        }
                    });
                });
                egui::SidePanel::right("p5").resizable(true).default_width(350.0).show(ctx, |ui| {
                    render_source_code(ui, &self.app, &mut source_clicked_token_idx, &self.highlight_colors, should_scroll);
                });
                egui::CentralPanel::default().show(ctx, |ui| {
                    render_ast_content(ui, &self.app, &self.highlight_colors, should_scroll);
                });
            }
        }

        if let Some(idx) = source_clicked_token_idx { self.app.select_token(idx); self.scroll_to_selection = true; }
        if let Some(idx) = ast_clicked_idx { self.app.select_ast(idx); self.scroll_to_selection = true; }

        if !self.app.diagnostics.is_empty() {
             egui::TopBottomPanel::bottom("diagnostics").resizable(true).default_height(100.0).show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(format!("Diagnostics ({})", self.app.diagnostics.len()));
                    if ui.button("Clear").clicked() { self.app.diagnostics.clear(); }
                });
                egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                    for diag in &self.app.diagnostics {
                        let color = if matches!(diag.level, vex_diagnostic::diagnostic::DiagnosticLevel::Error) { egui::Color32::LIGHT_RED } else { egui::Color32::KHAKI };
                        ui.label(egui::RichText::new(format!("[{:?}] {}", diag.level, diag.message)).color(color));
                    }
                });
            });
        }

        self.scroll_to_selection = false;
        ctx.request_repaint();
    }
}

// HELPERS

fn render_trace_node(ui: &mut egui::Ui, node: &TraceNode) {
    let label = if let Some(args) = &node.args {
        format!("{} ({})", node.name, args)
    } else {
        node.name.clone()
    };

    if node.children.is_empty() {
        ui.label(egui::RichText::new(format!("○ {}", label)).color(egui::Color32::from_gray(180)));
    } else {
        egui::CollapsingHeader::new(egui::RichText::new(format!("▼ {}", label)).strong())
            .default_open(true)
            .show(ui, |ui| {
                for child in &node.children {
                    render_trace_node(ui, child);
                }
            });
    }
}

fn render_ast_list(ui: &mut egui::Ui, app: &mut InspectorApp, should_scroll: bool) -> Option<usize> {
    ui.heading("AST Nodes");
    ui.separator();
    let mut clicked_ast = None;
    egui::ScrollArea::vertical().id_salt("ast_nodes").auto_shrink([false, false]).show(ui, |ui| {
        let current_ast_selected = app.ast_list_state.selected();
        for (idx, _decl) in app.ast.iter().enumerate() {
            let is_selected = current_ast_selected == Some(idx);
            let resp = ui.selectable_label(is_selected, egui::RichText::new(format!("#{} DECL", idx)).strong());
            if resp.clicked() { clicked_ast = Some(idx); }
            if is_selected && should_scroll { resp.scroll_to_me(None); }
        }
    });
    clicked_ast
}

fn render_ast_content(ui: &mut egui::Ui, app: &InspectorApp, colors: &[egui::Color32], should_scroll: bool) {
    ui.heading("AST Detail");
    ui.separator();
    if let Some(idx) = app.get_selected_ast_idx() {
        if let Some(decl) = app.ast.get(idx) {
            render_ast_detail(ui, decl, app, colors, should_scroll);
        }
    } else {
        ui.centered_and_justified(|ui| { ui.label("Select a node from AST List to see details"); });
    }
}

fn render_source_code(ui: &mut egui::Ui, app: &InspectorApp, source_clicked_token_idx: &mut Option<usize>, colors: &[egui::Color32], should_scroll: bool) {
    ui.heading("Source Code");
    ui.separator();
    let selected_idx = app.selected_token_idx;
    let ast_range_info = app.get_selected_ast_idx().and_then(|idx| app.ast_token_ranges.get(idx).cloned());
    let source_text = app.source.clone();
    let tokens = app.current_tokens();

    egui::ScrollArea::vertical().id_salt("source_code").auto_shrink([false, false]).show(ui, |ui| {
        ui.vertical(|ui| {
            let mut offset = 0;
            for (i, line) in source_text.split_inclusive('\n').enumerate() {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label(egui::RichText::new(format!("{:3} | ", i + 1)).color(egui::Color32::from_gray(100)).monospace());
                    let line_chars: Vec<char> = line.chars().collect();
                    for (char_idx, &ch) in line_chars.iter().enumerate() {
                        let global_pos = offset + line_chars[..char_idx].iter().collect::<String>().len();
                        let mut text = egui::RichText::new(ch.to_string()).monospace();
                        let mut has_bg = false;
                        let mut is_selected_char = false;
                        if let Some(idx) = selected_idx {
                            if let Some(t) = tokens.get(idx) {
                                if global_pos >= t.span.start && global_pos < t.span.end {
                                    text = text.background_color(egui::Color32::from_rgb(200, 160, 0)).color(egui::Color32::BLACK).strong();
                                    has_bg = true;
                                    is_selected_char = true;
                                }
                            }
                        }
                        if !has_bg {
                            if let Some((start_idx, end_idx)) = ast_range_info {
                                for t_idx in start_idx..end_idx {
                                    if let Some(t) = tokens.get(t_idx) {
                                        if global_pos >= t.span.start && global_pos < t.span.end {
                                            text = text.background_color(colors[t_idx % colors.len()]).color(egui::Color32::WHITE);
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        let resp = ui.add(egui::Label::new(text).sense(egui::Sense::click()));
                        if resp.clicked() {
                            for (idx, token) in tokens.iter().enumerate() {
                                if global_pos >= token.span.start && global_pos < token.span.end { *source_clicked_token_idx = Some(idx); break; }
                            }
                        }
                        if is_selected_char && should_scroll { resp.scroll_to_me(None); }
                    }
                });
                offset += line.len();
            }
        });
    });
}

fn render_ast_detail(ui: &mut egui::Ui, decl: &vex_parser::declarations::Declaration, app: &InspectorApp, colors: &[egui::Color32], should_scroll: bool) {
    let pretty_ast = format!("{:#?}", decl);
    let selected_ast_idx = app.get_selected_ast_idx().unwrap();
    let token_range = app.ast_token_ranges.get(selected_ast_idx).cloned().unwrap_or((0, 0));
    let tokens = app.current_tokens();
    let selected_token_idx = app.selected_token_idx;

    egui::ScrollArea::both().id_salt("ast_detail").auto_shrink([false, false]).show(ui, |ui| {
        for line in pretty_ast.lines() {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                let parts = line.split_inclusive(|c: char| !c.is_alphanumeric() && c != '"' && c != '\'');
                for part in parts {
                    let mut style = egui::RichText::new(part).monospace();
                    let mut has_bg = false;
                    let mut is_highlighted = false;
                    let trimmed = part.trim_matches(|c: char| !c.is_alphanumeric());
                    if !trimmed.is_empty() {
                        let target = trimmed.to_lowercase();
                        for t_idx in token_range.0..token_range.1 {
                            if let Some(t) = tokens.get(t_idx) {
                                let lex = t.lexeme().to_lowercase();
                                let kind_name = format!("{:?}", t.kind).to_lowercase();
                                if lex == target || kind_name == target || format!("\"{}\"", lex) == target || lex.trim_matches('"') == target {
                                    let base_color = colors[t_idx % colors.len()];
                                    if Some(t_idx) == selected_token_idx {
                                        style = style.background_color(egui::Color32::from_rgb(200, 160, 0)).color(egui::Color32::BLACK).strong();
                                        is_highlighted = true;
                                    } else { style = style.background_color(base_color).color(egui::Color32::WHITE).strong(); }
                                    has_bg = true;
                                    break;
                                }
                            }
                        }
                    }
                    if !has_bg {
                        let p_trimmed = part.trim();
                        if p_trimmed.ends_with(':') { style = style.color(egui::Color32::from_rgb(150, 150, 250)); }
                        else if p_trimmed.contains('"') || p_trimmed.contains('\'') { style = style.color(egui::Color32::from_rgb(100, 200, 255)); }
                        else if p_trimmed.chars().all(|c| c.is_numeric() || c == '.') { style = style.color(egui::Color32::from_rgb(150, 250, 150)); }
                        else if matches!(p_trimmed, "Token" | "Span" | "Some" | "None" | "Vec" | "Stmt" | "Expr") { style = style.color(egui::Color32::from_rgb(200, 100, 200)); }
                    }
                    let resp = ui.label(style);
                    if is_highlighted && should_scroll { resp.scroll_to_me(None); }
                }
            });
        }
    });
}
