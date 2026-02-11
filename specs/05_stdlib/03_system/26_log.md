# Log Domain

The `Log` domain provides **Structured Logging** for modern observability.

`print()` is fine for debugging, but production software needs data. This domain lets you attach metadata (like `{ userId: 123 }`) to your logs, making them machine-readable and ready for ingestion by tools like Datadog or Splunk.

## Overview

```aivi
use aivi (log)

log.info "Server started" [("port", "8080"), ("env", "prod")]
```

## Types

```aivi
type Level = Trace | Debug | Info | Warn | Error
type Context = List (Text, Text)
```

## Record Fields

```aivi
log.log   : Level -> Text -> Context -> Effect Text Unit
log.trace : Text -> Context -> Effect Text Unit
log.debug : Text -> Context -> Effect Text Unit
log.info  : Text -> Context -> Effect Text Unit
log.warn  : Text -> Context -> Effect Text Unit
log.error : Text -> Context -> Effect Text Unit
```

## Goals for v1.0

- Standard levels: `Trace`, `Debug`, `Info`, `Warn`, `Error`.
- Structured context (key-value pairs) rather than just format strings.
- Pluggable backends (console by default, WASI logging).
