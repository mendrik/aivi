# M8 LSP — Architecture

## Core design

- Single analysis engine shared by CLI + LSP.
- Incremental pipeline: parse (lossless) → resolve → typecheck → formatting.
- Queries cached per file version; no full rebuild on every keystroke.

## Data model

- `FileId` and `WorkspaceId` are stable, opaque identifiers.
- `SourceMap` stores file text, line offsets, and `Span` → `Range` mapping.
- All diagnostics carry `(FileId, Span, code, message, severity)`.

## Incrementality

Suggested `salsa` (or equivalent) queries:

- `parse(FileId) -> ParsedFile`
- `module_graph(WorkspaceId) -> ModuleGraph`
- `resolve(FileId) -> ResolvedFile`
- `typecheck(FileId) -> TypedFile`
- `format(FileId) -> TextEdit[]`

Constraints:

- Must tolerate missing/broken nodes (error nodes in CST/HIR).
- Never panic on partial input; errors are values.

## LSP server topology

- `aivi_lsp` owns LSP state (open docs, versions, config).
- Compiler crates are pure, deterministic, and cacheable.
- Diagnostic publishing is debounced to avoid thrash.

## Multi-file mapping

- File path → module name mapping derived from `module ... = {}` header.
- Module graph rooted at workspace `aivi.toml` (if present) or folder root.
- Cross-file references resolved via the module graph cache.