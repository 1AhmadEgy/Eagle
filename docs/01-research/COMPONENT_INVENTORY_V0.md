# Eagle — Evidence & Component Inventory v0

**Audit date:** 2026-10-04  
**Base:** main @ 6c46bf53fce06ee7f7b5b2bf35c720ab5bcb7dee  
**Status:** Evidence audit in progress  
**Rule:** `Implemented` ≠ `Integrated` ≠ `Planned`

## 1. Repository reality

| Area | Status | Evidence | Decision |
|---|---|---|---|
| Android application | Implemented | `app/`, `MainActivity.kt`, Android manifest | KEEP / expand incrementally |
| Android build | Implemented | AGP 9.4.0, compile/target 37, min 29 | KEEP provisionally; verify compatibility before upgrades |
| Unit test harness | Implemented | JUnit 4.13.2 + `SmokeTest.kt` | KEEP, expand |
| Real crypto implementation | Not implemented | No crypto/Rust source or crypto dependency in current tree | BUILD/INTEGRATE only after protocol decision |
| Device identity | Not implemented | No identity implementation in `app/` | BUILD behind approved security boundary |
| E2E messaging protocol | Planned | ADR-0008 is Proposed/Pending | DO NOT IMPLEMENT custom protocol |
| Key management | Planned | ADR-0009 is Proposed/Pending | USE Android Keystore as platform primitive; exact model pending |
| Rust Security Core | Planned | PLATFORMS.md describes target architecture | DO NOT claim implemented |
| KMP Shared Layer | Planned | PLATFORMS.md describes target architecture | Introduce only after contracts are frozen |
| Mesh | Planned | No mesh implementation in current tree | Reuse/adapt proven networking; do not invent routing initially |
| Storage | Planned | ADR-0011 worklist; no storage dependency currently | Select after data/security model |
| Production threat model | Pending | SECURITY-BASELINE.md says product threat model requires authoritative architecture | BLOCK production security claims |
| CI verification | Implemented | workflows + scripts/ci/verify.sh | KEEP; strengthen for Android/Rust when added |

## 2. Current dependency baseline

The current Android module declares only:
- JUnit 4.13.2 for tests.

There is currently no libsignal, OpenMLS, libp2p, SQLCipher, Room, Rust crate, KMP module, or crypto library integrated into Eagle.

Therefore all such components are **external candidates**, not Eagle dependencies.

## 3. External component decisions — v0

| Component | Layer | Evidence | Eagle state | Decision | Rationale |
|---|---|---|---|---|---|
| Android Keystore / KeyMint | Device key protection | Official Android security documentation | External platform capability | USE | Mature platform primitive; non-exportable keys; hardware-backed/StrongBox when available |
| libsignal | 1:1 E2EE | Signal's official libsignal repository | External candidate | STUDY / WRAP ONLY AFTER LICENSE & SUPPORT REVIEW | Implements Signal protocol/Double Ratchet and is used by Signal clients, but upstream explicitly says use outside Signal is unsupported |
| OpenMLS | Group E2EE | OpenMLS official repository, RFC 9420 implementation | External candidate | DEFER / STUDY | Strong group protocol implementation; unnecessary complexity for initial 1:1 slice |
| rust-libp2p | P2P transport/discovery | Official libp2p Rust repository | External candidate | ADAPT LATER | Mature networking stack with transports/protocols, but it is not itself Eagle's BLE mesh application and adds substantial complexity |
| bitchat-android | BLE mesh reference | Public Android implementation with BLE mesh + Noise + tests | Reference project | STUDY / REUSE PATTERNS | Strong evidence for practical Android BLE mesh architecture; not adopted as a security dependency |
| Room | Android persistence | Official AndroidX documentation | External candidate | USE IF ANDROID-LOCAL STORAGE | Mature SQLite abstraction; does not by itself provide database encryption |
| SQLCipher | Encrypted SQLite | Official SQLCipher repository | External candidate | CONDITIONAL USE | Proven encrypted SQLite option; requires careful key lifecycle and platform/build integration |
| Kotlin Multiplatform | Shared application layer | Official Kotlin documentation | Planned architecture | USE LATER | Good fit for shared non-security application logic; security authority should remain outside UI/shared business code |
| Compose Multiplatform | Cross-platform UI | Official Kotlin documentation | Planned | DEFER | UI is not the current blocker; avoid premature platform expansion |
| Noise | Session/handshake framework | Referenced by Eagle ADR-0008 and existing mesh projects | Candidate | STUDY | Useful for transport/session security, but Eagle must not confuse a handshake framework with a complete async messaging protocol |
| Nostr | Relay transport | Present in reference mesh projects | Candidate | OPTIONAL / NOT CORE | Useful internet relay option but not required for the first Eagle security slice |

## 4. Important negative findings

### 4.1 No custom crypto
No cryptographic primitive should be implemented in Eagle.

### 4.2 No custom messaging protocol yet
ADR-0008 explicitly remains pending and prohibits implementing custom cryptographic protocol logic from the planning draft.

### 4.3 No mesh implementation
The current repository does not contain a working mesh engine. Mesh remains Planned.

### 4.4 No Rust implementation
Rust is an architectural target, not an implemented dependency in the current repository.

### 4.5 No KMP implementation
KMP is an architectural target, not an implemented module.

## 5. Recommended architecture after this audit

### Security authority
**Rust security core (planned)**

Responsibilities:
- protocol/session state;
- cryptographic operations;
- identity operations that require the security boundary;
- serialization of security-sensitive envelopes;
- zeroization/lifecycle rules.

### Android adapter
Responsibilities:
- Android Keystore/KeyMint integration;
- permissions;
- BLE/Wi-Fi/foreground-service lifecycle;
- OS storage integration;
- UI.

### Shared application layer
KMP should own:
- domain models;
- message state;
- synchronization orchestration;
- platform-neutral application contracts.

It must not become a second crypto implementation.

### Network layer
Start with a transport abstraction and a single reliable transport for the first vertical slice. Add BLE mesh only after the security envelope and message lifecycle are proven.

## 6. First vertical slice

The first executable slice is:

`Identity → Key Establishment → Encrypt → Envelope → Transport → Decrypt → Verify`

Acceptance evidence:
1. two independently generated identities;
2. authenticated peer binding;
3. no private-key export across the security boundary;
4. encrypt/decrypt round trip;
5. tamper detection;
6. replay/duplicate handling defined and tested;
7. deterministic protocol fixtures;
8. negative tests;
9. Android integration test;
10. reproducible evidence artifact.

## 7. Second slice

After Slice 1:

`Encrypted message → durable outbox → retry → inbound queue → deduplication → delivery state`

Only then:

`Discovery → Relay → BLE/Wi-Fi mesh`

## 8. Evidence grades

- **E0** — mentioned only.
- **E1** — documented/reference.
- **E2** — source exists and builds/tests independently.
- **E3** — integrated into Eagle and tested.
- **E4** — security/release evidence established.

Current Eagle core product components are mostly E0/E1. The Android bootstrap is E2-level infrastructure, not an E2E product.

## 9. Filebin intake

The supplied Filebin bin was verified as containing 8 files totaling about 7.3 MB, including:
- Download.zip
- Eagle Developer Execution Packs v2/v3/v4/V6
- Eagle Final Documentation Updated
- Eagle PrivateMesh Final Audit Package

The binary download endpoint could not be retrieved by the current execution environment. Therefore these files are **Pending external artifact intake**, not treated as reviewed evidence. Existing GitHub archive material can be used as a secondary source, but it must not be conflated with the Filebin originals.

## 10. Gate before integration

No external component becomes an Eagle dependency until:

`Requirement → Architecture Fit → Security History → Exact Version/Commit → License → Transitive Dependencies → Test Evidence → Operational Fit → Exit Strategy → Approval`

## 11. Immediate next actions

1. Resolve the exact E2E protocol implementation decision.
2. Define the security boundary and key lifecycle.
3. Create a minimal Rust workspace only after those contracts are frozen.
4. Evaluate libsignal integration/support constraints before using it.
5. Define the transport-neutral encrypted envelope.
6. Build the two-peer in-memory transport test before real networking.
7. Add Android Keystore adapter tests.
8. Add real cryptographic/protocol test categories to Test Lab.
9. Keep main unchanged; all implementation work proceeds through branches and PRs.

**Conclusion:** Eagle is currently a governed Android foundation with substantial architecture/security documentation, not yet a working private-mesh messenger. The highest-value reuse path is to consume mature primitives and reference implementations while keeping Eagle's security boundary and protocol contracts explicit.
