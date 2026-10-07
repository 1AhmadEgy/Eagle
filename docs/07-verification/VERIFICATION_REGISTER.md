# Eagle Verification Register

> **Canonical snapshot:** `main` @ `46aa86b6d71d34396b86b35124392cb3ba49c2e9`
> **Reconciliation date:** 2026-10-07

## Evidence authority

| Scope | Meaning |
|---|---|
| MAIN | Evidence for the exact canonical `main` commit |
| PULL_REQUEST | Candidate evidence scoped to a PR/head SHA |
| BRANCH | Candidate/non-canonical evidence |
| HISTORICAL | Historical provenance only |

**Invariant:** `PR PASS != main PASS`.

A verification result is valid only for the exact repository, branch/scope, commit SHA, workflow/run, environment, and artifact snapshot recorded with it.

## Current canonical state

| Item | Scope | Exact reference | Current status |
|---|---|---|---|
| Repository identity | MAIN | `1AhmadEgy/Eagle` | VERIFIED |
| Canonical branch | MAIN | `main` | VERIFIED |
| Canonical commit | MAIN | `46aa86b6d71d34396b86b35124392cb3ba49c2e9` | VERIFIED |
| CI | MAIN | exact-head snapshot | FAIL |
| Test Lab | MAIN | exact-head snapshot | FAIL |
| Branch protection | MAIN/Governance | live GitHub state | OFF per latest audit snapshot |
| Release | MAIN | release gate | NO-GO |

## PR #45

| Field | Value |
|---|---|
| PR | #45 |
| Head | `61f492f9...` |
| Scope | PULL_REQUEST |
| Claimed result | CI/Test Lab PASS |
| Authority | VERIFIED_CANDIDATE |
| Main validity | **NO** until merged and re-run on new main |

PR #45 must not be copied into the MAIN evidence section as a PASS.

## PR #77

Implementation candidate / non-canonical. It must be decomposed by requirement → ADR → contract → implementation → test → evidence before promotion.

## PR #89 / #91

Documentation/audit candidates are snapshot-dependent. They require refresh against the post-merge main SHA before any canonical status is assigned.

## Required evidence fields

`evidence_id`, `claim`, `scope`, `repository`, `branch`, `commit`, `workflow`, `run_id`, `environment`, `artifact`, `artifact_sha256`, `result`, `reviewer`, `reviewed_at`, `supersedes`, `notes`.

## Gate states

`PASS`, `FAIL`, `PENDING`, `NOT_APPLICABLE`, `UNCONFIRMED`.

Missing evidence is not PASS.

## Historical verification

Older verification entries remain historical unless explicitly tied to the current exact main SHA. Repository/account observations from 2026-10-01 remain useful provenance but do not override the current canonical snapshot.
