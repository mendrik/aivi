# Regex Domain

The `Regex` domain handles **Pattern Matching** for text. Whether you're validating emails, scraping data, or searching logs, simple substring checks often aren't enough. Regex gives you a powerful, concise language to describe *shapes* of text. AIVI's regex support is safe (checked at compile-time with `~r/.../`) and fast (compiling to native matching engines), so you don't have to worry about runtime crashes from bad patterns.

## Overview

```aivi
use aivi.std.regex (Regex)

email_pattern = ~r/^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$/
match = Regex.test(email_pattern, "user@example.com")

// With flags (example: case-insensitive)
email_ci = ~r/^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$/i
```
