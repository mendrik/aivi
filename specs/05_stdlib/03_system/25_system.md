# System Domain

<!-- quick-info: {"kind":"module","name":"aivi.system"} -->
The `System` domain connects your program to the operating system.

It allows you to read **Environment Variables** (like secret queries or API keys), handle command-line arguments, or signal success/failure with exit codes. It is the bridge between the managed AIVI runtime and the chaotic host machine.

<!-- /quick-info -->
## Overview

<<< ../../snippets/from_md/05_stdlib/03_system/25_system/block_01.aivi{aivi}

## Values

<<< ../../snippets/from_md/05_stdlib/03_system/25_system/block_02.aivi{aivi}

## Goals

Status:

- 🟢 Done
- 🟡 Partial
- 🔴 Missing

- 🟢 Environment variables (read-only or read-write depending on capabilities).
- 🟢 Command-line arguments.
- 🟢 Process termination (`exit`).
- 🔴 Spawning child processes (optional, but good to plan).
