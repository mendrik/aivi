# Sigils

Sigils provide custom parsing for complex literals. They start with `~` followed by a tag and a delimiter.

```aivi
// Regex
pattern = ~r/\w+@\w+\.\w+/

// URL
endpoint = ~u(https://api.example.com)

// Date
birthday = ~d(1990-12-31)
```

Domains define these sigils to validate and construct types at compile time.
