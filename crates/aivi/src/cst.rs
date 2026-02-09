use serde::Serialize;

use crate::diagnostics::{Diagnostic, Span};

#[derive(Debug, Clone, Serialize)]
pub struct CstToken {
    pub kind: String,
    pub text: String,
    pub span: Span,
}

#[derive(Debug, Serialize)]
pub struct CstFile {
    pub path: String,
    pub byte_count: usize,
    pub line_count: usize,
    pub lines: Vec<String>,
    pub tokens: Vec<CstToken>,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Serialize)]
pub struct CstBundle {
    pub files: Vec<CstFile>,
}
