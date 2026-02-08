# Generators

## Generator core encoding

Generator type:

* `Generator A ≡ ∀R. (R -> A -> R) -> R -> R`

Primitive “constructors” as definable macros:

* `genEmpty = ΛR. λk. λz. z`
* `genYield a = ΛR. λk. λz. k z a`
* `genAppend g1 g2 = ΛR. λk. λz. g2 k (g1 k z)`
* `genMap f g = ΛR. λk. λz. g (λacc a. k acc (f a)) z`
* `genFilter p g = ΛR. λk. λz. g (λacc a. case p a of | True -> k acc a | False -> acc) z`

## `generate { … }`

| Surface | Desugaring |
| :--- | :--- |
| `generate { yield e }` | `genYield ⟦e⟧` |
| `generate { s1; s2 }` | `genAppend ⟦gen s1⟧ ⟦gen s2⟧` |
| `generate { x <- g; body }` | `genBind ⟦g⟧ (λx. ⟦generate { body }⟧)` where `genBind g f = ΛR. λk. λz. g (λacc a. (f a) k acc) z` |
| `generate { x -> pred; body }` | `genFilter (λx. ⟦pred⟧[_ := x]) ⟦generate { body }⟧` |
| `generate { loop pat = init => body }` | define local `recurse` and start it: `let rec recurse pat = ⟦generate { body }⟧ in recurse ⟦init⟧` |

## `resource { ... }`

Resources are desugared into `bracket` calls.

| Surface | Desugaring |
| :--- | :--- |
| `resource { setup; yield r; cleanup }` | `Resource { acquire = ⟦setup; pure r⟧, release = λr. ⟦cleanup⟧ }` |

(Note: This is a simplification; the actual desugaring handles the `Resource` type wrapper).
