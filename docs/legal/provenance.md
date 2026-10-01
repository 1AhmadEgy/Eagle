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

## Chronology

The documentation agent appends significant provenance milestones here rather than rewriting prior history.

### Initial continuous-documentation-agent deployment

- Status: CONFIRMED
- Purpose: establish automated, reviewable provenance maintenance.
- Source branch: `legal/continuous-documentation-agent`
- Legal owner identity: UNCONFIRMED

## Important limitation

Git history and GitHub records can provide useful evidence of project development chronology, but they are not represented here as a substitute for jurisdiction-specific legal registration or other formal evidence mechanisms.
## Machine-readable evidence

The continuous documentation workflow produces a deterministic evidence artifact for each trusted implementation-branch run:

- Schema: `docs/provenance/schema.json`
- Run artifact: `.ci/documentation/provenance.json`
- Retention: 30 days in the workflow artifact store
- Test Lab status is never inferred by this workflow; it remains `UNCONFIRMED` unless independently evidenced by the trusted CI/Test Lab run.

The artifact is evidence for chronology and review, not a legal ownership determination and not a substitute for formal registration or other jurisdiction-specific evidence.
