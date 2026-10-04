# Repository Inventory

Audit date: 2026-10-04

## GitHub repository

- Repository: `1AhmadEgy/Eagle`
- Default branch: `main`
- Visibility: public
- Archived: no
- Current application baseline: Android/Gradle module under `app/`
- Current automated test baseline: JVM unit-test smoke test under `app/src/test/`
- Current CI/Test Lab workflows: present under `.github/workflows/`
- Current dependency/build manifests: Gradle Kotlin DSL files are present
- Repository metadata currently reports GitHub's detected primary language as Python; this reflects repository-wide detection and should not be confused with the Android application's Kotlin/Gradle stack.

## Current top-level areas

- `.github/` — CI, Test Lab, Dependabot and automation
- `app/` — Android application module
- `docs/` — canonical project documentation
- `issues/` — architecture/decision records
- `scripts/` — repository verification and CI helpers
- `archive/` — preserved historical project material
- `AGENTS.md` — agent governance
- `SECURITY.md` — security policy baseline
- `README.md` — public project entry point

## Implementation boundary

The current codebase exposes a small Android application bootstrap. The larger Rust Security Core, KMP Shared Layer, and additional platform targets described in the platform strategy are architectural targets unless their corresponding source/build/test evidence is present.

## Governance

Only artifacts actually accessible to the audit process are marked as reviewed. Missing project-member conversations, private files, or disconnected sources are not inferred or reconstructed.

The repository's agent rules prohibit automated direct pushes to `main`; implementation/documentation changes should use a branch and pull request with human review.
