use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticLabel {
    pub message: String,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    pub code: String,
    pub message: String,
    pub span: Span,
    pub labels: Vec<DiagnosticLabel>,
}

#[derive(Debug, Clone)]
pub struct FileDiagnostic {
    pub path: String,
    pub diagnostic: Diagnostic,
}

pub fn render_diagnostics(path: &str, diagnostics: &[Diagnostic]) -> String {
    let mut output = String::new();
    for (index, diagnostic) in diagnostics.iter().enumerate() {
        if index > 0 {
            output.push('\n');
        }
        output.push_str(&render_diagnostic(path, diagnostic));
    }
    output
}

pub fn render_diagnostic(path: &str, diagnostic: &Diagnostic) -> String {
    let mut output = String::new();
    let start = &diagnostic.span.start;
    output.push_str(&format!(
        "error[{}] {}:{}:{} {}\n",
        diagnostic.code, path, start.line, start.column, diagnostic.message
    ));
    for label in &diagnostic.labels {
        let pos = &label.span.start;
        output.push_str(&format!(
            "  note: {} at {}:{}:{}\n",
            label.message, path, pos.line, pos.column
        ));
    }
    output.trim_end().to_string()
}
