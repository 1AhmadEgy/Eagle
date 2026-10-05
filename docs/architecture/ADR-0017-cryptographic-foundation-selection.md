# ADR-0017: Cryptographic Foundation and Protocol-Library Selection

- **Status:** Proposed
- **Date:** 2026-10-06
- **Scope:** Rust Security Core cryptographic foundation
- **Related:** Rust Security Core, Trust/Auth, Session, FFI contract, future Wire Protocol
- **Decision owner:** Human project owner after legal/security review
- **Review gate:** Human legal review + independent security review required before production cryptography is enabled

## Context

Eagle's Rust Security Core currently contains the state, trust, session, protocol-version, identity-binding, and fail-closed transition logic, but it does not yet provide production cryptographic primitives or a complete authenticated session protocol.

The current architecture intentionally separates:

1. Rust Security Core as the security authority.
2. A typed UniFFI ABI for platform bindings.
3. A separate Wire Protocol / serialization layer.
4. Platform secure-storage and hardware-backed key custody.
5. The cryptographic protocol that provides authenticated key establishment and secure transport.

This ADR therefore must **not** select a single library as if it were interchangeable across all five layers.

## Decision

### 1. No custom cryptographic algorithms or custom cryptographic protocol

Eagle will not implement AES-GCM, X25519, Ed25519, HKDF, Double Ratchet, MLS, Noise, or similar cryptographic constructions from scratch.

Eagle will use maintained, reviewed implementations of standardized constructions and will keep the protocol state machine separate from the primitive implementations.

### 2. RustCrypto is accepted as the primitive dependency family

RustCrypto is the preferred source for individual cryptographic primitives where the required primitive is supported and the selected crate passes Eagle's dependency, platform, maintenance, and security gates.

Examples include AEAD, hashing/KDF support, signatures, and related low-level types. RustCrypto's published crates use permissive Apache-2.0 OR MIT licensing, and the AES-GCM project documents an external NCC Group audit. These facts do not replace Eagle's own security review.

RustCrypto is **not** considered a complete secure-session protocol.

### 3. libsignal is not adopted at this stage

The current upstream `signalapp/libsignal` repository is licensed under **AGPLv3 / AGPL-3.0-only**.

For Eagle, that licensing model is a release blocker until a qualified human legal review explicitly determines that the intended use, distribution model, linking model, and source obligations are acceptable.

This ADR makes no legal conclusion and does not grant or interpret any license.

Until that review exists:

- libsignal must not be added as a production dependency;
- Eagle must not copy libsignal source into the repository;
- Eagle must not design the architecture around an assumption that libsignal will later be approved.

### 4. OpenMLS is not selected as the general Eagle crypto/session backend

OpenMLS is a Rust implementation of **Messaging Layer Security (MLS), RFC 9420**.

MLS is a protocol designed for secure group messaging and has a different set of state and membership semantics from Eagle's current device-pairing, device-approval, identity-binding, and session-kernel problem.

Therefore OpenMLS is not rejected as cryptographically weak; it is rejected as the wrong abstraction for the current core contract.

OpenMLS may be reconsidered in a future ADR if Eagle explicitly adopts MLS semantics for group messaging.

### 5. The secure-session protocol must be selected independently

The next cryptographic decision is not "which crypto library?" but:

> Which standardized authenticated key-establishment / secure-session protocol best matches Eagle's threat model, device lifecycle, replay model, rotation model, and platform constraints?

Candidate protocol families may include Noise-based designs, HPKE-based constructions, or a purpose-built standard protocol implementation. Eagle must select an existing protocol implementation rather than inventing one.

A current example, `snow`, provides a Rust implementation of the Noise Protocol Framework and is permissively licensed, but its upstream README explicitly states that it has **not received a formal audit**. Therefore it is a research/prototyping candidate, not a production approval by this ADR.

The production protocol decision must be recorded in a separate ADR after threat-model review and independent security assessment.

## Consequences

### Positive

- Prevents the FFI layer from becoming coupled to cryptographic internals.
- Prevents a license decision from being hidden inside an implementation choice.
- Keeps primitives, protocol state machine, wire serialization, and platform key custody independently reviewable.
- Avoids custom cryptographic construction.
- Leaves room for a future Signal-compatible or MLS-compatible subsystem without contaminating the current Rust Security Core contract.

### Negative

- Production cryptography is intentionally gated until the protocol decision is complete.
- More ADRs and security review are required before KMP/UniFFI can expose final cryptographic APIs.
- RustCrypto alone does not provide the complete session security model; Eagle still needs a standardized protocol implementation and a documented threat model.
- Some candidate libraries may later be rejected for audit, maintenance, platform, or licensing reasons.

## Non-goals

This ADR does not define:

- the exact Wire Protocol serialization format;
- the QR payload schema;
- the Trust/Auth user-flow;
- secure-storage backends for Android/iOS/Desktop;
- key hierarchy or rotation semantics;
- the UniFFI ABI;
- an MLS group-messaging architecture;
- a final choice between Noise, HPKE, or another standardized protocol.

## Mandatory security rules

1. No raw private keys cross the UniFFI boundary.
2. No platform UI or lifecycle operations are embedded in the Rust cryptographic API.
3. No runtime algorithm-selection knob is exposed to application code.
4. Protocol version and downgrade policy remain Rust-owned.
5. Authentication must bind the peer identity and authorization context before a session becomes established.
6. Replay protection must be part of the authenticated session state, not an application-side convention.
7. Key material must use explicit zeroization/secure-memory handling where supported by the selected implementation.
8. Hardware-backed key custody remains platform-specific and must not be emulated by ordinary application storage.
9. Missing security tests are **pending**, never PASS.
10. Production promotion requires native-target CI, dependency/license review, negative/security tests, and independent human security review.

## Promotion gates

This ADR may move from **Proposed** only when all of the following exist:

- [ ] Human legal review of all candidate licenses and intended distribution.
- [ ] Eagle threat model for pairing, device approval, authentication, replay, compromise, rotation, and recovery.
- [ ] Separate ADR selecting the secure-session protocol.
- [ ] Dependency lock and provenance record.
- [ ] Cryptographic test vectors for every selected primitive/protocol component.
- [ ] Negative tests for identity mismatch, downgrade, replay, expiry, malformed input, and sequence exhaustion.
- [ ] Fuzz/property tests for parsers and state transitions.
- [ ] Cross-platform verification for all supported native targets.
- [ ] Independent human security review before production enablement.

## Evidence reviewed

- Signal's upstream libsignal repository currently declares AGPLv3 / AGPL-3.0-only licensing.
- OpenMLS currently identifies itself as an RFC 9420 MLS implementation.
- RustCrypto's AES-GCM and signature projects document permissive Apache-2.0 OR MIT licensing; AES-GCM documents an NCC Group audit.
- Snow currently states that it has not received a formal audit.

## References

- Signal libsignal: https://github.com/signalapp/libsignal
- OpenMLS: https://github.com/openmls/openmls
- RustCrypto AEADs: https://github.com/RustCrypto/AEADs
- RustCrypto Signatures: https://github.com/RustCrypto/signatures
- Snow / Noise: https://github.com/mcginty/snow
- RFC 9420 (MLS): https://www.rfc-editor.org/rfc/rfc9420

## Final position

**For Eagle today:**

- **RustCrypto primitives:** APPROVED AS A DEPENDENCY FAMILY, subject to per-crate review.
- **libsignal:** BLOCKED pending human legal review; not an architectural dependency.
- **OpenMLS:** NOT SELECTED for the current device/session core; reconsider only for an explicit MLS use case.
- **Secure-session protocol:** NOT YET SELECTED; must be decided in a separate ADR.
- **Custom cryptography/protocol:** PROHIBITED.

This ADR therefore establishes a safe dependency boundary without prematurely freezing Eagle to a protocol that has not yet been justified by the threat model.
