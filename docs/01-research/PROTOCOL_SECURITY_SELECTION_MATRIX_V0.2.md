# Eagle — Protocol & Security Component Selection Matrix v0.2

Audit date: 2026-10-04
Status: Evaluation record — no production protocol is approved by this document.

## Decision rule

Requirement → Architecture Fit → Security History → Exact Version/Commit → License → Transitive Dependencies → Test Evidence → Operational Fit → Exit Strategy → Approval

## Current evidence refresh

| Candidate | Current upstream evidence | Eagle decision |
|---|---|---|
| Signal/libsignal | Official libsignal workspace reports version 0.104.0; repository is AGPL-3.0-only; upstream explicitly says external use is unsupported and APIs may change. | STUDY; do not integrate yet |
| vodozemac | Official manifest reports v0.11.1, Rust 2024 edition, MSRV 1.96, Apache-2.0. Release commit: 3e11ae87d157da6bcac3cbf7825d37c8f5ed60af. | STUDY / ADAPT candidate |
| OpenMLS | Latest release listed as v0.9.0; workspace declares Rust 1.91 and openmls 0.9.0; security policy explicitly covers the RFC 9420 network threat model. | DEFER to group phase; study now |
| Noise | Protocol framework for authenticated handshakes and encrypted transport messages, but not Eagle's complete async messaging protocol. | STUDY as building block |
| Custom cryptographic protocol | No independent security review or conformance corpus. | REJECT |

## Why libsignal is still blocked

The technical quality of libsignal is not the only adoption criterion. Eagle's current repository intent is not to inherit an AGPL production dependency whose upstream explicitly says external use is unsupported. A legal/support review and an exact pinned integration experiment would be required before adoption.

## Why vodozemac remains a serious candidate

vodozemac is attractive because it is Rust-native, Apache-2.0, and already used in the Matrix ecosystem. It is not treated as a drop-in Signal replacement: Eagle still has to evaluate authentication, device lifecycle, protocol semantics, interoperability requirements, and the exact Eagle threat model.

## Why OpenMLS remains phase-separated

OpenMLS is a strong future group-messaging candidate and its security policy is explicit. Eagle should not introduce group-state complexity before the 1:1 identity, session, persistence, and transport-neutral envelope path is proven.

## Android identity boundary

Android Keystore remains the platform boundary. StrongBox is opportunistic rather than universally available, and individual device-ID attestation has managed-device restrictions. The normal consumer identity flow therefore requires a separate Eagle attestation policy and must not assume device-ID attestation is available.

## Primary references

- https://github.com/signalapp/libsignal
- https://signal.org/docs/specifications/x3dh/
- https://signal.org/docs/specifications/doubleratchet/
- https://github.com/matrix-org/vodozemac
- https://github.com/openmls/openmls
- https://github.com/openmls/openmls/security
- https://www.rfc-editor.org/rfc/rfc9420
- https://developer.android.com/reference/kotlin/android/app/admin/DevicePolicyManager

## External evidence notes

libsignal current repository facts were checked from the official repository and Cargo workspace. citeturn349281search0turn349281search12

OpenMLS release and security evidence were checked from the official repository/release pages. citeturn481330search0turn481330search1turn349281search8

Android StrongBox and attestation behavior were checked against current Android developer documentation. citeturn912704search0turn912704search6