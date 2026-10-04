# Eagle Corpus Canonicalization Status — 2026-10-05

## Scope
This record consolidates the repository-backed corpus audit for architecture/technical execution. It does not elevate historical or branch-local proposals to canonical status.

## Evidence hierarchy
1. Current Git object and file content.
2. Accepted ADR/architecture records with explicit approval state.
3. Verified branch-local implementation evidence.
4. Historical archives and ChatGPT artifacts.
5. Referenced-but-not-retrieved material.

## Canonicality rules
- A branch name, archive filename, version number, or document title is not evidence of canonicality.
- Historical material remains evidence until an explicit current decision supersedes it.
- Platform scope is canonical only within the scope explicitly assigned by PLATFORMS.md.
- Implementation branches remain candidate evidence until reconciled, reviewed, tested, and merged through the project gates.
- Unretrieved artifacts remain Pending.
- Conversation-only claims are not implementation facts.

## Current canonical set
| Domain | Current reference | Authority |
|---|---|---|
| Repository state | main @ 6c46bf53fce06ee7f7b5b2bf35c720ab5bcb7dee | Current Git state |
| Platform scope | docs/03-architecture/PLATFORMS.md | Accepted reference baseline |
| General planning | docs/03-architecture/CANONICAL_PLANNING_BASELINE.md | Proposed; not implementation authority |
| Security policy | SECURITY.md + docs/04-security/SECURITY_BASELINE.md | Baseline policy |
| Requirements | docs/09-requirements/REQUIREMENTS_TRACEABILITY.md | Incomplete |
| Verification | docs/07-verification/VERIFICATION_REGISTER.md | Partial |

## Candidate implementation references
- implementation/stack-baseline-2026-10
- execution/identity-trust-foundation-v1
- execution/phase-1-security-kernel-verified-2026-10-04

These are evidence candidates, not canonical implementation baselines.

## Critical conflicts
1. Current P2P-only project constraint conflicts with historical server-mediated transport proposals. Historical proposals must not be promoted.
2. Accepted platform strategy describes Rust Security Core + KMP + platform adapters, while main currently contains only the minimal Android app skeleton. Architecture intent is not implementation evidence.
3. Historical artifacts and current branch-local work contain overlapping decisions. Resolution requires decision-by-decision traceability, not package-level replacement.
4. Some referenced archives were registered but not independently re-retrieved in the current audit.

## Release implication
Canonical resolution is incomplete. Production release remains blocked until requirements, security architecture/ADR decisions, implementation evidence, adversarial tests, provenance, and release gates are all closed.

## Next deterministic execution sequence
Inventory → Provenance → Conflict Register → Canonical Resolution → Requirements Traceability → Architecture/ADR approval → Stack/component due diligence → Implementation → Tests → Security audit → Verification → Release Gate.
