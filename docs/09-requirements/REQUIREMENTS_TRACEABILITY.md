# Eagle — Requirements Traceability

Status: Baseline created; authoritative V1 requirements are not yet frozen.

## Traceability states

- Verified — supported by current repository evidence.
- Derived — directly inferred from verified evidence.
- Proposed — intended requirement, awaiting authority/evidence.
- Pending — source or acceptance evidence is missing.
- Rejected — explicitly not accepted.

## Current requirements baseline

| ID | Requirement / constraint | State | Evidence / next action |
|---|---|---|---|
| REQ-001 | Android application bootstrap must remain buildable | Verified | app/ and Android Gradle configuration |
| REQ-002 | Security-sensitive development must preserve repository controls | Verified | AGENTS.md, SECURITY.md, workflow policy |
| REQ-003 | Historical inputs require provenance before promotion | Verified | source index + audit/intake policy |
| REQ-004 | Mature reusable components should be evaluated before replacement implementations | Accepted policy | reuse-first register |
| REQ-005 | Cryptographic primitives should not be reinvented without exceptional justification | Accepted policy | decision DEC-003 |
| REQ-006 | Production E2EE messaging | Proposed / Pending | product source and V1 acceptance criteria required |
| REQ-007 | Device cryptographic identity | Proposed / Pending | identity lifecycle contract required |
| REQ-008 | Authenticated protocol envelope | Proposed / Pending | protocol contract and test vectors required |
| REQ-009 | Secure local persistence | Proposed / Pending | data model + threat model required |
| REQ-010 | Transport between devices | Proposed / Pending | transport trust boundary and offline behavior required |
| REQ-011 | PrivateMesh discovery/routing/relay | Proposed / Pending | Mesh threat model + routing contract required |
| REQ-012 | Cross-platform support | Proposed / Pending | platform matrix and architecture decision required |
| REQ-013 | Production-grade CI/security/release evidence | Proposed / Pending | gates and reproducible evidence required |
| REQ-014 | Recovery/rotation/revocation semantics | Proposed / Pending | identity/key lifecycle decision required |

## Traceability rule

No requirement may become an implementation claim until it has:

1. authoritative source;
2. acceptance criteria;
3. architecture impact;
4. security impact;
5. implementation location;
6. executable verification.

## Current blocking requirements

The following prevent a Production Ready claim:

- V1 product requirements;
- trust boundaries;
- threat model;
- identity/key lifecycle;
- protocol contract;
- production dependency decisions;
- live CI/release evidence;
- behavior-level product tests.

## Historical-source intake

REQ-006 through REQ-014 must be reconciled against the supplied historical archives when those archive bytes become retrievable. Until then they remain Proposed/Pending and must not be treated as confirmed historical requirements.
