# M9 MCP — Host Architecture

## Build outputs

From `aivi build`:

- `target/aivi/<pkg>/<module>.wasm`
- `target/aivi/<pkg>/<module>.aivi-meta.json`
- Optional: `target/aivi/<pkg>/<module>.component.wasm`

## Host responsibilities

- Load the WASM module and metadata bundle.
- Expose tools/resources via MCP transport.
- Marshal JSON ↔ AIVI values (or component-model values later).
- Enforce capability policies for effects (filesystem, network, env).

## Execution models

### Model A: JSON bridge (initial)

1. MCP request arrives as JSON.
2. Host converts JSON → runtime AIVI values.
3. Calls tool export.
4. Converts return value to JSON.

### Model B: component model (long-term)

1. JSON maps to WIT types.
2. Host calls exports via `wasmtime::component`.
3. Canonical ABI handles value crossing.

## Error handling

- Typed errors from AIVI map to MCP error payloads.
- Host errors include structured codes (parse, invoke, capability denied).

## Capability gating

- Effects required by a tool are declared in metadata.
- Host checks capability config before invocation.
- WASI permissions are set from the same policy.