# Testing Domain

The `Testing` domain provides first-class support for writing and running tests within AIVI.

## Why this exists

Built-in testing encourages reliability from day one. It standardizes how tests are defined, discovered, and executed, removing the need for third-party test runners.

## Overview

```aivi
import aivi.std.testing use { assert, assert_eq }

@test
def addition_works() {
    assert_eq(1 + 1, 2)
}
```

## Goals for v1.0

- `test` keyword or block construct.
- Assertions with rich diffs (`assert_eq`, etc.).
- Test discovery and execution via `aivi test`.
- Property-based testing basics (generators) integration.
