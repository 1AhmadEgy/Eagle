# ADR-0016 — Android module path reconciliation and cross-platform baseline

- **Status:** Accepted
- **Date:** 2026-10-06
- **Scope:** Android module naming and platform-architecture documentation
- **Supersedes:** Any undocumented assumption that the Android module path is `androidApp/`

## Context

The platform strategy in `docs/03-architecture/PLATFORMS.md` describes the Android application as `androidApp/`, while the repository's current implementation uses the Gradle module `app/` and `settings.gradle.kts` contains `include(":app")`.

The implementation is the authoritative evidence for the current repository state. Renaming the existing Android module solely to match a stale documentation path would create unnecessary churn and could obscure the actual history of the project.

At the same time, future KMP adoption must not silently redefine the current Android boundary.

## Decision

1. `app/` is the canonical Android application module path for the current baseline.
2. The existing Gradle declaration `include(":app")` remains authoritative until a separate KMP migration ADR explicitly changes the module layout.
3. `androidApp/` is not an implementation requirement and must not be referenced as the current Android module path.
4. When KMP is introduced, the resulting module structure must be recorded in a separate migration decision; this ADR does not prescribe a future module name.
5. Platform documentation must distinguish **current implementation paths** from **target architecture paths**.
6. No Android module rename is performed as part of this reconciliation.

## Consequences

### Positive

- Documentation matches repository evidence.
- No unnecessary source-tree rename is introduced.
- Future KMP work has an explicit architectural change point.
- Reviewers can distinguish current state from target state.

### Negative

- Some historical documents may continue to contain the old `androidApp/` terminology until they are individually reconciled.
- A later KMP migration will require a new explicit module-layout decision.

## Verification

Current evidence:

- `settings.gradle.kts` contains `include(":app")`.
- The current Android implementation resides under `app/`.

This ADR is documentation-only. Repository verification remains governed by `AGENTS.md`.
