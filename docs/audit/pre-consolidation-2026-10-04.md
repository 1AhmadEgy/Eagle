# Pre-Consolidation Evidence — 2026-10-04

## 1. Baseline

- Repository: `1AhmadEgy/Eagle`
- Target branch: `implementation/v1-foundation`
- Baseline `main`: `6c46bf53fce06ee7f7b5b2bf35c720ab5bcb7dee`
- The target branch was reset to the exact `main` baseline.
- Remote comparison after reset: `implementation/v1-foundation` is identical to `main` (`ahead_by=0`, `behind_by=0`).
- Historical PR #41 remains separate on `execution/core-foundation-v1`; it was not merged or rebased into the target branch.

## 2. Commits NOT ported — implementation/v1-foundation baseline reset

The previous target branch was six commits ahead of the then-common ancestor `48eaddd89ccb4f72b39bd42e2fb7367af7b9bc9e`. The exact six-commit chain was recovered before finalizing the baseline reset:

| SHA | Subject | Disposition | Reason |
|-----|---------|-------------|--------|
| `a2cb3a262df3d492be42ab788d18dd928b690a22` | `chore: establish secure v1 execution foundation` | not ported | `SECURITY.md` is already present on current `main` with the exact same blob/content. |
| `9f1bab6ea741d78a1b6e0eedf4a78c11ddb97dfa` | `chore: establish secure v1 execution foundation` | not ported | `docs/EXECUTION-READINESS.md` is already present on current `main` with the exact same blob/content. |
| `20974b40d1617ebfedd88eb69487d4c095888519` | `chore: establish secure v1 execution foundation` | not ported | `docs/REPOSITORY-INVENTORY.md` is already present on current `main` with the exact same blob/content. |
| `619084dae48e60ba6592e2faeeedea72b1697933` | `chore: establish secure v1 execution foundation` | not ported | `docs/SECURITY-BASELINE.md` is already present on current `main` with the exact same blob/content. |
| `241c9e4c184ace8d527284133a1a017cd69ad5f8` | `chore: establish secure v1 execution foundation` | not ported | `.github/dependabot.yml` is already present on current `main` with the exact same blob/content. |
| `4dfb8e08a679cfd0f455d57044e86056e3fff5c` | `chore: establish secure v1 execution foundation` | not ported | `.github/workflows/ci.yml` is already present on current `main`, but the current file is a later, expanded version (same path, different blob); the historical commit is superseded rather than copied. |

### Provenance note

These six historical commits were **not discarded semantically**. Their relevant repository content is present in the current `main` tree, either byte-for-byte identical or superseded by the current implementation of the same path. The commits themselves are intentionally not replayed onto `implementation/v1-foundation`.

## 3. Baseline reset event

- date: 2026-10-04
- branch: `implementation/v1-foundation`
- previous head: `4dfb8e08a679cfd0f455d57044e86056e3fff5c`
- previous relation to `main`: 6 commits ahead / 115 commits behind at the time of reset
- new head: `6c46bf53fce06ee7f7b5b2bf35c720ab5bcb7dee`
- method: remote Git ref update with force enabled (equivalent in effect to a forced branch ref reset; no merge, no rebase)
- protected: no
- verification: target branch and `main` are identical after reset
- human reviewer recorded in this operation: none

## 4. Pre-Phase-1 repository checks

Verified against the remote repository baseline:

- `implementation/v1-foundation` HEAD = `6c46bf53fce06ee7f7b5b2bf35c720ab5bcb7dee`
- `main` HEAD = `6c46bf53fce06ee7f7b5b2bf35c720ab5bcb7dee`
- root `rust-toolchain.toml`: absent
- root `.cargo/config.toml`: absent
- root `Cargo.toml`: absent
- root `.github/workflows/rust-core.yml`: absent
- `core/` on current `main`: absent
- existing `.github/workflows/ci.yml`: present
- existing `scripts/ci/verify.sh`: present

A local working-tree `git status`, `git rev-parse origin/main`, and `git diff --check` could not be executed in this connector environment; no local checkout was available. Remote ref equality was verified independently through GitHub.

## 5. Governing principle

### Principle — Compiler findings are findings, not obstacles

A compilation failure during Step 2 is not a bug to suppress. It is a signal that a dependency exists which the current architecture did not intend.

Rule:

- If `eagle-core-ffi` imports `Memory*`, treat that as an architectural dependency on a reference implementation, not on the contract.
- Do not re-export `Memory*` from `eagle-core` merely to satisfy compilation.
- Fix the consumer to depend on `SecureTransport` / `RecordStore`, or keep reference-only usage inside the appropriate reference/test boundary.
- Apply the same rule to every unexpected compiler/test finding.

### Finding 6.7 — OpaqueId invariant

If a supported construction path allows `OpaqueId::new(vec![])` to succeed, or a public API permits bypassing the invariant, this is an architectural/security finding rather than a test defect.

Step 2 remains open until the actual supported construction path is established by compilation/tests and resolved as:

- A: impossible by invariant → remove or document `EmptyOwnerId` as defensive-only.
- B: valid supported path exists → test the real path.
- C: public bypass exists → BLOCKER; do not start SLICE-01.

## 6. Phase 1 — Toolchain policy applied

- Step 1 commit: `3561d7b6259a4f2727a7c7c6cde7212559883ce1`
- Commit message: `chore(build): pin toolchain to 1.99.0 via repo-level policy`
- The Step 1 code/config commit contains exactly:
  - root `rust-toolchain.toml`
  - `.github/workflows/rust-core.yml`
  - `.github/workflows/ci.yml`
  - `scripts/ci/verify.sh`
- Root `Cargo.toml` was not modified or created in Step 1.
- The repository policy is:
  - channel: `1.99.0`
  - profile: `minimal`
  - components: `rustfmt`, `clippy`
- CI reads the root TOML with Python `tomllib`, passes the parsed channel/components to the SHA-pinned `dtolnay/rust-toolchain` action, then explicitly verifies rustc/cargo/rustup and checks that the active toolchain is not merely the rustup default.
- `scripts/ci/verify.sh` performs the same Rust-toolchain assertion immediately before the first Rust cargo call.

### Phase 1 repository-state verification

- Remote `main` = `6c46bf53fce06ee7f7b5b2bf35c720ab5bcb7dee`
- Remote `implementation/v1-foundation` = `3561d7b6259a4f2727a7c7c6cde7212559883ce1`
- Relation: target branch is 2 commits ahead and 0 behind `main`:
  1. baseline Evidence commit
  2. Step 1 toolchain-policy commit
- Current branch remains unprotected.
- The combined GitHub status endpoint currently reports no status entries for the Step 1 commit.

### Local command evidence limitation

The exact local outputs requested by the operating checklist could not be executed in this environment because there is no local Eagle checkout and outbound Git access is unavailable:

```
rustc --version
cargo --version
rustup show active-toolchain
git rev-parse HEAD
git status --short
git diff --check
```

Remote GitHub state was verified instead. This does not count as a local toolchain PASS.

## 7. Workspace architecture finding before Step 2

The current `main` baseline does not contain a root `Cargo.toml` or a `core/` directory.

The historical `execution/core-foundation-v1` branch does contain:

```toml
[workspace]
resolver = "2"
members = ["core", "core/ffi"]
```

Therefore Step 2 must port the existing Rust workspace foundation from the historical execution branch onto the clean `main` baseline before adding `eagle-core-reference`. This is an execution dependency, not a justification to modify the Step 1 commit.
