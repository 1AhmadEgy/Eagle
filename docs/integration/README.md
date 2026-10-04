# Foundation Integration

This branch is the documentation-only Layer A of the phased foundation integration.

- Source branch: `execution/core-foundation-v1`
- Source head: `cdd84a13dff684f66c23bffc9f3ceeedca669484`
- Target baseline: `main`
- Order: A → B → C → D → E

Each layer requires fresh review and CI evidence. A dependent layer starts from the actual `main` head after its prerequisite merges.

## Mixed-commit audit

The 105 commits from `14a6f1d6` to `execution/core-foundation-v1` were checked by changed-path classification against Layers A–E.

The only confirmed mixed commit is:

- `105ab03889b3caa835928f11c1866cd88d822137` — `fix: align UniFFI generation and Android CI SDK with supported tooling`
  - Layer B: `.github/workflows/rust-core.yml`, `.github/workflows/testlab.yml`
  - Layer C: `core/ffi/Cargo.toml`, `core/ffi/src/bin/uniffi-bindgen.rs`, `scripts/ffi/generate-kotlin-bindings.sh`
  - Layer D: `shared/build.gradle.kts`
  - Layer E: `app/build.gradle.kts`

No other commit in that 105-commit range was found to cross two or more A–E layers.

### Split rule for 105ab038

The source commit remains the provenance reference. Its changes must be split without rewriting the source history:

- B receives only the two workflow files.
- C later receives only `core/ffi/**` and `scripts/ffi/**`.
- D later receives only `shared/build.gradle.kts`.
- E later receives only `app/build.gradle.kts`.

Each derived commit must include:

`Source: 105ab03889b3caa835928f11c1866cd88d822137`

and a layer-specific `Split:` line. The split must be tested independently in the destination layer.

## Layer B source candidates

The clean CI commits already identified for the B reconstruction are:

- `ea68b102b908563c470001c94d564946597d6531`
- `c0988f09bc1cfc8cf1bf04f974bd8d9605238149`
- `a3a6cd5985dccb24842036b45969a3037c75fd11`
- `25e1c552ded7a9f607b30ebe720a0863db595ada`
- `5f986023d2e91b972d7d824a0f1dbaf4eeef5ad3`
- `c877e8f13cf3dfd27a312bf8ec261bc072913cd9`
- `5978d1edd6b92e97f5bc96b845d4923cb056e254`
- `292111a613f4762219f559028a16b602fb761167`
- `59317f3f2de5a37c1c167d4a527ebcde33a9d078`

These are transfer candidates, not proof that the final B branch is complete. Local Git reconstruction must run the classification script and inspect the resulting diff before push.

## Workflow-scope constraint

The current GitHub integration credential cannot write `.github/workflows/**` because it lacks the GitHub `workflow` permission.

This is a tooling/credential constraint, not an architecture decision.

The B branch must therefore be created and pushed through an authenticated Git client with workflow permission, or the workflow files must be committed through an authorized repository maintainer's GitHub UI.

The credential owner is the authorized repository maintainer/project lead. The credential must never be pasted into chat or repository contents.

## Audit helper

`docs/integration/classify-foundation-commits.sh` is a local audit helper. It classifies every commit in the source range, prints per-layer counts, and emits mixed commits with their exact paths before any cherry-pick is attempted.
