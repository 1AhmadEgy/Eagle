# ADR-0018: Eagle Threat Model and Security Requirements

- **Status:** Proposed
- **Date:** 2026-10-06
- **Scope:** Rust Security Core, device pairing, authenticated one-to-one sessions, replay protection, future secure-session protocol
- **Related:** ADR-0017, ADR-0016, FFI_CONTRACT.md
- **Decision owner:** Human project owner
- **Review gate:** Human security review required before production cryptographic enablement

## 1. Purpose

This ADR defines what Eagle is protecting, which adversaries are in scope, which guarantees are required, and which guarantees are explicitly not promised.

A secure-session protocol must be selected **after** these requirements are fixed. Protocol preference alone is not an acceptable architecture criterion.

This threat model is intentionally focused on Eagle's current target: **private one-to-one communications with asynchronous setup, authenticated devices, secure session state, and multi-platform clients**.

## 2. Architectural assumptions

The current architecture separates:

1. Rust Security Core as the security authority.
2. Platform adapters for Android/iOS/Desktop.
3. Typed UniFFI bindings for cross-language invocation.
4. A separate Wire Protocol and serialization layer.
5. Platform secure storage / hardware-backed key custody.
6. A signaling service used for rendezvous and message delivery.
7. Optional TURN/media infrastructure for transport where required.

The following deployment constraint is assumed for this ADR:

- Eagle is intended to remain proprietary unless a human owner explicitly chooses another licensing model.
- AGPL dependencies are therefore treated as release blockers pending qualified legal review.
- This is a project policy, not a legal conclusion.

## 3. Security objectives

Eagle SHALL provide the following protocol-level objectives for supported one-to-one messaging sessions:

### O1 — End-to-end confidentiality

A compromised signaling server, relay, or transport intermediary must not be able to decrypt protected application plaintext.

### O2 — End-to-end integrity and authenticity

An attacker must not be able to forge accepted protected messages as a legitimate peer without defeating the cryptographic authentication and device authorization model.

### O3 — Forward secrecy

Compromise of current or future session state must not by itself reveal previously protected messages outside the protocol's explicitly documented recovery limits.

### O4 — Post-compromise recovery

For an attacker who temporarily obtains session state but loses ongoing access, the protocol SHALL support documented key evolution that allows the session to recover confidentiality/authenticity after subsequent honest ratchet progress.

The exact recovery boundary is protocol-specific and must be verified during implementation and testing.

### O5 — Replay resistance

A valid protected message captured from the network must not be accepted again as a fresh message once its authenticated sequence state has already been consumed.

### O6 — Downgrade resistance

An attacker must not force the endpoints to accept a protocol version or security mode below the locally required minimum.

### O7 — Identity binding

A session must be bound to authenticated device identities and the expected authorization context before it becomes established.

### O8 — Key isolation

Long-term identity keys and session secrets must not cross the FFI boundary as raw private-key material or be exposed through ordinary application logging.

### O9 — Fail closed

Invalid authentication, replay, downgrade, malformed input, state-transition violations, expired authorization, and identity mismatches must produce explicit failure and must not silently fall back to an insecure mode.

### O10 — Recovery and rotation

Device replacement, revocation, rekey, and session closure must invalidate the relevant prior authorization state according to documented policy.

## 4. Assets

| Asset | Sensitivity | Primary threat |
|---|---|---|
| Message plaintext | Critical | Network/server compromise |
| Long-term identity private keys | Critical | Device theft, endpoint compromise |
| Session/root/chain keys | Critical | Memory/storage compromise |
| File/media keys | Critical | Server/relay compromise, endpoint theft |
| Device fingerprints / identity bindings | High | MITM, spoofing, privacy leakage |
| Pairing authorization artifacts | Critical | Device substitution, replay |
| Session sequence state | High | Replay and state desynchronization |
| Call/message history | High | Device theft, local extraction |
| Signaling metadata | Medium/High | Traffic analysis, insider access |
| Timing and message-size patterns | Medium/High | Traffic analysis |
| Local audit/security events | Medium | Information leakage and forensic privacy |

## 5. Adversaries

### A1 — Passive network adversary

Can observe, capture, delay, and store network traffic.

Goal: recover plaintext, correlate traffic, or prepare replay/analysis.

Expected defense: authenticated encryption, forward secrecy, replay protection, transport-layer protection, and avoidance of plaintext protocol metadata beyond what is required.

### A2 — Active network / MITM adversary

Can modify, reorder, inject, replay, delay, and drop messages.

Goal: impersonate a peer, downgrade the protocol, or desynchronize a session.

Expected defense: authenticated key establishment, identity verification, transcript binding, downgrade rejection, replay windows, and explicit session state validation.

### A3 — Compromised signaling server

Can read and modify signaling data, substitute routing information, drop delivery, and attempt device substitution.

Cannot be trusted with message plaintext or session secrets.

Expected defense: cryptographic peer authentication must remain meaningful when signaling is malicious.

Availability is not guaranteed against a malicious signaling service.

### A4 — Compromised TURN/relay infrastructure

Can observe relay traffic and metadata and may drop or alter transport packets.

Expected defense: application-layer end-to-end encryption and authentication independent of the relay.

### A5 — Stolen device, locked state

Attacker has physical possession but does not have legitimate unlocked access or platform credentials.

Expected defense: platform secure storage / hardware-backed protection where available, authenticated unlock gates, encrypted local state, key invalidation/revocation, and minimization of persistent plaintext.

### A6 — Stolen device, unlocked user state

Attacker acquires an already-unlocked or otherwise authenticated device.

This is a materially stronger threat. Eagle SHALL minimize exposure but does not claim that cryptographic protections can preserve secrets that are intentionally available to the currently authorized endpoint.

Expected defense: session locking, re-authentication for administrative/security-sensitive actions, platform-protected keys, and short-lived authorization where practical.

### A7 — Unprivileged co-resident malicious application

Can attempt to access shared resources, logs, clipboard, notifications, IPC surfaces, or other exposed application interfaces.

Expected defense: platform sandboxing, minimal exported interfaces, no secret logging, explicit IPC validation, and platform-specific hardening.

### A8 — Privileged OS compromise / root / administrator

Has broad control of the endpoint, process memory, storage, or input/output paths.

This is outside the cryptographic confidentiality guarantee.

### A9 — Malicious conversation peer

Is a legitimately authenticated device but intentionally abuses the application.

Expected defense: authorization boundaries, revocation, rate limits, device management, content handling policy, and clear user-visible identity state.

Eagle does not promise to make a willingly authorized peer unable to read content that was sent to that peer.

### A10 — Operational insider

Has excessive access to signaling, telemetry, infrastructure, or administrative systems.

Expected defense: least privilege, separation of duties, encrypted content, auditable administrative actions, minimized telemetry, access controls, and key separation.

### A11 — Well-resourced / nation-state network adversary

May possess substantial network visibility, traffic-analysis capability, exploitation resources, and long-term collection capacity.

Eagle will defend against protocol-level cryptographic attacks within the selected algorithm security margin and against active network manipulation.

Eagle does **not** claim protection against a fully compromised endpoint, privileged OS compromise, hardware implants, or complete global traffic-analysis resistance.

## 6. Threat scenarios

### T1 — Server substitutes a malicious device key

**Attack:** Signaling service returns an attacker-controlled public key as the peer device key.

**Impact:** MITM if identity/authentication is weak.

**Required controls:** identity binding, authenticated device enrollment, user-verifiable fingerprints/SAS, transcript binding, and explicit approval state.

### T2 — Captured message is replayed

**Attack:** Attacker retransmits a valid ciphertext.

**Impact:** duplicate command/message processing.

**Required controls:** authenticated sequence state, replay window, persistent state where required, duplicate rejection, and tests across restart/recovery boundaries.

### T3 — Protocol downgrade

**Attack:** Active network attacker offers an older or weaker protocol version.

**Impact:** weaker cryptographic properties.

**Required controls:** minimum-version policy, authenticated negotiation, no silent legacy fallback.

### T4 — Pairing QR is replayed

**Attack:** Old pairing authorization is presented after expiry or after it has already been consumed.

**Impact:** unauthorized device enrollment.

**Required controls:** nonce, expiry, single-use authorization state, identity binding, authenticated approval, and server-side revocation/consumption state.

### T5 — Signaling server reads message contents

**Attack:** Server operator or server compromise accesses stored/transmitted application payloads.

**Impact:** confidentiality loss.

**Required controls:** encryption keys exclusively controlled by endpoints; server stores only ciphertext and required routing metadata.

### T6 — Session state is partially compromised

**Attack:** Current session state is exposed temporarily.

**Impact:** attacker attempts to decrypt historical and future traffic.

**Required controls:** forward secrecy, ratcheting, documented post-compromise recovery, key erasure/rotation, and compromise-window analysis.

### T7 — Stale revoked device reconnects

**Attack:** A device that was revoked attempts to continue an old session.

**Impact:** unauthorized access.

**Required controls:** revocation epochs/authorization state, session binding to current device authorization, and explicit re-establishment rules.

### T8 — Malformed protocol input

**Attack:** Large, truncated, duplicated, invalid, or structurally malformed input is submitted.

**Impact:** crash, parser confusion, memory/resource exhaustion, or state corruption.

**Required controls:** strict length bounds, canonical parsing rules, typed errors, fuzz/property tests, and fail-closed parsing.

## 7. Out of scope / non-promises

Eagle does **not** promise cryptographic protection against:

1. A fully compromised endpoint operating system.
2. Root/administrator-level malware controlling the device.
3. Hardware implants or physical memory extraction beyond the platform threat model.
4. Screen photography, shoulder surfing, or observation of plaintext while it is intentionally displayed.
5. A malicious authorized conversation peer reading content delivered to that peer.
6. A user intentionally exporting or copying plaintext.
7. Complete metadata anonymity against the signaling service.
8. Perfect resistance to global traffic analysis based on timing, packet size, endpoint availability, or traffic volume.
9. Availability under a malicious signaling or relay service.
10. Security properties that a selected protocol does not actually provide.

These exclusions are deliberate and must be reflected in product/security claims.

## 8. Security requirements derived from the threat model

The following requirements are normative acceptance criteria for the secure-session protocol.

| ID | Requirement | Acceptance criterion |
|---|---|---|
| SR-01 | Authenticated key establishment | Peer identity is cryptographically authenticated before session establishment |
| SR-02 | Async establishment | A legitimate recipient can establish a session while the peer is temporarily offline, using approved pre-published key material or an equivalent mechanism |
| SR-03 | Forward secrecy | Compromise of current ratchet/session state cannot decrypt protected messages outside the documented compromise window |
| SR-04 | Post-compromise recovery | After attacker access ends, subsequent honest protocol progress can restore confidentiality/authenticity under documented assumptions |
| SR-05 | Replay protection | Replayed authenticated ciphertext is rejected deterministically |
| SR-06 | Reordering tolerance | Valid out-of-order messages are accepted within a bounded authenticated window when supported by the selected protocol |
| SR-07 | Downgrade resistance | Older/weak protocol modes cannot be silently selected by an active adversary |
| SR-08 | Identity continuity | Device identity changes require an explicit authorization/replacement event |
| SR-09 | Pairing expiry | Pairing authorization expires and cannot be reused after consumption |
| SR-10 | Key custody | Private identity/session keys remain within the Rust security boundary and platform secure storage boundary |
| SR-11 | State persistence | Security-critical replay/revocation state survives the persistence boundaries required by the protocol |
| SR-12 | Parser robustness | Malformed inputs fail closed without panic, unsafe memory access, or unbounded allocation |
| SR-13 | Algorithm agility control | Application code cannot arbitrarily select weaker cryptographic algorithms at runtime |
| SR-14 | Revocation | Revoked devices cannot silently resume previously authorized sessions |
| SR-15 | Testability | Each normative security property has negative, regression, and property/fuzz coverage before production |
| SR-16 | Resource bounds | Protocol inputs and stored state are subject to explicit bounded resource policies |

## 9. Requirements intentionally not used as hard protocol selectors

The following are desirable but are **not** assumed to be mandatory until product requirements explicitly require them:

- strong metadata hiding;
- global traffic-analysis resistance;
- cryptographic deniability as a formal product guarantee;
- group messaging;
- anonymous credentials;
- sealed-sender-like metadata minimization;
- protection against compromised endpoints.

They may influence protocol ranking, but they must not be treated as requirements merely because a candidate protocol advertises them.

## 10. Protocol evaluation criteria

The future Secure-Session Protocol ADR SHALL score candidate protocols against at least:

1. Async one-to-one session establishment.
2. Forward secrecy.
3. Post-compromise recovery / self-healing.
4. Peer/device identity binding.
5. Replay and out-of-order handling.
6. Device replacement and revocation compatibility.
7. Mature, maintained Rust implementation.
8. Independent security audit evidence.
9. License compatibility with Eagle's deployment model.
10. Android/iOS/Desktop integration feasibility.
11. State persistence and migration complexity.
12. Interoperability requirements and protocol maturity.
13. Implementation attack surface and operational complexity.
14. Cryptographic primitive quality and algorithm choices.
15. Ability to keep the Rust Core authoritative while exposing only a typed FFI surface.

No candidate is approved merely because it is "Signal-like".

## 11. Security testing gates

Before production promotion, the following categories MUST have implemented evidence:

- Build
- Unit
- Integration
- Cryptography
- Protocol
- Security
- Static analysis
- Dependency/license checks
- Regression
- Fuzz/property

Missing implementation is **PENDING**, never PASS.

Required negative cases include at minimum:

- replay;
- duplicate sequence;
- old sequence outside replay window;
- downgrade;
- identity mismatch;
- expired pairing;
- consumed pairing;
- revoked device;
- expired session;
- malformed ciphertext;
- malformed serialization;
- oversized input;
- sequence exhaustion;
- persistence corruption;
- restart/resume edge cases.

## 12. Consequences

### Positive

- Protocol choice becomes evidence-driven rather than preference-driven.
- Claims about server compromise and endpoint compromise become explicit.
- Security requirements can be tested and traced to implementation.
- Protocol, FFI, storage, and product layers remain independently reviewable.
- Metadata and availability limitations are documented before product claims are written.

### Negative

- More up-front analysis is required.
- The strongest properties may increase implementation complexity and storage requirements.
- The threat model may rule out superficially attractive protocols.
- Some requirements cannot be met purely by cryptography and require platform or operational controls.

## 13. Decision

**Proposed security posture for Eagle:**

- Protect message/file plaintext from passive and active network attackers, signaling-server compromise, and relay compromise.
- Require strong device identity binding and explicit pairing authorization.
- Require asynchronous one-to-one session establishment.
- Require forward secrecy, replay resistance, downgrade resistance, and documented post-compromise recovery.
- Treat ordinary unprivileged co-resident applications as a platform-hardening threat.
- Do not claim cryptographic protection against a fully compromised endpoint or privileged OS.
- Do not promise complete metadata anonymity or global traffic-analysis resistance.
- Do not select a secure-session protocol until a candidate is evaluated against these requirements and the evidence gates above.

The next ADR may therefore compare candidates such as **vodozemac/Olm, MLS/OpenMLS, Noise-based protocols, and other independently maintained protocol implementations** using SR-01 through SR-16. That comparison is intentionally deferred until this threat model is reviewed.

## 14. Evidence references

Current external evidence relevant to future protocol evaluation includes:

- Signal libsignal currently declares AGPLv3 / AGPL-3.0-only.
- vodozemac is a Rust implementation of Olm/Megolm, describes Olm as a Double Ratchet implementation with forward secrecy and self-healing, and documents a Least Authority audit with no significant findings.
- Matrix has deprecated libolm in favor of vodozemac and recommends the Rust implementation for current Matrix cryptography.
- OpenMLS implements the MLS protocol specified by RFC 9420.

These external facts do not constitute Eagle security approval; they are inputs to the separate protocol-selection ADR.

## References

- https://github.com/signalapp/libsignal
- https://github.com/matrix-org/vodozemac
- https://docs.rs/vodozemac/latest/vodozemac/
- https://github.com/openmls/openmls
- https://www.rfc-editor.org/rfc/rfc9420
