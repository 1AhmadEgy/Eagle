# G0-A — Canonical Reconciliation Status

> **Snapshot:** `main@46aa86b6d71d34396b86b35124392cb3ba49c2e9`
> **Date:** 2026-10-07
> **Purpose:** reconcile existing project records without creating a parallel taxonomy.

## 1. Canonical authority

```text
Repository: 1AhmadEgy/Eagle
Branch: main
Commit: 46aa86b6d71d34396b86b35124392cb3ba49c2e9
Tree: a7718840805359b042ec20a1781daaa5531ee47f
```

The exact `main` tree is the implementation authority for Sprint 0.

## 2. Record mapping

| Requested record | Actual canonical record on main | G0-A action |
|---|---|---|
| REPOSITORY-INVENTORY.md | No file with this exact name was found | **Do not create parallel taxonomy.** Use `docs/MASTER_PROJECT_SOURCE_INDEX.md` plus exact Git tree/commit evidence as the canonical inventory coordination point. |
| FILE_PROVENANCE_REGISTER.md | `docs/FILE_PROVENANCE_REGISTER.md` | Reconciled authority model while preserving the historical 44-file inventory. |
| ADR_INDEX.md | `docs/03-architecture/ADR_INDEX.md` | Reconciled lifecycle/status authority without renumbering ADRs. |
| VERIFICATION_REGISTER.md | `docs/07-verification/VERIFICATION_REGISTER.md` | Reconciled PR/main scope and exact-head evidence rules. |
| README.md | `README.md` | Replaced stale AI Studio identity with the current canonical implementation/release state. |
| DECISION_LOG.md | `docs/01-decisions/DECISION_LOG.md` | Recorded G0-A governance invariants and DOC-DRIFT-001. |
| Platform reference | `docs/03-architecture/PLATFORMS.md` | Corrected current implementation path to `app/` and excluded Web/Wasm. |

## 3. Current implementation truth

The canonical main snapshot is an Android implementation skeleton plus repository documentation, CI/scripts, and security/test scaffolding.

The following are not established as production implementation on this exact main commit:

- Rust Security Core production workspace;
- KMP shared production layer;
- iOS implementation;
- Desktop implementation;
- production E2EE provider/runtime;
- production P2P runtime;
- production release verification.

## 4. Candidate evidence

| Candidate | Status | Rule |
|---|---|---|
| PR #45 | VERIFIED_CANDIDATE | Valid only for its exact PR/head evidence; not main evidence until merged and re-run |
| PR #77 | NON-CANONICAL IMPLEMENTATION CANDIDATE | Decompose requirement → ADR → contract → implementation → test → evidence |
| PR #89 | SNAPSHOT-DEPENDENT DOCUMENTATION CANDIDATE | Refresh after main changes |
| PR #91 | SNAPSHOT-DEPENDENT AUDIT CANDIDATE | Refresh after main changes |

## 5. Governance invariants

```text
PR PASS ≠ main PASS

PROPOSED
  ≠ ACCEPTED
  ≠ IMPLEMENTED
  ≠ VERIFIED
  ≠ RELEASED
```

No document, branch, PR, archive, or conversation can promote a state without the required acceptance/evidence gate.

## 6. G0-A status

| Area | Status |
|---|---|
| Exact canonical main snapshot | PASS |
| Existing-record reconciliation | PASS on this branch |
| Provenance preservation | PASS |
| ADR authority/lifecycle | PASS on this branch |
| Verification PR/main separation | PASS on this branch |
| README implementation truth | PASS on this branch |
| Platform path drift | Documented; DOC-DRIFT-001 remains OPEN |
| Branch governance | BLOCKED / administrative |
| PR #45 merge | PENDING |
| Exact-head verification after merge | NOT STARTED |
| Sprint 0 | OPEN |
| Sprint 1 | BLOCKED |
| Release | NO-GO |

## 7. Exit criteria for G0-A

G0-A may be closed only after:

1. this reconciliation is accepted;
2. the updated existing records are merged to canonical `main`;
3. branch governance is enabled;
4. PR #45 is reviewed and merged under that governance;
5. a new main SHA is established;
6. G0-D exact-head verification is executed on that new SHA.

Until then, this file is a reconciliation candidate, not a claim that canonical `main` has been repaired.

## 8. Safety boundary

This reconciliation does not introduce product code, cryptographic protocol choices, key-management choices, transport implementation, or release approval.
