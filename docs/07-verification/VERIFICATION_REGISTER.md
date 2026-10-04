# Eagle — Verification Register

## Verification vocabulary

- Verified — direct reproducible evidence exists.
- Configured — configuration exists but runtime evidence is not established.
- Pending — evidence is missing.
- Failed — verification was executed and failed.
- Not applicable — intentionally outside current scope.

## Current register

| ID | Verification target | State | Evidence / next action |
|---|---|---|---|
| VER-001 | Android module exists and is configured | Verified | app/build.gradle.kts |
| VER-002 | MainActivity executable bootstrap exists | Verified | app/src/main/.../MainActivity.kt |
| VER-003 | Unit-test source exists | Verified | app/src/test/.../SmokeTest.kt |
| VER-004 | CI/Test Lab workflow configuration exists | Verified | .github/workflows/testlab.yml |
| VER-005 | Main CI workflow configuration exists | Verified | .github/workflows/ci.yml |
| VER-006 | Current smoke test provides product assurance | Failed / Insufficient | test is tautological; replace with behavior-level verification |
| VER-007 | Current commit has release-grade CI evidence | Pending | record workflow run ID + commit SHA + artifacts |
| VER-008 | Production E2EE correctness | Pending | protocol implementation + vectors + tests |
| VER-009 | Device identity/key lifecycle | Pending | identity implementation + lifecycle tests |
| VER-010 | Authenticated protocol envelope | Pending | canonical serialization + authentication + replay tests |
| VER-011 | Secure persistence | Pending | storage implementation + migration/corruption/recovery tests |
| VER-012 | PrivateMesh correctness | Pending | discovery/routing/relay implementation + adversarial tests |
| VER-013 | Supply-chain evidence | Pending | dependency lock/SBOM/advisory/provenance evidence |
| VER-014 | Production security review | Pending | threat model + review record |
| VER-015 | Filebin historical archive intake | Pending | archive bytes, hashes, manifest, scans and classification |

## Evidence retention

For each completed verification, record at minimum:

- commit SHA;
- workflow/run identifier where applicable;
- test command or procedure;
- result;
- artifact identifiers;
- reviewer/date;
- follow-up defects.

A green workflow without a commit reference is not sufficient for release evidence.

## Verification priority

1. Bootstrap reproducibility.
2. First component contract.
3. Identity/key lifecycle.
4. Protocol envelope.
5. End-to-end encrypted message.
6. Persistence/recovery.
7. Transport.
8. Mesh.
9. Release/supply-chain/security evidence.
