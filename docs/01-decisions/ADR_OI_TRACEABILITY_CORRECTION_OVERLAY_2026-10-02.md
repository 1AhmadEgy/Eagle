# Eagle — ADR/OI Traceability Correction Overlay

- **Date:** 2026-10-02
- **Repository baseline:** `main` at branch creation
- **Status:** Documentation correction proposal; not an architecture approval
- **Scope:** Reconcile ADR identifiers and preserve open-decision state without silently selecting technical options.

## 1. Governing rules

1. Historical ADR packs remain immutable evidence; do not overwrite or delete them during reconciliation.
2. An ADR is not approved merely because an option is described as preferred, provisionally settled, or recommended.
3. An OI remains open until its decision authority records the selected option, rationale, security/privacy impact, implementation owner, evidence, reviewers, approval date, and affected baseline.
4. No protocol, cryptographic, identity, storage, or transport implementation may treat this overlay as a substitute for an approved ADR.
5. Changes to the canonical baseline require an explicit owner decision and a traceable change record.

## 2. Identifier reconciliation

The repository contains historical ADR packs whose numbering and topic mappings are not fully aligned. In particular, the ADR Decision Pack v1.0 maps ADR-001..008 to OI-001..008, while ADR Decision Pack v1.2 uses a different topic ordering in its summary. Therefore, numeric ADR identifiers alone are not a safe cross-document join key.

Until an owner-approved crosswalk is recorded, use the related OI identifier as the stable reconciliation key:

| Stable key | Topic | v1.0 mapping | Reconciliation state |
|---|---|---|---|
| OI-001 | Exact Signal implementation and version | ADR-001 | Open; confirm against v1.2 and canonical baseline |
| OI-002 | PQXDH profile and integration | ADR-002 | Open; confirm against v1.2 and canonical baseline |
| OI-003 | V1 transport and offline behavior | ADR-003 | Open; confirm against v1.2 and canonical baseline |
| OI-004 | Server ciphertext retention | ADR-004 | Open; confirm against v1.2 and canonical baseline |
| OI-005 | Identity and device-trust state machine | ADR-005 | Open; confirm against v1.2 and canonical baseline |
| OI-006 | Device pairing and linking | ADR-006 | Open; confirm against v1.2 and canonical baseline |
| OI-007 | Account recovery versus data recovery | ADR-007 | Open; confirm against v1.2 and canonical baseline |
| OI-008 | Deletion guarantees and verification | ADR-008 | Open; confirm against v1.2 and canonical baseline |
| OI-009..016 | Additional open issues in the project register | No mapping asserted here | Open; map from the authoritative OI register |

This table is an interim traceability aid, not a claim that the v1.0 numbering is canonical or that any decision is approved. The v1.2 pack and canonical architecture must be compared line-by-line before the crosswalk is finalized.

## 3. Required ADR record

Each decision record must contain all of the following before approval:

- Stable OI key and ADR identifier
- Decision statement and bounded scope
- Context and alternatives considered
- Evidence and source links
- Security and privacy impact, including threat-model changes
- Compatibility, migration, and rollback implications
- Implementation owner and target milestone
- Required tests and measurable acceptance criteria
- Named reviewers and authorized approver
- Approval date and resulting baseline/version
- Supersedes/superseded-by relationship

Missing fields mean the decision remains **Open / Incomplete**.

## 4. Evidence and closure contract

Use the following lifecycle for every OI:

`Open → Evidence collected → Options assessed → ADR approved → Implemented → Verified → Closed`

A state transition must link to durable evidence. Planning documents do not prove implementation; a successful build does not prove security; and an internal test does not constitute independent verification.

Minimum closure evidence:

| Stage | Required evidence |
|---|---|
| Evidence collected | Source path, revision/hash where available, and relevant requirement |
| Options assessed | Trade-off record and threat/privacy analysis |
| ADR approved | Complete decision record and authorized approval |
| Implemented | Commit/PR references and implementation notes |
| Verified | Reproducible test results tied to the relevant commit |
| Closed | Reviewer confirmation, residual risks, and updated registers |

## 5. Release-gate protection

This overlay does not change any existing gate status. Release remains blocked until the authoritative readiness register demonstrates that all mandatory implementation, test, security-review, independent-verification, operational, and approval criteria have passed. Any exception must be explicit, risk-accepted by the authorized owner, time-bounded, and recorded; security-critical failures cannot be silently waived.

## 6. Change log

| Date | Change | Status |
|---|---|---|
| 2026-10-02 | Added interim OI-keyed ADR reconciliation and evidence-based lifecycle rules | Proposed; pending review |

## 7. Review checklist

- [ ] Compare ADR Decision Pack v1.0, v1.2, current ADR directory, and canonical architecture.
- [ ] Confirm authoritative OI-001..016 titles and exact source locations.
- [ ] Approve a single ADR identifier crosswalk.
- [ ] Assign decision owners and authorized reviewers.
- [ ] Link each approved ADR to requirements, specifications, tests, and evidence.
- [ ] Update readiness registers only after evidence is reviewed.
