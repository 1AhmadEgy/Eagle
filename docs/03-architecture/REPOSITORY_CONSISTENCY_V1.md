# Eagle — Repository Consistency & Evidence Gate v1

## Purpose

This gate turns the evidence-first rule into executable repository policy. It is deterministic and network-free so local runs and CI evaluate the same baseline.

## What it verifies

- the documented Android module matches the real Gradle module (`app/`);
- active platform documentation does not retain the stale `androidApp/` path;
- ADR-0008..0010 have dedicated proposed records;
- ADR-0011..0014 remain explicitly pending and are represented by the shared worklist;
- Android build/tooling facts match the current implementation baseline;
- the current app does not silently gain libsignal, OpenMLS, libp2p, SQLCipher, or Room before the component gate is passed;
- product-security maturity claims remain false until actual implementation evidence exists.\n- external GitHub Actions and reusable workflows use full 40-character commit SHAs; local reusable workflows may use relative paths.

## Why this exists

The repository already distinguishes implementation from planning. The remaining problem is drift: a future documentation or code edit can accidentally claim a component exists, or refer to a module/path that does not exist.

The checker makes those contradictions fail the verification path rather than relying on memory or manual review.

## Command

```bash
python3 scripts/ci/verify-repository-consistency.py
```

It is also invoked by `scripts/ci/verify.sh`, so the same check participates in the local pre-push verification path.

## Update protocol

Whenever a protected fact genuinely changes:

1. implement the change;
2. add tests/evidence;
3. update `docs/03-architecture/REPOSITORY_STATE_V1.json`;
4. update the affected architecture documentation/ADR;
5. run the consistency gate;
6. run the full repository verification;
7. record the exact commit/version and evidence in the relevant research/ADR record.

Do not edit the registry merely to make the checker pass.

## Current baseline

As of 2026-10-04, Eagle has an Android bootstrap under `app/`, JUnit smoke coverage, CI/Test Lab infrastructure, and architecture/security documentation. It does not yet contain the production E2E cryptographic stack, Rust Security Core, KMP shared module, or mesh engine.

This status is intentionally machine-checked.

## Validation history

- 2026-10-04 CI run 37210145630 exposed two gate defects: AGP was read from the app module instead of the root build file, and the stale-token detector matched the gate's own documentation/example text.
- The gate was corrected to read the root AGP declaration and to exclude the self-documenting gate files from that specific stale-token assertion.
- CI artifact upload behavior was changed from an error on missing evidence files to a warning so a secondary artifact failure cannot obscure a primary verification failure.\n- Test Lab actions were upgraded/pinned to current immutable releases observed on 2026-10-04: checkout v7.0.1, setup-java v6.0.1, setup-gradle v5.0.1. Current releases were verified against their upstream GitHub release histories.

- 2026-10-04 second CI validation isolated the AGP matcher as still failing despite the correct source file; the gate was simplified to exact-text matching for the root plugin declaration. The workflow parser was also corrected so YAML whitespace escaping is interpreted by Python rather than as literal backslashes.
