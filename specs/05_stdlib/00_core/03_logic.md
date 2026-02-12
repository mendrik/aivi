# Standard Library: Logic (Algebraic Hierarchy)

The `aivi.logic` module defines the standard algebraic hierarchy for AIVI, based on the **Fantasy Land Specification**. These classes provide a universal language for data transformation, equality, and composition.

```aivi
module aivi.logic
```

See also:

- Syntax: classes and instances ([The Type System](../../02_syntax/03_types.md))
- Syntax: effects as monads ([Effects](../../02_syntax/09_effects.md))
- Fantasy Land upstream spec (naming + laws): https://github.com/fantasyland/fantasy-land

## 1. Equality and Ordering

### Setoid
A `Setoid` has an equivalence relation.
```aivi
class Setoid A = {
  equals: A -> A -> Bool
}
```

### Ord
An `Ord` must be a `Setoid` and have a [total](https://en.wikipedia.org/wiki/Total_order) ordering.
```aivi
class Ord A = Setoid A & {
  lte: A -> A -> Bool
}
```

## 2. Monoids and Semigroups

### Semigroup
A `Semigroup` has an associative binary operation.
```aivi
class Semigroup A = {
  concat: A -> A -> A
}
```

### Monoid
A `Monoid` must be a `Semigroup` and have an `empty` value.
```aivi
class Monoid A = Semigroup A & {
  empty: A
}
```

### Group
A `Group` must be a `Monoid` and have an `invert` operation.
```aivi
class Group A = Monoid A & {
  invert: A -> A
}
```

## 3. Categories

### Semigroupoid
```aivi
class Semigroupoid (F * *) = {
  compose: F B C -> F A B -> F A C
}
```

### Category
```aivi
class Category (F * *) = {
  id: F A A
}
```

## 4. Functional Mappings

### Functor
```aivi
class Functor (F *) = {
  map: F A -> (A -> B) -> F B
}
```

### Apply
```aivi
class Apply (F *) = Functor (F *) & {
  ap: F A -> F (A -> B) -> F B
}
```

### Applicative
```aivi
class Applicative (F *) = Apply (F *) & {
  of: A -> F A
}
```

### Chain
```aivi
class Chain (F *) = Apply (F *) & {
  chain: F A -> (A -> F B) -> F B
}
```

### Monad
```aivi
class Monad (M *) = Applicative (M *) & Chain (M *)
```

## 5. Folds and Traversals

### Foldable
```aivi
class Foldable (F *) = {
  reduce: F A -> (B -> A -> B) -> B -> B
}
```

### Traversable
```aivi
class Traversable (T *) = {
  traverse: T A -> (A -> F B) -> F (T B)
}
```

## 6. Higher-Order Mappings

### Bifunctor
```aivi
class Bifunctor (F * *) = {
  bimap: F A B -> (A -> C) -> (B -> D) -> F C D
}
```

### Profunctor
```aivi
class Profunctor (F * *) = {
  promap: F B C -> (A -> B) -> (C -> D) -> F A D
}
```

## Examples

### `Functor` for `Option`

```aivi
use aivi.logic

instance Functor (Option *) = {
  map: opt f =>
    opt ?
      | None => None
      | Some x => Some (f x)
}
```

### `Monoid` for `Text`

```aivi
use aivi.logic
use aivi.text as text

instance Semigroup Text = {
  concat: a b => text.concat [a, b]
}

instance Monoid Text = Semigroup Text & {
  empty: ""
}
```

### `Effect` sequencing is `chain`/`bind`

`effect { ... }` is surface syntax for repeated sequencing (see [Effects](../../02_syntax/09_effects.md)):

```aivi
// Sugar
val = effect {
  x <- fetch
  y <- decode x
  pure y
}

// Desugared shape (conceptually)
val2 = (fetch |> bind) (x => (decode x |> bind) (y => pure y))
```
