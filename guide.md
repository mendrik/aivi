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
```

## Test

```bash
cargo test
```
