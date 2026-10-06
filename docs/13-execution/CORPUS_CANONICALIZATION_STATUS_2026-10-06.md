# Eagle Canonicalization Status — 2026-10-06

## Current authority

**Canonical execution reference:** `main @ 46aa86b6d71d34396b86b35124392cb3ba49c2e9`.

Branch/PR names, version labels, archives and prior snapshots are evidence/provenance unless their content is merged into `main` and passes the applicable evidence gates.

## Evidence hierarchy

1. Current Git object/file content on `main`.
2. Explicitly accepted project ADR/requirement records.
3. Verified branch/PR evidence, classified as candidate until integrated.
4. Historical archives and conversation artifacts.
5. External research, classified separately and never authoritative by itself.
6. Unretrieved or unresolved references: Pending/Unverified.

## Current verified repository facts

| Area | Verified state | Classification |
|---|---|---|
| Repository | public, active, default branch `main` | Canonical |
| Main HEAD | `46aa86b6d71d34396b86b35124392cb3ba49c2e9` | Canonical |
| Main branch protection | `protected=false`; rulesets `[]` | Verified finding / Blocked |
| Main CI | run `37290062138` failed security policy | Verified / Blocked |
| Main Test Lab | run `37290062004` failed Android 37 SDK package lookup | Verified / Blocked |
| Product code on main | Android skeleton + smoke test; no production Rust/KMP/Desktop/iOS implementation | Verified / Partial |
| Security baseline | present | Canonical baseline |
| Identity/trust threat model | present | Canonical design baseline / implementation evidence incomplete |
| Crypto protocol ADR | ADR-0008 Proposed | Proposed/Pending |
| Key management ADR | ADR-0009 Proposed | Proposed/Pending |
| Serialization ADR | ADR-0010 Proposed | Proposed/Pending |
| Storage/transport/observability ADR scopes | ADR-0011..0014 worklist | Proposed/Pending |
| Platform strategy | accepted reference baseline | Accepted scope; implementation incomplete |
| External AI research | PR #90 only, not merged | Candidate / Proposed |

## Active PR reconciliation

| PR | Head | Relation to current main | Current evidence | Status |
|---|---|---|---|---|
| #45 | `61f492f9aa7ce411fa6b296f01c64d318d119ea3` | 1 ahead / 0 behind | CI + Test Lab SUCCESS | Candidate / Verified remediation |
| #77 | `6de9af43315c1e69fdb52b2c32ecb83927cfcdc2` | diverged; 2 behind | CI SUCCESS, Test Lab FAIL, Rust SUCCESS | Candidate / Partial / Blocked |
| #79 | `bfb9403652d5ae72e62a7de0334a240b4987717b` | diverged; 2 behind | CI FAIL, Rust FAIL | Candidate / Blocked |
| #81 | `abb8e3342c31f081b2269425d78be68d409d5ec9` | diverged; 2 behind | CI/Test Lab/Rust FAIL | Candidate / Blocked |
| #84 | `dc769dfd43408b89c97d0642cfcd6846cc8b2751` | diverged; 2 behind | CI/Rust FAIL | Candidate / Blocked |
| #85 | `d662c2c4e635be61215678cdb819a5795836c86e` | 192 ahead / 0 behind | CI/Rust FAIL | Candidate / Blocked |
| #88 | `5952e696cb97cbe99fa11698eb75fee99fa2a5f6` | 4 ahead / 0 behind | CI/Test Lab FAIL | Candidate / Documentation |
| #89 | `429898ace353a8918ee4828596592cd3d000a885` | 25 ahead / 0 behind | CI/Test Lab FAIL | Candidate / Documentation |
| #90 | `af7e1ba7fcc0ff1d2e6e52760f83650ebe76b96f` | 2 ahead / 0 behind | Documentation/research; not merged | Candidate / Proposed |

No reviewed PR above is treated as merged merely because an API field exposes a `merge_commit_sha`.

## Architecture / security reconciliation

### Confirmed
- Security baseline requires least privilege, protected main, dependency review, tests, explicit authn/authz, sanitized audit logs and recovery design.
- Identity/trust model explicitly separates account/device/session/pairing/trust/recovery.
- Dependency graph forbids plaintext handling in Mesh/Protocol, private-key exposure through Storage/UI, and Crypto↔Mesh circularity.
- Desktop/iOS custody contracts require real platform custody level and fail-closed behavior.

### Pending
- Final cryptographic protocol/provider and interoperability/conformance.
- Final key-management architecture/provider approval.
- Canonical serialization.
- Production transport/P2P runtime, NAT traversal, relay policy, reconnect/offline synchronization.
- Production storage/database, deletion/backup/recovery semantics.
- Cross-platform implementation and device evidence.
- UI security-state implementation/evidence.
- Independent security review and adversarial/fuzz/property coverage.
- SBOM, reproducible release, signing and provenance evidence.

## Provenance and historical corpus

The Library corpus confirms a durable execution/handoff baseline with explicit BLOCKED/GATED work items covering source intake, toolchain/lockfile, ADR closure, security kernel, protocol conformance, storage/recovery/deletion, transport/sync, mobile integration, executable test IDs, fuzz/property testing, SBOM/provenance/signing and independent verification.

Historical/current corpus material remains a graph rather than a linear replacement chain. Package/version labels must not override current `main` evidence.

## External research

PR #90 preserves the rule that external material is advisory:
`Source → Verification → Impact → Requirement candidate → ADR/Test/Gate → Human Review → Accepted`.
Sources EXT-001..EXT-020 remain research evidence and non-decisions unless separately promoted through Eagle governance.

## Release decision

**RELEASE = BLOCKED.**

Blocking classes currently include:
1. main CI/Test Lab failures;
2. missing technical enforcement of main protection;
3. unapproved crypto/key/serialization/storage/transport decisions;
4. incomplete P2P and cross-platform implementation evidence;
5. incomplete adversarial/security verification;
6. missing release provenance/SBOM/signing/independent verification.

## Next canonicalization rule

Do not merge an entire branch because its title says canonical/final. Reconcile each requirement/ADR/component/test/evidence chain. When a candidate is merged, re-run the full exact-head verification and update this record with the resulting main SHA.
