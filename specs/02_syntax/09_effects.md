# Effects

## 9.1 The `Effect ε A` Type

Effectful operations in AIVI are modeled using the `Effect ε A` type, where:
- `ε` is the **error domain** (describing what could go wrong).
- `A` is the **successful return value**.

### Semantics
- **Atomic Progress**: Effects are either successfully completed, failed with `ε`, or **cancelled**.
- **Cancellation**: Cancellation is an asynchronous signal that stops the execution of an effect. When cancelled, the effect is guaranteed to run all registered cleanup (see [Resources](file:///home/mendrik/desk/mendrik/aivi/specs/02_syntax/15_resources.md)).
- **Transparent Errors**: Errors in `ε` are part of the type signature, forcing explicit handling or propagation.

---

## 9.2 `effect` blocks

```aivi
main = effect {
  cfg = load (file.json "config.json")
  print "loaded"
}
```

This is syntax sugar for monadic binding (see Desugaring section). All effectful operations within these blocks are automatically sequenced.

---

## 9.3 Effects and patching

```aivi
user = fetchUser 123

authorized = user <= {
  roles: _ ++ ["Admin"]
  lastLogin: now
}
```

Automatic lifting handles `Result` and other effect functors seamlessly.

---

## 9.4 Comparison and Translation

The `effect` block is the primary way to sequence impure operations. It translates directly to monadic binds.

| `effect` Syntax | Explicit Monadic Syntax |
| :--- | :--- |
| `val = effect { x = f; g x }` | `val = f |> bind (x => g x)` |
| `effect { f; g }` | `f |> bind (_ => g)` |

Example translation:

```aivi
// Sequence with effect block
transfer = from to amount => effect {
  balance = getBalance from
  if balance >= amount then {
    withdraw from amount
    deposit to amount
    Ok Unit
  } else {
    Err InsufficientFunds
  }
}

// Equivalent functional composition
transfer = from to amount =>
  getBalance from |> bind (balance =>
    if balance >= amount then
      withdraw from amount |> bind (_ =>
        deposit to amount |> bind (_ =>
          pure (Ok Unit)))
    else
      pure (Err InsufficientFunds)
  )
```
## 9.5 Expressive Effect Composition

Effect blocks can be combined with pipelines and pattern matching to create very readable business logic.

### Concatenating effectful operations
```aivi
// Fetch config, then fetch data, then log
setup = effect {
  loadConfig "prod.json"
    |> filter _.enabled
    |> bind fetchRemoteData
    |> map logSuccess
}
```

### Expressive Error Handling
```aivi
// Attempt operation, providing a typed default on error
getUser = id => effect {
  api.fetchUser id ? {
    Ok user => user
    Err _   => GuestUser
  }
}

// Composition with Result domains
validatedUser = getUser 123
  |> filter (_.age > 18)
  |> map toAdmin
```
