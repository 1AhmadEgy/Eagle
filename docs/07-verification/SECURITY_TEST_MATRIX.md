# Eagle Security Test Matrix

> **Status:** Required verification matrix
> **Date:** 2026-10-06
> **Targets:** Android, Desktop (Windows/macOS/Linux), iOS
> **Web:** Out of scope

## 1. Matrix rules

This matrix maps security domains to test layers and evidence.

The matrix is a planning and acceptance artifact. A row becomes PASS only when the corresponding executable or formally accepted evidence exists.

## 2. Core matrix

| ID | Domain | Core/shared | Android | Desktop | iOS | Gate |
|---|---|---|---|---|---|---|
| SEC-001 | Secret leakage | ✓ | ✓ | ✓ | ✓ | Mandatory |
| SEC-002 | Dependency vulnerabilities | ✓ | ✓ | ✓ | ✓ | Mandatory |
| SEC-003 | SAST/static analysis | ✓ | ✓ | ✓ | ✓ | Mandatory |
| SEC-004 | Key generation/lifecycle | ✓ | ✓ | ✓ | ✓ | Mandatory |
| SEC-005 | Key purpose/domain separation | ✓ | ✓ | ✓ | ✓ | Mandatory |
| SEC-006 | Session security | ✓ | ✓ | ✓ | ✓ | Mandatory |
| SEC-007 | Protocol transcript integrity | ✓ | ✓ | ✓ | ✓ | Mandatory |
| SEC-008 | Replay resistance | ✓ | ✓ | ✓ | ✓ | Mandatory |
| SEC-009 | Downgrade resistance | ✓ | ✓ | ✓ | ✓ | Mandatory |
| SEC-010 | Identity binding | ✓ | ✓ | ✓ | ✓ | Mandatory |
| SEC-011 | Authorization/revocation | ✓ | ✓ | ✓ | ✓ | Mandatory |
| SEC-012 | Serialization/parser robustness | ✓ | ✓ | ✓ | ✓ | Mandatory |
| SEC-013 | Recovery boundaries | ✓ | ✓ | ✓ | ✓ | Mandatory |
| SEC-014 | Deletion/destruction | ✓ | ✓ | ✓ | ✓ | Mandatory |
| SEC-015 | Native secure storage | — | ✓ | ✓ | ✓ | Mandatory |
| SEC-016 | Platform permissions/lifecycle | — | ✓ | ✓ | ✓ | Mandatory |
| SEC-017 | Package/signing integrity | — | ✓ | ✓ | ✓ | Mandatory |
| SEC-018 | Backup/restore migration | ✓ | ✓ | ✓ | ✓ | Mandatory |
| SEC-019 | Logging/crash-data hygiene | ✓ | ✓ | ✓ | ✓ | Mandatory |
| SEC-020 | Adversarial malformed input | ✓ | ✓ | ✓ | ✓ | Mandatory |
| SEC-021 | Fuzz/property testing | ✓ | ✓* | ✓* | ✓* | Required where applicable |
| SEC-022 | Regression suite | ✓ | ✓ | ✓ | ✓ | Mandatory |
| SEC-023 | Evidence provenance | ✓ | ✓ | ✓ | ✓ | Mandatory |
| SEC-024 | Human security review | ✓ | ✓ | ✓ | ✓ | Mandatory for material changes |

`*` Platform execution is required when the relevant parser/protocol boundary exists on that target.

## 3. Platform security matrix

### Android

| Control | Required evidence |
|---|---|
| Keystore | Key creation/use/deletion tests and custody-level evidence |
| Hardware-backed support | Capability detection and truthful policy mapping |
| App permissions | Manifest/runtime permission review |
| Components | Exported component and intent-boundary review |
| Local storage | Sensitive-data-at-rest tests |
| Backup | Backup/export policy for protected domains |
| Signing | Release artifact signature verification |
| Debug boundary | Debug-only capabilities excluded from production profile |
| Lifecycle | Background/resume/kill/restart security tests |

### iOS

| Control | Required evidence |
|---|---|
| Keychain | Access-control and lifecycle tests |
| Secure Enclave | Only for supported key types/operations; evidence of actual use |
| Entitlements | Reviewed entitlement set |
| Signing | Application signature/profile verification |
| Data protection | Required protection class verification |
| Backup/migration | Recovery and migration boundary tests |
| Debug boundary | Production build isolation |
| Logging | Sensitive-data suppression tests |

### Desktop

| Control | Required evidence |
|---|---|
| Windows protected storage | Native adapter tests |
| macOS Keychain | Native adapter tests |
| macOS Secure Enclave | Capability-specific tests only |
| Linux protected storage | Backend discovery and custody reporting |
| File permissions | Local file/IPC boundary tests |
| Package/signing | Artifact integrity and signature checks |
| Update path | Replacement/update integrity |
| Import/export | Key-domain and lifecycle preservation tests |
| Destruction | Irreversible lifecycle/state evidence |

## 4. Fuzz/property matrix

Priority targets:

1. serialization and canonicalization;
2. protocol message parsing;
3. state-machine transitions;
4. pairing/replay inputs;
5. identity/trust records;
6. recovery artifacts;
7. storage metadata;
8. malformed/oversized input;
9. cryptographic boundary APIs.

Fuzz failures are security findings until triaged and resolved or formally accepted.

## 5. Negative-test requirement

Every security-sensitive component must have negative tests for at least:

- wrong key;
- wrong identity;
- stale epoch;
- revoked device;
- malformed message;
- replay;
- unexpected state;
- unsupported algorithm/configuration;
- insufficient platform custody;
- unauthorized access.

## 6. Traceability

Each test case should map to:

```text
Requirement
   ↓
ADR / Security Decision
   ↓
Threat / Abuse Case
   ↓
Test ID
   ↓
Executable Test / Evidence
   ↓
Gate Decision
```

A test without an identified requirement/threat or acceptance rule should not be promoted to a release-critical PASS merely because it executes successfully.

## 7. Current implementation truth

The repository contains a Test Lab framework, CI category runner, security-policy checks, and a smoke test proving the framework can bootstrap.

That does **not** establish that all matrix rows are implemented.

Until executable category-specific evidence exists, the corresponding status remains PENDING.
