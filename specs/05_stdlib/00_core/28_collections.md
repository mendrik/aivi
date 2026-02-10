# Collections Domain

The `Collections` domain is your toolbox for structured data. While `List` is great for simple sequences, real-world software needs more. Whether you need to look up users by their ID (Map), keep a list of unique tags (Set), or process tasks by priority (Heap), this domain provides persistent data structures designed for functional code.

## Overview

```aivi
use aivi.collections (Map, Set)

scores = Map.empty()
  |> Map.insert("Alice", 100)
  |> Map.insert("Bob", 95)

if scores |> Map.has("Alice") {
  print("Alice is present")
}
```

## v1.0 Scope

- **Map/Dict**: persistent ordered maps (AVL/Red-Black) and/or HashMaps (HAMT).
- **Set**: persistent sets corresponding to map types.
- **Queue/Deque**: efficient FIFO/LIFO structures.
- **Heap/PriorityQueue**.

## Literals and Merging (v1.0)

Collections introduce sigil-based literals for concise construction. These are domain literals and are validated at compile time.

### Map literal

```aivi
users = ~map{
  "id-1" => { name: "Alice", age: 30 }
  "id-2" => { name: "Bob", age: 25 }
}
```

Rules:
- Entries use `key => value`.
- Keys and values are full expressions.
- `...expr` spreads another map into the literal.
- When duplicate keys exist, the **last** entry wins (right-biased).

```aivi
defaults = ~map{ "theme" => "light", "lang" => "en" }
settings = ~map{ ...defaults, "theme" => "dark" }
```

### Set literal

```aivi
primes = ~set[2, 3, 5, 7, 11]
combined = ~set[...a, ...b]
```

Rules:
- Elements are expressions.
- `...expr` spreads another set.
- Duplicates are removed (set semantics).

### Merge operator

The `Collections` domain provides `++` as a right-biased merge for `Map` and union for `Set`.

```aivi
use aivi.collections (Map, Set, domain Collections)

merged = map1 ++ map2
allTags = set1 ++ set2
```

## Core API (v1.0)

The following functions are required for v1.0 implementations. Exact module layout may vary, but names and behavior should match.

### Map

```aivi
Map.empty : Map k v
Map.size : Map k v -> Int
Map.has : k -> Map k v -> Bool
Map.get : k -> Map k v -> Option v
Map.insert : k -> v -> Map k v -> Map k v
Map.update : k -> (v -> v) -> Map k v -> Map k v
Map.remove : k -> Map k v -> Map k v
Map.map : (v -> v2) -> Map k v -> Map k v2
Map.mapWithKey : (k -> v -> v2) -> Map k v -> Map k v2
Map.keys : Map k v -> List k
Map.values : Map k v -> List v
Map.entries : Map k v -> List (k, v)
Map.fromList : List (k, v) -> Map k v
Map.toList : Map k v -> List (k, v)
Map.union : Map k v -> Map k v -> Map k v
```

Notes:
- `Map.union` is right-biased (keys from the right map override).
- `Map.update` applies only when the key exists; otherwise it is a no-op.

### Set

```aivi
Set.empty : Set a
Set.size : Set a -> Int
Set.has : a -> Set a -> Bool
Set.insert : a -> Set a -> Set a
Set.remove : a -> Set a -> Set a
Set.union : Set a -> Set a -> Set a
Set.intersection : Set a -> Set a -> Set a
Set.difference : Set a -> Set a -> Set a
Set.fromList : List a -> Set a
Set.toList : Set a -> List a
```

### Queue / Deque

```aivi
Queue.empty : Queue a
Queue.enqueue : a -> Queue a -> Queue a
Queue.dequeue : Queue a -> Option (a, Queue a)
Queue.peek : Queue a -> Option a

Deque.empty : Deque a
Deque.pushFront : a -> Deque a -> Deque a
Deque.pushBack : a -> Deque a -> Deque a
Deque.popFront : Deque a -> Option (a, Deque a)
Deque.popBack : Deque a -> Option (a, Deque a)
Deque.peekFront : Deque a -> Option a
Deque.peekBack : Deque a -> Option a
```

### Heap / PriorityQueue

```aivi
Heap.empty : Heap a
Heap.push : a -> Heap a -> Heap a
Heap.popMin : Heap a -> Option (a, Heap a)
Heap.peekMin : Heap a -> Option a
```

`Heap` ordering is determined by `Ord` for the element type.
