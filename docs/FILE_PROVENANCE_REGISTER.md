# Eagle — Project File Provenance Register

> **Canonical reconciliation:** main @ `46aa86b6d71d34396b86b35124392cb3ba49c2e9` (2026-10-05)
> **Purpose:** distinguish canonical implementation evidence from PR, branch, archive, and conversation provenance.

## Authority model

| Source type | Authority | Meaning |
|---|---|---|
| MAIN | CANONICAL_IMPLEMENTATION | Exact file/tree state on canonical `main` |
| PULL_REQUEST | VERIFIED_CANDIDATE | Candidate evidence; never main evidence until merged and re-verified |
| BRANCH | VERIFIED_CANDIDATE / UNVERIFIED | Non-main implementation or documentation candidate |
| ARCHIVE | HISTORICAL_PROVENANCE | Historical input/reference only |
| CONVERSATION | HISTORICAL_PROVENANCE / UNVERIFIED | Context only unless independently committed and verified |
| EXTERNAL_REFERENCE | ACCEPTED_REFERENCE | External material explicitly accepted as a reference |

## Invariants

- `PR PASS != main PASS`.
- Proposed, Accepted, Implemented, Verified, and Released are distinct states.
- A Git blob SHA is provenance evidence, not a SHA-256 checksum.
- A claim without an exact source/commit/snapshot remains unverified.
- Historical material is never promoted automatically.
- Independent SHA-256 hashes are required when artifact integrity must be established.

## Canonical implementation snapshot

- Repository: `1AhmadEgy/Eagle`
- Branch: `main`
- Commit: `46aa86b6d71d34396b86b35124392cb3ba49c2e9`
- Tree: `a7718840805359b042ec20a1781daaa5531ee47f`

The current `main` tree is the implementation authority for Sprint 0 reconciliation.

## Existing historical inventory

The 44 historical files recorded below remain provenance only. They must not be treated as current implementation.

| Historical source commit | Count | Status |
|---|---:|---|
| `812389dc9a19c52ca8089397c96a09d46957336b` | 44 | Historical / removed from current main |

The detailed 44-file table from the prior register is preserved below this reconciliation header.

## Required provenance fields for promoted artifacts

`path`, `source_type`, `source_location`, `branch`, `commit_or_tag`, `git_blob_sha`, `sha256` (when applicable), `authority`, `status`, `related_pr`, `related_adr`, `evidence_id`, `last_reviewed`.

## Scope limitation

GitHub history proves committed repository material. It does not prove ChatGPT-only attachments that were never committed. Such items remain pending intake.
