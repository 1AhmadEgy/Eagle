# Eagle — Private, Security-First Communication Platform

Eagle is a security-first private communication platform focused on strong end-to-end confidentiality, cryptographic device identity, explicit device trust, and a minimal-trust service architecture.

## Project status

This repository is the canonical engineering repository for the Eagle project.

The project is currently in **foundation / active research and development**. Architecture, protocol profiles, identity and recovery semantics, transport scope, server-side retention, and deletion guarantees must be explicitly approved before they are treated as implementation contracts.

**Release status: NOT RELEASE READY.**

## Engineering principles

- Security and privacy are first-class requirements.
- Do not invent cryptographic protocols or security primitives.
- Prefer maintained, auditable, tested components over bespoke implementations.
- Separate application security, end-to-end protocol security, transport security, and platform security.
- Treat identity, device identity, device trust, sessions, recovery, and deletion as separate security domains.
- Fail closed on security-critical ambiguity and invalid state.
- AI/automation may recommend or analyze; deterministic policy and authorization control execution.
- Every security claim must be traceable to implementation and test evidence.
- Historical material is evidence/provenance input, not automatic authority.

## Evidence-driven development

The project uses the following controlled path:

```text
Inventory
  → Provenance
  → Classification
  → Triage
  → Version comparison
  → Canonical reference
  → Requirements / gap matrix
  → Correction
  → Implementation
  → Tests
  → Security audit
  → Independent verification
  → Release Gate
```

A document, library, protocol, archive, or historical filename does not become authoritative merely because it is labelled FINAL, MASTER, CONSOLIDATED, or similar.

## Security baseline

The engineering baseline includes:

- explicit least-privilege CI permissions;
- immutable pinning of third-party CI actions where required;
- secret handling through repository secret mechanisms rather than source control;
- provenance tracking and exact-SHA evidence;
- deterministic Test Lab evidence;
- dependency and supply-chain review;
- security policy verification gates;
- no automatic closure of unresolved architectural/security decisions;
- no automatic release approval from documentation alone.

## Critical open decisions

The following areas remain controlled decision gates until their exact implementation contract and evidence are approved:

- Signal-family / session protocol implementation and version;
- PQXDH profile and integration boundary;
- V1 transport and offline behavior;
- server-side ciphertext retention;
- identity and device-trust state machine;
- device pairing/linking;
- account recovery versus encrypted-data recovery;
- deletion guarantees across local storage, caches, queues, replicas, and backups.

These are not silently resolved by this README.

## Test and release policy

Feature readiness requires, as applicable:

- specification;
- implementation;
- unit tests;
- integration tests;
- negative/security tests;
- regression tests;
- reproducible evidence;
- review.

Release readiness additionally requires supply-chain verification, release verification, security review, and the required independent/external review for the applicable release scope.

## Historical material

Historical uploads and project archives are preserved for provenance and reconciliation. They must be inspected and promoted through the project's canonicalization rules before becoming implementation authority.

## Ownership and repository visibility

The project documentation currently records an intended private / all-rights-reserved posture, while the GitHub repository is presently public. This mismatch is a governance item requiring owner-controlled repository settings and legal confirmation. No ownership or licensing claim is inferred from repository state alone.

## Documentation

Authoritative project decisions, requirements, architecture, security controls, testing evidence, provenance, and release gates are maintained under `docs/`.

See the repository history and project registers for the evidence trail.
