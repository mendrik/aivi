# Effects: `effect` block

Kernel effect primitives:

* `pure : A -> Effect E A`
* `bind : Effect E A -> (A -> Effect E B) -> Effect E B`
* `fail : E -> Effect E A`

## `effect { … }`

`effect` is the same pattern but over `Effect` with `bind/pure`:

- Bind:

  ```aivi
  effect {
    x <- e
    body
  }
  ```

  desugars to `bind ⟦e⟧ (λx. ⟦effect { body }⟧)`.

- Pure let-binding:

  ```aivi
  effect {
    x = e
    body
  }
  ```

  desugars to `let x = ⟦e⟧ in ⟦effect { body }⟧`.

- Sequencing an `Effect E Unit` expression:

  ```aivi
  effect {
    e
    body
  }
  ```

  desugars to `bind ⟦e⟧ (λ_. ⟦effect { body }⟧)` (if `e : Effect E Unit`).

- Final expression:

  `effect { e }` desugars to `⟦e⟧` (the final expression must already be an `Effect`).

- Empty block:

  `effect { }` desugars to `pure Unit`.

- No final expression:

  ```aivi
  effect {
    s1
    ...
    sn
  }
  ```

  desugars to `⟦effect { s1 ... sn pure Unit }⟧` (i.e. insert `pure Unit` as the final expression).

If you want to return a pure value from an effect block, write `pure value` as the final expression.

If the surface allows `print` etc as effectful calls, those are already `Effect`-typed; no special desugaring beyond `bind`.

## `or` fallback (surface sugar)

`or` is not a general matcher. It is fallback-only sugar.

- Result fallback:

  `res or rhs` desugars to a match on `res` with an implicit `Ok` passthrough arm.

- Effect fallback (only after `<-` inside `effect {}`):

  `x <- eff or rhs` desugars by inserting `attempt` and matching on `Result`:

  `attempt eff` produces `Effect E (Result E A)`, then `Ok a` becomes `pure a` and `Err e` becomes `pure rhs` (or falls through to `fail e` if no fallback arm matches).

Implementation note (v0.1 parser):

- In `effect { ... }`, a fallback written as `x <- eff or | Err ... => ...` is treated as **Result** fallback syntax to avoid confusion with effect-fallback arms (which match the raw error `E`).
