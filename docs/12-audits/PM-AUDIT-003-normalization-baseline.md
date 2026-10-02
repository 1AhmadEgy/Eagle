# PM-AUDIT-003 — Normalization & Hardening Baseline

## Status

IN REVIEW

## Scope

This internal review records documentation and protocol-consistency issues identified during the PrivateMesh normalization pass. It is not an external security audit.

## Findings

| ID | Area | Finding | Severity | Required Action | Status |
|---|---|---|---|---|---|
| AUD-003-001 | Registry | 0x6201–0x6205 contain conflicting semantic assignments across drafts | HIGH | Adopt PM-SPEC-011 authority model | OPEN |
| AUD-003-002 | Trust | compute_trust_score() is referenced without a defined scoring model | HIGH | Replace with evidence → freshness → policy → decision → state flow unless scoring is formally specified | OPEN |
| AUD-003-003 | Crypto | PQXDH KDF/transcript construction is underspecified | CRITICAL | Specify canonical encoding, salt/info/domain separation, transcript binding, algorithm IDs and failure semantics | OPEN |
| AUD-003-004 | Wire | Header fields total 50 bytes before extensions while an earlier example states 44 bytes | HIGH | Define exact length semantics and regenerate vectors | OPEN |
| AUD-003-005 | Queue | Exactly-once delivery is not enforceable over crash/retry transport | HIGH | Specify at-least-once transport plus idempotent acceptance/ack | OPEN |
| AUD-003-006 | Recovery | Recovery authority and epoch/generation semantics are incomplete | HIGH | Make recovery authorization cryptographically explicit and invalidate old authorizations by epoch | OPEN |
| AUD-003-007 | Deletion | Physical secure erase on flash is not universally guaranteed | HIGH | Define cryptographic completion separately from best-effort physical cleanup | OPEN |
| AUD-003-008 | Test vectors | Existing crypto/wire examples are placeholders rather than executable deterministic vectors | CRITICAL | Generate canonical vectors from implementation | OPEN |
| AUD-003-009 | Transport | Tor wording must be v3-only and must distinguish transport/privacy profile from cryptographic protocol | MEDIUM | Normalize PM-SPEC-007 | OPEN |
| AUD-003-010 | Config | Parameter scopes conflict for padding, attachment/message limits, and queue size | HIGH | Establish one authoritative baseline and registry | OPEN |
| AUD-003-011 | Standards | Some cited RFC and audit references require verification before publication as normative evidence | MEDIUM | Verify source titles, versions, applicability and provenance | OPEN |
| AUD-003-012 | Claims | Terms such as certified, sealed, complete, or absolute privacy claims require evidence or removal | HIGH | Use status language tied to documented evidence | OPEN |

## Release gate

PM-AUDIT-003 is not closed until every CRITICAL and HIGH finding has:

- a canonical specification change;
- implementation impact identified;
- deterministic tests or vectors where applicable;
- review evidence;
- traceability to the relevant security invariant.

## Next review

Re-run this matrix after PM-SPEC-001 through PM-SPEC-011 have been reconciled against the repository implementation.
