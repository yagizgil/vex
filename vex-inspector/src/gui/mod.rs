use crate::InspectorApp;

use eframe::egui;
use vex_core::smap;
use vex_core::token::TokenType;
use vex_core::trace::TraceNode;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ViewMode {
    Lexer,
    PreParser,
    Parser,
    Trace,
    Full,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AstViewMode {
    Debug,
    Tree,
}

pub struct GuiInspector {
    app: InspectorApp,
    highlight_colors: Vec<egui::Color32>,
    scroll_to_selection: bool,
    view_mode: ViewMode,
    ast_view_mode: AstViewMode,
}

impl GuiInspector {
    pub fn new(file_id: usize) -> Self {
        let (source, filename) = {
            let map = smap!();
            let file = map
                .get_file(file_id)
                .expect("Inspector: File not found in SourceMap");
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
            ast_view_mode: AstViewMode::Tree,
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
            Box::new(|_cc| Ok(Box::new(Self::new(file_id)))),
        )
        .map_err(|e| e.to_string())
    }

    fn get_token_color(&self, kind: &TokenType) -> egui::Color32 {
        match kind {
            TokenType::Var
            | TokenType::Const
            | TokenType::Pub
            | TokenType::Priv
            | TokenType::Static
            | TokenType::Async => egui::Color32::from_rgb(200, 100, 255),
            TokenType::Fn | TokenType::Struct | TokenType::Enum | TokenType::Impl => {
                egui::Color32::from_rgb(100, 150, 255)
            }
            TokenType::TInt
            | TokenType::TStr
            | TokenType::TFloat
            | TokenType::TBool
            | TokenType::TAny
            | TokenType::TList
            | TokenType::TDict => egui::Color32::from_rgb(255, 180, 100),
            TokenType::Identifier => egui::Color32::from_rgb(255, 255, 255),
            TokenType::NumberLiteral(_) => egui::Color32::from_rgb(150, 255, 150),
            TokenType::StringLiteral(_) => egui::Color32::from_rgb(150, 220, 255),
            _ => egui::Color32::from_rgb(150, 150, 150),
        }
    }

    fn render_token_list(
        &self,
        ui: &mut egui::Ui,
        tokens: &Vec<vex_core::token::Token>,
        title: &str,
        primary_sel: Option<usize>,
        other_sel: Option<(usize, &Vec<vex_core::token::Token>)>,
        ast_range: Option<(usize, usize, usize)>,
        should_scroll: bool,
    ) -> Option<usize> {
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
                    let label = egui::RichText::new(format!(
                        "{:2} {:<12} '{}'",
                        idx,
                        format!("{:?}", token.kind),
                        token.lexeme().replace("\n", "\\n")
                    ))
                    .color(if is_primary || is_linked {
                        egui::Color32::BLACK
                    } else {
                        color
                    });

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
                ui.heading(
                    egui::RichText::new("VEX COMPILER INSPECTOR")
                        .strong()
                        .color(egui::Color32::from_rgb(255, 100, 255)),
                );
                ui.separator();

                ui.selectable_value(&mut self.view_mode, ViewMode::Lexer, "1. Lexer");
                ui.selectable_value(&mut self.view_mode, ViewMode::PreParser, "2. PreParser");
                ui.selectable_value(&mut self.view_mode, ViewMode::Parser, "3. Parser");
                ui.selectable_value(&mut self.view_mode, ViewMode::Trace, "4. Execution Trace");
                ui.selectable_value(&mut self.view_mode, ViewMode::Full, "⚡ Full View");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⏭ Skip Phase").clicked() {
                        self.app.skip_phase();
                        self.scroll_to_selection = true;
                    }
                    if ui.button("Step (Space)").clicked() {
                        self.app.step();
                        self.scroll_to_selection = true;
                    }
                    if ui.button("🔄 Reload (R)").clicked()
                        || ctx.input(|i| i.key_pressed(egui::Key::R))
                    {
                        let _ = self.app.reload();
                        self.scroll_to_selection = true;
                    }

                    ui.separator();
                    ui.selectable_value(&mut self.ast_view_mode, AstViewMode::Tree, "🌳 Tree");
                    ui.selectable_value(&mut self.ast_view_mode, AstViewMode::Debug, "🔍 Debug");
                    ui.label("AST View:");
                });
            });
        });

        // BOTTOM STATUS BAR
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!(" PH: {:?} ", self.app.phase))
                        .background_color(egui::Color32::from_rgb(100, 50, 150))
                        .color(egui::Color32::WHITE)
                        .strong(),
                );
                ui.separator();
                if let Some(idx) = self.app.selected_token_idx {
                    let tokens = self.app.current_tokens();
                    if let Some(t) = tokens.get(idx) {
                        ui.label(format!(
                            "Token: {} (L: {}, P: {})",
                            idx, t.span.line, t.span.start
                        ));
                    }
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!(
                        "Traces: {} | Diags: {}",
                        self.app.traces.len(),
                        self.app.diagnostics.len()
                    ));
                });
            });
        });

        // DIAGNOSTICS PANEL
        if !self.app.diagnostics.is_empty() {
            egui::TopBottomPanel::bottom("diagnostics")
                .resizable(true)
                .default_height(120.0)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(
                            egui::RichText::new(format!(
                                "Diagnostics ({})",
                                self.app.diagnostics.len()
                            ))
                            .color(egui::Color32::LIGHT_RED),
                        );
                        if ui.button("🗑 Clear").clicked() {
                            self.app.diagnostics.clear();
                        }
                    });
                    ui.separator();
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            for diag in &self.app.diagnostics {
                                let (level_str, level_color) = match diag.level {
                                    vex_diagnostic::diagnostic::DiagnosticLevel::Error => {
                                        (" ERR ", egui::Color32::from_rgb(220, 50, 50))
                                    }
                                    vex_diagnostic::diagnostic::DiagnosticLevel::Warning => {
                                        (" WRN ", egui::Color32::from_rgb(220, 200, 50))
                                    }
                                    vex_diagnostic::diagnostic::DiagnosticLevel::Note => {
                                        (" NOT ", egui::Color32::from_rgb(50, 150, 220))
                                    }
                                    vex_diagnostic::diagnostic::DiagnosticLevel::Hint => {
                                        (" HNT ", egui::Color32::from_rgb(50, 220, 180))
                                    }
                                };

                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(level_str)
                                            .background_color(level_color)
                                            .color(egui::Color32::BLACK)
                                            .strong(),
                                    );
                                    ui.label(
                                        egui::RichText::new(format!(" {} ", diag.code.as_str()))
                                            .color(level_color)
                                            .strong(),
                                    );
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "L:{:<2} C:{:<2} ",
                                            diag.span.line, diag.span.col
                                        ))
                                        .color(egui::Color32::GRAY),
                                    );
                                    ui.label(&diag.message);
                                });

                                for label in &diag.labels {
                                    ui.horizontal(|ui| {
                                        ui.label("      ");
                                        ui.label(
                                            egui::RichText::new(format!("└─ {}", label.message))
                                                .color(egui::Color32::GRAY)
                                                .italics(),
                                        );
                                    });
                                }
                                ui.add_space(2.0);
                            }
                        });
                });
        }

        // SHARED UI DATA
        let mut source_clicked_token_idx = None;
        let selected_token_idx = self.app.selected_token_idx;
        let ast_idx = self.app.get_selected_ast_idx();
        let ast_range_info = ast_idx.and_then(|idx| {
            self.app
                .ast_token_ranges
                .get(idx)
                .map(|(s, e)| (*s, *e, idx))
        });
        let should_scroll = self.scroll_to_selection;

        // PANELS
        let mut ast_clicked_idx = None;
        match self.view_mode {
            ViewMode::Lexer => {
                egui::SidePanel::left("p1")
                    .resizable(true)
                    .default_width(300.0)
                    .show(ctx, |ui| {
                        if let Some(idx) = self.render_token_list(
                            ui,
                            &self.app.lexer_tokens,
                            "Lexer Tokens",
                            selected_token_idx,
                            None,
                            None,
                            should_scroll,
                        ) {
                            source_clicked_token_idx = Some(idx);
                        }
                    });
                egui::CentralPanel::default().show(ctx, |ui| {
                    render_source_code(
                        ui,
                        &self.app,
                        &mut source_clicked_token_idx,
                        &self.highlight_colors,
                        should_scroll,
                    )
                });
            }
            ViewMode::PreParser => {
                egui::SidePanel::left("p1")
                    .resizable(true)
                    .default_width(220.0)
                    .show(ctx, |ui| {
                        let other = selected_token_idx.map(|idx| (idx, &self.app.refined_tokens));
                        self.render_token_list(
                            ui,
                            &self.app.lexer_tokens,
                            "Lexer Tokens",
                            None,
                            other,
                            None,
                            should_scroll,
                        );
                    });
                egui::SidePanel::left("p2")
                    .resizable(true)
                    .default_width(220.0)
                    .show(ctx, |ui| {
                        let other = selected_token_idx.map(|idx| (idx, &self.app.lexer_tokens));
                        if let Some(idx) = self.render_token_list(
                            ui,
                            &self.app.refined_tokens,
                            "Refined Tokens",
                            selected_token_idx,
                            other,
                            None,
                            should_scroll,
                        ) {
                            source_clicked_token_idx = Some(idx);
                        }
                    });
                egui::CentralPanel::default().show(ctx, |ui| {
                    render_source_code(
                        ui,
                        &self.app,
                        &mut source_clicked_token_idx,
                        &self.highlight_colors,
                        should_scroll,
                    )
                });
            }
            ViewMode::Parser => {
                egui::SidePanel::left("p1")
                    .resizable(true)
                    .default_width(200.0)
                    .show(ctx, |ui| {
                        if let Some(idx) = self.render_token_list(
                            ui,
                            &self.app.refined_tokens,
                            "Refined Tokens",
                            selected_token_idx,
                            None,
                            ast_range_info,
                            should_scroll,
                        ) {
                            source_clicked_token_idx = Some(idx);
                        }
                    });
                egui::SidePanel::left("p2")
                    .resizable(true)
                    .default_width(180.0)
                    .show(ctx, |ui| {
                        if let Some(idx) = render_ast_list(ui, &mut self.app, should_scroll) {
                            ast_clicked_idx = Some(idx);
                        }
                    });
                egui::SidePanel::right("p3")
                    .resizable(true)
                    .default_width(400.0)
                    .show(ctx, |ui| {
                        render_source_code(
                            ui,
                            &self.app,
                            &mut source_clicked_token_idx,
                            &self.highlight_colors,
                            should_scroll,
                        )
                    });
                egui::CentralPanel::default().show(ctx, |ui| {
                    render_ast_content(
                        ui,
                        &self.app,
                        &self.highlight_colors,
                        should_scroll,
                        self.ast_view_mode,
                        &mut source_clicked_token_idx,
                    )
                });
            }
            ViewMode::Trace => {
                egui::SidePanel::left("p1")
                    .resizable(true)
                    .default_width(300.0)
                    .show(ctx, |ui| {
                        ui.heading("Execution History");
                        ui.separator();
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for (idx, node) in self.app.traces.iter().enumerate() {
                                if ui
                                    .selectable_label(false, format!("{}. {}", idx, node.name))
                                    .clicked()
                                {
                                    // Potentially jump back or highlight
                                }
                            }
                        });
                    });
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Step Trace (Call Stack)");
                    ui.separator();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (idx, node) in self.app.last_step_traces.iter().enumerate() {
                            render_trace_node(ui, node, idx);
                        }
                    });
                });
            }
            ViewMode::Full => {
                egui::SidePanel::left("p1")
                    .resizable(true)
                    .default_width(150.0)
                    .show(ctx, |ui| {
                        let other = selected_token_idx.map(|idx| (idx, &self.app.refined_tokens));
                        self.render_token_list(
                            ui,
                            &self.app.lexer_tokens,
                            "Lexer",
                            None,
                            other,
                            None,
                            should_scroll,
                        );
                    });
                egui::SidePanel::left("p2")
                    .resizable(true)
                    .default_width(150.0)
                    .show(ctx, |ui| {
                        let other = selected_token_idx.map(|idx| (idx, &self.app.lexer_tokens));
                        if let Some(idx) = self.render_token_list(
                            ui,
                            &self.app.refined_tokens,
                            "Refined",
                            selected_token_idx,
                            other,
                            ast_range_info,
                            should_scroll,
                        ) {
                            source_clicked_token_idx = Some(idx);
                        }
                    });
                egui::SidePanel::left("p3")
                    .resizable(true)
                    .default_width(150.0)
                    .show(ctx, |ui| {
                        if let Some(idx) = render_ast_list(ui, &mut self.app, should_scroll) {
                            ast_clicked_idx = Some(idx);
                        }
                    });
                egui::SidePanel::right("p4")
                    .resizable(true)
                    .default_width(300.0)
                    .show(ctx, |ui| {
                        ui.heading("Execution Trace");
                        ui.separator();
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for (idx, node) in self.app.last_step_traces.iter().enumerate() {
                                render_trace_node(ui, node, idx);
                            }
                        });
                    });
                egui::SidePanel::right("p5")
                    .resizable(true)
                    .default_width(350.0)
                    .show(ctx, |ui| {
                        render_source_code(
                            ui,
                            &self.app,
                            &mut source_clicked_token_idx,
                            &self.highlight_colors,
                            should_scroll,
                        );
                    });
                egui::CentralPanel::default().show(ctx, |ui| {
                    render_ast_content(
                        ui,
                        &self.app,
                        &self.highlight_colors,
                        should_scroll,
                        self.ast_view_mode,
                        &mut source_clicked_token_idx,
                    );
                });
            }
        }

        if let Some(idx) = source_clicked_token_idx {
            self.app.select_token(idx);
            self.scroll_to_selection = true;
        }
        if let Some(idx) = ast_clicked_idx {
            self.app.select_ast(idx);
            self.scroll_to_selection = true;
        }

        self.scroll_to_selection = false;
        ctx.request_repaint();
    }
}

// HELPERS

fn render_trace_node(ui: &mut egui::Ui, node: &TraceNode, idx: usize) {
    let label = if let Some(args) = &node.args {
        format!("{} ({})", node.name, args)
    } else {
        node.name.clone()
    };

    // Use a multi-part ID source to prevent clashes across depth levels and siblings
    ui.push_id((idx, &node.name, node.depth), |ui| {
        if node.children.is_empty() {
            ui.label(
                egui::RichText::new(format!("○ {}", label)).color(egui::Color32::from_gray(180)),
            );
        } else {
            egui::CollapsingHeader::new(egui::RichText::new(format!("▼ {}", label)).strong())
                .default_open(true)
                .show(ui, |ui| {
                    for (i, child) in node.children.iter().enumerate() {
                        render_trace_node(ui, child, i);
                    }
                });
        }
    });
}

fn render_ast_list(
    ui: &mut egui::Ui,
    app: &mut InspectorApp,
    should_scroll: bool,
) -> Option<usize> {
    ui.heading("AST Nodes");
    ui.separator();
    let mut clicked_ast = None;
    egui::ScrollArea::vertical()
        .id_salt("ast_nodes")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let current_ast_selected = app.ast_list_state.selected();
            for (idx, _decl) in app.ast.iter().enumerate() {
                let is_selected = current_ast_selected == Some(idx);
                let resp = ui.selectable_label(
                    is_selected,
                    egui::RichText::new(format!("#{} DECL", idx)).strong(),
                );
                if resp.clicked() {
                    clicked_ast = Some(idx);
                }
                if is_selected && should_scroll {
                    resp.scroll_to_me(None);
                }
            }
        });
    clicked_ast
}

fn render_ast_content(
    ui: &mut egui::Ui,
    app: &InspectorApp,
    colors: &[egui::Color32],
    should_scroll: bool,
    view_mode: AstViewMode,
    source_clicked_token_idx: &mut Option<usize>,
) {
    ui.horizontal(|ui| {
        ui.heading("AST Detail");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(match view_mode {
                AstViewMode::Debug => "Mode: Debug",
                AstViewMode::Tree => "Mode: Tree",
            });
        });
    });
    ui.separator();
    if let Some(idx) = app.get_selected_ast_idx() {
        if let Some(decl) = app.ast.get(idx) {
            match view_mode {
                AstViewMode::Debug => render_ast_detail(
                    ui,
                    decl,
                    app,
                    colors,
                    should_scroll,
                    source_clicked_token_idx,
                ),
                AstViewMode::Tree => render_ast_tree(
                    ui,
                    decl,
                    app,
                    colors,
                    should_scroll,
                    source_clicked_token_idx,
                ),
            }
        }
    } else {
        ui.centered_and_justified(|ui| {
            ui.label("Select a node from AST List to see details");
        });
    }
}

fn render_source_code(
    ui: &mut egui::Ui,
    app: &InspectorApp,
    source_clicked_token_idx: &mut Option<usize>,
    colors: &[egui::Color32],
    should_scroll: bool,
) {
    ui.heading("Source Code");
    ui.separator();
    let selected_idx = app.selected_token_idx;
    let ast_range_info = app
        .get_selected_ast_idx()
        .and_then(|idx| app.ast_token_ranges.get(idx).cloned());
    let source_text = app.source.clone();
    let tokens = app.current_tokens();

    egui::ScrollArea::vertical()
        .id_salt("source_code")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.vertical(|ui| {
                let mut byte_offset = 0;
                for (i, line) in source_text.split_inclusive('\n').enumerate() {
                    let line_num = i + 1;
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label(
                            egui::RichText::new(format!("{:3} | ", line_num))
                                .color(egui::Color32::from_gray(100))
                                .monospace(),
                        );

                        let line_chars: Vec<char> = line.chars().collect();
                        let mut current_pos = byte_offset;

                        for (_char_idx, &ch) in line_chars.iter().enumerate() {
                            let mut text = egui::RichText::new(ch.to_string()).monospace();
                            let mut has_bg = false;
                            let mut is_selected_char = false;

                            // 1. Selection Highlight (Yellow)
                            if let Some(idx) = selected_idx {
                                if let Some(t) = tokens.get(idx) {
                                    if current_pos >= t.span.start && current_pos < t.span.end {
                                        text = text
                                            .background_color(egui::Color32::from_rgb(200, 160, 0))
                                            .color(egui::Color32::BLACK)
                                            .strong();
                                        has_bg = true;
                                        is_selected_char = true;
                                    }
                                }
                            }

                            // 2. Diagnostic Highlight (Red/Yellow underline-style bg)
                            if !has_bg {
                                for diag in &app.diagnostics {
                                    if current_pos >= diag.span.start && current_pos < diag.span.end
                                    {
                                        let diag_color = if matches!(
                                            diag.level,
                                            vex_diagnostic::diagnostic::DiagnosticLevel::Error
                                        ) {
                                            egui::Color32::from_rgb(120, 30, 30) // Dark Red bg
                                        } else {
                                            egui::Color32::from_rgb(100, 100, 0) // Dark Yellow bg
                                        };
                                        text = text
                                            .background_color(diag_color)
                                            .color(egui::Color32::WHITE);
                                        has_bg = true;
                                        break;
                                    }
                                }
                            }

                            // 3. AST Range Highlight
                            if !has_bg {
                                if let Some((start_idx, end_idx)) = ast_range_info {
                                    for t_idx in start_idx..end_idx {
                                        if let Some(t) = tokens.get(t_idx) {
                                            if current_pos >= t.span.start
                                                && current_pos < t.span.end
                                            {
                                                text = text
                                                    .background_color(colors[t_idx % colors.len()])
                                                    .color(egui::Color32::WHITE);
                                                break;
                                            }
                                        }
                                    }
                                }
                            }

                            let resp = ui.add(egui::Label::new(text).sense(egui::Sense::click()));
                            if resp.clicked() {
                                for (idx, token) in tokens.iter().enumerate() {
                                    if current_pos >= token.span.start
                                        && current_pos < token.span.end
                                    {
                                        *source_clicked_token_idx = Some(idx);
                                        break;
                                    }
                                }
                            }
                            if is_selected_char && should_scroll {
                                resp.scroll_to_me(None);
                            }

                            current_pos += ch.len_utf8();
                        }

                        // Line End Diagnostic Marker
                        let line_diag = app.diagnostics.iter().find(|d| d.span.line == line_num);
                        if let Some(diag) = line_diag {
                            let color = if matches!(
                                diag.level,
                                vex_diagnostic::diagnostic::DiagnosticLevel::Error
                            ) {
                                egui::Color32::from_rgb(255, 100, 100)
                            } else {
                                egui::Color32::from_rgb(255, 255, 150)
                            };
                            ui.label(
                                egui::RichText::new(format!(
                                    "  << {}: {} ",
                                    diag.code.as_str(),
                                    diag.message
                                ))
                                .color(color)
                                .italics(),
                            );
                        }
                    });
                    byte_offset += line.len();
                }
            });
        });
}

fn render_ast_detail(
    ui: &mut egui::Ui,
    decl: &vex_parser::declarations::Declaration,
    app: &InspectorApp,
    colors: &[egui::Color32],
    should_scroll: bool,
    source_clicked_token_idx: &mut Option<usize>,
) {
    let pretty_ast = format!("{:#?}", decl);
    let selected_ast_idx = app.get_selected_ast_idx().unwrap();
    let token_range = app
        .ast_token_ranges
        .get(selected_ast_idx)
        .cloned()
        .unwrap_or((0, 0));
    let tokens = app.current_tokens();
    let selected_token_idx = app.selected_token_idx;

    egui::ScrollArea::both()
        .id_salt("ast_detail")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for line in pretty_ast.lines() {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    let parts = line
                        .split_inclusive(|c: char| !c.is_alphanumeric() && c != '"' && c != '\'');
                    for part in parts {
                        let mut style = egui::RichText::new(part).monospace();
                        let mut has_bg = false;
                        let mut is_highlighted = false;
                        let mut target_token_idx = None;

                        let trimmed = part.trim_matches(|c: char| !c.is_alphanumeric());
                        if !trimmed.is_empty() {
                            let target = trimmed.to_lowercase();
                            for t_idx in token_range.0..token_range.1 {
                                if let Some(t) = tokens.get(t_idx) {
                                    let lex = t.lexeme().to_lowercase();
                                    let kind_name = format!("{:?}", t.kind).to_lowercase();
                                    if lex == target
                                        || kind_name == target
                                        || format!("\"{}\"", lex) == target
                                        || lex.trim_matches('"') == target
                                    {
                                        let base_color = colors[t_idx % colors.len()];
                                        target_token_idx = Some(t_idx);
                                        if Some(t_idx) == selected_token_idx {
                                            style = style
                                                .background_color(egui::Color32::from_rgb(
                                                    200, 160, 0,
                                                ))
                                                .color(egui::Color32::BLACK)
                                                .strong();
                                            is_highlighted = true;
                                        } else {
                                            style = style
                                                .background_color(base_color)
                                                .color(egui::Color32::WHITE)
                                                .strong();
                                        }
                                        has_bg = true;
                                        break;
                                    }
                                }
                            }
                        }
                        if !has_bg {
                            let p_trimmed = part.trim();
                            if p_trimmed.ends_with(':') {
                                style = style.color(egui::Color32::from_rgb(150, 150, 250));
                            } else if p_trimmed.contains('"') || p_trimmed.contains('\'') {
                                style = style.color(egui::Color32::from_rgb(100, 200, 255));
                            } else if p_trimmed.chars().all(|c| c.is_numeric() || c == '.') {
                                style = style.color(egui::Color32::from_rgb(150, 250, 150));
                            } else if matches!(
                                p_trimmed,
                                "Token" | "Span" | "Some" | "None" | "Vec" | "Stmt" | "Expr"
                            ) {
                                style = style.color(egui::Color32::from_rgb(200, 100, 200));
                            }
                        }

                        let resp = ui.add(egui::Label::new(style).sense(egui::Sense::click()));
                        if let Some(t_idx) = target_token_idx {
                            if resp.clicked() {
                                *source_clicked_token_idx = Some(t_idx);
                            }
                        }

                        if is_highlighted && should_scroll {
                            resp.scroll_to_me(None);
                        }
                    }
                });
            }
        });
}

fn render_ast_tree(
    ui: &mut egui::Ui,
    decl: &vex_parser::declarations::Declaration,
    app: &InspectorApp,
    colors: &[egui::Color32],
    _should_scroll: bool,
    source_clicked_token_idx: &mut Option<usize>,
) {
    use vex_parser::declarations::Declaration;
    egui::ScrollArea::vertical()
        .id_salt("ast_tree")
        .show(ui, |ui| match decl {
            Declaration::Var(stmt) => render_stmt_tree(
                ui,
                stmt,
                "Variable Declaration",
                app,
                colors,
                source_clicked_token_idx,
            ),
            Declaration::Fn(stmt) => render_stmt_tree(
                ui,
                stmt,
                "Function Declaration",
                app,
                colors,
                source_clicked_token_idx,
            ),
            _ => {
                ui.label(format!("{:?} (Tree view placeholder)", decl));
            }
        });
}

fn render_stmt_tree(
    ui: &mut egui::Ui,
    stmt: &vex_core::ast::Stmt,
    label: &str,
    app: &InspectorApp,
    colors: &[egui::Color32],
    source_clicked_token_idx: &mut Option<usize>,
) {
    use vex_core::ast::Stmt;
    match stmt {
        Stmt::VarDecl {
            modifiers,
            keyword,
            name,
            vtype,
            initializer,
        } => {
            egui::CollapsingHeader::new(
                egui::RichText::new(format!("{} : {}", label, name.lexeme()))
                    .strong()
                    .color(egui::Color32::LIGHT_BLUE),
            )
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Name: ");
                    render_token_link(ui, name, app, colors, source_clicked_token_idx);
                });
                ui.horizontal(|ui| {
                    ui.label("Keyword: ");
                    render_token_link(ui, keyword, app, colors, source_clicked_token_idx);
                });
                if !modifiers.is_empty() {
                    ui.collapsing("Modifiers", |ui| {
                        for m in modifiers {
                            render_token_link(ui, m, app, colors, source_clicked_token_idx);
                        }
                    });
                }
                if let Some(vt) = vtype {
                    ui.collapsing("Type", |ui| {
                        render_type_tree(ui, vt, app, colors, source_clicked_token_idx);
                    });
                }
                if let Some(init) = initializer {
                    ui.collapsing("Initializer", |ui| {
                        render_expr_tree(ui, init, app, colors, source_clicked_token_idx);
                    });
                }
            });
        }
        Stmt::FnDecl {
            modifiers,
            name,
            params,
            rtype,
            body,
            is_async,
        } => {
            egui::CollapsingHeader::new(
                egui::RichText::new(format!("{} : {}", label, name.lexeme()))
                    .strong()
                    .color(egui::Color32::from_rgb(100, 255, 100)),
            )
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Name: ");
                    render_token_link(ui, name, app, colors, source_clicked_token_idx);
                    if *is_async {
                        ui.label(
                            egui::RichText::new("async")
                                .strong()
                                .color(egui::Color32::KHAKI),
                        );
                    }
                });
                if !modifiers.is_empty() {
                    ui.collapsing("Modifiers", |ui| {
                        for m in modifiers {
                            render_token_link(ui, m, app, colors, source_clicked_token_idx);
                        }
                    });
                }
                if !params.is_empty() {
                    ui.collapsing(format!("Parameters ({})", params.len()), |ui| {
                        for p in params {
                            render_parameter_tree(ui, p, app, colors, source_clicked_token_idx);
                        }
                    });
                }
                if let Some(rt) = rtype {
                    ui.collapsing("Return Type", |ui| {
                        render_type_tree(ui, rt, app, colors, source_clicked_token_idx);
                    });
                }
                if !body.is_empty() {
                    ui.collapsing(format!("Body ({} stmts)", body.len()), |ui| {
                        for (idx, s) in body.iter().enumerate() {
                            render_stmt_tree(
                                ui,
                                s,
                                &format!("[{}]", idx),
                                app,
                                colors,
                                source_clicked_token_idx,
                            );
                        }
                    });
                }
            });
        }
        _ => {
            ui.label(format!("Stmt: {:?}", stmt));
        }
    }
}

fn render_expr_tree(
    ui: &mut egui::Ui,
    expr: &vex_core::ast::Expr,
    app: &InspectorApp,
    colors: &[egui::Color32],
    source_clicked_token_idx: &mut Option<usize>,
) {
    use vex_core::ast::Expr;
    match expr {
        Expr::Literal(val) => {
            ui.label(format!("Literal: {:?}", val));
        }
        Expr::Variable { name } => {
            ui.horizontal(|ui| {
                ui.label("Variable: ");
                render_token_link(ui, name, app, colors, source_clicked_token_idx);
            });
        }
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            egui::CollapsingHeader::new(format!("Binary Op: {}", operator.lexeme()))
                .default_open(true)
                .show(ui, |ui| {
                    ui.collapsing("Left", |ui| {
                        render_expr_tree(ui, left, app, colors, source_clicked_token_idx)
                    });
                    ui.horizontal(|ui| {
                        ui.label("Op: ");
                        render_token_link(ui, operator, app, colors, source_clicked_token_idx);
                    });
                    ui.collapsing("Right", |ui| {
                        render_expr_tree(ui, right, app, colors, source_clicked_token_idx)
                    });
                });
        }
        _ => {
            ui.label(format!("Expr: {:?}", expr));
        }
    }
}

fn render_parameter_tree(
    ui: &mut egui::Ui,
    param: &vex_core::ast::Parameter,
    app: &InspectorApp,
    colors: &[egui::Color32],
    source_clicked_token_idx: &mut Option<usize>,
) {
    ui.horizontal(|ui| {
        render_token_link(ui, &param.name, app, colors, source_clicked_token_idx);
        if let Some(ty) = &param.var_type {
            ui.label(": ");
            render_type_tree(ui, ty, app, colors, source_clicked_token_idx);
        }
    });
}

fn render_type_tree(
    ui: &mut egui::Ui,
    ty: &vex_core::ast::TypeExpr,
    app: &InspectorApp,
    colors: &[egui::Color32],
    source_clicked_token_idx: &mut Option<usize>,
) {
    use vex_core::ast::TypeExpr;
    match ty {
        TypeExpr::Simple(t) => {
            ui.horizontal(|ui| {
                ui.label("Simple: ");
                render_token_link(ui, t, app, colors, source_clicked_token_idx);
            });
        }
        _ => {
            ui.label(format!("Type: {:?}", ty));
        }
    }
}

fn render_token_link(
    ui: &mut egui::Ui,
    token: &vex_core::token::Token,
    app: &InspectorApp,
    colors: &[egui::Color32],
    source_clicked_token_idx: &mut Option<usize>,
) {
    let tokens = app.current_tokens();
    let mut token_idx = None;
    for (idx, t) in tokens.iter().enumerate() {
        if t.span.start == token.span.start && t.span.end == token.span.end {
            token_idx = Some(idx);
            break;
        }
    }

    let is_selected = token_idx == app.selected_token_idx;
    let color = if is_selected {
        egui::Color32::from_rgb(255, 100, 100)
    } else {
        colors[token_idx.unwrap_or(0) % colors.len()]
    };

    let text = egui::RichText::new(format!("'{}'", token.lexeme()))
        .monospace()
        .color(egui::Color32::WHITE)
        .background_color(color);

    if ui.button(text).clicked() {
        if let Some(idx) = token_idx {
            *source_clicked_token_idx = Some(idx);
        }
    }
}
