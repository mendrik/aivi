# Golden Testing Guidelines

- Store inputs next to expected outputs.
- Normalize:
    - paths
    - line endings
    - nondeterministic IDs
- Prefer structured snapshots (JSON) for diagnostics.
- Keep snapshots small and focused.
- Require `--accept` style workflow to update goldens intentionally.
