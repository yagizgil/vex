use vex_core::span::Span;
use crate::error_codes::DiagnosticCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
    Hint,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub struct Label {
    pub message: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub code: DiagnosticCode,
    pub message: String,
    pub span: Span,
    pub labels: Vec<Label>,
    pub notes: Vec<String>,
}

impl Diagnostic {
    pub fn new(level: DiagnosticLevel, code: DiagnosticCode, message: impl Into<String>, span: Span) -> Self {
        Self {
            level,
            code,
            message: message.into(),
            span,
            labels: Vec::new(),
            notes: Vec::new(),
        }
    }

    pub fn with_label(mut self, message: impl Into<String>, span: Span) -> Self {
        self.labels.push(Label {
            message: message.into(),
            span,
        });
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }
}
