# Rust Testing Guidelines (AIVI)

- Unit tests: small, pure, no filesystem unless required.
- Integration tests: invoke CLI binaries; isolate temp dirs.
- Snapshot tests: deterministic normalization step.
- Property tests: seed logged; cap sizes; shrinkable generators.
- Always test both:
    - success path (compiled result)
    - failure path (diagnostic structure + span)
