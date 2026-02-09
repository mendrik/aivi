---
title: AIVI Language Specification
---

# AIVI Language Specification

A high-integrity functional language with a Rust-first compilation pipeline.

## Table of Contents

### Core Specification

1. [Introduction](01_introduction)

### Roadmap

- [Roadmap](roadmap/README.md)

### Syntax

2. [Concrete Syntax (EBNF draft)](02_syntax/00_grammar)
3. [Bindings and Scope](02_syntax/01_bindings)
4. [Functions and Pipes](02_syntax/02_functions)
5. [The Type System](02_syntax/03_types)
6. [Predicates](02_syntax/04_predicates)
7. [Patching Records](02_syntax/05_patching)
8. [Domains, Units, and Deltas](02_syntax/06_domains)
9. [Generators](02_syntax/07_generators)
10. [Pattern Matching](02_syntax/08_pattern_matching)
11. [Effects](02_syntax/09_effects)
12. [Modules](02_syntax/10_modules)
13. [Domain Definitions](02_syntax/11_domain_definition)
14. [External Sources](02_syntax/12_external_sources)
15. [Decorators](02_syntax/14_decorators)
16. [Resources](02_syntax/15_resources)

### Kernel (Core Calculus)

19. [Core Terms](03_kernel/01_core_terms)
20. [Types](03_kernel/02_types)
21. [Records](03_kernel/03_records)
22. [Patterns](03_kernel/04_patterns)
23. [Predicates](03_kernel/05_predicates)
24. [Traversals](03_kernel/06_traversals)
25. [Generators](03_kernel/07_generators)
26. [Effects](03_kernel/08_effects)
27. [Classes](03_kernel/09_classes)
28. [Domains](03_kernel/10_domains)
29. [Patching](03_kernel/11_patching)
30. [Minimality Proof](03_kernel/12_minimality)

### Desugaring (Syntax → Kernel)

31. [Bindings](04_desugaring/01_bindings)
32. [Functions](04_desugaring/02_functions)
33. [Records](04_desugaring/03_records)
34. [Patterns](04_desugaring/04_patterns)
35. [Predicates](04_desugaring/05_predicates)
36. [Generators](04_desugaring/06_generators)
37. [Effects](04_desugaring/07_effects)
38. [Classes](04_desugaring/08_classes)
39. [Domains and Operators](04_desugaring/09_domains)
40. [Patching](04_desugaring/10_patching)

### Standard Library

41. [Prelude](05_stdlib/01_prelude)
42. [Calendar Domain](05_stdlib/02_calendar)
43. [Duration Domain](05_stdlib/03_duration)
44. [Color Domain](05_stdlib/04_color)
45. [Vector Domain](05_stdlib/05_vector)
46. [HTML Domain](05_stdlib/06_html)
47. [Style Domain](05_stdlib/07_style)
48. [SQLite Domain](05_stdlib/08_sqlite)
49. [Matrix Domain](05_stdlib/09_matrix)
50. [Complex Domain](05_stdlib/10_complex)
51. [Quaternion Domain](05_stdlib/11_quaternion)
52. [Rational & BigInt Domains](05_stdlib/12_rational_bigint)
53. [Probability & Distribution Domain](05_stdlib/13_probability)
54. [FFT & Signal Domain](05_stdlib/14_signal)
55. [Geometry Domain](05_stdlib/15_geometry)
56. [Units Domain](05_stdlib/16_units)
57. [Graph Domain](05_stdlib/17_graph)
58. [Linear Algebra Domain](05_stdlib/18_linear_algebra)
59. [HTTP Domain](05_stdlib/19_http)
60. [File Domain](05_stdlib/20_file)
61. [Console Domain](05_stdlib/21_console)
62. [Crypto Domain](05_stdlib/22_crypto)

### Runtime

107. [Concurrency](06_runtime/01_concurrency)
108. [Rustc Native Pipeline](06_runtime/02_rustc_native_pipeline)
