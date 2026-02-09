# System Domain

The `System` domain provides access to the environment and process control, abstracting over the host runtime.

## Overview

```aivi
import aivi.std.system use { Env, Process }

let path = Env.get("PATH")
let args = Env.args()
```

## Goals for v1.0

- Environment variables (read-only or read-write depending on capabilities).
- Command-line arguments.
- Process termination (`exit`).
- Spawning child processes (optional for v1.0, but good to plan).
