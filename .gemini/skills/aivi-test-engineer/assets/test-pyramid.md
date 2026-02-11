# AIVI Test Pyramid

- Tier 0: Rust unit tests (lexer/parser/resolve/types/desugar)
- Tier 1: Rust integration tests (CLI commands; compile+run)
- Tier 2: Golden tests (diagnostics/fmt/codegen snapshots)
- Tier 3: Property tests (parser roundtrips; kernel laws)
- Tier 4: AIVI self-tests (host-runner, then self-runner)
