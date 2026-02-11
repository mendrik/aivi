mod backend;
mod completion;
mod diagnostics;
mod document_symbols;
mod navigation;
mod semantic_tokens;
mod server;
mod signature;
mod state;
mod workspace;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod repro_lsp;

pub use server::run;
