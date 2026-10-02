# Eagle — Conversation-to-Repository Protocol

## Objective
Make GitHub the authoritative project memory and make ChatGPT conversations disposable.

## Rules
1. Repository first: every material decision, artifact, correction, test result, audit finding, or release statement becomes a repository artifact before being treated as durable.
2. Evidence before claims: use commit SHA, file path, SHA-256, issue/PR number, test result, and review evidence.
3. Provenance before normalization: preserve originals before renaming, extraction, merging, correction, or deduplication.
4. Canonicality is explicit: FINAL/LATEST/version names do not establish authority by themselves.
5. Duplicate preservation: duplicates and superseded artifacts remain available as provenance evidence.
6. Security: never commit secrets, private keys, credentials, access tokens, or unredacted sensitive data.
7. Architecture: ADR = decision; Issue = implementation/verification/migration/follow-up; Proposed ADR is not approved.
8. Conversation deletion: delete only after durable outputs are transferred and independently verified.
9. New clean conversations: start from the repository handoff index, master archive index, active Issues/PRs, and current branch/commit evidence.
10. No silent reconciliation: preserve conflicts, record evidence, and require explicit decisions.

## Continuity principle
**Chat is a working interface. GitHub is the durable project record.**
