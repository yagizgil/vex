pub mod error_codes;
pub mod diagnostic;

use std::sync::RwLock;
use once_cell::sync::Lazy;
use crate::diagnostic::{Diagnostic, DiagnosticLevel};
use vex_core::span::Span;

/// GLOBAL_DIAGNOSTICS is a global list of all errors and warnings found during compilation.
/// We use RwLock so multiple threads can safely read or write to it.
pub static GLOBAL_DIAGNOSTICS: Lazy<RwLock<DiagnosticHandler>> = Lazy::new(|| {
    RwLock::new(DiagnosticHandler::new())
});

/// This macro makes it easy to create and emit an error.
/// Example: diag_emit!(Error, L001, "Message", span);
#[macro_export]
macro_rules! diag_emit {
    ($level:ident, $code:ident, $msg:expr, $span:expr) => {
        $crate::GLOBAL_DIAGNOSTICS.write().expect("Diagnostics lock poisoned").emit(
            $crate::diagnostic::Diagnostic::new(
                $crate::diagnostic::DiagnosticLevel::$level,
                $crate::error_codes::DiagnosticCode::$code,
                $msg,
                $span
            )
        )
    };
}

/// This macro is a shortcut to read the diagnostic handler.
#[macro_export]
macro_rules! diag {
    () => { $crate::GLOBAL_DIAGNOSTICS.read().expect("Diagnostics lock poisoned") };
    (write) => { $crate::GLOBAL_DIAGNOSTICS.write().expect("Diagnostics lock poisoned") };
}

/// DiagnosticHandler manages the list of diagnostics (errors, warnings, etc.).
pub struct DiagnosticHandler {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticHandler {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    /// Add a new diagnostic to the list.
    pub fn emit(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Get all current diagnostics.
    pub fn get_diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Clear all diagnostics.
    pub fn reset(&mut self) {
        self.diagnostics.clear();
    }

    /// Check if there are any errors in the list.
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| matches!(d.level, DiagnosticLevel::Error))
    }

    pub fn print_all(&self) {
        use ariadne::{Report, ReportKind, Label, Source, Color};
        use vex_core::smap;

        let smap = smap!();

        for diag in &self.diagnostics {
            let (kind, color) = match diag.level {
                DiagnosticLevel::Error => (ReportKind::Error, Color::Red),
                DiagnosticLevel::Warning => (ReportKind::Warning, Color::Yellow),
                DiagnosticLevel::Note => (ReportKind::Advice, Color::Blue),
                DiagnosticLevel::Hint => (ReportKind::Advice, Color::Cyan),
            };

            let (file_name, source) = if let Some(file) = smap.get_file(diag.span.file_id) {
                (file.path.clone(), Source::from(file.content.as_str()))
            } else {
                ("Unknown".to_string(), Source::from(""))
            };

            let mut builder = Report::build(
                kind, 
                (file_name.clone(), diag.span.start..diag.span.end)
            )
            .with_message(&diag.message)
            .with_code(diag.code.as_str());

            builder = builder.with_label(
                Label::new((file_name.clone(), diag.span.start..diag.span.end))
                    .with_message("Here")
                    .with_color(color)
            );

            for lbl in &diag.labels {
                builder = builder.with_note(format!("+ {}", lbl.message));
            }

            for note in &diag.notes {
                builder = builder.with_note(note);
            }

            builder.finish().print((file_name.clone(), source)).unwrap();
        }
    }
}
