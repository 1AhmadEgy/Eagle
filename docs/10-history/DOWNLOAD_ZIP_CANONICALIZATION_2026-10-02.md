# Eagle — Download.zip Canonicalization & Repository Reconciliation
Date: 2026-10-02
Source: ChatGPT uploaded `Download.zip`
SHA-256: `ade73a06e74f856b8160e0789093625f195030e270ec59053ed63ffcc0fe0608`
Size: 6,903,508 bytes

## 1. Safety / handling
The archive was inspected statically. No scripts, binaries, installers, or project executables from the archive were executed. Nested archives and documents are untrusted inputs until separately reviewed.

## 2. Package inventory
| Package | Files |
|---|---:|
| Eagle_Consolidated_v 3.2.zip | 3 |
| Eagle_Developer_Designer_Handoff_FINAL_v1.0.zip | 31 |
| Eagle_FINAL_RESEARCH_APPLIED_v1.2.0.zip | 44 |
| Eagle_FINAL_RESEARCH_APPLIED_v1.3.0.zip | 56 |
| Eagle_RESEARCH_APPLIED_v1.1.0.zip | 50 |
| Eagle_v0.1.0-dev_HANDOFF.zip | 52 |
| Eagle_v1.0.0-beta_HANDOFF.zip | 52 |
| Eagle_v1.0.0-rc_HANDOFF.zip | 52 |

Cross-package analysis found 226 unique artifact paths, 44 paths shared by two or more packages, 33 shared paths byte-identical, and 11 shared paths with different SHA-256 values.

## 3. Conflict set
The 11 shared-content conflicts are:
- `.github/workflows/ci.yml`
- `00_CONTROL/FINAL_STATUS.md`
- `00_CONTROL/MANIFEST_SHA256_APPLIED.csv`
- `00_CONTROL/README_AR.md`
- `16_CHANGELOG/CHANGELOG.md`
- `docs/ci/CI_CD.md`
- `docs/release/VERSION_POLICY.md`
- `releases/RELEASE_MATRIX.md`
- `releases/v0.1.0-dev/STATUS.md`
- `releases/v1.0.0-beta/STATUS.md`
- `releases/v1.0.0-rc/STATUS.md`

These must not be overwritten by filename-based merging.

## 4. Version transition
### v1.1.0 -> v1.2.0
v1.2.0 introduces execution/evidence artifacts including:
- `17_EXECUTION/IMPLEMENTATION_REPORT_APPLIED.md`
- `17_EXECUTION/OPEN_BLOCKERS.md`
- `docs/qa/EXECUTION_CONTRACT.md`
- `docs/release/RELEASE_EVIDENCE.md`
- `docs/security/SECURITY_EVIDENCE.md`

### v1.2.0 -> v1.3.0
v1.3.0 adds security/provenance/release controls, including a security workflow, versioned manifest, next-execution plan, provenance/signing documentation, supply-chain evidence, release checklists, checksum scripts, and release-structure verification. Existing content is largely preserved; the changelog is updated.

### v1.3.0 -> Developer/Designer Handoff
The handoff is materially smaller and should be treated as a delivery projection/subset, not a replacement for the full evidence/release package.

## 5. Repository reconciliation
The current `main` already contains the project-control/security layer, including:
- `docs/MASTER_PROJECT_SOURCE_INDEX.md`
- `docs/FILE_PROVENANCE_REGISTER.md`
- `docs/CHATGPT_ATTACHMENT_INTAKE_STATUS.md`
- `docs/SECURITY-BASELINE.md`
- `docs/04-security/SECURITY_BASELINE.md`
- `docs/04-security/...` security controls
- `docs/07-verification/VERIFICATION_REGISTER.md`
- `docs/10-history/HISTORICAL_MATERIAL_REGISTER.md`
- `docs/13-execution/EXECUTION_CONTINUATION_PLAN.md`
- `docs/legal/provenance.md`
- `docs/provenance/schema.json`
- `scripts/ci/verify-provenance.py`
- `scripts/ci/verify-security-policy.py`
- `scripts/pre-push-gate.sh`
- application and Gradle baseline files.

The repository therefore has an established provenance/security framework and should not absorb the uploaded packages as a second parallel documentation tree.

## 6. Canonicalization decision
The uploaded package set is classified as:
- **Historical evidence:** all original package versions.
- **Reference material:** architecture, requirements, design, security, QA and implementation documents after content review.
- **Overlay candidates:** later package additions that add evidence or controls without replacing the established baseline.
- **Conflict candidates:** the 11 shared paths above.
- **Not canonical by upload status alone:** no file becomes authoritative merely because it is inside a package named FINAL, MASTER, RELEASE, or CONSOLIDATED.

## 7. Security-critical promotion gate
Before any artifact is promoted from archive/reference to implementation authority:
1. Preserve source package and SHA-256.
2. Check for secrets, credentials, certificates, private keys and unnecessary personal data.
3. Resolve dependency/version claims against authoritative project requirements.
4. Bind security decisions to an approved ADR.
5. Bind architecture changes to the active architecture baseline.
6. Bind release claims to executable evidence and CI results.
7. Record the promotion/rejection in provenance and change ledgers.

## 8. Current repository gap
The most important remaining work is not bulk extraction. It is **authority reconciliation**: map the uploaded package artifacts against the existing `docs/01-decisions/DECISION_LOG.md`, security baseline, source register, verification register, and execution continuation plan; then promote only evidence-backed deltas.

## 9. Conclusion
Do not flatten `Download.zip` into `main`. Preserve it as an externally supplied historical source, reconcile its 11 content conflicts explicitly, and promote only validated deltas. This avoids silently replacing security, CI, release, or decision controls with an older or differently scoped package.
