# Generators

## 7.1 Concept

Generators are **pure, pull-based sequence producers**.

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

```aivi
generate {
  for x in xs
  when price > 80
  yield x
}
```

Predicate rules are identical to `filter`.

---

## 7.6 Async Generators

Async generators combine production with asynchronous effects.

```aivi
stream = generate async {
  for url in urls
  data = http.get url
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
  for x in [0..width]
  for y in [0..height]
  yield (x, y)
}
```

### Complex Filtering and Transformation
```aivi
// Find active premium users with valid emails
processed = generate {
  for u in users
  when u.active
  when u.tier == Premium
  when isValidEmail u.email
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
