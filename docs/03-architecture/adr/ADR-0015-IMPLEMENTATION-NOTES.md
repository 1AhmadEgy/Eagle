# ADR-0015 Implementation Notes

The Rust-to-Kotlin bridge is kept separate from the shared domain layer.

- `core/ffi` owns the bridge.
- generated bindings are build artifacts.
- `commonMain` uses the small EagleCore contract.
- platform adapters remain separate from shared application logic.
