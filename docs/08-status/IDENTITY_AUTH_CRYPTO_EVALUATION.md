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

| Component | Role | Current evidence | Eagle fit | Decision |
|---|---|---|---|---|
| Signal libsignal | Full Signal protocol + Rust implementation + Java/Swift/TypeScript wrappers | Current project README says use outside Signal is unsupported; current repository is AGPL-3.0-only | Strong technical fit, but direct dependency has explicit support, provenance, and licensing constraints for an independently-developed project | **Reject as direct dependency for now**; retain as protocol/reference material; revisit only with explicit compatibility, support, and legal review |
| OpenMLS | MLS / group E2EE | RFC 9420 implementation; latest release observed v0.9.0 (2026-08-03); MIT; CI coverage includes major desktop targets, while Android/iOS runtime support needs separate validation | Strong candidate for future group messaging; not automatically a fit for Eagle's 1:1 and delivery-service requirements | **Adapt** as group-protocol candidate |
| vodozemac | 1:1 Olm + Megolm ratchets | Latest crate observed v0.11.1 (2026-09-30); 1:1 Olm is Double Ratchet with forward secrecy and self-healing; one reported security audit with no significant findings; Apache-2.0 | Technically attractive for 1:1; Eagle still needs its own controlled Rust-to-platform integration boundary | **Adapt** as 1:1 candidate for an integration proof |
| Noise Protocol Framework | Handshake framework | Official specification; authenticated handshakes and forward secrecy | Does not provide Eagle's complete asynchronous message-ratcheting protocol by itself | **Reject as complete E2E protocol**; possible scoped handshake building block only |
| libsodium | Cross-platform crypto primitives | Mature portable crypto library with Android/iOS/Windows support and bindings | Good primitive toolbox, not a message-ratchet protocol | **Adapt** only where a selected protocol requires its primitives |
| RustCrypto | Rust crypto primitives | Active pure-Rust algorithm collection with broad primitive coverage | Good provider/substrate for Rust components, subject to exact crate-by-crate review | **Adapt** where the selected protocol/provider requires it |
| Google Tink | High-level crypto API | Google-maintained high-level primitive/keyset library with broad language support | Useful for application/data protection, but not Eagle's primary E2E ratchet authority | **Reject for primary E2E core**; reevaluate for local application data protection |
| Android Keystore | Native Android key protection | Official Android key storage boundary; hardware-backed StrongBox available on supported devices | Direct match for Eagle's Android native key boundary | **Adopt** as Android key-storage boundary |
| Apple Keychain | Native Apple secret/key storage | Official Apple secure storage/key boundary | Direct match for iOS/macOS native key boundary | **Adopt** as Apple secure-storage boundary |
| Windows CNG / DPAPI | Native Windows key/data protection | Official Microsoft key-storage and data-protection facilities | Direct match for Windows native adapter boundary | **Adopt** as Windows native key/storage boundary; exact API depends on key purpose |

## Protocol implications

RFC 9420 defines MLS as an asynchronous group key-establishment protocol with forward secrecy and post-compromise security for groups from two to thousands of members. This makes MLS strategically relevant to Eagle's group roadmap, but it does not remove the need to evaluate application authentication, delivery-service, lifecycle, storage, and 1:1 requirements separately.

For 1:1 messaging, the repository keeps vodozemac as the leading candidate for a controlled integration proof because its current Olm implementation provides an established Double Ratchet API rather than forcing Eagle to implement the ratchet itself.

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
- Group E2E protocol: **OpenMLS is the leading candidate to prototype after the 1:1 contract stabilizes.**
- Primitive provider: **unselected globally until the protocol candidate defines exact primitive requirements.**
- Native key storage: **platform-native adapters selected at the boundary level.**
- Shared technology (KMP/Rust/UniFFI): **still a separate decision; this record does not authorize adding it prematurely.**

## Sources reviewed

- https://github.com/signalapp/libsignal
- https://github.com/openmls/openmls
- https://github.com/matrix-org/vodozemac
- https://docs.rs/vodozemac/latest/vodozemac/
- https://www.rfc-editor.org/rfc/rfc9420
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
