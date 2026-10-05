# Eagle Security Master Architecture — 2026-10-05

## Scope

This document is the security-first execution baseline for Eagle. It reconciles the project corpus, current Git state, the P2P-only constraint, the Rust Security Core foundation, and current mature-project research.

This is an execution baseline. It does not silently convert proposed ADRs into approved protocol or key-management decisions.

## Non-negotiable security requirements

1. P2P-only data plane.
2. No relay data path for application content. If direct connectivity cannot be established, the application remains offline/unavailable rather than silently routing through a server.
3. Rendezvous/discovery is never a trust root, message store, authorization authority, or key escrow.
4. Rust Security Core is the security TCB. Security-critical state machines, protocol adapters, authorization decisions, and cryptographic orchestration stay behind this boundary.
5. No custom cryptographic primitives or custom cryptographic protocol.
6. Platform/UI code must not duplicate security logic.
7. Unknown, stale, revoked, malformed, downgraded, unauthenticated, or unsupported states fail closed.
8. Private key material stays inside the narrowest platform secure-storage boundary and never enters ordinary databases or telemetry.
9. Every production dependency requires exact version, source, license, security history, configuration, tests, provenance, owner, and upgrade plan.

## Target trust architecture

User
  |
Platform UI
  |
Platform Adapter
  |
KMP Shared Layer
  |
Rust Security Core
  |------------------------|
  |                        |
Direct P2P Transport       Secure Platform Key Boundary
  |                        |
Authenticated Peer         Local Encrypted Storage

The network is hostile. Reachability does not equal trust.

## Identity and trust

The target model is per-device cryptographic identity with explicit account membership, pairing, revocation, monotonic trust epochs, and quarantine for identity changes.

Mandatory invariants:
- usernames never authorize devices;
- trust promotion is never public;
- pairing is authenticated, time-bounded, single-use, and endpoint-bound;
- revocation is monotonic;
- stale or revoked membership cannot silently regain privilege;
- account recovery is separate from historical message decryption;
- platform attestation is evidence, not the root of authorization;
- AI/tools cannot bypass security policy.

## Session and message security

The production protocol must provide authenticated key establishment, forward secrecy, post-compromise recovery where supported, replay resistance, downgrade resistance, device/session binding, explicit identity-change verification, bounded parsing, and multi-device lifecycle handling.

Transport encryption alone never authorizes a message or peer.

## Component posture

### 1:1 messaging

Preferred research direction: a mature Rust double-ratchet implementation behind the Security Core.

vodozemac is a maintained Rust implementation of Olm/Megolm. Its Olm sessions are asynchronous Double Ratchet channels with forward secrecy and self-healing properties. It remains a candidate until Eagle interoperability, device lifecycle, key custody, and maintenance risks are proven.

libsignal is used by Signal's Android, iOS, and Desktop clients and its implementations are Rust-based, but its upstream documentation explicitly says use outside Signal is unsupported and APIs may change. Therefore Eagle does not adopt it automatically. A dedicated support/maintenance decision is required before production use.

### Group messaging

OpenMLS is the preferred research direction for a future group phase based on MLS (RFC 9420). Group protocol selection is independent from the 1:1 security boundary.

### Transport

Preferred research direction: direct rust-libp2p QUIC v1, subject to ADR-0012 and platform build verification.

libp2p documents QUIC as TLS 1.3 encrypted, stream-multiplexed transport with cryptographic peer-ID authentication. libp2p also supports relays and hole punching. Eagle policy disables relays for application data because the project constraint is P2P-only. Direct hole punching may be used where supported; otherwise the node stays offline.

## Key management

Separate:
- long-lived device identity keys;
- protocol provisioning/prekeys;
- ephemeral/session keys;
- recovery authorization material;
- application data-encryption keys.

Private key export should be prohibited where supported. Databases store encrypted records/public metadata, not private keys.

The exact secure-storage APIs remain platform-specific implementation decisions and must be tested on the supported matrix.

## Serialization and parser hardening

The production encoding must be schema-defined, versioned, length-bounded, strictly validated, and canonical wherever authenticated bytes require canonical representation. Test malformed, truncated, duplicated, reordered, oversized, and version-invalid inputs.

Custom binary framing is exceptional-risk.

## Supply chain

For each production dependency:

Component -> exact version -> source -> license -> transitive graph -> vulnerability status -> configuration -> tests -> provenance -> owner -> upgrade plan

Required CI controls:
- full 40-character SHA pinning for third-party GitHub Actions;
- secret scanning;
- dependency/SCA scanning once the stack is fixed;
- SBOM generation;
- controlled/reproducible builds where practical;
- artifact provenance and attestation;
- reviewed dependency upgrades;
- time-limited security exceptions.

## Release gates

G0 Corpus: available artifacts classified; unretrieved artifacts explicitly pending.

G1 Architecture: requirements, trust boundaries, flows, platform scope, and ADRs approved.

G2 Cryptography: protocol, key management, serialization, vectors, interoperability, and misuse-resistant APIs approved.

G3 Implementation: common security boundary implemented without platform duplication.

G4 Adversarial verification: negative, fuzz/property, replay, downgrade, identity, pairing, compromise, recovery, deletion, and platform-parity tests pass.

G5 Supply chain: exact versions, SBOM, vulnerability policy, provenance, signing/attestation, and dependency review pass.

G6 Independent review: independent cryptographic/security review completed.

G7 Release: all P0/P1 blockers closed, recovery/rollback tested, supported platforms verified, human approval recorded.

## Explicit no-go conditions

Release is blocked when protocol/key management remains Proposed/Pending, P2P-only is violated, self-authorization is possible, stale/revoked devices regain privilege, malformed input bypasses guards, secrets appear in source/build/logs, required test categories are missing, CI security actions are unpinned, or independent review is absent.

## External references

- https://github.com/signalapp/libsignal
- https://matrix-org.github.io/vodozemac/vodozemac/
- https://docs.libp2p.io/concepts/transports/quic/
- https://docs.libp2p.io/connectivity/
- https://github.com/openmls/openmls/releases
- https://latest.openmls.tech/doc/openmls/index.html
- https://www.rfc-editor.org/rfc/rfc9420

## Status

Security architecture baseline: strong foundation; production remains blocked by gated protocol, key-management, storage, transport, adversarial verification, and independent review.
