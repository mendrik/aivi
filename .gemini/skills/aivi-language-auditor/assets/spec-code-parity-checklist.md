# Spec ↔ Code Parity Checklist

## Inventory
- [ ] List implemented surface forms (parser)
- [ ] List implemented elaborations (resolver, typechecker)
- [ ] List implemented desugarings (HIR→Kernel)
- [ ] List implemented runtime/codegen behaviors

## Mismatch classes
- [ ] Documented but unimplemented
- [ ] Implemented but undocumented
- [ ] Implemented with different semantics than spec
- [ ] Diagnostics differ (missing code actions, wrong spans)

## Evidence
- [ ] Link each claim to: spec section + Rust module/function + test

## Outputs
- [ ] Parity report (table or bullet list)
- [ ] Proposed doc patch
- [ ] Proposed code patch outline
- [ ] Tests to lock behavior
