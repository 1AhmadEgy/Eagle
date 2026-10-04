# ADR-0001: Canonical Source Selection for V1 Foundation

- Status: Proposed
- Date: 2026-10-05
- Repository: 1AhmadEgy/Eagle
- Decision scope: canonical development lineage, security-core source, release-baseline candidate

## Context

Eagle currently contains multiple historical and execution branches with overlapping Rust-core work. A production baseline must have one traceable development lineage and must not be inferred from branch names such as "verified" or "final".

Verified repository references for this decision:

- main: 6c46bf53fce06ee7f7b5b2bf35c720ab5bcb7dee
- implementation/v1-foundation: b6a6708654a5966bfc44e7925e46e95e979b4977
- execution/core-foundation-v1: compared against main and implementation/v1-foundation; diverged
- execution/phase-1-security-kernel-verified-2026-10-04: compared against main and implementation/v1-foundation; diverged
- execution/phase-1-security-kernel: diverged from main

The repository evidence file docs/audit/pre-consolidation-2026-10-04.md records the consolidation policy, snapshot provenance, reference-boundary design, and remaining acceptance blockers.

## Options

### A — implementation/v1-foundation

Select the current consolidation line.

Evidence:

- It is the explicit consolidation target in ADR-0000.
- It was established from the current main baseline.
- It contains the Rust workspace, eagle-core, FFI boundary, reference implementation package, tests, and Gate 5.7.
- Production/reference separation is explicit: eagle-core does not depend on eagle-core-reference.
- The branch remains the strongest lineage candidate.

### B — execution/core-foundation-v1

Reject as canonical lineage.

Reason:

- It is a historical execution branch with divergent history.
- Direct comparison with implementation/v1-foundation is diverged at +105 / -64.
- Its work is valuable as provenance and source material, but must not be merged or rebased wholesale.

### C — execution/phase-1-security-kernel-verified-2026-10-04

Reject as canonical lineage.

Reason:

- The branch is historically close to main but still diverged.
- Direct comparison with implementation/v1-foundation is +17 / -29.
- The "verified" name is not evidence of production security; the branch contains foundation/state-machine work without production cryptography.

### D — main

Retain as the release/root baseline reference, but not as the active V1 implementation line because the current main baseline does not contain the Rust workspace/core introduced by the consolidation work.

## Decision

The project SHALL use:

- Canonical development line candidate: implementation/v1-foundation
- Canonical security-core candidate: eagle-core within implementation/v1-foundation
- Historical evidence sources: execution/core-foundation-v1 and both phase-1-security-kernel branches
- Canonical release baseline: NOT APPROVED

No historical execution branch SHALL be merged or rebased wholesale into the candidate line.

Feature or subsystem transfers from historical branches SHALL follow ADR-0000 snapshot/feature-port provenance rules.

## Security interpretation

This decision does NOT claim:

- cryptographic identity;
- production encryption;
- 1:1 E2E;
- forward secrecy;
- replay protection;
- real P2P;
- relay;
- production encrypted storage;
- production readiness.

The selected candidate currently provides a foundation consisting of security state machines, device-trust state transitions, protocol/storage/transport contracts, reference implementations, FFI scaffolding, and tests.

## Open gates before acceptance

This ADR SHALL remain Proposed until:

1. the current candidate build/test evidence is reproducibly attached to its exact commit;
2. ADR-0000 acceptance criteria are satisfied;
3. Gate 5.7 is executed and green on the current candidate;
4. the supply-chain/security-policy verifier false-positive is corrected without weakening the policy;
5. moving GitHub Action tags in Test Lab are pinned to verified commit SHAs;
6. Cargo.lock/reproducible-build policy is explicitly decided;
7. FFI security boundary review is completed;
8. a human review/approval is recorded;
9. the resulting baseline is frozen by exact commit SHA.

## Non-goals

This ADR does not authorize P2P, relay, production cryptography, production storage, or UI messaging implementation.

## Rollback

If implementation/v1-foundation fails the acceptance gates, the decision SHALL remain Proposed and the branch SHALL not be promoted. Historical branches remain immutable evidence sources; no forced rewrite is authorized by this ADR.

## Evidence

- docs/01-decisions/ADR-0000-branch-toolchain-consolidation.md
- docs/audit/pre-consolidation-2026-10-04.md
- main commit: 6c46bf53fce06ee7f7b5b2bf35c720ab5bcb7dee
- candidate tip: b6a6708654a5966bfc44e7925e46e95e979b4977
