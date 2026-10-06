# The Eagle — Canonical Requirements & Gap Matrix

## Status legend

- CANONICAL: accepted implementation authority
- REQUIRED: must be implemented/tested
- DEFERRED: intentionally postponed
- FORBIDDEN: prohibited by current baseline
- UNKNOWN: evidence not yet available
- HISTORICAL: preserved for provenance, not implementation authority

## A. Product scope

| ID | Requirement | Status | Acceptance evidence |
|---|---|---|---|
| PROD-001 | One-to-one messaging | CANONICAL | protocol rejects >2-party topology |
| PROD-002 | Text/Image/Audio/Video | REQUIRED | feature integration tests |
| PROD-003 | One-to-one audio/video calls | REQUIRED | call E2E tests |
| PROD-004 | No groups/channels | FORBIDDEN | protocol + server/signaling validation |
| PROD-005 | No screen sharing | FORBIDDEN | API/permission surface + runtime tests |
| PROD-006 | No call recording | FORBIDDEN | capability scan + runtime tests |
| PROD-007 | No public broadcast | FORBIDDEN | protocol/feature checks |
| PROD-008 | No automatic external media export | FORBIDDEN | Intent/output boundary tests |

## B. Runtime independence

| ID | Requirement | Status | Acceptance evidence |
|---|---|---|---|
| DEP-001 | Google Play used for distribution/update only | CANONICAL | architecture/release documentation |
| DEP-002 | No Firebase runtime dependency | FORBIDDEN | dependency scan |
| DEP-003 | No FCM runtime dependency | FORBIDDEN | dependency/code scan |
| DEP-004 | No Google Play Services runtime dependency | FORBIDDEN | dependency scan |
| DEP-005 | No Play Integrity dependency | FORBIDDEN | dependency/code scan |
| DEP-006 | No external analytics runtime dependency | FORBIDDEN | dependency/network scan |
| DEP-007 | No third-party messaging backend | FORBIDDEN | network-path audit |

## C. Data plane

| ID | Requirement | Status | Acceptance evidence |
|---|---|---|---|
| NET-001 | Application content is P2P-only under current baseline | CANONICAL | architecture + network tests |
| NET-002 | Direct P2P preferred | REQUIRED | connectivity matrix |
| NET-003 | TURN may be used as connectivity fallback | REQUIRED | TURN integration test |
| NET-004 | Signaling is bounded to session/connectivity control | REQUIRED | server contract + negative content tests |
| NET-005 | Server-mediated application-content relay is forbidden | FORBIDDEN | architecture gate + static/network audit |
| NET-006 | Server Outbox is forbidden in current application baseline | FORBIDDEN | repository/code scan |
| NET-007 | PostgreSQL application-content pipeline is not part of current baseline | FORBIDDEN | architecture/code scan |

## D. Rust security core

| ID | Requirement | Status | Acceptance evidence |
|---|---|---|---|
| CORE-001 | Rust is canonical security implementation | CANONICAL | module ownership map |
| CORE-002 | Identity implemented once | REQUIRED | code search + architecture test |
| CORE-003 | Session state owned by Rust | REQUIRED | API/FFI contract |
| CORE-004 | Message/file crypto owned by Rust | REQUIRED | dependency/code review |
| CORE-005 | Replay/TTL policy in Rust | REQUIRED | property/integration tests |
| CORE-006 | Secure state serialization | REQUIRED | corruption/fail-closed tests |
| CORE-007 | Typed FFI boundary | REQUIRED | ABI/ownership/error tests |
| CORE-008 | No panic/unwind across FFI | REQUIRED | fault-injection tests |

## E. Android platform

| ID | Requirement | Status | Acceptance evidence |
|---|---|---|---|
| AND-001 | minSdk 31 | CANONICAL | Gradle policy + CI enforcement |
| AND-002 | targetSdk 36 current release baseline | CANONICAL | build verification |
| AND-003 | API 37 compatibility/security testing | REQUIRED | device test matrix |
| AND-004 | Compose/ViewModel/Application/Repository separation | CANONICAL | architecture review |
| AND-005 | Keystore integration | REQUIRED | key/lifecycle tests |
| AND-006 | WorkManager only for deferred/persistent work | CANONICAL | code policy + lint/checks |
| AND-007 | no direct DAO access from UI/ViewModel | REQUIRED | architecture/static review |

## F. Storage and privacy

| ID | Requirement | Status | Acceptance evidence |
|---|---|---|---|
| DATA-001 | Room stores metadata/indexes, not plaintext content | CANONICAL | schema review + data scan |
| DATA-002 | Object store contains ciphertext | CANONICAL | runtime storage tests |
| DATA-003 | No private keys in Room/Preferences/logs | FORBIDDEN | static/runtime scan |
| DATA-004 | Backup does not expose secrets | REQUIRED | backup test |
| DATA-005 | Lock invalidates/blocks sensitive readers | REQUIRED | lock tests |
| DATA-006 | Expiry blocks subsequent decryption/access | REQUIRED | TTL tests |

## G. Calls/media

| ID | Requirement | Status | Acceptance evidence |
|---|---|---|---|
| CALL-001 | WebRTC candidate for call media | REQUIRED | two-device call POC |
| CALL-002 | Core-Telecom POC before production call integration | REQUIRED | POC evidence |
| CALL-003 | TURN fallback tested | REQUIRED | network impairment matrix |
| MEDIA-001 | Internal playback canonical | CANONICAL | output-boundary tests |
| MEDIA-002 | Streaming/chunked encryption | REQUIRED | memory/performance tests |
| MEDIA-003 | Oversized/malformed media rejected | REQUIRED | negative tests |

## H. Performance and size

| ID | Requirement | Status | Acceptance evidence |
|---|---|---|---|
| PERF-001 | Baseline measured before optimization | REQUIRED | stored benchmark baseline |
| PERF-002 | Baseline Profiles | REQUIRED | benchmark report |
| PERF-003 | Macrobenchmark | REQUIRED | CI artifact |
| PERF-004 | Minimize sensitive ByteArray copies | REQUIRED | profiling/code review |
| PERF-005 | Main-thread crypto/media/network work prohibited | REQUIRED | trace/tests |
| PERF-006 | R8/resource shrinking evaluated for release | REQUIRED | AAB/APK report |
| PERF-007 | ABI minimization evaluated | REQUIRED | build-size report |
| PERF-008 | No optimization may add an external runtime service | FORBIDDEN | dependency/network gate |

## I. CI/release/security

| ID | Requirement | Status | Acceptance evidence |
|---|---|---|---|
| CI-001 | Kotlin lint/format | REQUIRED | CI |
| CI-002 | Rust fmt/clippy/tests | REQUIRED | CI |
| CI-003 | FFI tests | REQUIRED | CI |
| CI-004 | secret scanning | REQUIRED | CI |
| CI-005 | dependency/SBOM scanning | REQUIRED | CI |
| CI-006 | security static analysis | REQUIRED | CI |
| REL-001 | release AAB audited | REQUIRED | release artifact |
| REL-002 | build provenance recorded | REQUIRED | provenance record |
| REL-003 | checksums recorded | REQUIRED | artifact manifest |
| REL-004 | release security sign-off | REQUIRED | signed gate |
| REL-005 | independent penetration/security review | REQUIRED | report + closure evidence |

## J. Historical conflicts to archive

| Conflict | Classification | Resolution |
|---|---|---|
| FCM wake path | HISTORICAL | superseded by no-FCM baseline |
| Firebase/Play Services runtime path | HISTORICAL | superseded by runtime independence |
| minSdk 29 | HISTORICAL | superseded by minSdk 31 |
| server encrypted-envelope pipeline | HISTORICAL/DEFERRED | not current application-data path |
| server Outbox implementation | HISTORICAL/DEFERRED | blocked by architecture gate / Issue #86 |
| server PostgreSQL content store | HISTORICAL/DEFERRED | requires explicit re-authorization |

## K. Evidence-state rule

A document, branch, sample, or generated source file is not evidence of implementation by itself.

Each requirement must eventually link to:

- implementation path
- test path
- test result/artifact
- reviewer
- approval state

Unknown remains UNKNOWN until repository evidence is produced.
