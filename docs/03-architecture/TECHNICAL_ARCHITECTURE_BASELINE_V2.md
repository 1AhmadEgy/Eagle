# Eagle — Technical Architecture Baseline V2

**Role:** Architecture / Technical Lead  
**Date:** 2026-10-05  
**Status:** Implementation baseline prepared; human approval gates remain authoritative.  
**Canonical repository baseline:** `main` at `abfc263e6ac28ff7b19a40a4d8e1da93c565a6e9`

## 1. Architectural authority

The authority order is:

1. explicit V1 product requirements once durably sourced and approved;
2. Accepted ADRs;
3. current `main` repository state;
4. reviewed PRs as candidate changes only;
5. historical archives as supporting evidence only;
6. ChatGPT conversations as working context, never canonical authority.

A Proposed ADR does not authorize implementation of the undecided technical choice.

## 2. Target architecture

Eagle is a local-first, P2P-only multi-platform messaging system.

```text
Platform UI
   │
   ▼
KMP Shared Application Layer
   │
   ├── Domain / Messaging / Sync
   │
   ▼
Security API
   │
   ▼
Rust Security Core
   │
   ├── Identity / Trust
   ├── Key-use boundary
   ├── Session security
   └── Security validation
   │
   ├───────────────┬─────────────────┐
   ▼               ▼                 ▼
Protocol       Secure Storage      Platform Adapters
   │               │                 │
   ▼               ▼          Android / Desktop / iOS
Opaque frames   Encrypted records
   │
   ▼
P2P transport / peer
```

No central message broker, server-side decryption path, or centralized messaging database is part of this baseline.

## 3. Trust boundaries

### UI
May handle user-visible authenticated application state.  
Must not access private keys, crypto internals, database internals, or raw transport state.

### KMP Shared Layer
Owns reusable domain/application behavior.  
Must remain platform-neutral and call security through narrow interfaces.

### Rust Security Core
Security authority for trust-sensitive state, key use, security policy enforcement, and cryptographic operation boundaries.

### Protocol
Handles encrypted envelopes, framing, versioning and parsing contracts.  
Must not receive application plaintext.

### Mesh/Transport
Handles discovery, connectivity, routing, retries and opaque frames.  
Must not require application plaintext.

### Storage
Persists opaque encrypted records plus bounded non-secret lifecycle metadata.  
Must not represent private keys or live key handles as ordinary application records.

### Platform adapters
Implement OS-specific capabilities behind interfaces.  
Must not duplicate domain or security-critical logic.

## 4. Dependency contract

Allowed direction:

```text
platform → adapters → KMP/shared
KMP/shared → contracts
security API → Rust Security Core
messaging → protocol
messaging/sync → transport/storage interfaces
verification → audit/test every sensitive layer
observability → sanitized contracts
```

Forbidden:

- UI → private keys
- UI → crypto internals
- UI → database internals
- shared → Android/iOS/Desktop framework internals
- Mesh → plaintext
- Protocol → application plaintext
- Storage → private keys
- Storage → UI
- Identity → UI
- AI → Security Core implementation
- Crypto/Security ↔ Mesh circular dependency

## 5. P2P security model

Network connectivity never implies identity trust.

Each device has an independent device identity and trust lifecycle. Pairing is an authenticated security ceremony, not a transport event.

The P2P layer may forward encrypted/opaque packets without possessing the decryption authority.

Offline revocation and stale-trust handling must fail closed before privileged operations.

## 6. Message lifecycle

Send:

```text
plaintext
 → application/domain
 → Security API
 → encrypted envelope
 → protocol validation
 → opaque frame
 → P2P transport
```

Receive:

```text
opaque frame
 → transport validation
 → protocol validation
 → Security API
 → authenticated plaintext
 → application/domain
 → UI
```

No component between Security API boundaries may be given unrestricted plaintext access.

## 7. Cryptographic architecture constraint

The architecture permits only an established, independently reviewed cryptographic implementation path.

No custom cryptographic primitive, bespoke ratchet, or ad-hoc key agreement is permitted.

Repository evidence currently records PQXDH + Double Ratchet as the reference cryptographic direction, while exact implementation/dependency approval, interoperability, and independent cryptographic review remain release gates.

## 8. Key-management constraint

Private keys remain inside the security/key-management boundary.

Platform secure-key facilities are adapters, not application storage.

Key lifecycle must explicitly cover:

- generation;
- purpose separation;
- rotation/rekey;
- revocation;
- destruction;
- prekey single-use semantics where applicable;
- recovery without fabricated trust state;
- migration/upgrade compatibility.

## 9. Storage constraint

The pre-crypto storage contract is:

```text
record_id
owner_id
schema_version
ciphertext
created_at
```

Lifecycle states:

```text
Healthy → RecoveryRequired / Unavailable
```

RecoveryRequired and Unavailable are security-relevant fail-closed states. The storage layer must never silently recreate missing security state or fall back to an alternate unapproved store.

## 10. Platform strategy

Phase 1:
- Android: primary validation target; current Gradle module is `:app`.
- Desktop: parallel shared-layer target.

Phase 2:
- iOS: after shared/security boundary stabilization.

Deferred:
- Web/Wasm: explicit later readiness decision.

Any active documentation that calls the current Android module `androidApp/` is stale and must use `app/` for the verified repository state.

## 11. Migration strategy

Migration is incremental:

1. architecture/dependency contract;
2. Security API;
3. Rust/FFI boundary;
4. minimal KMP shared module;
5. Android composition-root migration;
6. identity slice;
7. messaging/protocol slice;
8. storage/sync slice;
9. desktop;
10. iOS readiness and implementation;
11. Web readiness only after explicit approval.

Do not create the entire target module tree in one change.

## 12. AI boundary

AI remains optional and non-authoritative.

```text
AI Gateway
  → Policy Guard
  → Application Service Contracts
  → Security/Domain authority
```

AI cannot mutate trust, access private keys, bypass authorization, access raw storage internals, or change security policy.

## 13. Non-negotiable invariants

1. P2P-only messaging architecture.
2. No plaintext in mesh transport.
3. No private keys in UI/shared presentation.
4. No private keys as ordinary storage records.
5. No implicit trust from transport connectivity.
6. No downgrade acceptance.
7. No security-state fabrication after failure.
8. No custom cryptographic primitives.
9. No unapproved ADR promoted to an implementation mandate.
10. Missing verification categories remain PENDING, never PASS.
11. Security-sensitive changes require dual independent human review.
12. Supported platforms consume the same shared/security contracts.

## 14. Current architecture disposition

**Repository baseline:** verified on current `main`.

**Architecture candidate:** this document plus the existing dependency/platform records.

**Security implementation candidates:** specialized PRs #65 and #69 and the cryptography/key-management continuation PRs; none are merged into `main` at this time.

**Release posture:** BLOCKED until requirements, ADR approvals, executable cross-boundary evidence, independent security review, and platform/release gates are satisfied.
