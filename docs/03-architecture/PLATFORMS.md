# Eagle Platform Strategy

> **Status:** Accepted reference baseline  
> **Scope:** Platform targets, delivery order, and cross-platform architecture boundaries  
> **Related architecture:** ADR-0015 + ADR-0016 + A→E architecture split (working baseline)  
> **Last updated:** 2026-10-06

## 1. Purpose

This document is the canonical platform reference for Eagle.

It defines the target platforms, delivery priority, Rust Security Core boundary, KMP Shared Layer boundary, platform adapters, and constraints on future expansion.

It describes platform scope. It does not replace ADRs that define cryptography, key management, serialization, transport, storage, or other technical decisions.

> **Current-state rule:** repository paths and implementation status in this document describe evidence currently present in the repository. Target architecture is explicitly labelled as such.

## 2. Target platforms

| Phase | Platform | Priority | Current repository state |
|---|---|---:|---|
| Phase 1 | Android | 1 | Skeleton implementation in `app/` |
| Phase 1 | Desktop (Windows / macOS / Linux) | 2 | Planned |
| Phase 2 | iOS | 3 | Planned |
| Deferred | Web | — | Deferred |

The names `desktopApp/`, `iosApp/`, and `webApp/` are target-architecture labels only; they are not claims that those directories currently exist.

## 3. Cross-platform target architecture

The intended dependency direction is:

```
                    Rust Security Core
                 (cross-compile + UniFFI)
                            │
                            ▼
                     KMP Shared Layer
          ┌─────────────────┴─────────────────┐
          │ Domain / Auth / Messaging / Sync │
          │             Presentation          │
          └─────────────────┬─────────────────┘
                            │
                    Platform Adapters
             ┌──────────┬──────────┬──────────┐
             ▼          ▼          ▼          ▼
          Android    Desktop      iOS        Web
                                  (deferred)
```

This is a **target architecture**, not a statement that KMP, UniFFI, iOS, Desktop, or Web are currently implemented in `main`.

## 4. Rust Security Core

The Rust Security Core is intended to be the platform-independent security boundary.

Platform-specific code must not duplicate security-critical primitives in Kotlin, Swift, or UI/application layers unless explicitly approved by an ADR.

The current Rust implementation is present on the dedicated execution branch:

`execution/rust-core-security-kernel-complete-2026-10-05`

That branch is evidence of implementation progress, not evidence that the Rust core is production-complete or already integrated into `main`.

The current kernel still defers cryptographic primitives, key hierarchy/secure storage, canonical serialization, transport, platform keystore integration, UniFFI ABI, and authenticated replay/sequence semantics. Those areas require their own verified implementation gates.

## 5. KMP Shared Layer

KMP is a target architectural boundary and is not yet present in the current `main` implementation.

When introduced, the shared layer is expected to own reusable application behavior such as:

- domain models and contracts;
- authentication orchestration;
- messaging;
- synchronization;
- shared state/presentation where appropriate.

It must remain independent of Android-only lifecycle, storage, notification, permission, and UI APIs.

## 6. Platform adapters

| Platform | Intended technology | Boundary responsibilities |
|---|---|---|
| Android | Kotlin + Compose | lifecycle, permissions, services, notifications, platform storage/integration |
| Desktop | JVM + Compose Multiplatform | windowing, desktop lifecycle, OS integration, packaging |
| iOS | Kotlin/Native + Swift/Compose Multiplatform | Apple lifecycle, signing, native integration, platform services |
| Web | Wasm + Compose Multiplatform | deferred browser/runtime integration |

## 7. Delivery order

1. Reconcile architecture documentation with implementation evidence.
2. Stabilize the Rust security-core contract and security invariants.
3. Implement and verify approved cryptographic/key-management boundaries.
4. Establish the Rust ↔ KMP/UniFFI ABI.
5. Introduce the KMP shared layer.
6. Integrate Android first.
7. Add Desktop after shared-layer stabilization.
8. Add iOS after the shared/security boundary is stable.
9. Keep Web deferred until explicitly approved.

## 8. Architectural invariants

1. Platform UI must not own security-critical primitives.
2. Platform adapters consume shared contracts rather than fork domain behavior.
3. Rust remains the security authority across supported platforms.
4. UniFFI is an integration boundary, not a license to duplicate Rust security logic.
5. Adding a platform must not silently change cryptographic or key-management decisions.
6. Platform-specific dependencies remain behind explicit adapter boundaries.
7. CI must validate each supported native target before release readiness.
8. A platform is not considered supported merely because an application directory exists.

## 9. Relationship to ADRs

Relevant technical ADRs include ADR-0008 through ADR-0015 as applicable.

ADR-0016 records the current Android module-path reconciliation and does not authorize a KMP migration by itself.

## 10. Current baseline

- **Current implementation:** Android skeleton under `app/`.
- **Current Gradle module:** `include(":app")`.
- **KMP:** not implemented in `main`.
- **Rust Security Core:** implementation work exists on a dedicated execution branch; not yet integrated into `main`.
- **UniFFI:** not implemented in `main`.
- **Desktop/iOS:** planned.
- **Web:** deferred.
