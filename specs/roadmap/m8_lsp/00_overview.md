# M8 LSP “Daily Driver” — Overview

Goal: make AIVI comfortable to edit day-to-day while the compiler evolves.

## Scope

- Diagnostics, formatting, definition/references, rename, hover/types, semantic tokens, code actions.
- Incremental analysis over open documents and workspace files.
- VS Code extension driven by `aivi lsp`.

## Deliverables

- A stable `aivi lsp` server with feature parity across the core LSP requests.
- Robust diagnostics that never crash on partial code.
- Formatting and code actions that mirror the compiler’s CST/HIR behavior.

## Acceptance criteria

- Editing is comfortable on the existing `specs/` and `examples/`.
- All listed LSP features work across multi-module workspaces.
- Latency is “good enough” (keystroke-to-diagnostic within ~200ms for small files).

## Parallel workstreams

1. Analysis engine + incrementality.
2. LSP protocol glue + VS Code client integration.
3. Feature-by-feature rollout (diagnostics → navigation → types → editing tools).

## Related docs

- [LSP Plan](../05_language_server_lsp)
- [M8 Architecture](01_architecture)
- [M8 Features](02_features)
- [M8 Workplan](03_workplan)