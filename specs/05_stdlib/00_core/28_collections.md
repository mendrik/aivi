# Collections Domain

The `Collections` domain expands the core data structures beyond `List` and `Vector`.

## Overview

```aivi
import aivi.std.collections use { Map, Set, HashMap }

let scores = Map.empty() |> Map.insert("Alice", 100)
```

## Goals for v1.0

- **Map/Dict**: Persistent ordered maps (AVL or Red-Black Tree) and/or HashMaps (HAMT).
- **Set**: Persistent sets corresponding to map types.
- **Queue/Deque**: Efficient FIFO/LIFO structures.
- **Heap/PriorityQueue**.
