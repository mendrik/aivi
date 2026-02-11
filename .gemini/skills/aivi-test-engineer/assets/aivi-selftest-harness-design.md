# AIVI Self-Test Harness Design

## Goal
Run `tests/**/*.aivi` via `aivi test`.

## Phase 1: Host-runner (recommended first)
- `aivi test` discovers AIVI test modules
- compiles each module to Rust (or bytecode if exists)
- executes entrypoints and collects structured results
- prints a deterministic report and returns non-zero on failure

## Phase 2: Self-runner (optional)
- compile an AIVI `test_runner.aivi` program
- runtime discovers tests (or uses generated registry)
- runner prints report

## Test representation
- `Test` is pure data
- execution is `Effect TestError Unit` or returns `Result TestError Unit`

## Reporting
- failures include expected/got
- optional location data (span) if runner can map back
