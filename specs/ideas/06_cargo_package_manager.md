# Cargo As AIVI Package Manager

## Goals

- Use Cargo for dependency resolution, caching, and publishing.
- Keep AIVI metadata close to Rust metadata (single `Cargo.toml`).
- Support workspaces with mixed Rust + AIVI packages.


## Workspace Layout

```text
my-aivi-workspace/
├── Cargo.toml
├── crates/
│   ├── app/
│   │   ├── Cargo.toml
│   │   └── aivi/
│   │       ├── main.aivi
│   │       └── ui/
│   └── lib/
│       ├── Cargo.toml
│       └── aivi/
│           └── lib.aivi
└── target/
    └── aivi/
```

Packages can be "pure AIVI" crates with an empty Rust `lib.rs`, or hybrid crates
that ship both AIVI and Rust code.


## Metadata In Cargo.toml

```toml
[package.metadata.aivi]
entry = "aivi/main.aivi"
modules = ["aivi/**.aivi"]
tests = ["aivi/tests/**.aivi"]
target = "wasm32-wasi"
```

The CLI reads `cargo metadata` to discover entries, modules, and test roots.


## Dependency Resolution

- AIVI dependencies live in `[dependencies]` and resolve through Cargo.
- The AIVI compiler reads `Cargo.lock` to get exact versions.
- `cargo fetch` or `cargo vendor` supports offline builds.


## Build And Run Integration

Two integration points:

1. `cargo aivi build|run|test` (subcommand)
2. `build.rs` that calls the AIVI compiler during `cargo build`

Artifacts land in `target/aivi/<crate>/<target>/` with a stable layout.


## Publishing

- Use `cargo package` to include `.aivi` sources (`include = ["aivi/**"]`).
- Publish to crates.io or a private registry.
- AIVI-only crates can export a dummy Rust library to satisfy Cargo.


## Open Questions

- How to encode multiple entrypoints (CLI, lib, tests) in metadata?
- How to version the AIVI stdlib alongside Rust dependencies?
