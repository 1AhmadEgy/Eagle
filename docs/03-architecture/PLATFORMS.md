# Eagle Platform Strategy

> **Status:** Accepted reference baseline
> **Scope:** Applications-only, multi-platform execution targets and cross-platform boundaries
> **Last updated:** 2026-10-06

## 1. Purpose

Eagle is an **applications-only, cross-platform system**. The supported application targets are:

- Android
- Desktop: Windows, macOS, Linux
- iOS

**Web/Wasm applications are out of scope.** Web is not a deferred application target and must not be introduced implicitly by implementation tooling or framework defaults.

This document is the canonical platform reference. It does not replace ADRs for cryptography, key management, serialization, transport, storage, or other technical decisions.

## 2. Target platforms

| Phase | Platform | Priority | Repository status |
|---|---|---:|---|
| Phase 1 | Android | 1 | Current implementation focus — `app/` skeleton |
| Phase 1 | Desktop (Windows / macOS / Linux) | 2 | Planned target |
| Phase 2 | iOS | 3 | Planned target |
| Out of scope | Web / WebAssembly | — | Explicitly excluded |

### 2.1 Delivery intent

**Android is the primary implementation platform.** It provides the first application validation loop for the shared domain, security boundary, messaging, synchronization, and UI architecture.

**Desktop is a supported application target.** It reuses the shared layer and Compose Multiplatform without creating a separate domain/security architecture.

**iOS is a supported application target.** It follows stabilization of the shared layer and security boundary and retains native Swift/Xcode integration where required.

**Web is excluded.** Browser application delivery, Web/Wasm runtime support, browser storage, browser cryptography, and web-specific release tracks are not part of the current Eagle architecture.

## 3. Cross-platform architecture

```text
                 Rust Security Core
             (cross-compile + UniFFI)
                         │
                         ▼
                  KMP Shared Layer
          ┌──────────────┴──────────────┐
          │ Domain / Auth / Messaging   │
          │ Sync / Shared application   │
          │ contracts and state         │
          └──────────────┬──────────────┘
                         │
                  Platform Adapters
             ┌───────────┼───────────┐
             ▼           ▼           ▼
          Android      Desktop      iOS
          Kotlin +     JVM +        Kotlin/
          Compose      Compose MP   Native +
                                    Swift
```

### 3.1 Rust Security Core

The Rust Security Core is the platform-independent security authority.

Security-critical primitives and protocol logic must not be duplicated in Kotlin, Swift, desktop UI/application code, or another platform layer unless explicitly approved by an ADR.

Platform bindings are integration boundaries, not alternate security implementations.

### 3.2 KMP Shared Layer

The KMP Shared Layer is responsible for reusable application behavior, including:

- Domain contracts;
- Authentication orchestration;
- Messaging;
- Synchronization;
- Shared state/presentation where appropriate;
- Platform-neutral application contracts.

The shared layer must remain independent from platform-only lifecycle, storage, permission, notification, signing, packaging, and native UI details.

### 3.3 Platform adapters

| Platform | Primary technology | Adapter responsibilities |
|---|---|---|
| Android | Kotlin + Compose | Lifecycle, permissions, services, notifications, Android storage and OS integration |
| Desktop | JVM + Compose Multiplatform | Windowing, desktop lifecycle, OS integration, packaging |
| iOS | Kotlin/Native + Swift + Compose Multiplatform | Apple lifecycle, signing, native integration, platform services |

## 4. Security boundary implications

Each supported platform must consume the same security/domain contracts while mapping platform-specific custody and OS controls behind an explicit adapter.

Examples include:

- Android Keystore / hardware-backed facilities where available;
- macOS Keychain / supported Secure Enclave operations;
- Windows protected credential/cryptographic facilities;
- Linux protected credential stores appropriate to the deployment environment.

The exact custody level must be reported truthfully to the security policy/gate. Software custody must not be relabeled as hardware-backed custody.

## 5. Verification implications

A platform is not a supported release target merely because an application directory or build script exists.

Release readiness requires, at minimum:

1. Build evidence;
2. Unit and integration evidence;
3. Cryptography and protocol evidence;
4. Security testing;
5. Dependency/supply-chain checks;
6. Platform-specific storage/key-custody checks;
7. Regression evidence;
8. Fuzz/property evidence where applicable;
9. Signing/package verification;
10. Human security review for material security changes.

The common security/domain tests should run once in the shared/core layers where possible. Platform-specific tests must validate the native adapter and runtime boundary.

## 6. Explicit non-target: Web

Web/Wasm is a scope exclusion.

Therefore:

- no Web application directory is part of the target architecture;
- no Web/Wasm CI matrix is required;
- no browser-specific security boundary is assumed;
- no web storage model is accepted as a substitute for native protected storage;
- no web release is implied by Compose Multiplatform support.

A future proposal to add Web requires an explicit scope decision and architecture review before implementation.

## 7. Change policy

Changing platform priority, adding a new platform, or changing the Rust/KMP/platform boundary requires architecture review.

A new platform proposal must define:

- purpose and threat model;
- reused shared contracts;
- platform adapters;
- security/native custody model;
- Rust/UniFFI targets;
- CI/build requirements;
- test and release gates;
- ADR impact.

No platform should be added merely by creating a new application directory.

## 8. Current baseline

**Implementation focus:** Android.

**Cross-platform architectural target:** Android + Desktop + iOS.

**Shared security authority:** Rust Security Core.

**Shared application layer:** KMP.

**Web/Wasm:** Out of scope.
