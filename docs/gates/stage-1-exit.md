# Stage 1 Exit Gates (pre-crypto)

This is the executable checklist for closing the Phase 1 security-kernel gate and entering the next implementation stage.

## PR #41 gate

- [ ] Test Lab: PASS on the final execution/core-foundation-v1 head
- [ ] Rust Core: PASS
- [ ] Repository/security-policy checks: PASS
- [ ] Secret scan: PASS
- [ ] Android application compileSdk == targetSdk == 37
- [ ] No Lint suppression/baseline is used to hide a blocking error
- [ ] PR #45 evidence remains independent; its success does not satisfy #41

## Stage 2 entry gates

- [ ] SessionCoordinator mandatory cases covered:
  - [ ] Idle -> Connecting -> NeedsTrust -> Active
  - [ ] Idle -> Connecting -> Failed(DeviceNotTrusted)
  - [ ] Idle -> Connecting -> Failed(ContractNotReady)
  - [ ] cancellation while Connecting
  - [ ] endSession on a non-active handle
- [ ] unsafe in our authored core/ffi code: 0
- [ ] Generated UniFFI unsafe is treated as generated output, not authored source
- [ ] ABI decision recorded and verified: arm64-v8a + armeabi-v7a; x86_64 only when required by CI/emulator usage
- [ ] Rust native .so has SBOM, checksum, and provenance evidence
- [ ] Negative FFI test proves no private-key material crosses UniFFI
- [ ] No artifact is trusted without executable provenance/evidence
- [ ] No production cryptographic implementation starts before the applicable ADR decisions are closed

## Evidence rule

A checked box requires a repository or CI artifact that can be independently inspected. Narrative claims alone do not satisfy a gate.

## Current interpretation

The previous 36/36 Android configuration was rejected by current Lint as OldTargetApi. The gate therefore follows the actual CI-tested Android 37 environment rather than suppressing Lint. This is an evidence-driven CI decision, not a cryptographic or protocol decision.
