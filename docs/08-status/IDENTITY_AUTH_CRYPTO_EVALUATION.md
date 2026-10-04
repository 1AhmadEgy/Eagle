# Eagle — Identity / Authentication / Crypto / Storage Component Evaluation

**Status:** Evidence review recorded; no final cryptographic protocol ADR approved  
**Date:** 2026-10-04  
**Branch:** ai/reverse-engineering-foundation

## Scope

This record evaluates mature components against Eagle's current requirements before any custom cryptographic implementation is introduced.

The evaluation separates protocol/ratchet capability, cryptographic primitive capability, key-management/storage capability, integration/maintenance evidence, and licensing/provenance/support risk.

An evaluation result is not a legal determination and is not a substitute for formal dependency review.

## Eagle requirements currently established

- asynchronous/unreliable transport;
- forward secrecy;
- post-compromise/self-healing properties must be supported or explicitly characterized;
- device/multi-device lifecycle;
- future group messaging;
- private keys behind a key-management boundary;
- no custom E2E protocol from an architecture draft;
- deterministic cross-platform contract semantics.

## Decision matrix

| Component | Role | Evidence | Eagle fit | Decision |
|---|---|---|---|---|
| Signal libsignal | Full Signal protocol + Rust implementation + Java/Swift/TypeScript wrappers | Current repo; used by Signal clients; README explicitly says outside use is unsupported; AGPL-3.0 | Strong technical fit, but direct dependency has explicit support/provenance and licensing constraints for an independently-owned project | **Reject as direct dependency for now**; retain as protocol/reference material; revisit only with explicit compatibility/support/legal review |
| OpenMLS | MLS / group E2EE | RFC 9420 implementation; current release line includes v0.9.0; MIT; CI-tested Linux/Windows/macOS; Android targets are built on CI but listed as unsupported/test-unverified | Strong candidate for future group messaging; not itself the answer to Eagle's 1:1 protocol/server async design | **Adapt** as group-protocol candidate |
| vodozemac | 1:1 Olm + Megolm ratchets | Current docs identify asynchronous Double Ratchet 1:1 support; project reports one security audit with no significant findings; Apache-2.0 | Technically attractive for 1:1; separate historical bindings repository is unmaintained, so Eagle would need a controlled Rust integration path | **Adapt** as 1:1 candidate for an integration proof |
| Noise Protocol Framework | Handshake framework | Official specification; authenticated handshakes and forward secrecy | Does not provide the complete asynchronous message ratchet Eagle needs by itself | **Reject as complete E2E protocol**; possible scoped handshake building block only |
| libsodium | Cross-platform crypto primitives | Current stable docs; portable across Android/iOS/Windows and more | Good primitive toolbox, not a message-ratchet protocol | **Adapt** only where a selected protocol needs its primitives |
| RustCrypto | Rust crypto primitives | Active pure-Rust algorithm collection with broad primitive coverage; permissive licensing on referenced utilities | Good provider/substrate for Rust components, subject to exact crate review | **Adapt** where the selected protocol/provider requires it |
| Google Tink | High-level crypto API | Google-maintained; broad Java/C++/Go/Python support plus Objective-C support | Useful primitive/keyset library, but not Eagle's cross-platform E2E authority and not a ratchet protocol | **Reject for primary E2E core**; reevaluate for local application data protection |
| Android Keystore | Native Android key protection | Android official docs; hardware-backed StrongBox available on supported devices | Direct match for Android native key boundary | **Adopt** as Android key-storage boundary |
| Apple Keychain | Native Apple secret/key storage | Apple official docs; intended for small secrets and cryptographic keys | Direct match for iOS/macOS native key boundary | **Adopt** as Apple secure-storage boundary |
| Windows CNG / DPAPI | Native Windows key/data protection | Microsoft official docs; CNG KSPs and DPAPI | Direct match for Windows native adapter boundary | **Adopt** as Windows native key/storage boundary; exact API depends on key purpose |

## Integration conclusion

Eagle must not add a custom Double Ratchet, X3DH/PQXDH, MLS implementation, or bespoke key schedule at this stage.

The current implementation increment is therefore contracts plus testable boundaries.

### Current boundary shape

Identity
  |
Authentication
  |
SessionStateMachine
  |
CryptoBoundary
  |
KeyManagementBoundary
  |
SecureStorageBoundary

DeterministicSecurityEngine remains responsible only for deterministic session/replay/rate decisions. Authentication must establish the authenticated-session precondition before ReplayGuard evaluation.

## Current selection posture

- 1:1 E2E protocol: **unselected; vodozemac is the leading integration candidate to prototype; libsignal remains reference-only for now.**
- Group E2E protocol: **OpenMLS is the leading candidate to prototype after 1:1 contract stabilization.**
- Primitive provider: **unselected globally until the protocol candidate defines exact primitive requirements.**
- Native key storage: **platform-native adapters selected at the boundary level.**
- Shared technology (KMP/Rust/UniFFI): **still a separate decision; this record does not authorize adding it prematurely.**

## Sources reviewed

- https://github.com/signalapp/libsignal
- https://github.com/openmls/openmls
- https://github.com/matrix-org/vodozemac
- https://docs.rs/vodozemac/latest/vodozemac/
- https://noiseprotocol.org/noise.html
- https://doc.libsodium.org/doc
- https://github.com/RustCrypto
- https://developers.google.com/tink
- https://developer.android.com/privacy-and-security/keystore
- https://developer.apple.com/documentation/security/keychain-services
- https://learn.microsoft.com/en-us/windows/win32/seccng/cng-key-storage-providers
- https://learn.microsoft.com/en-us/windows/win32/seccng/cng-dpapi

## Verification state

This is an evidence/evaluation record only.

It does not mean any third-party component is integrated into Eagle.

The Eagle verification gate remains authoritative.
