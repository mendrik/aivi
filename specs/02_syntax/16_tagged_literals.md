# Tagged Literals

AIVI supports custom literal syntax via **tagged literals**, similar to template literals in other languages but typed at compile time.

## Overview

Tagged literals allow domains to define custom parsing logic for string content. Common built-in tags include `rex` for regular expressions and `url` for uniform resource locators.

```aivi
let pattern = rex"\w+@\w+\.\w+"
// -> Regex

let endpoint = url"https://api.example.com"
// -> Url
```

## How it works

A tagged literal `tag"content"` is desugared into a call to `tag` macro or factory function, validated at compile-time if possible.

### `rex` (Regex)

Constructs a `Regex` object.
- **Validation**: The string content is parsed as a regular expression. Invalid regex patterns cause a compile-time error.
- **Escaping**: raw string semantics (backslashes are preserved).

### `url` (URL)

Constructs a `Url` object.
- **Validation**: Verified to be a valid URL.
- **Type**: Returns a `Url` record, not a string.

## Custom Tags

(Future work: allowing user-defined domains to register new literal tags.)
