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

## 8. CI diagnostic results

### Run inventory for implementation/v1-foundation

For Step 1 commit 3561d7b6259a4f2727a7c7c6cde7212559883ce1, GitHub recorded five workflow runs:

| Workflow | Run ID | Event | Result |
|----------|--------|-------|--------|
| Rust Core | 37200662991 | push | failure |
| CI | 37200662977 | push | failure |
| Eagle Test Lab | 37200662985 | push | failure |
| Eagle Continuous Documentation | 37200663055 | push | failure |
| Automatic Dependency Submission (Gradle) | 37200662716 | dynamic | success |

The Rust Core and CI failures both occurred at the repository Rust toolchain installer because the action received an empty toolchain input.

### Root-cause classification

The workflow DID trigger, so this was not:

- A: workflow started and immediately failed because Cargo.toml was absent;
- B: path filter mismatch;
- C: approval-required fork workflow;
- D: workflow YAML syntax preventing startup;
- E: verify.sh reaching a Cargo failure.

It was F: incorrect GitHub Actions output propagation. The Python heredoc used a quoted delimiter and wrote to the literal string GITHUB_OUTPUT placeholder rather than the path held by the environment variable.

### Corrective verification

Commit 33858674ad51cadee81a4b9b1c8a34013eb68262 corrected the output handling.

For that commit:

| Workflow | Run ID | Result |
|----------|--------|--------|
| Rust Core | 37200925434 | success |
| CI | 37200925448 | failure |
| Eagle Test Lab | 37200925500 | failure |
| Eagle Continuous Documentation | 37200925447 | failure |
| Automatic Dependency Submission (Gradle) | 37200926693 | success |

Rust Core step evidence on 33858674:

- Read repository Rust toolchain policy: success
- Install repository Rust toolchain: success
- Assert rust-toolchain.toml was actually applied: success
- Check Rust workspace presence: success
- Cache/build/test/UniFFI/clippy: skipped because root Cargo.toml is intentionally absent at this phase

The runner log reports Rust 1.99.0 successfully installed and active.

### Independent CI blocker discovered

After the toolchain issue was corrected, the main CI workflow advanced through toolchain setup, repository hygiene, and secret scanning, then failed in scripts/ci/verify-security-policy.py.

The verifier reported:

- .github/workflows/rust-core.yml:30 for a line using a valid 40-character checkout SHA;
- .github/workflows/testlab.yml:21 using actions/checkout@v5;
- .github/workflows/testlab.yml:24 using actions/setup-java@v5;
- .github/workflows/testlab.yml:30 using gradle/actions/setup-gradle@v5.

Inspection of the verifier shows its action-pinning regex expects a line beginning with uses:, while normal list syntax begins with - uses:. Therefore the verifier itself is currently producing a false-positive for list-form action entries.

The Test Lab moving-tag entries remain a genuine separate supply-chain finding and are not being silently reclassified as resolved.

This finding is separate from ADR-0000 toolchain policy and is not being fixed as part of the Rust toolchain correction.

### Gate interpretation

The Rust toolchain path is now proven in CI.

Overall repository CI is NOT green yet. ADR-0000 therefore remains Proposed.

Step 2a may proceed as the explicitly recorded foundational snapshot import, but no consolidation gate may be marked green until the independent CI/security-policy issue and the actual Rust workspace verification are resolved.


## 9. Step 2a — foundational Rust snapshot import

- source branch: execution/core-foundation-v1
- source snapshot: cdd84a13dff684f66c23bffc9f3ceeedca669484
- target snapshot commit: cb4b0897864989926382520bfe5d0c8917716c28
- method: verbatim snapshot import; not a cherry-pick
- excluded from snapshot: .github/workflows/rust-core.yml because the Step 1 corrected workflow remained authoritative
- imported files: 17
- source/target blob verification: 17/17 exact matches

| path | source blob | target blob |
|---|---|---|
| Cargo.toml | 700e52a6f97b350e8ff1b7c1648643c2b83f4da8 | 700e52a6f97b350e8ff1b7c1648643c2b83f4da8 |
| core/Cargo.toml | e601e4e533eb651e8613efcc08444e60982bef68 | e601e4e533eb651e8613efcc08444e60982bef68 |
| core/README.md | be3b609fc4629db0b6642fa0291793f5e8c9561a | be3b609fc4629db0b6642fa0291793f5e8c9561a |
| core/ffi/Cargo.toml | 5ce173bfc74aba6459c9e36e3179de0b5e21bed1 | 5ce173bfc74aba6459c9e36e3179de0b5e21bed1 |
| core/ffi/src/bin/uniffi-bindgen.rs | f6cff6cf1d99f5cd651e48956141b0c99c504fa8 | f6cff6cf1d99f5cd651e48956141b0c99c504fa8 |
| core/ffi/src/lib.rs | bb20b56a33206ecf4690347e78d6c0821a7f3ea0 | bb20b56a33206ecf4690347e78d6c0821a7f3ea0 |
| core/ffi/tests/contract.rs | ec176c3d94a96a3f27726eb01aa8573eeb58eef2 | ec176c3d94a96a3f27726eb01aa8573eeb58eef2 |
| core/ffi/uniffi.toml | 88488af748e6c88582b4b642ea9aa33cf9f4c2d0 | 88488af748e6c88582b4b642ea9aa33cf9f4c2d0 |
| core/src/devices.rs | df08f785d791db7bfc02dfdad34aa3277d86a4a2 | df08f785d791db7bfc02dfdad34aa3277d86a4a2 |
| core/src/identity.rs | d6037209e7749f6591a58f4803bd6e30a743faba | d6037209e7749f6591a58f4803bd6e30a743faba |
| core/src/lib.rs | 323380ef97c11d5faf56d9cf34b53b03d2d6652b | 323380ef97c11d5faf56d9cf34b53b03d2d6652b |
| core/src/policy.rs | ea9d30d3f5c05b4e34d122fac06641f61666b310 | ea9d30d3f5c05b4e34d122fac06641f61666b310 |
| core/src/protocol.rs | 9d26b23b48101eb48eb63706a7c54a9db69d5ad8 | 9d26b23b48101eb48eb63706a7c54a9db69d5ad8 |
| core/src/session.rs | c0436d038f315ec8c908d31a41a4f9aad27fef6c | c0436d038f315ec8c908d31a41a4f9aad27fef6c |
| core/src/storage.rs | 443cdbc601fe2c760e71fd01998155bdf6a88d7d | 443cdbc601fe2c760e71fd01998155bdf6a88d7d |
| core/src/transport.rs | ed48077405793abe5393245d66db1d03df2564c5 | ed48077405793abe5393245d66db1d03df2564c5 |
| core/tests/security_integration.rs | be062ca2332d3de3a16f92b74ea9cf2afa69bac0 | be062ca2332d3de3a16f92b74ea9cf2afa69bac0 |

CI evidence previously recorded for cb4b089: Rust Core run 37201056258 passed all workflow steps, including cargo test, UniFFI generation, and clippy. Historical source cdd84a13 produced the same 43 Rust tests; the snapshot produced the same 43 tests, zero failures.

## 10. Step 2b — reference implementation boundary

Implementation completed on implementation/v1-foundation.

Production core now owns contracts only:
- RecordStore and storage data/error contracts;
- SecureTransport and transport data/error contracts;
- validate_record remains a production contract validator;
- MemoryRecordStore and MemoryTransport are no longer exported by eagle-core.

Reference implementations now live in:
- core-reference/src/record_store.rs
- core-reference/src/transport.rs
- core-reference/tests/record_store.rs
- core-reference/tests/transport.rs

Workspace package:
- eagle-core-reference
- depends on eagle-core
- eagle-core has no dependency, including dev-dependency, back to eagle-core-reference.

Gate 5.7 executable verifier added:
- scripts/ci/verify-reference-boundary.py
- Rust Core workflow invokes it after workspace detection.
- The gate uses cargo metadata/resolve graph data, not source-text grep.
- Any normal/build production edge to reference/test/mock packages fails the gate.

Important: Step 2b is implementation-complete but its CI execution is still required before the slice is marked green.

## 11. Technology/research reuse register

Created:
- docs/research/technology-radar.md

The radar records reusable external projects, libraries, runtimes, research patterns and benchmarks, with dispositions ADOPT, ADAPTER, REFERENCE, PROTOTYPE, WATCH, REJECT.

The governing rule is to keep Eagle-owned contracts stable and integrate mature external systems behind adapters. No third-party agent framework, memory system, vector database, model runtime or research implementation is authorized to become an Eagle production dependency merely because it appears on the radar.

## 12. Supply-chain hardening backlog

Still open and intentionally separate from Step 2b:

- P0a: replace line-oriented action-pin parsing with YAML-aware workflow parsing plus regression tests.
- P0b: replace moving GitHub Action tags in Test Lab with verified full commit SHAs.
- dependency vulnerability/license enforcement;
- SBOM generation for releasable artifacts;
- build provenance/attestation;
- verification of released artifacts.

Current GitHub guidance supports dependency review for detecting vulnerable dependency changes before merge and artifact attestations for cryptographically signed build provenance/SBOM claims. These are recorded as future evidence gates, not as completed Eagle controls.

## 13. Current state after this slice

- Step 1 toolchain path: proven in CI.
- Step 2a snapshot: implemented and historically verified.
- Step 2b reference boundary: implemented; Gate 5.7 added.
- ADR-0000: Proposed.
- P0a/P0b security workflow issues: open.
- No release or production security claim is made by this consolidation branch.
