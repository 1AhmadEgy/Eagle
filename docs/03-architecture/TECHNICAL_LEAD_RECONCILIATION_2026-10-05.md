# Technical Lead Reconciliation — 2026-10-05

## 1. Inventory result

Live GitHub branch inventory contained 64 branches including `main` and the current architecture branch.

All 62 pre-existing non-main branches were compared directly with `main`; the remaining comparison branch is the current Technical Lead branch and `main` is the authority baseline.

Result across the 62 pre-existing non-main branches:
- 45 diverged
- 12 ahead of main
- 5 behind main

This makes branch names unsafe as canonicality evidence. Canonicality is determined from ancestry, scope, merge state, and evidence.

## 2. Canonical repository baseline

`main` at commit `abfc263e6ac28ff7b19a40a4d8e1da93c565a6e9` is the current repository authority.

Older PRs based on `6c46bf53fce06ee7f7b5b2bf35c720ab5bcb7dee` are historical candidates and must not be treated as current-base changes without revalidation.

## 3. Classification rules

### Canonical
Current `main` only, until a reviewed PR is merged.

### Canonical candidate
A focused PR/branch based on current `main`, with a single technical scope and explicit verification evidence.

### Supporting
A specialized branch whose useful artifacts can be reused after conflict/scope review.

### Superseded
A branch explicitly replaced by a cleaner/current candidate.

### Historical
Old branches that materially diverge from current main and are no longer required for active implementation.

## 4. High-value candidates

| Area | Candidate | Disposition |
|---|---|---|
| Identity & Trust | PR #69 / `execution/identity-trust-clean-v1` | canonical candidate; focused, current-base |
| Rust Security Core | PR #65 / `execution/rust-core-security-kernel-complete-2026-10-05` | canonical candidate for specialty implementation, current-base, verification pending |
| Crypto/key management | PR #70 / `security/crypto-key-management-canonical-v2-2026-10-05` | canonical specialty candidate, not approved release state |
| Key lifecycle | PR #71 / `execution/crypto-key-lifecycle-v3-2026-10-05` | supporting candidate pending comparison with PR #70 |
| Android stack | PR #56 | supporting; documentation-only; mergeability currently false |
| Evidence/repository consistency | PR #54 | major supporting/candidate integration stream; review/merge required |
| AI | PR #57/#58 | optional, isolated, non-authoritative; outside release critical path |

## 5. Supersession findings

PR #59 explicitly identifies itself as superseded by PR #69.

Older Rust security PRs #22 and #40 are explicitly superseded by PR #54.

PR #67 is closed without merge and superseded in the crypto/key-management continuation by later focused branches.

These dispositions preserve provenance while preventing parallel branches from being interpreted as simultaneous canonical implementations.

## 6. Historical source authority

The 44-file historical provenance register is authoritative for the historical Git artifact list.

The two 2026-10-02 ChatGPT-only artifacts remain provenance-recorded but their durable binary GitHub transfer is not proven.

Therefore historical archive material is supporting evidence, not implementation authority.

## 7. Architecture conflicts found

1. Stale Android path: `androidApp/` vs actual current `:app` module.
2. Proposed ADR status is inconsistent with some later specialty implementation wording that can sound final; implementation documentation must preserve ADR approval status.
3. Multiple divergent specialty branches reuse overlapping files; only focused current-base branches should progress.
4. Historical PrivateMesh documents include server-oriented diagrams; these conflict with the current P2P-only architecture and must remain historical/non-canonical unless explicitly reconciled.
5. Some records still describe the product as pre-implementation even though current main now contains Android bootstrap/CI evidence; readiness documents require timestamped updating rather than retroactive rewriting.

## 8. Canonical authority outcome

The canonical chain is:

```text
Current main
   ↓
Accepted requirements/ADRs
   ↓
Focused current-base PR
   ↓
CI/Test Lab/security evidence
   ↓
Human review
   ↓
Merge
   ↓
New canonical main
```

No branch, archive, or conversation may bypass this chain.
