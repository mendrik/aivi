# Log Domain

The `Log` domain provides structured logging facilities suitable for modern observability.

## Overview

```aivi
import aivi.std.log use { info, warn, error }

info("Server started", { port: 8080 })
```

## Goals for v1.0

- Standard levels: `Trace`, `Debug`, `Info`, `Warn`, `Error`.
- Structured context (key-value pairs) rather than just format strings.
- Pluggable backends (console by default, WASI logging).
