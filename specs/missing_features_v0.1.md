# Missing Features (v0.1 Gap Analysis)

This document lists features, modules, and behaviors described in the **AIVI Language Specification** vs. the **v0.1 Rust Implementation**.

> **Note to Users:** AIVI v0.1 is primarily an interpreted language embedding a CST-to-Kernel pipeline.
> Native Rust code generation exists as an **experimental** backend that emits standalone Rust logic for a limited subset of AIVI
> (see **Native Codegen** below). The interpreter remains the most complete backend today.

## 1. Syntax & Language Features

| Feature | Spec Section | Implementation Status | Notes |
| :--- | :--- | :--- | :--- |
| **Generators** | [Generators](02_syntax/07_generators.md) | **Implemented** | Desugared to Church-encoded lambdas in `kernel.rs`. `generate` blocks supported. `loop`/`recurse` inside generators partial. |
| **Decorators** | [Decorators](02_syntax/14_decorators.md) | **Implemented (Syntax)** | Parsed and validated; only standard decorators allowed. |
| **User-defined Domains** | [Domains, Units, and Deltas](02_syntax/06_domains.md) | **Implemented** | `DomainDecl` exists in CST/HIR. |
| **Patching** | [Patching Records](02_syntax/05_patching.md) | **Implemented** | `Patch` alias exists; desugaring logic present. |
| **`or` fallback** | [Effects: `or` fallback](02_syntax/09_effects.md#fallback-with-or-fallback-only) | **Implemented** | Fallback-only sugar for `Effect` (after `<-` in `effect {}`) and `Result` (expression form). |

## 2. Type System

| Feature | Spec Section | Status | Notes |
| :--- | :--- | :--- | :--- |
| **Higher-Kinded Types** | [The Type System](02_syntax/03_types.md) | **Structurally Implemented** | `Kind` enum (Star/Arrow) and builtins (`List: *->*`, `Effect: *->*->*`) exist in `checker.rs`. Complex inference scenarios may vary. |
| **Row Polymorphism** | [The Type System](02_syntax/03_types.md) | **Implemented** | Open records and row extension/restriction logic present in `checker.rs`. |
| **Effect Typing** | [Effects](02_syntax/09_effects.md) | **Implemented** | `Effect E A` is a first-class type; `attempt`/`pure`/`fail` are built-ins. |

## 3. Standard Library Status

| Module | Status | Backend |
| :--- | :--- | :--- |
| `aivi.regex` | **Implemented** | Backed by `runtime/builtins/regex.rs`. |
| `aivi.i18n` | **Implemented (Minimal)** | Properties catalogs + key/message sigils + placeholder type checking. Placeholder rendering uses the runtime's default formatting (locale-neutral; no CLDR/ICU formatting in v0.1). |
| `aivi.net.http` | **Implemented** | Backed by `runtime/url_http.rs`. |
| `aivi.net.server` | **Implemented** | Backed by `runtime/http.rs` (using `aivi_http_server`). |
| `aivi.db` | **Partial** | `database.rs` exists in stdlib/runtime, likely SQLite wrapper. |
| `aivi.math` | **Implemented** | Extensive `math.rs` in stdlib. |

## 4. Tooling & Execution

| Component | Status | Notes |
| :--- | :--- | :--- |
| **Native Codegen** | **Experimental (Partial)** | `aivi build` emits standalone Rust logic (native backend). Coverage is improving (including `[*]` traversal selectors, Map indexing, and Map key/predicate patch selectors), but the backend is still not feature-complete vs. the interpreter. |
| **Package Manager** | **Implemented (Minimal)** | Cargo-backed `search`/`install` plus `package`/`publish` wrappers. Dependency installs validate `[package.metadata.aivi]` and enforce `kind = "lib"`; publishing validates `aivi.toml` ↔ `Cargo.toml` metadata consistency. |
| **LSP** | **Implemented** | `aivi_lsp` crate exists with diagnostics, formatting, and definition lookup. |

---

## Walkthrough: The v0.1 Reality

If you are using AIVI v0.1 today, you are using a **high-integrity interpreter**.

1.  **The Code is the Truth**: The `crates/aivi/src/` directory contains the definition of the language.
    *   `syntax.rs` / `cst.rs` define what you can write.
    *   `checker.rs` defines a surprisingly capable type system (HKTs, Classes, Rows).
    *   `runtime/` implements the "magical" effects and IO.

2.  **Performance**:
    *   Code is lowered to a Kernel IR and interpreted.
    *   It is fast enough for scripting, servers (via Tokio integration), and tooling.
    *   It is **not yet** generating optimized WASM for high-performance compute, though the type system allows expressing it.

3.  **The "Rust" Target**:
    *   When you run `aivi build`, you get a Rust binary (via `cargo`).
    *   The generated Rust corresponds to the AIVI logic directly (experimental; partial coverage).
