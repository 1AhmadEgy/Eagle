# Eagle Platform Strategy

> **Status:** Accepted reference baseline  
> **Scope:** Platform targets, delivery order, and cross-platform architecture boundaries  
> **Related architecture:** ADR-0015 + A→E architecture split (working baseline)  
> **Last updated:** 2026-10-04

## 1. Purpose

This document is the canonical platform reference for Eagle.

It defines:

- the target platforms;
- the delivery priority and timing;
- the role of the Rust Security Core;
- the role of the KMP Shared Layer;
- the platform-specific adapter boundary;
- the constraints imposed on future platform expansion.

This document describes platform scope. It does **not** replace ADRs that define cryptography, key management, serialization, transport, storage, or other technical decisions.

## 2. Target platforms

| Phase | Platform | Priority | Status |
|---|---|---:|---|
| Phase 1 | Android | 1 | In progress — `app/` |
| Phase 1 | Desktop (Windows / macOS / Linux) | 2 | Planned — `desktopApp/` |
| Phase 2 | iOS | 3 | Planned — `iosApp/` |
| Deferred | Web | — | Deferred — `webApp/` later |

### 2.1 Delivery intent

**Android is the primary implementation platform.** It provides the first production-oriented validation loop for the shared domain, security boundary, messaging, synchronization, and UI architecture.

**Desktop is developed in parallel with KMP stabilization.** It reuses the shared layer and Compose Multiplatform without introducing a separate application architecture.

**iOS follows the stabilization of the shared layer and Layer C security boundary.** It reuses KMP and the Rust/UniFFI boundary while retaining platform-specific Swift/Xcode integration where required.

**Web is explicitly deferred.** Web support must not force premature changes to the security boundary or constrain ADR-0008/0009/0010.

## 3. Cross-platform architecture

The intended dependency direction is:

```text
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
          ┌────────────┬────────────┬────────────┐
          ▼            ▼            ▼            ▼
       Android      Desktop        iOS         Web
       Kotlin +     JVM +          Kotlin/     Wasm +
       Compose      Compose MP     Native +    Compose MP
                                  Swift       (deferred)
```

### 3.1 Rust Security Core

The Rust Security Core is the platform-independent security boundary.

Its implementation is shared through cross-compilation and UniFFI bindings. Platform-specific integration must not duplicate security-critical primitives in Kotlin, Swift, or other UI/application layers unless explicitly approved by an ADR.

The initial target set includes:

**Android**
- `aarch64-linux-android`
- `armv7-linux-androideabi`

**Desktop**
- `x86_64-pc-windows-msvc`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-unknown-linux-gnu`

Additional targets are additive decisions and must not silently alter the security contract.

### 3.2 KMP Shared Layer

The KMP Shared Layer is responsible for reusable application behavior, including:

- Domain;
- Authentication orchestration;
- Messaging;
- Synchronization;
- Shared presentation/state where appropriate;
- Platform-neutral application contracts.

The shared layer must remain independent of Android-only APIs and must not absorb platform-specific lifecycle, storage, notification, permission, or UI behavior.

### 3.3 Platform adapters

Platform adapters isolate platform-specific concerns.

| Platform | Primary technology | Adapter responsibilities |
|---|---|---|
| Android | Kotlin + Compose | Android lifecycle, permissions, services, notifications, platform storage/integration |
| Desktop | JVM + Compose Multiplatform | Windowing, desktop lifecycle, OS integration, packaging |
| iOS | Kotlin/Native + Swift + Compose Multiplatform | Apple lifecycle, signing, native integration, platform services |
| Web | Wasm + Compose Multiplatform | Browser APIs and web-specific runtime integration; deferred |

## 4. Why this order

### Android first

Android is the first implementation target because it offers the fastest validation loop for the current KMP architecture and avoids introducing Apple-specific build/signing constraints during the earliest security and domain stabilization work.

### Desktop in parallel

Desktop shares the KMP layer and Compose Multiplatform approach. It therefore provides a second runtime environment without requiring a fundamentally separate shared architecture.

Desktop also expands validation across JVM and multiple operating systems while the shared layer is still being stabilized.

### iOS second phase

iOS is intentionally scheduled after the shared layer and Layer C security boundary are stable.

The iOS application should consume the same security core and shared contracts rather than becoming a second source of security-critical logic.

### Web deferred

Web is a separate expansion decision.

The current architecture must **not** be redesigned around Web/Wasm requirements before Web is an approved target. In particular, Web must not weaken, bypass, or redefine the Rust Security Core boundary or the cryptographic decisions in ADR-0008, ADR-0009, and ADR-0010.

## 5. Layer A→E implications

The platform strategy maps to the existing A→E planning model as follows:

- **Layer A — Architecture / governance:** platform target policy and dependency direction.
- **Layer B — Shared application architecture:** KMP shared contracts and reusable application behavior.
- **Layer C — Security boundary:** Rust Security Core, cross-compilation, UniFFI, and platform bindings.
- **Layer D — Platform implementation:** Android, Desktop, and later iOS/Web adapters.
- **Layer E — Delivery / verification:** CI matrices, packaging, platform-specific tests, signing, release verification, and operational readiness.

The exact responsibilities of each layer remain governed by the corresponding architecture/ADR records.

## 6. Architectural invariants

The following are mandatory constraints:

1. Platform UI must not own security-critical primitives.
2. Platform adapters must consume shared contracts rather than fork domain behavior.
3. Rust Security Core remains the security authority across supported platforms.
4. UniFFI is an integration boundary, not a license to duplicate Rust security logic in platform code.
5. Desktop and iOS must reuse the KMP Shared Layer.
6. Web must remain isolated from current security-boundary decisions until explicitly approved.
7. Adding a platform must not silently change the cryptographic protocol or key-management model.
8. Platform-specific dependencies must remain behind explicit adapter boundaries.
9. CI must validate every supported native target before that target is considered release-ready.
10. A platform becomes a supported release target only after its security, build, integration, and verification gates are satisfied.

## 7. CI and build implications

The platform matrix is expected to grow with the implementation:

```text
Android
 ├─ arm64-v8a
 └─ armeabi-v7a

Desktop
 ├─ Windows x86_64
 ├─ macOS x86_64
 ├─ macOS arm64
 └─ Linux x86_64

iOS
 ├─ device
 └─ simulator

Web
 └─ deferred
```

The CI system should keep platform-independent tests in the shared layer and add platform-specific verification only where the runtime requires it.

## 8. Relationship to ADRs

This document is a platform-scope reference, not a replacement for technical ADRs.

Relevant existing ADRs include:

- ADR-0008 — Cryptographic Protocol
- ADR-0009 — Key Management
- ADR-0010 — Serialization
- ADR-0011 — Local Storage
- ADR-0012 — Transport Architecture
- ADR-0013 — Architecture Enforcement
- ADR-0014 — Observability

The current working architecture also refers to **ADR-0015**. If/when ADR-0015 is committed to the repository, it should link back to this document for the platform matrix rather than duplicating the full platform strategy.

## 9. Change policy

Changes to platform priority, platform phase, or the Rust/KMP/platform boundary require an architecture review.

Adding a new platform should answer at minimum:

- Why is the platform needed?
- Which shared contracts are reused?
- Which platform adapters are required?
- Which Rust targets and UniFFI bindings are required?
- What CI/build changes are required?
- What security and verification gates apply?
- Does the change affect an existing ADR?

No platform should be added merely by creating a new application directory.

## 10. Current baseline

**Current implementation focus:** Android.

**Parallel architectural focus:** KMP Shared Layer + Desktop readiness.

**Next platform expansion:** iOS after Layer C and shared contracts stabilize.

**Deferred expansion:** Web/Wasm.

This baseline is the reference point for planning, implementation issues, CI matrices, and future platform ADRs.
