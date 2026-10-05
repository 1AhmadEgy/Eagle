# Eagle — Identity & Trust Threat Model

**Scope:** account identity, device identity, pairing, trust, revocation, recovery, platform assurance  
**Baseline:** `main` @ `abfc263e6ac28ff7b19a40a4d8e1da93c565a6e9`  
**Execution candidate:** `implementation/security-first-v1-2026-10-05` / PR #77  
**Posture:** security-first / fail-closed / P2P-only

## 1. Assets

| Asset | Protection goal |
|---|---|
| Account identity authority | authenticity, integrity, continuity |
| Device identity private key | confidentiality, integrity, non-exportability where available |
| Account-to-device membership | authenticity, freshness, revocation |
| Pairing transcript | integrity, freshness, endpoint binding |
| Trust state | integrity, authorization correctness |
| Trust epoch | monotonicity, freshness |
| Recovery authority/material | confidentiality, authorization, non-replay |
| Identity-change evidence | authenticity, explicit user verification |
| Audit events | integrity, privacy, non-secret observability |

## 2. Trust boundaries

```text
[User]
  |
  v
[Platform UI / OS APIs]
  |
  v
[Platform Adapter]
  |
  v
[Rust Security Core]
  |
  +------ [P2P Peer]
  |
  +------ [Local Encrypted Storage]
  |
  +------ [Optional Rendezvous/Service]
```

Security-critical decisions cross into the Rust Security Core and must not be reimplemented in UI code.

## 3. Security assumptions

The model assumes:

1. The cryptographic protocol and key-management modules provide their specified primitives correctly.
2. Long-term private keys are kept inside an approved secure storage boundary.
3. An attacker may fully control the network.
4. An attacker may possess a malicious Eagle device.
5. A legitimate device may be lost, stolen, or compromised.
6. A user may accidentally approve a malicious pairing if the ceremony is confusing.
7. P2P peers may be offline during a revocation event.
8. Platform attestation can be absent, stale, unavailable, or semantically misunderstood.
9. Recovery material can be copied or replayed unless the recovery protocol prevents this.

## 4. Threat register

| ID | Threat | Impact | Primary control | Residual status |
|---|---|---|---|---|
| IDT-01 | Account impersonation | Full identity takeover | self-certifying identity + authenticated membership | Pending protocol proof |
| IDT-02 | Device impersonation | Unauthorized device access | per-device identity + membership proof | Baseline implemented |
| IDT-03 | Pairing MITM | Wrong device becomes trusted | mutual authentication + human-verifiable short code | Pending protocol proof |
| IDT-04 | QR / short-code phishing | User approves attacker device | cryptographic session binding + explicit approval + human-verifiable comparison | Protocol implementation pending |
| IDT-05 | Pairing replay | Reuse old approval | single-use + expiry + exact session/transcript binding + epoch binding | Baseline boundary implemented; protocol proof pending |
| IDT-06 | Identity substitution | Contact silently replaced | quarantine + exact candidate match + explicit reverification | Baseline boundary implemented |
| IDT-07 | Stale trust state | Revoked device regains authority | trust epoch + fail-closed guards | Baseline implemented |
| IDT-08 | Offline revocation race | Continued temporary access | local revocation + reconciliation before privileged operation | Pending end-to-end evidence |
| IDT-09 | Lost device | Unauthorized use of device | revocation + local lock/security policy | Partial |
| IDT-10 | Compromised device | Key/session abuse | compartmentalized membership + revocation + protocol PCS | Pending protocol/security review |
| IDT-11 | Recovery takeover | Attacker gains account authority | separate recovery protocol + explicit authorization | Pending ADR-007 |
| IDT-12 | Recovery decrypts history unexpectedly | Historical disclosure | account/data recovery separation | Baseline implemented |
| IDT-13 | Attestation over-trust / replay | False security decision or reused evidence | assurance is advisory; challenge/freshness and provenance validation at integration boundary | Baseline specified |
| IDT-14 | Identifier correlation | Privacy loss | self-certifying IDs; no hardware IDs | Baseline specified |
| IDT-15 | Audit secret leakage | Credential compromise | safe redaction and event schema | Baseline implemented |
| IDT-16 | Rollback / epoch rollback | Restoration of obsolete trust | monotonic epoch + authoritative state transition + persistent rollback detection | Persistent storage integration pending |
| IDT-17 | Downgrade | weaker protocol or key policy | authenticated version negotiation | Pending protocol ADR |
| IDT-18 | AI confused deputy | Unauthorized trust action | AI outside deterministic policy | Baseline specified |

## 5. Abuse cases

### UC-01 — Malicious new device

Attacker attempts to introduce a new DeviceID and membership statement.

Required result:

```text
UNKNOWN/PENDING
     ↓
no self-promotion
     ↓
explicit authorized approval required
```

### UC-02 — Replayed pairing

Attacker replays a previously accepted pairing transcript.

Required result:

```text
consumed/expired context → DENY
```

### UC-03 — Revoked device reconnects

A device revoked while offline reconnects with stale state.

Required result:

- stale epoch cannot authorize new privileged operations;
- reconciliation occurs before new authorization;
- revoked membership cannot silently reactivate.

### UC-04 — Account recovery after device compromise

An attacker presents account recovery evidence after a device was revoked.

Required result:

- recovery policy evaluates current account state;
- revoked membership remains revoked;
- historical message keys are not recovered unless separately authorized.

## 6. Security invariants

- Identity authenticity is cryptographic, not username-based.
- Device authorization is explicit and stateful.
- Pairing is time-bounded and single-use.
- Revocation is monotonic.
- Historical data recovery is independent from account recovery.
- Platform assurance never bypasses account authorization.
- No network service is the root of P2P trust.
- AI cannot bypass or rewrite security policy.

## 7. Required evidence

Before Identity & Trust can pass its release gate, evidence is required for:

1. formal identity/key hierarchy review;
2. pairing transcript security review;
3. QR/code phishing usability tests;
4. replay and cross-account replay tests;
5. identity-key substitution tests;
6. offline revocation convergence tests;
7. recovery abuse tests;
8. storage rollback tests;
9. cross-platform authorization parity;
10. independent security review.

## 8. Residual risk policy

A residual risk may remain only when:

- the risk is explicitly documented;
- the affected capability is scoped;
- no security invariant is violated;
- compensating controls exist;
- the release gate explicitly accepts the residual risk.

Security-critical unknowns are **PENDING**, not PASS.

## 9. Deep-research-derived controls

The current research pass strengthens the model with the following requirements:

1. A displayed or manually entered pairing code is not sufficient phishing resistance by itself. Final authorization must be cryptographically bound to the exact authenticated P2P session/transcript.
2. Human comparison of a short value remains a user-facing anti-phishing control, but it cannot replace cryptographic endpoint binding.
3. A key/identity change keeps the prior verified identity authoritative until exact-candidate reverification succeeds.
4. Any hardware/app attestation must be checked for freshness/challenge binding and revocation where the platform supplies those signals; attestation remains separate from Eagle account trust.
5. Device/authenticator uniqueness must be enforced per account context; a valid authenticator must not be reusable across unrelated accounts.
6. Local trust epochs may advance only through an authoritative state transition. Arbitrary remote/future epoch observation must not mutate local authority.
7. Optional key-transparency-style consistency evidence may detect key swaps, but cannot become an implicit centralized root of trust in the P2P-only architecture.

These controls are documented in `docs/06-security/IDENTITY_TRUST_RESEARCH_2026-10-05.md`.

## 10. Ownership boundary

This threat model governs Identity & Trust. Protocol, cryptography, key management, storage, network, and platform-specific threat models remain authoritative for their own boundaries.

Any change that crosses those boundaries requires cross-linking the affected ADRs and evidence.
