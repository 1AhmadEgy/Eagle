# Rust Core / Security Kernel — Full Lifecycle

**Date:** 2026-10-05  
**Specialization:** Rust Core / Security Kernel  
**Branch:** `execution/rust-core-security-kernel-complete-2026-10-05`  
**Current code head:** `bc05c18420b2c0a29bb935466db6fa2ae0c9e766`

## 01 Inventory
Located Rust Core work across the active specialization branch and divergent historical execution branches. Current specialization source is isolated under `core/`.

## 02 Provenance
Recorded branch history, source paths, security/architecture documents, and CI evidence. Historical PrivateMesh material remains provenance evidence, not automatic authority.

## 03 Classification
Separated accepted architecture, proposed ADRs, historical references, implementation files, tests, and verification evidence.

## 04 Triage
Split work into deterministic kernel behavior that can be safely implemented now and cryptographic/key/protocol work that remains gated.

## 05 Deep Analysis
Reviewed trust/session state machines, API mutability, protocol bounds, frame/envelope construction, device lifecycle, FFI rules, and CI workflow.

## 06 Reconciliation
Compared prior Rust baselines with the current specialization. Smaller-TCB fail-closed behavior was retained; externally callable trust promotion was rejected.

## 07 Conflicts
Resolved within scope:
- public trust elevation → rejected;
- externally mutable sensitive representation → rejected;
- unbounded protocol acceptance → rejected;
- value-copyable authority state → rejected and remediated;
- public rekey completion without cryptographic proof → restricted to test/internal seam;
- interrupted authentication/rekey without explicit safe exit → fail-closed cancellation paths.

Unresolved out-of-scope conflicts remain under the relevant ADRs.

## 08 Gaps
Remaining specialization gaps are independent security review and the explicit pre-production protocol/crypto/FFI dependencies. Cryptographic/session semantics, key management, serialization, replay protection, transport, and FFI remain dependencies.

## 09 Canonical Authority
Authority order:
1. `PLATFORMS.md` accepted platform boundary;
2. project security baseline;
3. current Security Kernel specialization contract;
4. accepted repository evidence;
5. proposed ADRs for pending decisions;
6. historical material as evidence only.

## 10 Remediation Plan
Harden public representations, remove authority duplication, validate frame construction, expand negative-path tests, document the Rust-only threat model, and maintain release evidence.

## 11 Correction
Completed:
- private Device/Envelope/Frame state;
- read-only accessors;
- guarded trust/session transitions;
- bounded monotonic negotiation;
- terminal revocation/replacement;
- fail-closed constructors;
- no public trust promotion;
- no `Copy`/`Clone` on authority-bearing values;
- public rekey completion removed until a real cryptographic proof exists;
- authentication abort returns to Untrusted/Idle;
- rekey abort closes the session.
- validated `FrameHeader` constructor and integration-test repair;
- bounded structural frame decoder with truncation/length checks;
- test-only monotonic send sequence and receive replay-window seams;
- device authority re-check at `establish` and `begin_rekey` to close revocation race windows.

## 12 Implementation
The deterministic Security Kernel is implemented as a dependency-free Rust library with pinned toolchain metadata.

## 13 Testing
Unit and integration negative-path coverage is present. Historical Rust Security Kernel CI run `37286323259` passed Format, Tests, and Clippy for code head `3ef9fcd0647ab945e872f4a5607a2449e772394c`. Current head `bc05c18420b2c0a29bb935466db6fa2ae0c9e766` has a fresh queued Rust workflow; current-head verification is therefore PENDING. Crypto/fuzz/interop/key-storage/FFI categories remain PENDING.

## 14 Security Review
Static specialization review is in progress. The main authority-duplication risk has been remediated, and a second revocation race was identified and closed at state transitions.

Independent human security review remains required.

## 15 Verification
The preceding Rust CI failures were resolved through fail-closed API/test corrections. Historical run `37286323259` passed Format, Tests, and Clippy for an older executable head. Current-head verification for `bc05c18420b2c0a29bb935466db6fa2ae0c9e766` is PENDING.

## 16 Evidence
Evidence is persisted in:
- Security Kernel specialization contract;
- deep security research;
- Rust-only threat model;
- requirements/gap matrix;
- test matrix;
- execution record;
- release gate;
- draft PR #81.

## 17 Release Gate
Implementation hardening: **COMPLETE**.  
Current-head Rust verification: **PENDING**.  
Independent review: **PENDING**.

## 18 Release
No production merge/release is authorized by this specialization while verification and human review remain incomplete.

## 19 Post-Release
Defined but not activated. On release, monitor parser failures, policy-denial anomalies, state-machine violations, dependency/toolchain changes, and security reports without logging secrets.

## 20 Re-entry
Any protocol/key/FFI/security-boundary change reopens the lifecycle from Inventory and repeats reconciliation, threat analysis, tests, evidence, and release gating.
