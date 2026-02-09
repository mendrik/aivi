# File Domain

The `File` domain allows reading and writing files.

```aivi
use std.File
```

## Functions

### `read_text`

```aivi
read_text : String -> Effect (Result String Error)
```

Reads the content of a file as a string.

### `write_text`

```aivi
write_text : String -> String -> Effect (Result Unit Error)
```

Writes a string to a file, overwriting it if it exists.

### `exists`

```aivi
exists : String -> Effect Bool
```

Checks if a file exists at the given path.

### `delete`

```aivi
delete : String -> Effect (Result Unit Error)
```

Deletes a file.
