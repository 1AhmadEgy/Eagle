# Eagle Cryptographic Protocol Research — 2026-10-05

**Status:** Research only — no protocol selected  
**Related:** ADR-0008, ADR-0009, ADR-0010  
**Scope:** asynchronous P2P messaging, 1:1, multi-device, groups, forward secrecy, post-compromise security, mobile constraints, auditability, maintenance, supply chain.

## Executive conclusion

The research does not justify implementing a custom cryptographic protocol.

Two mature families deserve first-class evaluation:

- **Signal-family:** strongest direct conceptual fit for 1:1 asynchronous and multi-device session management.
- **MLS / OpenMLS:** strongest direct fit for explicit group membership and epoch evolution.

**Noise** is valuable as a handshake framework, but is not itself a complete asynchronous messaging protocol; adopting it would leave Eagle responsible for message ratcheting, replay, device lifecycle and group semantics.

A combined Signal-family + MLS architecture is a hypothesis only. It must not be treated as approved until cross-protocol identity binding, lifecycle, persistence, downgrade policy, and audit boundaries are proven.

## Evidence matrix

| Dimension | Signal-family | Noise | MLS / OpenMLS |
|---|---|---|---|
| Async/offline 1:1 | Strong | Framework only | Possible, but group-oriented |
| Multi-device | Strong; Sesame explicitly models it | Eagle must design it | Membership/leaf model helps, but Eagle must design device-account semantics |
| Group messaging | Mature Signal group machinery exists, but protocol details must be evaluated | Eagle must design | Native strength via RFC 9420 epochs/proposals/commits |
| Forward secrecy | Double Ratchet / modern Signal family | Depends on Eagle's construction | MLS key schedule/secret tree |
| Post-compromise security | Modern Signal specifications address ratcheting / continuous key agreement | Eagle must design | MLS epochs/updates provide the group model |
| Replay / sequencing | Defined by protocol/session machinery; must be preserved by implementation | Eagle must design | Epoch and message framing provide structure; application delivery semantics still matter |
| P2P-only fit | Requires removing/abstracting server assumptions in components such as Sesame | Good transport fit, but more bespoke protocol work | Possible, but delivery/group-state synchronization must be designed around direct P2P |
| Rust integration | libsignal is Rust-based with Java/Swift/TypeScript APIs | Depends on chosen Noise implementation | OpenMLS is Rust |
| Android/iOS readiness | Signal's own clients use the stack | Depends on library | OpenMLS currently builds Android/iOS targets but official CI does not test those targets |
| External-use / legal | libsignal currently says external use is unsupported and is AGPLv3 | Library-specific | OpenMLS repository is MIT |
| Maintenance / provenance | Very high project activity; upstream API changes expected | Depends on selected implementation | Active maintained project; release 0.9.0 current as of this research |
| Main Eagle risk | Licensing/supportability + adapting server-oriented session management to P2P | Eagle becomes responsible for too much security-critical protocol surface | 1:1 complexity + platform test gap + storage/FFI integration |

## Security constraints carried into implementation

### Authentication precedes replay-state mutation

A receiver must not mark a sequence number as seen before message authenticity has been established. Otherwise forged traffic can poison the replay window.

### Separate send and receive state

A sender and receiver require independent sequence state. Sequence values must not wrap; a new cryptographic key context must have explicitly defined sequence semantics.

### Offline and restart are part of the protocol

The accepted design must specify:

- state persistence;
- rollback detection;
- duplicated messages;
- out-of-order delivery;
- delayed delivery;
- concurrent session establishment;
- device add/remove/replace;
- revocation propagation;
- crash recovery.

Signal's Sesame specification explicitly discusses state deletion/rollback, multiple devices, unreliable delivery, duplication and reordering. MLS defines explicit epochs and requires an application-level strategy for conflicting commits based on the same state.

## Current Eagle implementation boundary

The Rust Core now includes:

- bounded structural frame parsing;
- fail-closed protocol-version/flag checks;
- device authority binding with monotonic revocation epoch;
- test-only send-sequence and receive-replay-window seams.

It intentionally does **not** include:

- a cryptographic handshake;
- authenticated ciphertext verification;
- a production replay API;
- key derivation;
- persistence/recovery protocol;
- group-state cryptography.

This separation prevents a structural parser or generic counter from being mistaken for a complete security protocol.

## Research references

1. Signal PQXDH — https://signal.org/docs/specifications/pqxdh/
2. Signal Double Ratchet — https://signal.org/docs/specifications/doubleratchet/
3. Signal Sesame — https://signal.org/docs/specifications/sesame/
4. Signal libsignal — https://github.com/signalapp/libsignal
5. Noise Protocol Framework — https://noiseprotocol.org/noise.pdf
6. RFC 9420 (MLS) — https://www.rfc-editor.org/rfc/rfc9420/
7. OpenMLS — https://book.openmls.tech/
8. OpenMLS current crate documentation — https://latest.openmls.tech/doc/openmls/index.html
9. RFC 4347 (DTLS anti-replay window) — https://www.rfc-editor.org/rfc/rfc4347.html
10. RFC 8446 (TLS 1.3 sequence numbers) — https://www.rfc-editor.org/rfc/rfc8446.html
11. RFC 9180 (HPKE sequencing/replay limitations) — https://www.rfc-editor.org/rfc/rfc9180.html
12. Rust u64 checked addition — https://doc.rust-lang.org/std/primitive.u64.html
13. Rust slice safety — https://doc.rust-lang.org/std/primitive.slice.html

## Decision gate

ADR-0008 must remain **Pending Review** until the team has:

- completed direct security review of the selected protocol/library;
- verified license/supportability;
- verified all target platforms needed by Eagle;
- defined P2P-specific delivery semantics;
- mapped identity/device/revocation semantics;
- defined serialization and storage contracts;
- produced conformance/negative test vectors;
- defined FFI boundaries;
- produced independent review evidence.

