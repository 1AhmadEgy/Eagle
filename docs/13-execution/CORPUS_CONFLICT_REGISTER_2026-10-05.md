# Eagle Corpus Conflict Register — 2026-10-05

| ID | Conflict | Evidence | Disposition |
|---|---|---|---|
| CR-001 | P2P-only current constraint vs historical server-mediated transport proposal | current project constraint + historical ADR package | Historical proposal rejected as current authority; replacement P2P ADR required |
| CR-002 | Platform architecture intent vs current main implementation | PLATFORMS.md vs main tree | Keep platform strategy as scope; implementation gap remains open |
| CR-003 | Proposed planning baseline vs execution branches | CANONICAL_PLANNING_BASELINE.md + branch evidence | Planning remains Proposed; branch evidence is candidate only |
| CR-004 | Historical archives vs current repository state | archive/chatgpt-historical + current main | Preserve history; do not import without provenance/security review |
| CR-005 | Registered local artifacts vs GitHub identity | session artifact manifest | Local SHA proves observed bytes only; Git blob identity must be established separately |

## Rule
No conflict is silently resolved by deletion or assumption. Each conflict must end in an explicit ADR, supersession record, or verified evidence disposition.
