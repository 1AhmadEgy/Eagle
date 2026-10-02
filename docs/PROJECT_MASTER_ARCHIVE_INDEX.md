# Eagle — Project Master Archive Index

**Date:** 2026-10-02
**Repository:** 1AhmadEgy/Eagle
**Purpose:** Durable project-wide index intended to survive deletion of ChatGPT conversations.

## 1. Authority and evidence model
Git/GitHub is the durable project record. This index distinguishes repository evidence, conversation evidence, pending transfer, and proposed decisions. No missing conversation artifact is inferred or fabricated.

## 2. Existing durable registries
- docs/FILE_PROVENANCE_REGISTER.md — historical repository provenance; records 44 historical files.
- docs/03-architecture/REPOSITORY_HANDOFF_INDEX.md — continuation entry point.
- docs/03-architecture/CONVERSATION_HANDOFF_2026-10-02.md — architecture-planning continuity.
- docs/03-architecture/CANONICAL_PLANNING_BASELINE.md — proposed baseline.
- docs/03-architecture/MASTER_TASK_MAP.md — task map.
- docs/03-architecture/DEPENDENCY_GRAPH.md — proposed dependency graph.
- docs/03-architecture/DEFINITION_OF_DONE.md — proposed completion gate.
- docs/03-architecture/ADR_INDEX.md — ADR sequence and governance.
- docs/03-architecture/adr/ADR-0011-0014-WORKLIST.md — pending decision scopes.

## 3. Historical repository inventory
The existing provenance register records 44 historical files from Git history, including documentation, PDFs, DOCX files, ZIP packages, research artifacts, PrivateMesh packages, and Eagle release/engineering packages.

The repository history also contains a documented restoration of 44 historical uploaded project artifacts. The provenance register remains the authority for the exact historical list and Git blob identifiers.

## 4. Current conversation-source artifacts
1. Eagle_Master_File_Registry_Full_Documentation_v3.2.docx
   - SHA-256: 765404cc42211dc8a23470d0959e2b68177efb2003384290393d531196035288
   - Source declared by the document: Download.zip
   - Registry version: v3.2
2. Download.zip
   - SHA-256: ade73a06e74f856b8160e0789093625f195030e270ec59053ed63ffcc0fe0608
   - Size: 6,903,508 bytes
   - Direct entries: 12
   - Contains nested Eagle packages and registry/provenance material.

These current conversation attachments are recorded as pending durable transfer, not as already archived GitHub binaries.

## 5. Master File Registry v3.2 evidence
The supplied v3.2 document records:
- 41 top-level uploads
- 30 ZIP files
- 5 excess duplicate archives
- 1,302 file appearances across packages
- 191 duplicate-content groups
- 502 unique contents across packages
- v2.3 content plus duplicates: 711
- Inventory, provenance, classification and triage marked complete
- Analysis marked partial
- Reconciliation marked complete/partial
- Multiple conflicts and open issues remain
- OI-001 and OI-002 are P0 and OPEN
- OI-003 through OI-008 remain OPEN
- OI-009 through OI-016 remain untracked in v2.x according to the supplied registry
- Production status: PRODUCTION NOT CLEARED

The document explicitly says v3.2 is a documentation package, not a production release.

## 6. Canonicality rule
The supplied v3.2 registry states that v2.3.1 + overlays is the operational canonical reference, while preserving the v3.0 policy where documented. This is source-derived evidence, not a new approval by this index.

## 7. Architecture status
The repository planning baseline remains Proposed:
- ARCH-001 / ADR-0007: proposed
- ADR-0008: cryptographic protocol — proposed scope
- ADR-0009: key management — proposed scope
- ADR-0010: serialization — proposed scope
- ADR-0011 through ADR-0014: pending decision scopes

Do not treat a proposed ADR as an approved technology choice.

## 8. GitHub issue traceability
- ARCH-001 → #24
- ADR-0007 → #25
- ARCH-002 → #26
- ARCH-003 → #27
- ARCH-004 → #28
- ARCH-005 → #29
- ADR-0008 → #30
- ADR-0009 → #31
- ADR-0010 → #32

## 9. Security preservation rules
- Never execute unknown binaries or scripts from archived packages merely to inspect them.
- Preserve original bytes and provenance before normalization.
- Calculate independent SHA-256 for transferred artifacts.
- Keep duplicate originals; classify/fold them instead of silently deleting provenance.
- Do not promote an artifact to canonical status based on filename or version number alone.
- Keep private keys, credentials, tokens, and secrets out of Git.
- Record security-sensitive decisions in ADRs and link implementation evidence to issues.

## 10. Deletion continuity rule
Before deleting ChatGPT conversations:
1. Verify this index exists on main.
2. Verify the conversation-deletion gate exists on main.
3. Verify every available attachment has a provenance row.
4. Transfer any attachment marked PENDING TRANSFER.
5. Verify SHA-256 after transfer.
6. Record destination Git path and commit SHA.
7. Close or explicitly defer the corresponding archival issue.
8. Start new conversations using only the repository handoff index.

## 11. Known limitation
GitHub cannot prove the existence of a file that was only attached to a ChatGPT conversation and never committed. Older ChatGPT uploads may also become unavailable to the current file-access layer. Such artifacts must be re-uploaded if their bytes are required for exact archival.

## 12. Clean-conversation starting point
Open docs/03-architecture/REPOSITORY_HANDOFF_INDEX.md, then this index, the deletion gate, the provenance register, architecture records, active Issues/PRs, and current branch/commit evidence.
