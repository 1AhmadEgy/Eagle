# Eagle — Project File Provenance Register

> **Canonical reconciliation:** `main@46aa86b6d71d34396b86b35124392cb3ba49c2e9` (2026-10-07)
> The historical inventory below is preserved verbatim in substance. This reconciliation adds authority/scope rules without deleting historical provenance.

## Current authority model

| Source type | Authority | Meaning |
|---|---|---|
| MAIN | CANONICAL_IMPLEMENTATION | Exact file/tree state on canonical main |
| PULL_REQUEST | VERIFIED_CANDIDATE | Candidate evidence; not main evidence until merged and re-verified |
| BRANCH | VERIFIED_CANDIDATE / UNVERIFIED | Non-main candidate |
| ARCHIVE | HISTORICAL_PROVENANCE | Historical reference only |
| CONVERSATION | HISTORICAL_PROVENANCE / UNVERIFIED | Context only unless independently committed and verified |
| EXTERNAL_REFERENCE | ACCEPTED_REFERENCE | Explicitly accepted external reference |

## Invariants

- `PR PASS != main PASS`.
- `PROPOSED != ACCEPTED != IMPLEMENTED != VERIFIED != RELEASED`.
- Git blob SHA is provenance evidence, not a SHA-256 checksum.
- Claims require an exact source/commit/snapshot.
- Historical material is not promoted automatically.
- Independent SHA-256 is required where artifact integrity must be established.

## Canonical implementation snapshot

- Repository: `1AhmadEgy/Eagle`
- Branch: `main`
- Commit: `46aa86b6d71d34396b86b35124392cb3ba49c2e9`
- Tree: `a7718840805359b042ec20a1781daaa5531ee47f`

## Register scope

This register preserves the historical file inventory below. Current implementation inventory is derived from the exact canonical main tree and coordinated through `docs/MASTER_PROJECT_SOURCE_INDEX.md`; no parallel repository-inventory taxonomy is introduced by G0-A.

---

> Registry generated from the historical Git tree at commit `812389dc9a19c52ca8089397c96a09d46957336b` (2026-10-01).
> It records files that existed in the repository history and were subsequently removed from the current `main` tree. It does not claim that this list exhausts files uploaded only inside ChatGPT conversations.

## Ownership / coordination
- Repository: `1AhmadEgy/Eagle`
- Project owner / coordination account: `1AhmadEgy`
- Historical source commit: `812389dc9a19c52ca8089397c96a09d46957336b`
- Current main tree: only `README.md` was present when checked.

## Security and provenance rules
- Git blob SHA is recorded as provenance evidence; it is **not** a SHA-256 checksum.
- Binary archives/documents are not re-created or modified by this register.
- Files are treated as historical project inputs until reviewed and classified.
- Do not execute scripts/binaries from these archives before security review.

## Historical file inventory (44 files)

| # | Historical path | Size (bytes) | Git blob SHA | Status |
|---:|---|---:|---|---|
| 1 | `ملفات تحتاج إلى مراجعة/A-Decade-of-GHOSTS-in-the-Machine-From-Cyber-Range-Realism-to-Cognitive-Missio_MoudN97.pdf` | 651614 | `1b109400a35abf4da7e73d9ed12d88783e5f4361` | Historical / removed from current main |
| 2 | `ملفات تحتاج إلى مراجعة/ADR_Decision_Pack.md` | 11824 | `d1ea9390da65a38ec155391d7cc664c94712519f` | Historical / removed from current main |
| 3 | `ملفات تحتاج إلى مراجعة/ADR_Decision_Pack_v1.2.md` | 16172 | `157ceba1e3c14a771dab1ca99a5cfe09b39cdfe1` | Historical / removed from current main |
| 4 | `ملفات تحتاج إلى مراجعة/Action_Plan_&_Gaps_Report.md` | 18316 | `2f15ae301d60eb9680c6d97900c7c3de7eb21a27` | Historical / removed from current main |
| 5 | `ملفات تحتاج إلى مراجعة/DeepSeek _ Into the Unknown_1790580016074.pdf` | 3065973 | `bd2a3fd1ef4d9e7a4b958a4d3cb53162170404a5` | Historical / removed from current main |
| 6 | `ملفات تحتاج إلى مراجعة/EAGLE_REFERENCE_INDEX_V2.md` | 1776 | `57e860bc0926d2b0ccb74c04425f8630cf369f1b` | Historical / removed from current main |
| 7 | `ملفات تحتاج إلى مراجعة/Eagle_ALL_PROJECT_DOCUMENTS_2026-09-29.zip` | 3950969 | `252357564a2440206d0ce11344c7e408724da77a` | Historical / removed from current main |
| 8 | `ملفات تحتاج إلى مراجعة/Eagle_COMPLETION_AND_CORRECTION_PACKAGE_v2.6_2026-09-30.zip` | 42043 | `b2d2e337f823412008a6217de6ec41fc9e0d4841` | Historical / removed from current main |
| 9 | `ملفات تحتاج إلى مراجعة/Eagle_CONTINUOUS_ENGINEERING_PRODUCTION_READINESS_v2.7_2026-09-30.zip` | 25849 | `92ac95e8ca5b56c2dc7157b206c24c0541fc0d64` | Historical / removed from current main |
| 10 | `ملفات تحتاج إلى مراجعة/Eagle_CORRECTED_ADDITIONAL_FILES_v1.0.zip` | 12653 | `e7749c9bb2719abac2d83ed2be9efc4a938776e0` | Historical / removed from current main |
| 11 | `ملفات تحتاج إلى مراجعة/Eagle_CORRECTED_DEVELOPMENT_PACKAGE_v2.4_2026-09-29.zip` | 20752 | `91fbfe1b277e1b1ea0f1b2c3e38d0f951d50f922` | Historical / removed from current main |
| 12 | `ملفات تحتاج إلى مراجعة/Eagle_ENGINEERING_MERGE_MATRIX_v3.1_2026-09-30.zip` | 8435 | `df6ac772b88271300d8924b0abd292465b586920` | Historical / removed from current main |
| 13 | `ملفات تحتاج إلى مراجعة/Eagle_FINAL_Documentation_Package_v1.0_2026-09-29.zip` | 424513 | `68c8fc5d18eeed7c6a446704b8473d18808f87ce` | Historical / removed from current main |
| 14 | `ملفات تحتاج إلى مراجعة/Eagle_FINAL_Documentation_Package_v2.0_2026-09-29.zip` | 3893671 | `342a440a0a0581d9cb9abc34908a779ac0e9b04f` | Historical / removed from current main |
| 15 | `ملفات تحتاج إلى مراجعة/Eagle_FINAL_Documentation_Package_v2.1_2026-09-29.zip` | 3903690 | `cde62df9f67e0330e1b4435b52441b1201b777dd` | Historical / removed from current main |
| 16 | `ملفات تحتاج إلى مراجعة/Eagle_FINAL_Documentation_Package_v2.2_2026-09-29.zip` | 3905454 | `67d6a648dcaf31a68a0e976e69f98d0a8ae5be8a` | Historical / removed from current main |
| 17 | `ملفات تحتاج إلى مراجعة/Eagle_FINAL_Documentation_Package_v2.3_2026-09-29.zip` | 3903026 | `b79c09f265f74c50f46b3b142ba599240f171298` | Historical / removed from current main |
| 18 | `ملفات تحتاج إلى مراجعة/Eagle_FULL_FILE_AUDIT_v3.0_2026-09-30.zip` | 61897 | `e9399d7c6ca7170831ac1a0fdd2b8b0c3e8db6d6` | Historical / removed from current main |
| 19 | `ملفات تحتاج إلى مراجعة/Eagle_GAP_CLOSURE_PACKAGE_v1.0.zip` | 11615 | `537e96a3a4b09551de1e13e56f19ea097f742e66` | Historical / removed from current main |
| 20 | `ملفات تحتاج إلى مراجعة/Eagle_GAP_CLOSURE_PACKAGE_v1.1.zip` | 7754 | `e4337a6e76770100083f79aa80b2eb2ef5ad2326` | Historical / removed from current main |
| 21 | `ملفات تحتاج إلى مراجعة/Eagle_IMPLEMENTATION_EVIDENCE_GATE_v1.1.zip` | 8848 | `756f6b0c298bab5cf0bebac73b5fbcd2c14c68c1` | Historical / removed from current main |
| 22 | `ملفات تحتاج إلى مراجعة/Eagle_IMPLEMENTATION_EVIDENCE_INTAKE_v1.0.zip` | 8480 | `e6ce829ac738294cdb4508fe4f01205a5a784a00` | Historical / removed from current main |
| 23 | `ملفات تحتاج إلى مراجعة/Eagle_MASTERIZATION_COMPARISON_REPORT_v1.0.md` | 5800 | `c8d3edff86d9693618b8e8d1f953992dc3fee38c` | Historical / removed from current main |
| 24 | `ملفات تحتاج إلى مراجعة/Eagle_MASTERIZATION_PACKAGE_v1.0.zip` | 16381 | `9100cc38cc4f12988b17bcea86a1551319216f75` | Historical / removed from current main |
| 25 | `ملفات تحتاج إلى مراجعة/Eagle_MASTER_PROJECT_REFERENCE.md` | 29244 | `98f87d81870ba919699f100833e6636409cef6f8` | Historical / removed from current main |
| 26 | `ملفات تحتاج إلى مراجعة/Eagle_MASTER_REFERENCE_INDEX.md` | 22600 | `5c0a2aa1ce819ba50dd1e80b4b97019b558dc6ea` | Historical / removed from current main |
| 27 | `ملفات تحتاج إلى مراجعة/Eagle_PRODUCTION_EXECUTION_BRIDGE_v2.8_2026-09-30.zip` | 13096 | `432199b91208f9d1f8a936a111e2995d19c32cae` | Historical / removed from current main |
| 28 | `ملفات تحتاج إلى مراجعة/Eagle_PrivateMesh_Baseline_Package_2026-09-29.zip` | 54164 | `3b763d8295281be778c11c0397a57296212a2e38` | Historical / removed from current main |
| 29 | `ملفات تحتاج إلى مراجعة/Eagle_PrivateMesh_Comprehensive_Baseline_2026-09-29.docx` | 41325 | `9120abedb0e98a6acb25939a7b1cf443260917ae` | Historical / removed from current main |
| 30 | `ملفات تحتاج إلى مراجعة/Eagle_PrivateMesh_Documentation_Pack_2026-09-28.zip` | 56910 | `ceb88219de705bb4b78a9aa34ee6aba28d930ccf` | Historical / removed from current main |
| 31 | `ملفات تحتاج إلى مراجعة/Eagle_PrivateMesh_Documentation_v1.3.zip` | 28660 | `490c57afe27e66b925b34cba3adeb289fb56d132` | Historical / removed from current main |
| 32 | `ملفات تحتاج إلى مراجعة/Eagle_PrivateMesh_Documentation_v1.4.zip` | 33951 | `554ca1c4b0b00cc35870c1666a07e26aa86b14f4` | Historical / removed from current main |
| 33 | `ملفات تحتاج إلى مراجعة/Eagle_PrivateMesh_Final_Documentation_v1.0.zip` | 15130 | `8fce7ba5d15c843bd17e9d7c7fb38cb18f2af6d0` | Historical / removed from current main |
| 34 | `ملفات تحتاج إلى مراجعة/Eagle_PrivateMesh_Final_Documentation_v1.1.zip` | 19779 | `1ed9ef6c54d513a16ccb4fdff5ceeff194339c57` | Historical / removed from current main |
| 35 | `ملفات تحتاج إلى مراجعة/Eagle_PrivateMesh_Implementation_Blueprint_v1.0.md` | 24193 | `22e837efb0bdf74a546af506db3368d536d99b23` | Historical / removed from current main |
| 36 | `ملفات تحتاج إلى مراجعة/Eagle_PrivateMesh_Project_Handbook.docx` | 38026 | `f33b5ae5b2139231488d1dc0ec3cb20c20380a80` | Historical / removed from current main |
| 37 | `ملفات تحتاج إلى مراجعة/Eagle_PrivateMesh_Unified_Consolidation_Register_v1.0.md` | 14514 | `23b467aa0aeb801086a7851ac455585acc057474` | Historical / removed from current main |
| 38 | `ملفات تحتاج إلى مراجعة/Eagle_REMAINING_COMPLETION_PACKAGE_v2.5_2026-09-30.zip` | 27151 | `1628d98713181fb1d3809d0a97d376ce62937ff3` | Historical / removed from current main |
| 39 | `ملفات تحتاج إلى مراجعة/Eagle_Team_Work_Packages_v1.1.zip` | 78892 | `467a43ded3ea0181e7059e67678f467fa1309377` | Historical / removed from current main |
| 40 | `ملفات تحتاج إلى مراجعة/PRIVATE_MESH_MASTER_PROJECT_REFERENCE.md` | 22230 | `b1ab4b31ef667577a7627ce00137684fd2e47887` | Historical / removed from current main |
| 41 | `ملفات تحتاج إلى مراجعة/PrivateMesh_Project_Documentation.zip` | 4395 | `bd0a5207d4de52f983879c403143af90e68764fe` | Historical / removed from current main |
| 42 | `ملفات تحتاج إلى مراجعة/PrivateMesh_Project_Documentation_v0.2_Repaired.zip` | 12229 | `1b1571ac4cb0dbeb1fe2ee25997ec59a60cbd237` | Historical / removed from current main |
| 43 | `ملفات تحتاج إلى مراجعة/بالضبط_5362697558946016807.docx` | 4541276 | `68b4653d5c4eb31118a484bbdee6bd2b44a74747` | Historical / removed from current main |
| 44 | `ملفات تحتاج إلى مراجعة/تابع_5461981517845702421.md` | 1123582 | `ae13a6e74fdc0a5a0ee0271b0ac4cd81af3fba6b` | Historical / removed from current main |

## Recommended classification workflow
1. Preserve the original Git object and provenance.
2. Calculate independent SHA-256 hashes after obtaining/downloaded copies.
3. Inspect archive contents without executing them.
4. Detect duplicates and superseded versions.
5. Classify as canonical, supporting evidence, obsolete, duplicate, or pending review.
6. Link each canonical artifact to the relevant project component and decision record.
7. Keep an immutable audit trail for every promotion, replacement, or deletion.

## Scope limitation
This register is based on repository history available through GitHub. ChatGPT attachments that were never committed to this repository are not directly visible from this GitHub history and therefore are not asserted here.