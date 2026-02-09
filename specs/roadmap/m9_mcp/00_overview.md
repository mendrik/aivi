# M9 MCP — Overview

Goal: provide `aivi mcp serve` that exposes AIVI modules as MCP tools/resources with typed schemas.

## Scope

- Rust MCP host that loads AIVI WASM artifacts.
- Tool/resource discovery via annotations.
- JSON Schema generation from AIVI types.

## Deliverables

- `aivi mcp serve` CLI with stdio/http transports.
- Metadata bundle from `aivi build` for tool/resource definitions.
- End-to-end tool invocation with typed errors.

## Acceptance criteria

- MCP clients can list tools/resources and invoke tools.
- Typed errors are reported with stable schema.
- Capability gates prevent unauthorized effects.

## Related docs

- [MCP Integration Plan](../06_mcp_integration)
- [M9 Host Architecture](01_host_architecture)
- [M9 Schema Mapping](02_schema_mapping)
- [M9 CLI + Ops](03_cli_ops)
- [M9 Test Plan](04_test_plan)