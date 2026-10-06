# The Eagle — Security-First Baseline

## 1. Purpose

This document converts the accepted project direction into a verifiable execution baseline.

It is a policy and traceability document, not proof that the implementation is complete.

## 2. Security objectives

Protect, at minimum:

- message content
- media content
- identity keys
- session keys/state
- peer verification state
- local message history
- temporary media
- notification-sensitive data
- in-memory sensitive state
- IPC boundaries
- transport/session metadata where controllable

The design must also explicitly document what cannot be guaranteed, including a compromised device/OS, an unlocked device in an attacker’s possession, malicious peers, screen photography, or leakage through external input/accessibility paths.

## 3. Dependency isolation

### Production runtime: prohibited

- Firebase
- FCM
- Google Play Services
- Play Integrity
- Analytics
- Crashlytics or equivalent external runtime telemetry
- External cloud messaging
- External messaging backends
- Third-party content relay

### Distribution: allowed

- Google Play for distribution and updates only

Google Play must not be treated as an application runtime dependency.

## 4. Network policy

Application content:

```text
Peer A ↔ Peer B
```

Connection assistance, where explicitly required:

```text
Peer ↔ Signaling
Peer ↔ STUN
Peer ↔ TURN ↔ Peer
```

Properties:

- content remains end-to-end encrypted
- private identity keys never leave the device
- signaling must not become content storage
- TURN must not become a generic message store
- no hidden fallback to a cloud messaging service
- no permanent polling loop
- no insecure cleartext transport
- authentication and certificate validation must fail closed

## 5. Identity and key ownership

Rust is canonical for:

- device identity
- key lifecycle
- session state
- protocol state
- verification
- cryptographic operations

Android Keystore is used for the platform protection boundary, including non-exportable/local wrapping material where appropriate.

The application must not silently recreate an identity after permanent key invalidation or unrecoverable secure-state loss.

## 6. Local storage

Required properties:

- no persistent plaintext message content in Room
- no persistent private keys in Room or preferences
- ciphertext-only media/object storage
- explicit Room migrations
- integrity checks and fail-closed behavior
- backup/restore policy must not export secrets or plaintext

## 7. Lock and process lifecycle

On lock:

- revoke access to sensitive Rust state
- stop/close sensitive readers and streams
- stop call/media activity as required by policy
- cancel rendering/decryption jobs
- clear sensitive UI state

After process death:

- application starts locked
- no plaintext appears before unlock
- no sensitive session is silently restored without the defined policy
- no new identity is silently generated
- microphone/camera state is not silently resurrected

## 8. IPC / external output

The secure baseline disallows uncontrolled export paths such as:

- generic share flows
- automatic external file opening
- public media copies
- clipboard-based message export
- WebView/JavaScript integration for sensitive application content

Any exception requires an explicit architecture/security decision.

## 9. Media

- use bounded/chunked processing
- encrypt before persistence
- never require a permanent raw-media copy for normal operation
- internal playback is canonical
- media expiry must stop access
- malformed/oversized input must be rejected
- chunk integrity and replay must be verified

## 10. Calls

- one-to-one only under the current scope
- WebRTC is the media transport candidate
- P2P direct path preferred
- TURN fallback allowed for connectivity
- no recording
- no screen sharing
- no multi-party calls
- call state must respect lock/permission/process-death rules

## 11. Background and offline behavior

The current no-push baseline accepts:

- synchronization when the application is opened
- deferred cleanup/retry work where OS scheduling permits
- no always-on external wake dependency

WorkManager is not a realtime call or realtime incoming-message transport.

## 12. Performance gates

Measure, do not assume:

- cold/warm startup
- TTID/TTFD
- frame timing
- memory
- CPU
- battery
- Rust call latency
- encryption/decryption latency
- APK/AAB size
- ANR/crash indicators
- background wakeups
- WebRTC behavior

Initial internal regression policy:

- startup regression ≤ 10%
- TTID regression ≤ 10%
- p95 frame regression ≤ 15%
- memory regression ≤ 15%
- APK regression ≤ 5%

These are internal gates and must be validated on representative real devices before being treated as production budgets.

## 13. Supply-chain controls

Every dependency requires:

- purpose
- version pinning
- license review
- maintenance review
- security/vulnerability review
- transitive dependency review
- binary-size impact review
- runtime network behavior review

CI must fail on prohibited runtime dependencies.

## 14. Test evidence

Minimum security evidence:

- modified envelope rejected
- replay rejected
- wrong recipient rejected
- expired object/message rejected
- corrupted state rejected
- key destruction blocks decrypt
- lock closes readers
- process death starts locked
- backup does not expose secrets/plaintext
- malformed FFI inputs handled safely
- no native panic crosses FFI boundary
- unauthorized external output blocked
- transport certificate/auth failure rejected
- P2P/TURN behavior verified
- no prohibited runtime dependency present

## 15. Release rule

Security failure blocks release.

Performance regression blocks release until:

1. a benchmark report exists;
2. the regression cause is identified;
3. the change is corrected or explicitly waived through the project release gate;
4. security has been re-verified.

## 16. Non-goals of this baseline

This baseline does not claim:

- perfect security
- immunity from a compromised OS/device
- guaranteed direct P2P in every network
- guaranteed instant delivery while the application is killed
- completed implementation without test evidence

