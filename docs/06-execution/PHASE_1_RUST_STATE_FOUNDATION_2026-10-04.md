# Eagle — Phase 1 Rust State Foundation

**Date:** 2026-10-04  
**Status:** Implemented foundation; production Security Core remains not cleared.

## What was implemented

- Dependency-free Rust workspace under `core/`.
- Exact Rust toolchain pin: 1.99.0.
- `#![forbid(unsafe_code)]`.
- Explicit trust state machine: Untrusted → Pending → Trusted / Revoked.
- Explicit session state machine: Idle → Authenticated → Closed.
- Capability policy boundary with administrative access denied by default.
- Protocol downgrade protection with no state mutation on rejection.\n- Protocol version is frozen once a session is established.
- Positive and negative unit/integration tests.
- Dedicated Rust CI with format, tests, and Clippy checks.

## What is deliberately NOT implemented

This is not the Eagle cryptographic Security Core.

It contains no:

- cryptographic primitive;
- device key generation;
- Android Keystore integration;
- Signal/PQXDH/Double Ratchet implementation;
- MLS/OpenMLS integration;
- secure envelope encryption;
- replay protection for real messages;
- persistent message storage;
- transport or mesh implementation.

The `from_verified_principal()` API records a principal only after an external authenticator has completed verification. It is not a cryptographic verifier.

## Evidence boundary

The state machine is useful as a controlled security-policy foundation, but it cannot be promoted to product-security PASS. The production security gate remains blocked until the protocol, key lifecycle, conformance vectors, and platform key boundary are implemented and tested.

## Verification

Rust CI verifies:

`cargo fmt --all -- --check`  
`cargo test --workspace --locked`  
`cargo clippy --workspace --all-targets --locked -- -D warnings`

Android CI/Test Lab remains a separate verification track.
