# ADR-0000: Branch, Toolchain, and Consolidation Policy

- Status: Proposed
- Date: 2026-10-04
- Deciders: Eagle maintainers
- Related: PR #41, execution/core-foundation-v1, implementation/v1-foundation

## Context

Eagle is consolidating a historical Rust foundation onto the current main baseline.

The current main baseline at 6c46bf53fce06ee7f7b5b2bf35c720ab5bcb7dee contains no root Rust workspace (Cargo.toml and core/ are absent). The historical execution branch execution/core-foundation-v1 contains the Rust workspace, core/, core/ffi/, and the Rust Core workflow.

The consolidation must preserve provenance, avoid hidden history replay, and keep production/reference boundaries explicit.

## Decisions

### D1 — Branch naming integrity

Branch names SHALL describe what is actually present, not an intended future state.

Names containing verified, final, stable, or ready are prohibited unless the corresponding claim is backed by evidence under docs/audit/.

### D2 — No merge / no rebase of historical execution branches

Historical execution branches SHALL NOT be merged or rebased into the new foundation line.

Historical work is ported feature-by-feature or imported as an explicitly recorded snapshot.

### D3 — Consolidation target

The consolidation branch is implementation/v1-foundation.

It SHALL start from the latest main and remain a single-purpose integration line.

### D4 — Feature-by-feature porting

Each feature port SHALL:

- compile against the current baseline;
- pass the applicable current CI gates;
- include tests that prove the imported contract where tests exist;
- remain independently reviewable.

The normal PR-size target is fewer than 500 changed lines.

#### D4-R1 — Foundational import exception

A foundational import MAY exceed the normal 500-line PR-size target when all of the following are true:

1. the subsystem is absent from the baseline;
2. the source is a named commit or commit snapshot;
3. the imported content is a textual transfer rather than an architectural rewrite;
4. the source commit and exact imported file set are recorded in docs/audit/pre-consolidation-2026-10-04.md;
5. the resulting repository state is buildable/testable at the end of the import step.

The 500-line target applies to subsequent modifications, not to the unavoidable initial introduction of an absent subsystem.

### D3-R1 — Snapshot import evidence standard

A foundational subsystem absent from the baseline MAY be imported as a single snapshot commit when:

1. The source commit is named explicitly in the commit message.
2. Per-file blob SHAs are verified equal between source and target.
3. The import leaves the repository in a buildable and testable state.
4. Intermediate states are documented in Evidence and closed within the immediately following slice.
5. The import is NOT a cherry-pick; per-commit review is not required.
6. The import commit message follows: feat(<area>): import <subsystem> snapshot from <sha>.

Standard PR size limits apply to subsequent modifications, not to the import itself. A separate Evidence annex records each snapshot import: source SHA, target SHA, file inventory, blob hashes, and the closing slice that stabilized it.

### D5 — Non-ported commit ledger

Every historical commit not replayed onto the new foundation line SHALL be recorded with:

- SHA;
- subject;
- disposition;
- reason.

#### D5-R1 — Snapshot import disposition

A historical subsystem MAY be imported as a snapshot without replaying its individual commits.

Such an import is considered a state transfer, not commit transfer.

The Evidence record SHALL identify:

- source branch;
- source snapshot commit;
- import date;
- exact imported paths;
- whether the transfer was verbatim or modified;
- any modifications made after import.

The six commits that previously existed only on implementation/v1-foundation were not replayed because their content is already represented on current main, either byte-for-byte or by a later replacement of the same path. They remain recorded in the Evidence ledger.

### D6 — Rust toolchain policy

The repository root rust-toolchain.toml is authoritative:

[toolchain]
channel = "1.99.0"
profile = "minimal"
components = ["rustfmt", "clippy"]

CI SHALL consume this policy and verify active rustc, cargo, and rustup toolchain state.

No root Cargo.toml modification belongs to Step 1.

Platform targets SHALL be added only when the corresponding platform slice requires them.

### D7 — Reference implementation boundary

MemoryRecordStore and MemoryTransport are reference/contract implementations, not test doubles.

They SHALL live in eagle-core-reference.

Production dependency direction:

eagle-core
   ✗
eagle-core-reference

The reverse dependency is intentionally prohibited. eagle-core SHALL NOT depend on eagle-core-reference, including as a dev-dependency, because that would create a cyclic/duplicate type-universe boundary.

The production dependency graph SHALL be checked by Gate 5.7 using Cargo metadata/resolve information rather than source-text grep.

### D8 — Rust edition

Retain Rust edition 2021 during consolidation.

Edition 2024 migration is deferred to a separate change after the foundation is stable.

## CI finding recorded during Step 1

The Step 1 Rust workflow did run.

For commit 3561d7b6259a4f2727a7c7c6cde7212559883ce1:

- Rust Core run 37200662991 — failed at Install repository Rust toolchain.
- CI run 37200662977 — failed at the same step.
- The workflow was therefore not blocked by the path filter and was not a missing-run/approval/YAML-syntax case.
- The installer received an empty toolchain input.

Root cause: the Python heredoc used a quoted delimiter while attempting to write to the literal string ${GITHUB_OUTPUT}. The output was therefore written to a literal filename rather than GitHub Actions' output file.

Corrective commit:

33858674ad51cadee81a4b9b1c8a34013eb68262

The correction uses os.environ["GITHUB_OUTPUT"].

No hashFiles('Cargo.toml') job-level guard was added because the actual failure was not the absence of the workspace. After the installer succeeds, the existing explicit workspace-presence step keeps the Rust build/test stages dormant until Step 2 introduces the workspace.

## Acceptance criteria

ADR-0000 SHALL remain Proposed until:

- Step 1 is verified by successful CI/toolchain evidence;
- Step 2a Rust workspace import is complete and green;
- Step 2b reference boundary and Gate 5.7 are green;
- the Evidence ledger is complete;
- required security/supply-chain blockers are either closed or explicitly carried as approved blockers.

Only then may this ADR move to Accepted.