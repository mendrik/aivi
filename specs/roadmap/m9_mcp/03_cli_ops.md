# M9 MCP — CLI + Ops

## CLI surface

`aivi mcp serve` options:

- `--entry <module>`: module containing tools/resources.
- `--transport stdio|http`.
- `--watch`: rebuild + restart on file change.
- `--allow-fs <dir>` / `--deny-fs`.
- `--allow-net <host>` / `--deny-net`.
- `--env KEY=VALUE` / `--deny-env`.

## Config file

- Optional `aivi.mcp.toml` for defaults (capabilities, transport, logging).
- CLI flags override config values.

## Observability

- Structured logs (request id, tool name, latency).
- Debug mode for JSON conversion traces.

## Security posture

- Deny-by-default for effects.
- Audit log for capability grants.
- Explicit warnings when running with broad permissions.