# Predicates (Unified Model)

## 4.1 Predicate expressions

Any expression of type `Bool` that uses only:

* literals
* field access
* patterns
* the implicit `_`

is a **predicate expression**.

Examples:

```aivi
price > 80
_.price > 80
email == Some "x"
Some _
Ok { value } when value > 10
```

---

## 4.2 Implicit binding rule

Inside a predicate expression:

* `_` is bound to the **current element**
* bare field names are resolved as `_.field`

> [!TIP]
> `filter active` is shorthand for `filter (_.active)` when `active` is a boolean field. If `active` is bound in scope, it refers to that binding instead.

```aivi
price > 80        // _.price > 80
active            // _.active
```

---

## 4.3 Predicate lifting

Whenever a function expects:

```aivi
A => Bool
```

a predicate expression may be supplied.

> [!NOTE]
> Predicates can also perform complex transformations by deconstructing multiple fields:
> `map { name, id } => if id > 10 then name else "no name"`

Desugaring:

```text
predicateExpr
⇒ (_ => predicateExpr)
```

Applies to:

* `filter`, `find`, `takeWhile`, `dropWhile`
* generator guards (`x -> pred`)
* patch predicates
* user-defined functions

Examples:

```aivi
users |> filter active
users |> filter (age > 18)
users |> find (email == Some "x")
xs |> takeWhile (_ < 10)
```

```aivi
generate {
  u <- users
  u -> active
  yield u
}
```

```aivi
store <= { items[price > 80].discount: 0.1 }
```

```aivi
where : (A => Bool) -> List A -> List A
where pred xs = xs |> filter pred

admins = where (role == Admin) users
activeUsers = where active users
```

---

## 4.4 No automatic lifting in predicates

Predicates do **not** auto-lift over `Option` or `Result`.

```aivi
filter (email == "x")      // ❌ if email : Option Text
filter (email == Some "x") // ✅
```

Reason: predicates affect **cardinality**.
