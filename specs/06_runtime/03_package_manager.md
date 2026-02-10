# Package Manager (Cargo-backed)

The AIVI CLI uses Cargo as the dependency resolver and build tool. AIVI sources
live in `src/`, and generated Rust is written to `target/aivi-gen/`.

## Package Discovery

- `aivi search <query>` searches crates.io with the `aivi` keyword and only
  presents AIVI packages.

## Installing Dependencies

- `aivi install <spec>` edits `[dependencies]` in the root `Cargo.toml`.
- Installs are **strict by default**: the dependency must declare
  `[package.metadata.aivi]` with `language_version` and `kind`.
- Missing metadata is a hard error (no warn-only mode).
- `--no-fetch` skips `cargo fetch`.

## AIVI Package Metadata

An AIVI package is a Rust crate that declares:

```toml
[package.metadata.aivi]
language_version = "0.1"
kind = "lib"
```
