# Eagle Rust ↔ KMP/UniFFI Boundary Contract

> **Status:** Draft contract — implementation gate, not an API guarantee  
> **Date:** 2026-10-06  
> **Scope:** Platform-neutral boundary between the Rust Security Core and future KMP/Swift consumers

## 1. Purpose

This document defines the rules for exposing the Rust Security Core to application platforms.

It intentionally does **not** expose cryptographic primitives directly to Kotlin or Swift. The Rust core remains the security authority.

The contract becomes binding only after the corresponding Rust implementation, tests, UniFFI generation, and platform integration gates pass.

## 2. Current status

The current `main` branch has no Rust core, KMP module, or UniFFI binding.

The execution branch `execution/rust-core-security-kernel-complete-2026-10-05` contains a platform-independent state-machine kernel, but it explicitly defers cryptography, key hierarchy/secure storage, serialization, transport, platform keystore integration, and UniFFI ABI.

Therefore this document defines a **future boundary**, not a claim that these APIs currently exist.

## 3. Boundary principles

1. Rust owns security-critical state transitions and authorization.
2. KMP orchestrates application behavior but cannot manufacture trusted Rust security state.
3. Swift/Android code must not reimplement cryptographic verification performed by Rust.
4. FFI data types must be bounded, explicit, and validated.
5. Errors are typed; null/empty values must not be used as implicit error signaling.
6. Sensitive material must not be serialized into logs or diagnostic strings.
7. Secret/key material must not cross the FFI boundary unless a later security ADR explicitly requires it.
8. Public FFI functions must be deterministic with respect to their documented inputs and Rust-owned state, except where cryptographic randomness or platform-backed secure storage is explicitly part of the operation.
9. No API is considered stable merely because UniFFI can generate bindings for it.

## 4. Proposed API surface

The initial stable boundary should be intentionally small.

### 4.1 Device/session lifecycle

Conceptual operations:

- `create_device(...)`
- `begin_authentication(...)`
- `establish_session(...)`
- `begin_rekey(...)`
- `abort_rekey(...)`
- `close_session(...)`
- `revoke_device(...)`

These names are provisional. The final names must follow the actual Rust domain model and approved ADRs.

### 4.2 Message boundary

Only after the cryptographic and serialization ADRs are implemented:

- `encrypt_message(...)`
- `decrypt_message(...)`
- `verify_envelope(...)`

The API must expose opaque protocol operations rather than raw primitive selection.

For example, the platform layer should not choose an AEAD algorithm or construct a nonce manually.

### 4.3 Identity/key lifecycle

Only after the key-management ADR is implemented:

- `create_identity(...)`
- `get_identity_metadata(...)`
- `rotate_identity(...)`
- `verify_device(...)`

Private keys and other long-lived secrets should remain inside the Rust/platform secure-storage boundary whenever the approved architecture permits.

## 5. Data types

Prefer UniFFI typed records/enums over JSON as the normal ABI.

### Records

Examples:

- `DeviceDescriptor`
- `SessionInfo`
- `MessageEnvelope`
- `ProtocolInfo`

Fields must have explicit size and semantic constraints.

### Enums

Examples:

- `Platform`
- `TrustState`
- `SessionState`
- `Capability`
- `SecurityError`

Unknown future enum values must not be silently interpreted as trusted states.

### Bytes

Binary protocol material should use bounded byte arrays/byte vectors at the FFI boundary.

JSON is permitted only as an explicitly versioned interchange format where an ADR requires it; it is not the default FFI encoding.

## 6. Error model

Every fallible operation returns a typed error.

The conceptual mapping is:

```
Rust Result<T, SecurityError>
             │
             ▼
        UniFFI error
             │
      ┌──────┴──────┐
      ▼             ▼
   Kotlin         Swift
```

Errors must distinguish, where security-relevant:

- unauthorized;
- invalid state transition;
- invalid identity;
- revoked/replaced device;
- protocol downgrade;
- unsupported protocol;
- malformed envelope;
- payload/identifier bounds violation;
- secure-storage failure;
- cryptographic verification failure;
- replay/sequence rejection.

An error must never be represented by a null object, empty byte array, sentinel integer, or human-readable string alone.

## 7. Handle and ownership rules

If UniFFI object interfaces are used, handles must be opaque.

The platform layer must not depend on Rust struct layout, pointer values, or internal enum discriminants.

Rust owns lifecycle-sensitive security state. Platform references are handles to Rust-owned state, not authority in themselves.

A revoked/replaced device or closed session must invalidate the relevant authorization path regardless of whether a platform still holds a valid object reference.

## 8. Serialization/versioning

The FFI ABI version and the wire-protocol version are separate concepts.

- FFI changes are governed by this contract and the UniFFI compatibility policy.
- Wire serialization is governed by the serialization ADR.
- Protocol negotiation is governed by the protocol ADR.

A wire-protocol downgrade must never be silently accepted because an older platform binding is installed.

## 9. Security invariants required before exposure

The following must pass before security-sensitive APIs are exported:

- trust-state transition tests;
- session-state transition tests;
- device revocation/replacement tests;
- identity-mismatch tests;
- protocol downgrade tests;
- malformed/truncated envelope tests;
- payload and identifier bound tests;
- replay/sequence tests;
- cryptographic verification tests;
- key lifecycle tests;
- cross-language ABI tests;
- fuzz/property tests for parser and protocol boundaries.

Missing tests are **PENDING**, never PASS.

## 10. Explicit non-goals

The FFI boundary must not expose:

- raw internal Rust structs;
- unsafe pointers;
- mutable global security state;
- direct algorithm-selection knobs;
- raw private keys by default;
- platform-specific Android/iOS lifecycle APIs;
- UI concerns;
- logging hooks that can receive secret material.

## 11. Promotion gate

This draft can be promoted to an accepted contract only after:

1. Rust core API is reviewed.
2. Cryptography/key-management decisions are accepted.
3. Serialization/wire format is accepted.
4. UniFFI generation succeeds for the supported platforms.
5. Kotlin and Swift integration tests pass.
6. Security and fuzz/property categories are implemented and passing.
7. CI verifies all declared native targets.

Until then, consumers must treat the boundary as unstable.
