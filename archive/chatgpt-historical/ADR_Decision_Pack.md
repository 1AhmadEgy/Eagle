# Eagle / PrivateMesh — Architecture Decision Records (ADR) Pack
**Version:** 1.0  
**Date:** 2026-09-30  
**Status:** Proposed decisions for review; none are approved by this document.

## Purpose and decision rules
This pack converts open issues OI-001–OI-008 into reviewable decision records. It does not silently resolve them. The project owner and designated security/architecture reviewers must record the decision, rationale, evidence, approvers, and date. Any selected option must be reflected in the architecture baseline, threat model, tests, and release gates.

## ADR-001 — Signal implementation and version (OI-001)
- **Status:** Proposed — Open
- **Context:** The protocol direction references Signal-style session establishment and Double Ratchet, but the exact implementation, version, license, platform support, maintenance posture, and integration boundary are not frozen.
- **Decision required:** Select a maintained, auditable implementation and exact version/commit, or explicitly approve a documented implementation strategy.
- **Options to evaluate:**
  1. Adopt a maintained compatible library with pinned version and reviewed platform bindings.
  2. Implement only the required protocol components in the shared core, subject to independent cryptographic review.
  3. Defer protocol integration until a qualifying implementation passes a proof of concept.
- **Evaluation criteria:** Protocol conformance; independent review; memory safety; license compatibility; platform support; update cadence; API stability; test vectors; interoperability; vulnerability response.
- **Security constraints:** No bespoke cryptographic primitives; secrets remain within approved core/platform boundaries; fail closed on invalid state or downgrade.
- **Required evidence:** License and maintenance review, dependency audit, threat-model update, test-vector results, interoperability PoC, and independent review plan.
- **Acceptance:** Named implementation/version, owner, upgrade policy, platform matrix, reproducible tests, and signed ADR approval.
- **Consequences:** Affects core interfaces, build dependencies, platform integration, and G2 exit.

## ADR-002 — PQXDH profile and integration (OI-002)
- **Status:** Proposed — Open
- **Context:** PQXDH is referenced, but the exact profile, supported key material, negotiation behavior, compatibility, and test corpus are not frozen.
- **Decision required:** Define the precise PQXDH profile and its relationship to identity, prekeys, session setup, and future migration.
- **Options to evaluate:** Adopt a maintained interoperable profile; stage a classical baseline with a formally specified post-quantum migration path; or defer feature exposure until implementation and interop evidence are sufficient.
- **Evaluation criteria:** Published specification alignment, implementation maturity, downgrade resistance, key lifecycle, cross-platform consistency, performance, recovery implications, and migration safety.
- **Security constraints:** No silent fallback; negotiation must be authenticated and downgrade-resistant; protocol versions and key identifiers must be explicit.
- **Required evidence:** Formal profile, state diagrams, test vectors, negative tests, interop results, threat analysis, and cryptographic review.
- **Acceptance:** Profile and compatibility policy frozen, tests pass on target platforms, and reviewers approve G2.
- **Consequences:** Controls session setup, identity/key records, compatibility, and migration design.

## ADR-003 — V1 transport and offline behavior (OI-003)
- **Status:** Proposed — Open
- **Context:** Direct connectivity, ICE, QUIC, and TURN/relay appear as architectural options. V1 scope and fallback behavior are not final.
- **Decision required:** Specify transport modes, connection establishment, fallback conditions, offline delivery, and user-visible failure behavior for V1.
- **Options to evaluate:** Direct-only; direct plus relay fallback; or direct/relay plus a bounded store-and-forward service.
- **Evaluation criteria:** Privacy metadata, reliability, latency, mobile battery/data use, operational cost, censorship/network constraints, abuse resistance, and platform feasibility.
- **Security constraints:** Transport security does not replace end-to-end message encryption; fallback cannot weaken identity or cryptographic guarantees.
- **Required evidence:** Sequence diagrams, network threat model, prototype measurements, reconnect/offline tests, and relay data inventory.
- **Acceptance:** V1 transport matrix and fallback policy approved; service-side data and retention are specified.
- **Consequences:** Changes backend scope, availability expectations, telemetry, and data retention.

## ADR-004 — Server ciphertext retention (OI-004)
- **Status:** Proposed — Open
- **Context:** Minimal-backend principles coexist with possible temporary ciphertext buffering. Retention, deletion, backups, and access controls require an explicit contract.
- **Decision required:** Decide whether server-side ciphertext buffering exists, and if so define exact lifecycle and controls.
- **Options to evaluate:** No server persistence; short-lived encrypted queue; or user-configurable/contract-defined bounded retention.
- **Evaluation criteria:** Delivery reliability, metadata exposure, abuse handling, storage cost, deletion verifiability, backup behavior, and legal/operational obligations.
- **Security constraints:** Server cannot decrypt content; no plaintext or client private/session keys; strict service identity, least privilege, retention expiry, and auditable deletion.
- **Required evidence:** Data-flow and field inventory, retention/deletion tests, backup analysis, access model, and privacy/threat review.
- **Acceptance:** Data contract specifies purpose, fields, encryption, TTL, access, backup treatment, deletion evidence, and incident handling.
- **Consequences:** Defines backend storage architecture and what privacy statements can accurately claim.

## ADR-005 — Identity and device-trust state machine (OI-005)
- **Status:** Proposed — Open
- **Context:** Account, device, identity, and session are distinct concepts, but trust transitions and ambiguous-state behavior need formalization.
- **Decision required:** Approve the trust states, transitions, evidence required for each transition, revocation, and recovery interactions.
- **Options to evaluate:** Explicit user-confirmed device trust; cryptographic device attestation where available; or a hybrid with platform-specific assurance labels.
- **Evaluation criteria:** Threat coverage, usability, cross-platform parity, revocation speed, resistance to account takeover, and false acceptance/rejection.
- **Security constraints:** Login alone must not imply device trust; unknown or inconsistent state fails closed; trust changes are auditable without logging secrets.
- **Required evidence:** State-transition table, abuse cases, UI flows, revocation tests, and security review.
- **Acceptance:** All states/transitions have guards, error behavior, audit events, and tests.
- **Consequences:** Affects pairing, session authorization, notifications, and incident response.

## ADR-006 — Device pairing and linking (OI-006)
- **Status:** Proposed — Open
- **Context:** Multi-device use requires a controlled way to establish trust and define what is shared between devices.
- **Decision required:** Specify pairing ceremony, mutual authentication, trust transfer, key handling, device limits, and revocation.
- **Options to evaluate:** QR/code-based proximity pairing; authenticated approval from a trusted device; or a hybrid with a recovery path.
- **Evaluation criteria:** Phishing resistance, usability, accessibility, device compromise containment, platform support, and offline constraints.
- **Security constraints:** Pairing must be explicit and bound to device identity; no transfer of long-term secrets without a reviewed protocol; revoke must invalidate future authorization.
- **Required evidence:** Pairing protocol, threat model, sequence diagram, replay/phishing tests, and lost-device scenarios.
- **Acceptance:** Approved ceremony and key lifecycle, documented device cap/policy, tested revocation and recovery.
- **Consequences:** Impacts identity model, local storage, session lifecycle, and UX.

## ADR-007 — Account recovery versus data recovery (OI-007)
- **Status:** Proposed — Open
- **Context:** Account access restoration and recovery of encrypted message history are different capabilities and must not be conflated.
- **Decision required:** Define supported recovery goals, proofing factors, recovery secrets, trusted contacts if any, and what happens when keys/devices are lost.
- **Options to evaluate:** Account-only recovery with no history recovery; user-held recovery material; or a separately designed multi-party recovery mechanism.
- **Evaluation criteria:** Account takeover resistance, irrecoverable-loss risk, user comprehension, availability, backup exposure, and support burden.
- **Security constraints:** Account recovery cannot silently decrypt historical content; no universal server-held recovery key; recovery actions require explicit authorization and audit.
- **Required evidence:** Threat analysis, key lifecycle and cryptographic design, usability research, loss/compromise tests, and precise product claims.
- **Acceptance:** Recovery guarantees and limitations are explicit, tested, and approved by security and product owners.
- **Consequences:** Determines whether history can be restored and what assurances can be made to users.

## ADR-008 — Deletion guarantees and verification (OI-008)
- **Status:** Proposed — Open
- **Context:** Deletion spans local databases, attachments, caches, queued ciphertext, backups, replicas, and recovery material. A UI delete action alone is not proof of erasure.
- **Decision required:** Define deletion scope, timing, cryptographic erasure applicability, backup expiry, and evidence available to users/operators.
- **Options to evaluate:** Logical deletion plus bounded expiry; cryptographic erasure where key design supports it; or a hybrid with documented residual copies.
- **Evaluation criteria:** Technical verifiability, platform storage behavior, backup lifecycle, legal requirements, reliability, and user expectations.
- **Security constraints:** Do not promise immediate physical erasure where the platform or backup layer cannot prove it; deletion must not create a recovery bypass or leave accessible key material.
- **Required evidence:** Data inventory, deletion tests across storage layers, backup/replica policy, key-destruction evidence, and user-facing wording review.
- **Acceptance:** Scope and residual limitations documented; tests demonstrate defined behavior; security/privacy reviewers approve claims.
- **Consequences:** Affects storage schema, backend queues, backup policy, recovery, and compliance messaging.

## Decision register
| ADR | Related issue | Current status | Decision owner | Approval date |
|---|---|---|---|---|
| ADR-001 | OI-001 | Open | To assign | — |
| ADR-002 | OI-002 | Open | To assign | — |
| ADR-003 | OI-003 | Open | To assign | — |
| ADR-004 | OI-004 | Open | To assign | — |
| ADR-005 | OI-005 | Open | To assign | — |
| ADR-006 | OI-006 | Open | To assign | — |
| ADR-007 | OI-007 | Open | To assign | — |
| ADR-008 | OI-008 | Open | To assign | — |

## Approval record template
For each ADR, append:
- **Decision:** (selected option and exact scope)
- **Rationale:** (evidence, trade-offs, rejected alternatives)
- **Security/privacy impact:**
- **Implementation owner and target milestone:**
- **Tests and evidence links:**
- **Reviewers / approvers:**
- **Date and baseline version:**
- **Supersedes / superseded by:**
