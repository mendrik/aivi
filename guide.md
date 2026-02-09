# Guide

## Prerequisites

- Rust toolchain (stable). Install via `rustup` if needed.

## Build

```bash
cargo build
```

## Run

```bash
# Show help
cargo run -p aivi -- --help

# Parse a single file
cargo run -p aivi -- parse examples/hello.aivi

# Parse all .aivi files under a directory (recursive)
cargo run -p aivi -- parse examples/...

# Check module name resolution
cargo run -p aivi -- check examples/...

# Desugar to a kernel-friendly HIR (JSON)
cargo run -p aivi -- desugar examples/...

# Format a single file
cargo run -p aivi -- fmt examples/hello.aivi

# Start the language server (requires aivi-lsp in PATH)
cargo run -p aivi -- lsp

# Dump kernel and Rust IR (JSON)
cargo run -p aivi -- kernel examples/11_concurrency.aivi
cargo run -p aivi -- rust-ir examples/11_concurrency.aivi

# Emit a native binary via direct rustc invocation
cargo run -p aivi -- build examples/11_concurrency.aivi --target rustc --out target/aivi-bin -- -C opt-level=3

# Run using the native effect runtime (M6)
cargo run -p aivi -- run examples/11_concurrency.aivi --target native
```

## Test

```bash
cargo test
```

## Notes

- The `rustc` backend currently targets a small, effect-centric subset (enough for hello-world style programs).
- The native runtime is the current home for effects, resources, and concurrency; it is intentionally minimal.
