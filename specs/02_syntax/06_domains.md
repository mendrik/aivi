# Domains, Units, and Deltas

Domains define **semantics**, not values.

```aivi
domain Calendar
domain Duration
domain Color
domain Vector
```

---

## 6.1 Delta literals

Deltas represent **change**, not quantities.

```aivi
1d
1m
1y
3l
90deg
```

Properties:

* deltas are not numbers
* deltas have no arithmetic
* deltas are interpreted by domains

---

## 6.2 Domain-directed operators

Operators have **no intrinsic meaning**.

```aivi
date + 1m
color + 3l
vector1 + vector2
```

Valid only if the domain defines the operation.
## 6.3 Expressive Domain Logic

Domains allow for "semantic arithmetic" where the types ensure that only operations that make sense in that domain are permitted.

### Semantic Time Calculation
```aivi
// Domains handle complex calendars (Leap years, DST) automatically
deadline = now + 2w + 3d
isLate = current_time > deadline

// Interval calculations
remaining = deadline - now // Returns a Duration
```

### Visual and Spatial Logic
```aivi
// Color blending and manipulation
highlight = baseColor + 20l - 10s // Lighter, less saturated
transparent = activeColor <= { alpha: 0.5 }

// Vector arithmetic
velocity = (10, 5) // Inferred as Vector
position2 = position1 + (velocity * 2.0s)
```

### Typed Custom Domains
```aivi
// Financial domain prevents adding different currencies
total = usd 100 + usd 50 // OK
err = usd 100 + eur 50   // Compile-time Error
```
