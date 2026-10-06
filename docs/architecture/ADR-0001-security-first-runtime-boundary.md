# ADR-0001 — Security-First Runtime Boundary

- Status: ACCEPTED
- Date: 2026-10-06
- Scope: The Eagle application runtime
- Source baseline: "10. فريق Android"
- Related gate: GitHub Issue #86 — architecture: gate server-side Outbox proposal against current P2P-only baseline
- Decision class: Canonical architecture decision

## Decision

The Eagle is a security-first, self-contained messaging application.

Google Play is a distribution/update channel only. Google runtime services and other external runtime service dependencies are not part of the product baseline.

The application-data plane is P2P-first and, for application content, P2P-only under the current accepted baseline. A server-mediated application-content relay, server Outbox, server message queue, or server-side content store is not permitted unless a new architecture decision explicitly re-authorizes it.

## Runtime dependency boundary

Forbidden in production runtime:

- Firebase
- FCM
- Google Play Services
- Play Integrity
- Analytics SDKs
- Crash-reporting/telemetry services that create external runtime dependencies
- Third-party messaging backends
- Third-party application-content relay services

Allowed only as explicitly bounded infrastructure:

- Signaling needed to establish connectivity/session negotiation
- STUN for connectivity discovery
- TURN as a connectivity fallback for encrypted calls/data when direct connectivity fails

The allowed infrastructure must not become an application-content store or generic message relay.

## Canonical data path

```text
Application Content
    ↓
End-to-End Encryption
    ↓
P2P Application Data Plane
    ↓
Peer Device
```

Connectivity/control path:

```text
Client
    ├── bounded signaling/control
    ├── STUN
    └── TURN fallback where required
```

## Security core

Rust is the canonical security/protocol core for:

- identity
- key lifecycle
- cryptography
- session state
- protocol rules
- message/file encryption
- verification
- anti-replay
- TTL/expiry enforcement
- secure state serialization

Android must not contain a second implementation of identity, session cryptography, or the canonical protocol.

## Android boundary

Android owns:

- UI and Compose
- lifecycle and process state
- permissions
- Android platform APIs
- Keystore integration
- local metadata/index storage
- encrypted object storage integration
- background/deferred execution
- media integration
- call/platform integration

The Android/Rust boundary must use typed FFI contracts with explicit ownership/error semantics.

## Storage boundary

- Room: metadata/indexes only.
- App-private object storage: ciphertext only.
- Android Keystore: local protection/wrapping material where required.
- Plaintext content must not become a persistent database/storage artifact.

## Background execution

No external push dependency is used to wake the application.

Background work is limited to OS-supported/deferred operations that do not create a permanent always-on application service.

The documented consequence is accepted: immediate receipt after the application is killed or the device is rebooted is not guaranteed without an approved runtime wake mechanism.

## Outbox decision

Server-side Outbox is NOT part of the current canonical application architecture.

Issue #86 remains an architecture gate. The proposal must not be merged into the application baseline.

A future server-side Outbox may be reconsidered only after a separate architecture decision explicitly authorizes a server-mediated application-data path and defines delivery semantics, data classification, retention, runtime/dependencies, threat model, operational tests, and release gates.

## Compatibility baseline

- minSdk: 31
- targetSdk: 36 for the current release baseline
- Android 17 / API 37: compatibility/security/performance test target before any target-sdk promotion

## Priority ordering

```text
Security
  ↓
Correctness
  ↓
Reliability / Stability
  ↓
Performance / Efficiency
  ↓
Responsiveness / Speed
  ↓
Binary Size / Resource Usage
```

No performance, battery, startup, or size optimization may weaken a security control or introduce a new external runtime dependency.

## Canonicalization rule

Earlier documents that contain FCM, Google runtime services, server-side application envelopes, PostgreSQL application-content pipelines, or server Outbox designs remain preserved for provenance/history but are not implementation authority.

Implementation authority is this ADR plus the current approved requirements and test gates.

## Acceptance evidence

This decision is considered implemented only when repository evidence exists for:

- dependency allow/deny scan
- runtime network-path audit
- Rust/Android ownership checks
- storage plaintext scan
- process-death/lock tests
- P2P transport tests
- release dependency/SBOM report
- security review
