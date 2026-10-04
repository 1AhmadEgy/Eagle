# Eagle Protocol Security Profile — 2026-10-05

Status: PROPOSED / SECURITY BASELINE — NOT ACCEPTED
Scope: Protocol engineering only.
Hard constraint: P2P only; no message relay service is part of the canonical transport architecture.

## 1. Security target
Eagle must not implement a new cryptographic messaging protocol.
The protocol layer is limited to authenticated metadata, version/capability negotiation, replay/idempotency/order/expiry rules, bounded framing, opaque ciphertext envelopes, deterministic wire representation, and conformance/negative/fuzz/interoperability evidence.

## 2. Preferred cryptographic composition

### 2.1 One-to-one / device-to-device
Preferred candidate: Signal protocol family using PQXDH for asynchronous session establishment and Double Ratchet / Triple Ratchet for ongoing message-key evolution.
Rationale: asynchronous first contact is a first-class requirement; the Double Ratchet provides fine-grained forward security and post-compromise recovery behavior; the current Signal specifications define a hybrid Triple Ratchet construction combining classical and post-quantum ratcheting.
No custom cryptographic reimplementation is approved.

### 2.2 Group messaging
Preferred candidate: MLS (RFC 9420), with a maintained Rust implementation such as OpenMLS, subject to exact-version security review.
Rationale: MLS is an IETF Standards Track protocol designed for asynchronous group key establishment and forward secrecy/post-compromise security for groups from two to thousands.

## 3. Library policy
Signal/libsignal provides Rust-based protocol implementations with Java, Swift, and TypeScript bindings used by Signal clients, but upstream states that use outside Signal is unsupported and APIs may change. Therefore any adoption requires exact revision pinning, provenance, licensing review, reproducible-build evidence, API-compatibility checks, and an isolated security-sensitive dependency boundary.
OpenMLS is a Rust implementation of RFC 9420 with interoperability/test infrastructure and multiple supported cipher suites. Its current security history includes published 2025–2026 vulnerabilities/advisories. Eagle must not approve a version until advisory status, patch level, fuzzing, regression tests, interoperability, and independent security review are all closed.

## 4. Eagle wire profile
Minimum conceptual envelope:
- profile_id
- protocol_version
- message_id
- conversation_id
- sender_device_id
- sender_epoch
- recipient_device_id (optional)
- sender_sequence
- created_at
- expires_at
- capability_set_hash / transcript binding value
- ciphertext
- optional delivery metadata that is never treated as authenticated identity

Rules: application plaintext never enters Mesh; private key material never enters the envelope; ciphertext is opaque to transport; identity/trust is never inferred from network addresses; time fields are advisory unless the accepted session protocol authenticates them.

## 5. Version and capability negotiation
Negotiation MUST be authenticated or cryptographically transcript-bound before it changes security state.
Rules: no silent downgrade; unknown versions fail closed; peer version sets are bounded; selected version is included in the authenticated transcript; security-sensitive capability changes trigger a new authenticated epoch/session state; lower security versions can never replace a higher established version; unsupported mandatory capabilities fail deterministically; optional capabilities are ignored only when explicitly defined as ignorable.
The existing Rust version checks are only local structural guards and are not authenticated negotiation.

## 6. Replay, duplication, ordering, expiry
Acceptance must bind session/trust epoch, sender identity, sequence/message identifier, cryptographic authentication, and local freshness policy.
Duplicate message_id is idempotent and must never execute application processing twice. Reuse of an authenticated sequence with different ciphertext is a protocol error. Out-of-order delivery is bounded. Excessive skipped-message state fails closed. Expired messages are rejected before expensive downstream processing where possible. Messages from revoked/replaced epochs are rejected without mutating the live session state.

## 7. Framing and parser hardening
Unauthenticated parser entry points MUST enforce fixed upper bounds, exact length accounting, bounded nesting/field counts, deterministic rejection, bounded allocation, safe decompression policy, explicit resource-exhaustion errors, and fuzz coverage.
Transport-level fragmentation is preferred. If application fragmentation is required, authenticated state must bind message_id, fragment_count, fragment_index, total_ciphertext_length, and fragment_epoch.

## 8. Serialization recommendation
Preferred candidate: deterministic CBOR for Eagle-owned protocol metadata, subject to ADR-0010 approval.
Reason: IETF Standards Track format, compact binary representation, explicit deterministic encoding rules, and suitability for reproducible transcript-bound bytes.
Eagle must define one deterministic encoding profile and reject non-conforming encodings wherever byte identity matters. Signal/MLS-owned cryptographic messages must not be re-encoded by Eagle application code.

## 9. P2P transport boundary
Canonical transport architecture: direct peer-to-peer connectivity only.
QUIC is the preferred transport candidate. ICE may be used only for direct candidate discovery/connectivity establishment. STUN may be used only for candidate discovery if required. TURN or other relay services are not part of the canonical Eagle architecture under the P2P-only requirement.
Transport failure MUST NOT fall back to a server-mediated message path.

## 10. Required protocol test suite
Positive: current-version valid envelope, authenticated negotiation, session establishment, ratchet send/receive, bounded out-of-order delivery, duplicate idempotence, device add/remove/revoke, version upgrade.
Negative: malformed/truncated/oversized frames, unsupported version, downgrade, capability downgrade, replay, sequence reuse with altered ciphertext, expiry, revoked device, wrong conversation binding, wrong sender/device binding, transcript mismatch, resource exhaustion, fuzzed unauthenticated input.
Cross-platform: Android↔iOS, Android↔Desktop, iOS↔Desktop, Desktop↔Desktop.
Required evidence: deterministic vectors, byte-for-byte serialization vectors, cryptographic vectors where upstream exposes them, interoperability results, fuzz corpus/results, independent review.

## 11. Current implementation assessment
The current Rust protocol contract supplies useful structural guards: bounded identifiers, bounded ciphertext, exact frame length checks, fail-closed local version validation, and no custom cryptographic primitives.
It does not yet establish authenticated negotiation, transcript binding, replay/idempotency semantics, ratchet/session profile, epoch semantics, canonical serialization implementation, fragmentation/reassembly contract, cross-platform interoperability, or production cryptographic integration.
Status: STRUCTURAL BASELINE ONLY.

## 12. Protocol release gate
Protocol becomes APPROVED only when ADR-0008, ADR-0009, ADR-0010, and ADR-0012 are accepted with exact protocol/crypto/key/serialization/P2P transport profiles; exact library revisions are pinned; all required tests pass; Android/iOS/Desktop conformance passes; independent protocol/security review is recorded; and no unresolved critical/high vulnerability applies to selected revisions.
Until then: PROTOCOL GATE = NOT APPROVED / PRODUCTION BLOCKED.

## 13. Evidence basis
Signal Double Ratchet specification; Signal PQXDH specification; current Signal Triple Ratchet material; RFC 9420 MLS; RFC 8949 CBOR; RFC 9000 QUIC; RFC 9221 QUIC DATAGRAM; OpenMLS security/advisory record; Eagle ADR-0008/0009/0010/0012 and the current Rust protocol structural contract.