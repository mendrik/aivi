# Record Patching (`<=`)

The `<=` operator applies a **declarative structural patch**.

```aivi
updated = record <= { path: instruction }
```

Patching is:

* immutable
* compositional
* type-checked

---

## 5.1 Path addressing

### Dot paths

```aivi
user.profile.avatar.url
```

### Traversals

```aivi
items[*]
```

### Predicates

```aivi
items[price > 80]
items[id == 1]
```

### Sum-type focus (prisms)

```aivi
Ok.value
Some.val
Circle.radius
```

If the constructor does not match, the value is unchanged.

---

## 5.2 Instructions

| Instruction | Meaning |
| :--- | :--- |
| `value` | Replace or insert |
| `Function` | Transform existing value |
| `:= Function` | Replace with function **as data** |
| `-` | Remove field (shrinks record type) |

---

## 5.3 Replace / insert

```aivi
user2 = user <= {
  name: "Grace"
  profile.avatar.url: "https://img"
}
```

Intermediate records are created if missing.

---

## 5.4 Transform

```aivi
user3 = user <= {
  name: toUpper
  stats.loginCount: _ + 1
}
```

---

## 5.5 Removal

```aivi
user4 = user <= {
  email: -
  preferences.notifications.email: -
}
```

Removal is structural and reflected in the resulting type.

---

## 5.7 Expressive Data Manipulation

Patching allows for very concise updates to deeply nested data structures and collections.

### Deep Collection Updates
```aivi
// Increase prices of all active items in a category
store2 = store <= {
  categories[_.name == "Hardware"]
    .items[_.active]
    .price: _ * 1.1
}
```

### Complex Sum-Type Patching
```aivi
// Move all shapes to the origin
scene2 = scene <= {
  shapes[*]
    .Circle.center: origin
    .Square.origin: origin
}
```

### Record Bulk Update
```aivi
// Set multiple fields based on previous state
user2 = user <= {
  name: toUpper
  status: if admin then SuperUser else Normal
  stats.lastVisit: now
}
```
