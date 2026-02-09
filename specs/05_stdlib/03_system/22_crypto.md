# Crypto Domain

The `Crypto` domain provides cryptographic functions.

```aivi
use std.Crypto
```

## Functions

### `sha256`

```aivi
sha256 : String -> String
```

Computes the SHA-256 hash of a string (hex encoded).

### `random_uuid`

```aivi
random_uuid : Unit -> Effect String
```

Generates a random UUID v4.

### `random_bytes`

```aivi
random_bytes : Int -> Effect Bytes
```

Generates `n` random bytes.
