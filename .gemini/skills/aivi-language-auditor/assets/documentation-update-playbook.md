# Documentation Update Playbook

## 1) Locate source-of-truth
- Spec section(s)
- Kernel/desugaring section(s)
- Tests that demonstrate behavior

## 2) Update structure
- Keep “Normative” rules separate from “Notes”
- Add “Desugaring” subsections for surface features

## 3) Update examples
- Prefer small, type-directed examples
- Add at least one “error example” with expected diagnostic

## 4) Cross-link
- Link from surface syntax → desugaring → kernel → runtime/codegen notes

## 5) Verify
- Run formatter on code blocks
- Ensure examples match current grammar keywords and constructs
