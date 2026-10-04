# Eagle Platform Strategy

> **Status:** Accepted reference baseline  
> **Scope:** Platform targets, delivery order, and cross-platform architecture boundaries  
> **Related architecture:** ADR-0015 + A→E architecture split (working baseline)  
> **Last updated:** 2026-10-04

## 0. Evidence correction — 2026-10-04

This document is a target platform strategy, not proof that Rust Security Core, KMP Shared Layer, UniFFI, or Compose Multiplatform implementation already exists.

The current source tree on ai/reverse-engineering-foundation contains a minimal Android/JVM Kotlin runtime and an implemented deterministic security foundation under app/src/main/java/com/eagle/app/security/.

Accordingly:

- Rust Security Core = Target / candidate, not verified implementation.
- KMP Shared Layer = Target / candidate, not verified implementation.
- Compose Multiplatform = Target / candidate, not verified implementation.
- iOS/Desktop/Web implementations = Target, not verified implementation.
- Android/JVM security foundation = Implemented source; runtime verification still pending.

Architecture documents must not outrank current source/test/build evidence.

## 1. Target platforms

| Phase | Platform | Priority | Status |
|---|---|---:|---|
| Phase 1 | Android | 1 | Current implementation runtime |
| Phase 1 | Desktop (Windows / macOS / Linux) | 2 | Planned |
| Phase 2 | iOS | 3 | Planned |
| Deferred | Web | — | Deferred |

## 2. Current implementation boundary

The current deterministic security slice is:

SessionStateMachine → ReplayGuard → TokenBucket → SecurityEvent → FeatureVector → StatisticalBaseline

and:

Authenticated Message → ReplayGuard → TokenBucket → Security Decision

The current contract extension is:

Identity → Authentication → Session → CryptoBoundary → KeyManagementBoundary → SecureStorageBoundary

These are Kotlin/JVM contracts only. They are not a second platform implementation.

## 3. Platform adapter policy

### Android

Android is the first executable validation environment. Security-critical persistent key material must be isolated behind the key-management adapter.

### iOS

Use Apple Keychain/Key APIs behind the same Eagle key-management contract. Do not fork security semantics.

### Windows

Use Windows CNG/DPAPI behind the same Eagle key/storage contract. Exact API depends on whether material is a long-lived key, user-scoped secret, or opaque encrypted record.

### macOS

Use Apple Keychain/Key APIs behind the same abstraction used by iOS, while retaining OS-specific lifecycle/access-control handling.

### Linux

Use an OS-native secret/key service behind the same abstraction. Exact provider remains a later selection based on supported desktop environments and threat model.

## 4. Technology selection rule

KMP, Rust, UniFFI, Compose Multiplatform, native Kotlin/Swift, or another approach may be selected only after source/build evidence, dependency maturity review, security boundary review, testability, licensing/provenance review, platform coverage, and a reproducible CI/build plan.

No framework is adopted by architecture text alone.

## 5. Validation invariant

Same Input → Same Core Semantics → Same Security Decision

Platform-specific differences must be explicit adapter behavior, not divergent security semantics.

## 6. Current status

**Implemented source:** Android/JVM deterministic security foundation + platform-neutral contract boundaries.

**Runtime verification:** Pending successful execution of bash scripts/ci/verify.sh or equivalent CI evidence.

**Shared technology:** Unselected.

**E2E protocol:** Unselected.

**Native key storage:** Adapter boundaries selected; implementations pending.

**Next execution slice:** protocol integration proof for the leading 1:1 candidate, followed by group-protocol proof, without duplicating cryptographic logic in platform UI/application layers.
