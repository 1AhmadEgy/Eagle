# The Eagle — P2P Security Threat Model

- Status: CANONICAL baseline for review
- Date: 2026-10-06
- Basis: reviewed Android source document + accepted security-first project policy
- Implementation status: NOT PROVEN until tests/evidence exist

## 1. Assets

Protected assets include:

- device identity
- private identity keys
- session state
- message content
- media content
- local message history
- object keys
- temporary plaintext
- authentication/session material
- peer verification state
- sensitive notification state
- FFI/native memory

## 2. Trust boundaries

### Boundary A — Android UI ↔ Application

Threats:
- sensitive state accidentally persisted
- plaintext retained in saved state/navigation
- lock state bypass
- rendering after lock

Controls:
- secure render gate
- bounded UI state
- cancellation on lock
- no plaintext persistence

### Boundary B — Android ↔ Rust

Threats:
- malformed input
- ownership errors
- buffer lifetime bugs
- panic crossing FFI
- duplicated/contradictory security logic

Controls:
- typed FFI
- explicit ownership rules
- bounded inputs
- error mapping
- native fault tests
- one canonical Rust security implementation

### Boundary C — Device storage

Threats:
- plaintext at rest
- private-key leakage
- backup leakage
- stale expired objects
- database corruption

Controls:
- ciphertext-only object persistence
- Room metadata/index role
- Keystore platform protection
- explicit migration tests
- expiry/key-destruction tests
- backup exclusion tests

### Boundary D — Network

Threats:
- man-in-the-middle
- transport downgrade
- replay
- duplicate delivery
- traffic analysis
- unauthorized relay/storage
- external service dependency leakage

Controls:
- authenticated encrypted transport
- replay protection
- bounded signaling
- fail-closed validation
- dependency/network audit
- P2P application-data boundary

### Boundary E — Signaling/TURN infrastructure

Threats:
- metadata exposure
- infrastructure compromise
- unintended content relay/storage
- availability failures

Controls:
- no private keys
- no plaintext content
- no application-content store
- explicit transport scope
- short-lived/limited credentials where applicable
- negative tests proving content is not accepted/stored

## 3. Attacker classes

### Remote network attacker

Capabilities:
- observe/modify/replay traffic
- attempt endpoint confusion
- attempt protocol downgrade

Must not obtain:
- plaintext content
- private keys
- usable session secrets

### Malicious/compromised peer

Capabilities:
- send malformed content
- replay messages
- present changed identity
- attempt resource exhaustion

Controls:
- identity verification
- authenticated envelopes
- replay checks
- input limits
- peer-identity change handling

### Compromised server/infrastructure

Capabilities:
- inspect metadata available to the infrastructure
- modify signaling
- deny service
- attempt unauthorized content handling

Security goal:
- server must not gain message/media plaintext or private identity keys
- unauthorized content relay/storage must be rejected by architecture and tests

### Local attacker with unlocked device

This is outside the fully preventable threat envelope. The design instead minimizes persistence, enforces lock behavior, and constrains external output.

### Compromised OS/rooted device

Complete confidentiality cannot be guaranteed. The project objective is to minimize attack surface and avoid unnecessary sensitive persistence.

## 4. Critical security invariants

1. Private identity keys never leave the approved secure boundary.
2. Rust is the only canonical implementation of protocol/crypto/identity logic.
3. Application plaintext is never a permanent storage artifact.
4. Expired/deleted objects cannot be decrypted through normal application APIs.
5. Replay and invalid authentication state are rejected.
6. Lock transitions revoke sensitive access.
7. Process death does not silently unlock or recreate identity.
8. No external runtime messaging/analytics service is required.
9. P2P application content is never silently redirected to a server-content path.
10. FFI failure is fail-closed.

## 5. Mandatory negative tests

- modified ciphertext/envelope
- replayed envelope
- wrong recipient
- stale session
- changed peer identity
- corrupted secure state
- expired object
- oversized input
- malformed media
- malformed FFI input
- simulated FFI panic/failure
- lock during decrypt/render
- process death during sensitive operation
- prohibited network route
- prohibited dependency present

## 6. Evidence rule

A threat is considered controlled only when:

- the control is documented;
- implementation exists;
- an automated or reproducible test exists where feasible;
- evidence is stored;
- review status is recorded.

Design prose alone is not evidence of control effectiveness.
