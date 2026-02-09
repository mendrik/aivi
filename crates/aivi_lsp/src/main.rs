use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use aivi::diagnostics::Span;
use aivi::surface::{parse_modules, DomainItem, ModuleItem};
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticSeverity, DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse,
    InitializeParams, InitializeResult, InitializedParams, OneOf, Position, Range, ServerCapabilities,
    SymbolKind, TextDocumentSyncCapability, TextDocumentSyncKind, Url,
};
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Default)]
struct DocumentState {
    text: String,
    version: Option<i32>,
}

#[derive(Default)]
struct BackendState {
    documents: HashMap<Url, DocumentState>,
}

struct Backend {
    client: Client,
    state: Arc<Mutex<BackendState>>,
}

impl Backend {
    fn span_to_range(span: Span) -> Range {
        let start_line = span.start.line.saturating_sub(1) as u32;
        let start_char = span.start.column.saturating_sub(1) as u32;
        let end_line = span.end.line.saturating_sub(1) as u32;
        let end_char = span.end.column as u32;
        Range::new(Position::new(start_line, start_char), Position::new(end_line, end_char))
    }

    fn path_from_uri(uri: &Url) -> String {
        uri.to_file_path()
            .unwrap_or_else(|_| PathBuf::from(uri.to_string()))
            .display()
            .to_string()
    }

    fn build_diagnostics(text: &str, uri: &Url) -> Vec<Diagnostic> {
        let path = PathBuf::from(Self::path_from_uri(uri));
        let (_, diagnostics) = parse_modules(&path, text);
        diagnostics
            .into_iter()
            .map(|file_diag| Diagnostic {
                range: Self::span_to_range(file_diag.diagnostic.span),
                severity: Some(DiagnosticSeverity::ERROR),
                code: None,
                code_description: None,
                source: Some("aivi".to_string()),
                message: file_diag.diagnostic.message,
                related_information: None,
                tags: None,
                data: None,
            })
            .collect()
    }

    fn build_document_symbols(text: &str, uri: &Url) -> Vec<DocumentSymbol> {
        let path = PathBuf::from(Self::path_from_uri(uri));
        let (modules, _) = parse_modules(&path, text);
        modules
            .into_iter()
            .map(|module| {
                let mut children = Vec::new();
                for export in module.exports {
                    let range = Self::span_to_range(export.span);
                    children.push(DocumentSymbol {
                        name: export.name,
                        detail: Some("export".to_string()),
                        kind: SymbolKind::PROPERTY,
                        tags: None,
                        deprecated: None,
                        range,
                        selection_range: range,
                        children: None,
                    });
                }
                for item in module.items {
                    if let Some(symbol) = Self::symbol_from_item(item) {
                        children.push(symbol);
                    }
                }
                let range = Self::span_to_range(module.span);
                DocumentSymbol {
                    name: module.name.name,
                    detail: Some("module".to_string()),
                    kind: SymbolKind::MODULE,
                    tags: None,
                    deprecated: None,
                    range,
                    selection_range: range,
                    children: Some(children),
                }
            })
            .collect()
    }

    fn symbol_from_item(item: ModuleItem) -> Option<DocumentSymbol> {
        match item {
            ModuleItem::Def(def) => {
                let range = Self::span_to_range(def.span);
                Some(DocumentSymbol {
                    name: def.name.name,
                    detail: None,
                    kind: SymbolKind::FUNCTION,
                    tags: None,
                    deprecated: None,
                    range,
                    selection_range: range,
                    children: None,
                })
            }
            ModuleItem::TypeSig(sig) => {
                let range = Self::span_to_range(sig.span);
                Some(DocumentSymbol {
                    name: sig.name.name,
                    detail: Some("type signature".to_string()),
                    kind: SymbolKind::FUNCTION,
                    tags: None,
                    deprecated: None,
                    range,
                    selection_range: range,
                    children: None,
                })
            }
            ModuleItem::TypeDecl(decl) => {
                let range = Self::span_to_range(decl.span);
                Some(DocumentSymbol {
                    name: decl.name.name,
                    detail: Some("type".to_string()),
                    kind: SymbolKind::STRUCT,
                    tags: None,
                    deprecated: None,
                    range,
                    selection_range: range,
                    children: None,
                })
            }
            ModuleItem::TypeAlias(alias) => {
                let range = Self::span_to_range(alias.span);
                Some(DocumentSymbol {
                    name: alias.name.name,
                    detail: Some("type alias".to_string()),
                    kind: SymbolKind::TYPE_PARAMETER,
                    tags: None,
                    deprecated: None,
                    range,
                    selection_range: range,
                    children: None,
                })
            }
            ModuleItem::ClassDecl(class_decl) => {
                let range = Self::span_to_range(class_decl.span);
                Some(DocumentSymbol {
                    name: class_decl.name.name,
                    detail: Some("class".to_string()),
                    kind: SymbolKind::CLASS,
                    tags: None,
                    deprecated: None,
                    range,
                    selection_range: range,
                    children: None,
                })
            }
            ModuleItem::InstanceDecl(instance_decl) => {
                let range = Self::span_to_range(instance_decl.span);
                Some(DocumentSymbol {
                    name: instance_decl.name.name,
                    detail: Some("instance".to_string()),
                    kind: SymbolKind::OBJECT,
                    tags: None,
                    deprecated: None,
                    range,
                    selection_range: range,
                    children: None,
                })
            }
            ModuleItem::DomainDecl(domain_decl) => {
                let mut children = Vec::new();
                for domain_item in domain_decl.items {
                    match domain_item {
                        DomainItem::TypeAlias(type_alias) => {
                            let range = Self::span_to_range(type_alias.span);
                            children.push(DocumentSymbol {
                                name: type_alias.name.name,
                                detail: Some("domain type".to_string()),
                                kind: SymbolKind::TYPE_PARAMETER,
                                tags: None,
                                deprecated: None,
                                range,
                                selection_range: range,
                                children: None,
                            });
                        }
                        DomainItem::Def(def) | DomainItem::LiteralDef(def) => {
                            let range = Self::span_to_range(def.span);
                            children.push(DocumentSymbol {
                                name: def.name.name,
                                detail: Some("domain def".to_string()),
                                kind: SymbolKind::FUNCTION,
                                tags: None,
                                deprecated: None,
                                range,
                                selection_range: range,
                                children: None,
                            });
                        }
                    }
                }
                let range = Self::span_to_range(domain_decl.span);
                Some(DocumentSymbol {
                    name: domain_decl.name.name,
                    detail: Some("domain".to_string()),
                    kind: SymbolKind::NAMESPACE,
                    tags: None,
                    deprecated: None,
                    range,
                    selection_range: range,
                    children: Some(children),
                })
            }
        }
    }

    async fn update_document(&self, uri: Url, text: String, version: Option<i32>) {
        let mut state = self.state.lock().await;
        state.documents.insert(uri, DocumentState { text, version });
    }

    async fn with_document_text<F, R>(&self, uri: &Url, f: F) -> Option<R>
    where
        F: FnOnce(&str) -> R,
    {
        let state = self.state.lock().await;
        state.documents.get(uri).map(|doc| f(&doc.text))
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                document_symbol_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
            server_info: Some(tower_lsp::lsp_types::ServerInfo {
                name: "aivi-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _params: InitializedParams) {
        self.client
            .log_message(tower_lsp::lsp_types::MessageType::INFO, "aivi-lsp initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: tower_lsp::lsp_types::DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        let version = params.text_document.version;
        self.update_document(uri.clone(), text, Some(version)).await;
        if let Some(diagnostics) = self.with_document_text(&uri, |content| {
            Self::build_diagnostics(content, &uri)
        }).await {
            self.client.publish_diagnostics(uri, diagnostics, Some(version)).await;
        }
    }

    async fn did_change(&self, params: tower_lsp::lsp_types::DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = params.text_document.version;
        if let Some(change) = params.content_changes.into_iter().next() {
            self.update_document(uri.clone(), change.text, version).await;
            if let Some(diagnostics) = self.with_document_text(&uri, |content| {
                Self::build_diagnostics(content, &uri)
            }).await {
                self.client.publish_diagnostics(uri, diagnostics, version).await;
            }
        }
    }

    async fn did_close(&self, params: tower_lsp::lsp_types::DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        let mut state = self.state.lock().await;
        state.documents.remove(&uri);
        self.client.publish_diagnostics(uri, Vec::new(), None).await;
    }

    async fn document_symbol(&self, params: DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;
        let symbols = self
            .with_document_text(&uri, |content| Self::build_document_symbols(content, &uri))
            .await
            .unwrap_or_default();
        Ok(Some(DocumentSymbolResponse::Nested(symbols)))
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(|client| Backend {
        client,
        state: Arc::new(Mutex::new(BackendState::default())),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
