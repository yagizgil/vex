use std::fs::File;
use std::io::{self, Write};
use crate::app::InspectorApp;

pub fn export_report(app: &InspectorApp) -> io::Result<String> {
    let tokens = app.current_tokens();
    let phase_name = format!("{:?}", app.phase).to_lowercase();
    let safe_filename = app.filename.replace('\\', "_").replace('/', "_").replace('.', "_");
    let final_path = format!("report_{}_{}.md", safe_filename, phase_name);

    let mut file = File::create(&final_path)?;
    let source_lines: Vec<&str> = app.source.lines().collect();

    // 1. Prepare data and calculate max widths for alignment
    let mut rows = Vec::new();
    let mut max_idx = 3;
    let mut max_type = 4;
    let mut max_lexeme = 6;
    let mut max_line_col = 8;
    let mut max_context = 12;

    for (idx, token) in tokens.iter().enumerate() {
        let idx_str = idx.to_string();
        let type_str = format!("{:?}", token.kind);
        let lexeme = token.lexeme().replace('\n', "\\n").replace('\r', "\\r").replace('|', "\\|");
        let line_col = format!("{}:{}", token.span.line, token.span.col);
        
        let line_idx = (token.span.line as usize).saturating_sub(1);
        let line_content = source_lines.get(line_idx).unwrap_or(&"").trim().replace('|', "\\|");
        let context = if line_content.is_empty() { "-".to_string() } else { format!("`{}`", line_content) };

        max_idx = max_idx.max(idx_str.len());
        max_type = max_type.max(type_str.len() + 2); 
        max_lexeme = max_lexeme.max(lexeme.len() + 2);
        max_line_col = max_line_col.max(line_col.len());
        max_context = max_context.max(context.len());

        rows.push((idx_str, type_str, lexeme, line_col, context));
    }

    // 2. Write Report
    writeln!(file, "# Vex Compilation Report")?;
    writeln!(file, "- **File:** `{}`", app.filename)?;
    writeln!(file, "- **Phase:** `{}`", phase_name)?;
    writeln!(file, "- **Total Tokens:** {}", tokens.len())?;
    writeln!(file, "\n## Token List\n")?;
    
    // Header
    writeln!(
        file, 
        "| {:<w1$} | {:<w2$} | {:<w3$} | {:<w4$} | {:<w5$} |", 
        "Idx", "Type", "Lexeme", "Line:Col", "Code Context",
        w1=max_idx, w2=max_type, w3=max_lexeme, w4=max_line_col, w5=max_context
    )?;
    
    // Separator
    writeln!(
        file, 
        "| {:-<w1$} | {:-<w2$} | {:-<w3$} | {:-<w4$} | {:-<w5$} |", 
        "", "", "", "", "",
        w1=max_idx, w2=max_type, w3=max_lexeme, w4=max_line_col, w5=max_context
    )?;
    
    // Rows
    for (idx, kind, lexeme, line_col, context) in rows {
        writeln!(
            file, 
            "| {:<w1$} | {:<w2$} | {:<w3$} | {:<w4$} | {:<w5$} |",
            idx, format!("`{}`", kind), format!("`{}`", lexeme), line_col, context,
            w1=max_idx, w2=max_type, w3=max_lexeme, w4=max_line_col, w5=max_context
        )?;
    }

    Ok(final_path)
}
