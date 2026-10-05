# Rust Core / Security Kernel — Specialization Contract

**Status:** Accepted for the current non-cryptographic kernel boundary  
**Date:** 2026-10-05  
**Scope:** Rust Core / Security Kernel only

## Authority

The Security Kernel is the Layer C security boundary defined by `docs/03-architecture/PLATFORMS.md`.

This contract does **not** approve a cryptographic protocol, key hierarchy, serialization format, transport, secure-storage implementation, or platform binding. Those remain controlled by their respective decision records.

## Trusted Computing Base

The current kernel owns only deterministic policy and state transitions:

- device trust-state transitions;
- authenticated-session state transitions;
- protocol-version bounds and downgrade rejection;
- capability authorization;
- structural encrypted-envelope validation;
- fail-closed error semantics.

Cryptographic verification, private-key custody, transport, persistence, UI, and platform lifecycle remain outside this crate.

## Security invariants

1. Unknown or pending trust cannot authorize.
2. Trust elevation cannot be performed by a public caller.
3. Session establishment requires verified trusted state.
4. Rekey is legal only for an established trusted session.
5. Rekey completion requires an internal/test verification seam until real cryptographic rekey proof is integrated.
6. An incomplete rekey fails closed by closing the session.
7. Authentication cancellation returns only to untrusted/idle state and never grants trust.
8. Revocation closes the session and is terminal for the context.
6. Replaced devices cannot authorize.
7. Protocol negotiation is bounded by configured minimum/maximum and current implementation version.
11. A rejected protocol offer cannot mutate state.
12. Identifier and payload bounds are enforced before acceptance.
13. Raw private key material is not represented by the public kernel API.
14. Unsafe Rust is forbidden.
15. Platform bindings must not expose trust elevation or policy bypass.
16. Authority-bearing state must not implement implicit value-copy semantics that can create stale independent security authority.

## API boundary rules

Public constructors must return validated domain values where construction invariants exist.

Security-sensitive state is private to the crate and exposed through read-only accessors or guarded transitions.

Authority-bearing values must not implement `Copy` or `Clone` unless a future design proves that duplicated values remain references to one canonical authority and cannot outlive or bypass revocation.

The following are intentionally unavailable to external callers until a real verifier exists:

- direct trust promotion;
- direct insertion of authentication success;
- direct rekey completion;
- direct mutation of negotiated protocol state;
- direct mutation of device trust state.

## Non-goals

This specialization does not claim:

- end-to-end cryptographic confidentiality;
- forward secrecy;
- post-compromise security;
- authenticated key exchange;
- secure hardware-backed key storage;
- protocol interoperability;
- production P2P transport security.

Those claims require accepted protocol/key-management decisions and independent verification.

## Exit condition for cryptographic implementation

A cryptographic implementation may enter this crate only after:

1. protocol ADR accepted;
2. key-management ADR accepted;
3. canonical serialization accepted where required;
4. threat-model update completed;
5. primitive/library due diligence recorded;
6. conformance vectors and negative tests defined;
7. independent security review scheduled and evidenced;
8. FFI contract updated without expanding the TCB unnecessarily.
