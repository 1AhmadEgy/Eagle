# Eagle — Chat Migration and Archive Procedure

## Goal

Delete obsolete conversations only after the durable project record has been migrated to GitHub.

## Required sequence

1. Inventory all project conversations and participants.
2. Identify all uploaded files and attachments.
3. Record provenance for each artifact.
4. Capture durable decisions, requirements, architecture, implementation notes, tests, security findings, and approvals.
5. Hash important binary/source artifacts when practical.
6. Compare duplicates and versions.
7. Select canonical versions explicitly.
8. Commit documentation and artifacts to GitHub.
9. Verify repository contents from a clean read.
10. Record unresolved gaps.
11. Only then delete conversations.

## Limitation

No tool should certify that every deleted conversation or attachment was migrated unless those materials were actually accessible and individually accounted for.

## New conversation structure

- 00 — Project Control & Master Registry
- 01 — Architecture
- 02 — Implementation
- 03 — Testing & QA
- 04 — Security
- 05 — Releases
- 06 — Research / Decisions

New conversations should point to GitHub paths, issues, and PRs instead of becoming a second undocumented source of truth.

## Deletion gate

Do not delete historical conversations while files, decisions, canonical versions, security evidence, test evidence, implementation changes, or provenance gaps remain unresolved.
