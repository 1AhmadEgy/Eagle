# Security-First Build Execution Evidence — 2026-10-05

## Scope
Executable verification target for the integrated architecture/security-first baseline.

## Target commit
Current branch head: 47bd6254aa14df475e7e39609d884a55d0e12ef3

## Expected CI checks
1. Repository hygiene
2. Secret scan
3. Security policy verification
4. Architecture security boundary verification
5. Product-surface consistency
6. Rust workspace format/test/clippy where applicable
7. Android build/test/lint where applicable
8. Test Lab category status

## Local execution limitation
The execution environment used for this session cannot resolve GitHub network endpoints and therefore cannot clone the branch locally. GitHub Actions remains the authoritative external build executor for this commit.

## Safety rule
No PASS is inferred from source inspection. Category status remains determined by exact-commit CI/Test Lab evidence.

## Release
NO-GO until all mandatory release gates are evidenced.


## Android TestLab correction — 2026-10-05
- TestLab run 1342 exposed a real defensive-copy defect in `EncryptedStoragePayload` (`payloadDefensivelyCopiesOutputArrays`).
- The payload now owns private copies of IV/ciphertext and exposes copies only; decryption consumes a copied IV.
- This is a security-hardening correction; Android TestLab must be rerun and pass before release-gate advancement.
