# Generators

Generators are **pure, pull-based sequence producers**. They are distinct from effects: a `generate { ... }` block is purely functional and cannot perform I/O.

They:

* do not perform effects
* do not suspend execution stacks
* model finite or infinite data


## 7.1 Generator type

```aivi
Generator A
```

## 7.2 Generator expressions

```aivi
gen = generate {
  yield 1
  yield 2
  yield 3
}
```

### From Python/JavaScript
Similar to `yield` syntax, but purely functional (no mutable iterator state).

### From Haskell/Scala (no list comprehension syntax)

AIVI does **not** use Haskell-style list comprehensions like:

```aivi
// Not AIVI syntax
[ x | x <- xs, p x ]
```

Instead, write the equivalent logic with a `generate` block:

```aivi
generate {
  x <- xs
  x -> p x
  yield x
}
```


## 7.3 Guards and predicates

Generators use a Scala/Haskell-style binder:

* `x <- xs` binds `x` to each element produced by `xs`
* `x = e` is a plain (pure) local binding
* `x -> pred` is a guard (filters `x`); multiple guards may appear

In a guard, `pred` is a predicate expression with the implicit `_` bound to `x` (so bare fields like `active` resolve to `x.active`).

This means these are equivalent:

```aivi
u -> isValidEmail email
u -> isValidEmail (_.email)
u -> isValidEmail u.email
```

Note: `.email` is an accessor function (`x => x.email`). It’s useful for `map .email`, but in a predicate position you usually want a value like `email` / `_.email`, not a function.

```aivi
generate {
  x <- xs
  x -> price > 80
  yield x
}
```

Predicate rules are identical to `filter`.


## 7.4 Effectful streaming (future direction)

The v0.1 surface syntax does **not** include `generate async`.

The recommended model is:

- keep `Generator` pure, and
- represent async / I/O-backed streams as an `Effect` that *produces* a generator, or via a dedicated `Stream` type in the standard library.

This aligns with `specs/OPEN_QUESTIONS.md` (“generators should be pure; use `Effect` for async pull”).
## 7.5 Expressive Sequence Logic

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
  yield { name: u.name, email: toLower u.email }
}
```

### Expressive Infinity
```aivi
// Infinite sequence of Fibonacci numbers
fibs = generate {
  loop (a, b) = (0, 1) => {
    yield a
    recurse (b, a + b)
  }
}
```

`loop (pat) = init => { ... }` introduces a local tail-recursive loop for generators.
Inside the loop body, `recurse next` continues with the next state.
