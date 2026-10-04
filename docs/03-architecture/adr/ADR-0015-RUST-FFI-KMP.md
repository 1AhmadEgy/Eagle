# ADR-0015 — Rust FFI Boundary + Kotlin Multiplatform Application Layer

**Status:** Proposed — Decision Pending Review  
**Date:** 2026-10-04  
**Related ADRs:** ADR-0007, ADR-0008, ADR-0009, ADR-0010, ADR-0012, ADR-0013

## Context

Eagle is moving from an Android-first shell toward shared application logic while preserving a narrow security boundary around the Rust core.

The application needs a cross-platform coordination layer for domain state, lifecycle, presentation state, and service orchestration. The security-critical implementation must remain in Rust.

The existing planning baseline does not approve a concrete cryptographic protocol, key-management design, or serialization format. Therefore this ADR establishes the language/module boundary and FFI integration shape, but does not approve any cryptographic implementation.

## Decision

Propose the following module boundary:

- Rust core remains the Security Kernel and owns security decisions.
- Rust core/ffi is the only foreign-function bridge for the security kernel.
- UniFFI proc-macro mode is the bridge mechanism for Kotlin/Swift bindings.
- KMP shared owns platform-neutral application/domain orchestration.
- shared/commonMain depends only on a Kotlin EagleCore port and opaque SessionHandle; generated UniFFI bindings remain platform-specific integration artifacts.
- Android, iOS, and desktop adapters bind the KMP port to generated Rust bindings for their respective native library artifacts.
- Private keys never cross the bridge. Public identity material may cross only when an approved API explicitly requires it.
- The bridge exposes typed errors and opaque handles rather than Rust implementation internals.
- AI and other advisory automation remains outside the Security Boundary.

## Why UniFFI

UniFFI provides generated Kotlin and Swift bindings from Rust definitions and supports async exported functions. Current UniFFI documentation lists Kotlin and Swift as fully supported binding languages. The project pins UniFFI at 0.32.1 for this scaffold; future upgrades require contract regeneration and compatibility review.

## KMP boundary rule

Do not place generated UniFFI code in commonMain. Generated Kotlin bindings are target/runtime-specific, while commonMain must remain platform-neutral. The application depends on a small port so tests can use fakes without linking the Rust library.

## Current scope

This ADR only introduces the contract and build skeleton. The following remain blocked until their governing ADRs are approved:

- real device-key generation/storage
- real handshake/session establishment
- protocol serialization/wire encoding
- concrete cryptographic primitives
- production Android Keystore integration
- transport-specific PrivateMesh implementation

## Rejected alternatives

### Hand-written JNI/C ABI as the default bridge

Rejected as the primary integration path because it creates a larger manually maintained ABI surface and makes ownership, error, and type drift easier to introduce. A low-level ABI may still exist underneath UniFFI as generated implementation detail.

### UniFFI generated code in commonMain

Rejected because generated Kotlin bindings are not the portable KMP domain layer. Keeping them target-specific prevents platform/runtime details from leaking into shared business logic.

### Moving security implementation into Kotlin

Rejected. The Rust core remains the single security-decision boundary.

## Compatibility and rollout constraints

The repository uses Gradle 9.6.0. Current Kotlin documentation lists Kotlin 2.4.20 as stable and documents compatibility with Gradle 7.6.3–9.7.0 and AGP 8.5.2–9.3.1. This change therefore pins KGP 2.4.20 and the Android-KMP library plugin/AGP 9.3.1.

## Approval gate

Before this ADR becomes Accepted:

1. Security review confirms the bridge exposes no private-key material.
2. Architecture review confirms dependency direction and platform isolation.
3. Contract tests and generated-binding checks are reproducible in CI.
4. ADR-0008/0009/0010 decisions are approved before their corresponding bridge methods become functional.
5. FFI provenance and generated artifact integrity are recorded in release evidence.
