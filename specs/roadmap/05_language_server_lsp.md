# LSP Plan (Rust `tower-lsp` + shared compiler engine)

Goal: make AIVI a “daily driver” language in editors while the compiler is still evolving.

## Parallel M8 track docs

- [M8 Overview](m8_lsp/00_overview)
- [M8 Architecture](m8_lsp/01_architecture)
- [M8 Features](m8_lsp/02_features)
- [M8 Workplan](m8_lsp/03_workplan)

## Architecture

- A single analysis engine shared by CLI + LSP:
  - parse (lossless) → resolve → typecheck
  - all of it is incremental via `salsa` (or equivalent)
- LSP maintains:
  - open document text
  - file-to-module mapping
  - an analysis “snapshot” per version

Recommended layering:
- `aivi_parser` produces CST + syntax diagnostics.
- `aivi_resolve` produces module graph + name diagnostics.
- `aivi_types` produces type diagnostics + per-node type info.
- `aivi_fmt` formats CST.
- `aivi_lsp` is just protocol glue + conversions to/from `lsp-types`.

## Features by milestone

### Milestone L1: parse diagnostics

- `textDocument/didOpen`, `didChange`:
  - parse file
  - publish syntax diagnostics
- Basic document symbols:
  - show module exports + top-level bindings

### Milestone L2: go-to-definition + completion (names)

- `textDocument/definition`:
  - find symbol under cursor
  - navigate within file and across imports
- `textDocument/completion`:
  - keywords and visible symbols
  - exported module members

### Milestone L3: types in hover + signature help

- `textDocument/hover`:
  - show inferred type and docstring (if available)
- `textDocument/signatureHelp`:
  - for function calls (including curried functions)

### Milestone L4: references + rename

- `textDocument/references`
- `textDocument/rename`

### Milestone L5: formatting + code actions

- `textDocument/formatting`:
  - CST-driven formatter
  - stable formatting profile:
    - deterministic output (same input -> same output)
    - preserves comments + blank lines
    - respects trailing commas + multiline layout
  - request params:
    - respect `tabSize`, `insertSpaces`, `trimTrailingWhitespace`, `insertFinalNewline`
    - formatting is file-scoped (whole document) in L5
- `textDocument/codeAction`:
  - quick fixes: “import missing name”, “add type annotation”, “add match cases”

### Milestone L6: semantic tokens + inlay hints (quality)

- Semantic tokens driven by:
  - resolved IDs (value/type/constructor/module)
  - typed info (effects, domains)
  - stable token legend aligned with VS Code defaults:
    - types: `type`, `class`, `enum`, `interface`, `typeParameter`
    - values: `function`, `method`, `variable`, `parameter`, `property`, `enumMember`
    - modules: `namespace`, `module`
    - keywords/literals: `keyword`, `number`, `string`, `operator`
  - modifiers:
    - `declaration`, `definition`, `readonly`, `static`, `async`
  - token sources:
    - CST for keywords/literals/operators
    - resolver for symbol identity + scope
    - typechecker for value/type distinction + effects
- Inlay hints:
  - inferred types for `let` bindings (optional)
  - effect requirements (optional)

## Key implementation details

### Spans and mapping

Everything depends on robust span mapping:
- `SourceMap` stores file text and line offsets.
- Diagnostics include `(FileId, Span, message, severity, code)`.
- LSP conversion maps spans to `Range`.

### Formatting pipeline

- Formatter consumes CST with trivia (comments/whitespace) preserved.
- Formatting options are derived from `FormattingOptions` and a project profile.
- Output is a single text edit (replace full document) to avoid range drift.
- Formatting is idempotent and safe on partially parsed files (best-effort).

### Semantic tokens pipeline

- LSP advertises `semanticTokensProvider` with `legend` + `full` (delta optional).
- Token emission is stable per file version and uses monotonic ranges.
- Provide `semanticTokens/full` for initial support; add `semanticTokens/full/delta` later.
- Mapping steps:
  - CST walk emits base tokens (keywords, literals, operators).
  - Resolver adds identifiers with symbol kinds + scope info.
  - Typechecker refines token types + modifiers (e.g., `typeParameter`).
- Errors do not stop tokenization; unknown nodes emit no tokens.

### Incrementality model

Use `salsa` queries like:
- `parse(FileId) -> ParsedFile`
- `module_graph(WorkspaceId) -> ModuleGraph`
- `resolve(FileId) -> ResolvedFile`
- `typecheck(FileId) -> TypedFile`

Avoid global “rebuild everything” on each keystroke.

### “Never crash on partial code”

Mandatory:
- parser recovery (synchronize on safe tokens)
- name resolver tolerates missing nodes
- typechecker uses “error types” and keeps going

## VS Code strategy

Short-term:
- update the vscode extension to use LSP (tower-lsp server).
- keep TextMate grammar for basic highlighting.
- enable semantic tokens (L6) for accurate, typed highlighting.

Long-term:
- Add tree-sitter grammar for better highlighting and folding.
- Consider semantic token customization (themes + modifiers).

