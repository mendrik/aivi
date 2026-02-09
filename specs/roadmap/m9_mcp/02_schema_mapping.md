# M9 MCP — Schema Mapping

## Canonical JSON Schema rules

### Primitives

- `Int` → `{ "type": "integer" }` (decide i64 vs bigints explicitly)
- `Float` → `{ "type": "number" }`
- `Bool` → `{ "type": "boolean" }`
- `Text` → `{ "type": "string" }`

### Records

```json
{ "type": "object", "properties": { ... }, "required": [ ... ] }
```

### ADTs

- Use tagged `oneOf` with explicit `tag` field.
- Example shape: `{ "tag": "Case", "fields": { ... } }`.

### Option + Result

- `Option A` → `anyOf: [A, null]` (or a tagged option; choose one).
- `Result A E` → tagged `oneOf` with `ok`/`err` cases.

### Lists

- `List A` → `{ "type": "array", "items": A }`.

## Versioning

- Schema mapping is versioned and stored in metadata.
- Host rejects incompatible versions with a typed error.

## Docs + naming

- Preserve source docstrings in schema descriptions.
- Exported tool/resource names are stable and case-preserving.