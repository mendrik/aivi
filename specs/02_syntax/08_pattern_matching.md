# Pattern Matching

## 8.1 `?` branching

```aivi
classify = _ ?
  | 0 => "zero"
  | _ => "nonzero"
```

This is a concise way to do case analysis, similar to `match` in Rust or `case` in Haskell/Elixir.

---

## 8.2 Multi-clause functions

```aivi
sum =
  | [] => 0
  | [h, ...t] => h + sum t
```

---

## 8.3 Record Patterns

```aivi
greet = _ ?
  | { role: Admin, name } => "Welcome back, Admin {name}!"
  | { role: Guest } => "Welcome, guest!"
  | { name } => "Hello, {name}!"
```

---

## 8.4 Nested Patterns

```aivi
processResult = _ ?
  | Ok { data: { users: [first, ...] } } => "First user: {first.name}"
  | Ok { data: { users: [] } } => "No users found"
  | Err { code: 404 } => "Not found"
  | Err { code, message } => "Error {code}: {message}"
```

---

## 8.5 Guards

Patterns can have guards using `when`:

```aivi
classify = n ?
  | _ when n < 0 => "negative"
  | 0 => "zero"
  | _ when n < 10 => "small"
  | _ when n < 100 => "medium"
  | _ => "large"
```

---

## 8.6 Usage Examples

### Option Handling

```aivi
Option A = None | Some A

getOrDefault = default => _ ?
  | None => default
  | Some value => value

userName = user.nickname |> getOrDefault "Anonymous"
```

### Result Processing

```aivi
Result E A = Err E | Ok A

handleResult = _ ?
  | Ok data => processData data
  | Err e => logError e

// With chaining
fetchUser id
  |> handleResult
  |> renderView
```

### List Processing

```aivi
// Safe head
head = _ ?
  | [] => None
  | [x, ...] => Some x

// Take first n
take = n => _ ?
  | _ when n <= 0 => []
  | [] => []
  | [x, ...xs] => [x, ...take (n - 1) xs]

// Zip two lists
zip =
  | ([], _) => []
  | (_, []) => []
  | ([x, ...xs], [y, ...ys]) => [(x, y), ...zip xs ys]
```

### Tree Traversal

```aivi
Tree A = Leaf A | Node (Tree A) (Tree A)

depth = _ ?
  | Leaf _ => 1
  | Node left right => 1 + max (depth left) (depth right)

flatten = _ ?
  | Leaf x => [x]
  | Node left right => flatten left ++ flatten right
```

### Expression Evaluation

```aivi
Expr = Num Int | Add Expr Expr | Mul Expr Expr

eval = _ ?
  | Num n => n
  | Add a b => eval a + eval b
  | Mul a b => eval a * eval b

// (2 + 3) * 4 = 20
expr = Mul (Add (Num 2) (Num 3)) (Num 4)
result = eval expr
```
## 8.7 Expressive Pattern Orchestration

Pattern matching excels at simplifying complex conditional branches into readable declarations.

### Deeply Nested Destructuring
```aivi
// Extract deeply buried data with fallback
headerLabel = response ?
  | { data: { user: { profile: { name } } } } => name
  | { data: { guest: True } }               => "Guest"
  | _                                       => "Unknown"
```

### Concise State Machines
```aivi
// Update application state based on event
nextState = (state, event) => (state, event) ?
  | (Idle, Start)    => Running
  | (Running, Pause) => Paused
  | (Paused, Resume) => Running
  | (Running, Stop)  => Idle
  | _                => state // Unchanged on invalid events
```

### Expressive Logic Branches
```aivi
// Business rule mapping
discount = user ?
  | { tier: Gold, age } when age > 65 => 0.3
  | { tier: Gold }                    => 0.2
  | { tier: Silver }                  => 0.1
  | _                                 => 0.0
```
