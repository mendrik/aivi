# Bindings, blocks, and shadowing

| Surface | Desugaring |
| :--- | :--- |
| `x = e` (top-level) | kernel `let rec x = ⟦e⟧ in …` (module elaboration; module-level bindings are recursive by default) |
| block: `f = a => b1 b2 b3` | `f = a => let _ = ⟦b1⟧ in let _ = ⟦b2⟧ in ⟦b3⟧` if `b1,b2` are effectless statements; if they are bindings, see next rows |
| block binding: `x = e` inside block | `let x = ⟦e⟧ in …` |
| shadowing: `x = 1; x = x + 1` | `let x = 1 in let x = x + 1 in …` |
