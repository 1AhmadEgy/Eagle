# ADR-0015 — Rust FFI Boundary + KMP Application Layer

Status: Proposed

Purpose: define the architectural boundary between the Rust security core and the shared Kotlin application layer.

This record is a boundary decision only. It does not select cryptographic algorithms, key-management mechanisms, serialization formats, storage backends, or transport implementations.

The application layer may coordinate security operations, but it must not bypass the Rust security boundary or expose private keys and crypto internals to UI code.

The current repository uses `core/ffi`, generated language bindings, and KMP `commonMain` as an illustrative reference implementation of this boundary. These names are illustrative, not normative; later implementation changes may relocate them without changing the architectural decision.

Concrete implementation choices remain deferred to their governing decisions and implementation reviews.
