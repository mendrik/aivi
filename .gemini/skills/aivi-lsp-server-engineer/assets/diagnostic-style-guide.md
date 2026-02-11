# Diagnostic Style Guide (AIVI)

## Structure
- Code: AIVI{area}{number} (e.g., AIVI-TYPE-0001)
- Severity: error | warning | info | hint
- Primary span: the most specific token/range
- Secondary spans: definitions, candidates, conflicting sites
- Message: one sentence, imperative
- Note(s): short bullets
- Fix(es): concrete edits when safe

## Fix patterns
- Prefer “add explicit syntax” over changing meaning.
- Suggest qualification for domain ambiguity.
- Suggest `<|` for deep updates (never allow deep keys in record literals).
- Suggest `x => ...` when `_` placeholder is illegal.
- Insert `_` arm for non-exhaustive match when appropriate.
