# Bindings and Scope

## 1.1 Definitions

All bindings use `=`:

* values
* functions
* types
* classes
* instances
* modules

```aivi
pi = 3.14159
add = x y => x + y
```

---

## 1.2 Shadowing

Bindings are lexical and may be shadowed.

```aivi
x = 1
x = x + 1
```

This introduces a new binding; no mutation exists. This is common in functional languages like OCaml and Rust (re-binding) but distinct from mutation.

---

## 1.3 Pattern Bindings

Structural patterns may appear in bindings.

```aivi
{ name } = user      // Shorthand for { name: name }
{ name: n } = user   // Rename binding to 'n'
[h, ...t] = xs       // List destructuring
```

* `=` may only be used where the compiler can prove the pattern is **total** (i.e., it covers all possible shapes of the data).
* Potentially failing matches (refutable patterns) must use `?` (case analysis) or appear in a context where failure can be handled.

> [!NOTE]
> Using `=` with a non-total pattern (like `[h, ...t] = []`) results in a compile-time error. For partial matches, use the `?` operator which converts a refutable pattern into an `Option` or branch.

---

## 1.4 Whole-value binding with `@`

Patterns may bind the **entire value** alongside destructuring.

```aivi
user@{ name: n } = input
user@{ name } = input
```

Semantics:

* `user` is bound to the whole value
* `{ name: n }` destructures the same value
* no duplication or copying occurs

Allowed in:

* Top-level and local bindings
* `?` pattern arms (allowing capture of the matched sub-structure)
* Function clauses 

Example:

```aivi
// u is used to pass the entire record, while id/name are used for interpolation
logUser = u@{ id, name } => 
  log u 
  "{id}: {name}"
```

---

## 1.5 Usage Examples

### Config Binding

```aivi
config = {
  host: "localhost"
  port: 8080
  debug: True
}

{ host, port } = config
serverUrl = "http://{host}:{port}" 
```
```

### Tuple Destructuring

```aivi
point = (10, 20)
(x, y) = point

distance = sqrt (x² + y²) // Unicode powers (², ³, etc.) are supported as syntactic sugar for `pow x 2`.
```

// Intermediate keys are automatically bound:
{ data: { user: { name } } } = response 

// Equivalent to:
// { data } = response
// { user } = data
// { name } = user

// name = "Alice"
// user = { id: 1, name: "Alice" }
// data = { ... }


> [!NOTE]
> Deep exposure significantly reduces the need for multiple destructuring lines when you need both a record and its fields.

### List Head/Tail

```aivi
numbers = [1, 2, 3, 4, 5]
[first, second, ...rest] = numbers

// first = 1, second = 2, rest = [3, 4, 5]
```

### Function Definitions

```aivi
// Named function
greet = name => "Hello, {name}!" or greet = "Hello, _!"

// Multi-argument
add = x y => x + y

// With type annotation
multiply : Int -> Int -> Int
multiply = a b => a * b
```
