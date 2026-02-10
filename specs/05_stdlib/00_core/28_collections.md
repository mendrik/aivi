# Collections Domain

The `Collections` domain is your toolbox for structured data. While `List` is great for simple sequences, real-world software needs more. Whether you need to look up users by their ID (HashMaps), keep a list of unique tags (Sets), or process tasks by priority (Heaps), this domain provides the optimized, persistent data structures you need to write efficient and semantic code.

## Overview

```aivi
use aivi.std.core.collections (Map, Set)

scores = Map.empty()
    |> Map.insert("Alice", 100)
    |> Map.insert("Bob", 95)

if scores |> Map.has("Alice") {
    print("Alice is present")
}
```

## Goals for v1.0

- **Map/Dict**: Persistent ordered maps (AVL or Red-Black Tree) and/or HashMaps (HAMT).
- **Set**: Persistent sets corresponding to map types.
- **Queue/Deque**: Efficient FIFO/LIFO structures.
- **Heap/PriorityQueue**.
