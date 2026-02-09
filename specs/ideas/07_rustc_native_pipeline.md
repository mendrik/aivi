# AIVI IR To Native Via rustc

## Overview

The compiler lowers AIVI through a typed IR, then emits a Rust crate and lets
`rustc` handle native code generation and linking.

```text
source -> CST -> AST -> HIR -> Core IR -> Rust crate -> rustc -> native binary
```


## IR Stages

- **HIR**: desugared surface syntax (blocks, JSX, pattern matches).
- **Core IR**: explicit effects, closures, and fully-specified match trees.
- **Lowered IR**: control-flow graph with basic blocks (MIR-like).

Each stage narrows the language surface and makes codegen simpler.


## Codegen Strategy

- Generate a Rust crate per AIVI package.
- One Rust function per AIVI definition (monomorphized where possible).
- Use a small `aivi_runtime` crate for effects, IO, concurrency, HTML render.
- Represent algebraic data as Rust enums or tagged structs.


## rustc Integration

Two options:

1. **Temporary crate + `cargo build`**
   - Emit Rust files into `target/aivi/<crate>/`.
   - Call `cargo` with a generated `Cargo.toml`.
2. **`rustc_interface`**
   - Invoke `rustc` directly with in-memory sources.
   - Allows tighter integration and custom diagnostics.


## Debugging And Diagnostics

- Emit a `.rs` file for source maps and easier debugging.
- Preserve spans to map Rust errors back to AIVI source.
- Use `#[track_caller]` and `panic` hooks in `aivi_runtime` for runtime errors.


## Linking And Targets

- Native targets use the system toolchain (`x86_64-unknown-linux-gnu`, etc.).
- Cross-compile through `rustc` target selection.
- `-C lto` and `-C opt-level=3` for optimized release builds.


## Open Questions

- How to cache IR and generated Rust for incremental builds?
- How to expose stable ABI for shared library use?
