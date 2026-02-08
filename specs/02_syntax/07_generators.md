Generators are **pure, pull-based sequence producers**. They are distinct from effects: regular `generate` blocks are purely functional and cannot perform I/O, while `generate async` blocks (see 7.6) are effectful and can await external data.

They:

* do not perform effects
* do not suspend execution stacks
* model finite or infinite data

---

## 7.2 Generator type

```aivi
Generator A
```

---

## 7.3 Generator expressions

```aivi
gen = generate {
  yield 1
  yield 2
  yield 3
}
```

### From Python/JavaScript
Similar to `yield` syntax, but purely functional (no mutable iterator state).

---

## 7.4 Guards and predicates

Generators use a Scala/Haskell-style binder:

* `x <- xs` binds `x` to each element produced by `xs`
* `x = e` is a plain (pure) local binding
* `x -> pred` is a guard (filters `x`); multiple guards may appear

In a guard, `pred` is a predicate expression with the implicit `_` bound to `x` (so bare fields like `active` resolve to `x.active`).

```aivi
generate {
  x <- xs
  x -> price > 80
  yield x
}
```

Predicate rules are identical to `filter`.

---

## 7.6 Async Generators

Async generators combine production with asynchronous effects.

```aivi
stream = generate async {
  url <- urls
  data <- http.get url
  yield data
}
```

### Safety and Cleanup
Async generators are integrated with the AIVI concurrency tree:
- **Cancellation**: If the consumer of an async generator stops (or is cancelled), the generator's current execution point is cancelled.
- **Cleanup**: Like regular effect blocks, `generate async` supports `defer` to ensure resources (like open HTTP connections) are released during early termination.
## 7.7 Expressive Sequence Logic

Generators provide a powerful, declarative way to build complex sequences without intermediate collections or mutation.

### Cartesian Products
```aivi
// Generate all pairs in a grid
grid = generate {
  x <- [0..width]
  y <- [0..height]
  yield (x, y)
}
```

### Complex Filtering and Transformation
```aivi
// Find active premium users with valid emails
processed = generate {
  u <- users
  u -> active && tier == Premium && isValidEmail email
  yield { 
    u.name, 
    u.email: toLower
  }
}
```

### Expressive Infinity
```aivi
// Infinite sequence of Fibonacci numbers
fibs = generate {
  yield 0
  yield 1
  loop (a, b) => {
    next = a + b
    yield next
    recurse (b, next)
  }
}
```
