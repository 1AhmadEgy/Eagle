# Eagle Security Component Due Diligence — 2026-10-05

## Purpose
This record converts current external security-component research into Eagle-specific decisions without treating popularity, latest-version status, or discussion text as authorization to adopt a dependency.

## 1. One-to-one cryptography

### Signal Protocol family
Role: protocol reference / target security properties.
Target profile:
- PQXDH session establishment.
- Double Ratchet message-key evolution.
- authenticated device identity and explicit device lifecycle.
- no custom ratchet and no custom primitive.
Status: architecture baseline; implementation gate remains open.

### libsignal
Observed upstream release: v0.104.0 (2026-10-02).
Evidence:
- Signal uses the project for official Android, iOS, Desktop, and server-side components.
- The upstream README states use outside Signal is unsupported and APIs/implementations may change without notice.
- The repository is AGPL-3.0-only.
Eagle disposition: Reference/conformance source only.
Do not add as a production dependency without an explicit legal/support/API/platform review and a decision that the project's licensing and maintenance posture is acceptable.

### vodozemac
Current Rust documentation identifies the crate as an implementation of Olm and Megolm. Olm provides an asynchronous Double Ratchet with forward secrecy and self-healing properties.
Eagle disposition: Candidate implementation component for ratchet/group-compatible research, not an automatic substitute for the selected Signal/PQXDH profile.
Any use would require a separately authenticated key-establishment design and interoperability proof. Eagle must not invent a new protocol merely to assemble components.

## 2. Group cryptography
### OpenMLS
Observed latest release: v0.9.0 (2026-08-03). The project is implementing MLS and has published security fixes in recent release history.
Eagle disposition: Future group-protocol candidate.
Do not activate group messaging until requirements, ciphersuite, storage, membership semantics, interoperability, and security review are complete.

## 3. Serialization
### minicbor
Observed current release: v2.3.0 (2026-07-23). The crate provides type-directed CBOR encoding/decoding and a non-allocating Decoder; definite-length byte strings are supported, while indefinite-length forms can be rejected by using the strict bytes decoder.
Eagle disposition: approved as the current serialization implementation baseline only. Eagle uses a fixed seven-field CBOR array, bounded inputs, strict decoding, and byte-for-byte re-encoding checks for canonical acceptance. Cross-language interoperability and protocol-level approval remain open.

## 3. Direct P2P transport
### rust-libp2p
Observed latest release: v0.56.0 (2026-06-28). Official libp2p documentation states QUIC uses TLS 1.3 and provides encrypted, multiplexed connections with cryptographic peer identity binding. Direct hole punching is supported. Relay functionality also exists in libp2p.
Eagle disposition: Preferred transport candidate: direct QUIC v1.
Relay/content-forwarding paths are disabled by Eagle policy. Hole punching may be used only to establish a direct path. A failed direct path remains offline.
Required evidence:
- direct-only enforcement;
- NAT/hole-punch tests;
- identity-to-transport binding;
- resource/DoS limits;
- platform support;
- proof that application content cannot traverse relays.

## 4. Android key protection
Android Keystore is the primary platform key boundary.
StrongBox-backed protection is available on supported devices and API level 28+ through the Android Keystore builder API.
Key attestation can provide hardware-backed evidence that a key is protected by a TEE or StrongBox and is not exportable/clonable under the documented model.
Eagle disposition:
- Keystore = required primary storage boundary.
- StrongBox = preferred higher-assurance tier when supported.
- existing-key reuse must fail closed when StrongBox is explicitly required; matching an alias is not sufficient.
- hardware backing must be measured and recorded, not assumed.
- attestation is assurance evidence, not the root of messaging identity.
- biometric invalidation and restore/migration behavior must be tested.

## 5. Selection rule
Component -> exact version -> immutable source/commit or release -> license -> transitive graph -> security history -> configuration -> platform matrix -> tests -> provenance -> upgrade plan -> owner review.

## 6. Security principle
Prefer a smaller trusted-computing base over a larger framework surface.
Use mature libraries for cryptographic primitives and protocols.
Keep Eagle-owned logic limited to orchestration, identity binding, state machines, policy, bounded parsing, persistence boundaries, and verification.

## Current 2026-10-05 research refresh
- `minicbor` v2.3.0 is the current serialization dependency used by the Rust foundation; its published license is Blue Oak Model License 1.0.0, so legal/license acceptance must be explicitly recorded before production dependency approval.
- libsignal v0.104.0 remains the latest observed release; upstream continues to scope support to Signal's own clients and warns APIs/implementations may change.
- OpenMLS v0.9.0 remains the latest observed release; its recent history includes security fixes and stricter storage expectations.
- rust-libp2p v0.57.0 is the current upstream workspace version observed on 2026-10-05; QUIC and hole punching remain viable transport candidates, but Eagle still requires explicit identity binding, DoS controls, and relay exclusion evidence.
- Android Keystore exposes security-level information through KeyInfo and StrongBox-backed status can be queried; Eagle therefore treats an explicit StrongBox request as a hard custody requirement rather than a best-effort hint.

## Status
Due diligence: COMPLETE FOR CURRENT CANDIDATES
Production dependency authorization: PENDING
Independent security review: REQUIRED

## References
https://github.com/signalapp/libsignal
https://github.com/signalapp/libsignal/releases/tag/v0.104.0
https://docs.rs/vodozemac/latest/vodozemac/
https://github.com/matrix-org/vodozemac/releases
https://github.com/openmls/openmls/releases/tag/openmls-v0.9.0
https://github.com/libp2p/rust-libp2p/releases/tag/libp2p-v0.56.0
https://docs.libp2p.io/concepts/transports/quic/
https://docs.libp2p.io/concepts/hole-punching
https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.Builder
https://developer.android.com/identity/digital-credentials/credential-issuer/keystore-attestation