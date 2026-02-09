# M9 MCP — Test Plan

## Metadata extraction

- Snapshot test for tool/resource metadata.
- Schema mapping version is included in metadata.

## Host integration

- Start MCP host and list tools/resources.
- Invoke a tool and assert JSON payload + typed errors.

## Capability checks

- Deny-by-default tests for filesystem/network.
- Allowlist tests for permitted dirs/hosts.

## Regression coverage

- JSON conversion round-trips for core types.
- Large payloads and deep ADT nesting.