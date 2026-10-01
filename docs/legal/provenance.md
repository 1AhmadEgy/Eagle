# Eagle Provenance Record

This file is maintained as a human-reviewable provenance index.

## Current state

- Project: Eagle
- Ownership intent: PRIVATE / ALL RIGHTS RESERVED
- Copyright holder: UNCONFIRMED
- Open-source license grant: NONE
- Last documentation-agent review: UNCONFIRMED

## Evidence model

Each documented change should be traceable to:

1. Git commit SHA
2. branch/ref
3. timestamp
4. author/committer metadata
5. pull request, when applicable
6. CI/Test Lab evidence, when available
7. affected paths
8. security impact and remaining limitations for material security changes

## Chronology

The documentation system records significant provenance milestones without rewriting prior history.

### Initial continuous-documentation-agent deployment

- Status: CONFIRMED
- Purpose: establish automated, reviewable provenance maintenance.
- Source branch: `legal/continuous-documentation-agent`
- Legal owner identity: UNCONFIRMED

### CI/Test Lab and guarded AI repair

- Status: CONFIRMED
- Pull request: #3
- Scope: repository verification, Test Lab status model, guarded OpenCode/DeepSeek repair workflow, specialized agents, and repair safety boundaries.
- Merge status: human review required; no automatic merge.

### Continuous provenance documentation

- Status: CONFIRMED
- Pull request: #4
- Scope: read-only documentation agent, exact-SHA Test Lab linkage, machine-readable provenance schema, and offline evidence validation.

### GitHub Actions supply-chain hardening

- Status: IN_REVIEW
- Pull request: #5
- Branch: `security/actions-supply-chain-hardening`
- Scope: immutable full-SHA action references, gitleaks v3 migration, and replacement of mutable OpenCode `@latest`.
- Verification boundary: changes are reviewable through the draft PR; no automatic merge.

### Continuous development ledger

- Status: IN_REVIEW
- Branch: `docs/continuous-development-audit`
- Scope: durable recording of repairs, security changes, research sources, evidence, and remaining gaps.
- New records:
  - `docs/development/continuous-change-ledger.md`
  - `docs/legal/security-change-register.md`

## Machine-readable evidence

The continuous documentation workflow produces a deterministic evidence artifact for each trusted implementation-branch run:

- Schema: `docs/provenance/schema.json`
- Run artifact: `.ci/documentation/provenance.json`
- Retention: 30 days in the workflow artifact store
- Test Lab status is never inferred by this workflow; it remains `UNCONFIRMED` unless independently evidenced by the trusted CI/Test Lab run.

The artifact is evidence for chronology and review, not a legal ownership determination and not a substitute for formal registration or other jurisdiction-specific evidence.

## Important limitations

- Git history and GitHub records can provide useful evidence of project development chronology, but they are not represented here as a substitute for jurisdiction-specific legal registration or other formal evidence mechanisms.
- Repository visibility is currently PUBLIC while the project documentation specifies PRIVATE / ALL RIGHTS RESERVED. This mismatch is explicitly recorded and requires owner-controlled repository settings action; it is not silently changed by documentation automation.
- Test Lab categories remain PENDING until category-specific evidence exists.
