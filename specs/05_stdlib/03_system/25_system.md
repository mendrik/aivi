# System Domain

The `System` domain connects your program to the operating system.

It allows you to read **Environment Variables** (like secret queries or API keys), handle command-line arguments, or signal success/failure with exit codes. It is the bridge between the managed AIVI runtime and the chaotic host machine.

## Overview

```aivi
use aivi.std.system (Env)

// Read an environment variable
port = Env.get("PORT") |> Option.default("8080")
```

## Goals for v1.0

- Environment variables (read-only or read-write depending on capabilities).
- Command-line arguments.
- Process termination (`exit`).
- Spawning child processes (optional for v1.0, but good to plan).
