# Eagle Corpus Conflict Register — current snapshot 2026-10-06

| ID | Conflict | Evidence | Disposition |
|---|---|---|---|
| CR-001 | P2P-only current constraint vs historical server-mediated transport proposal | current project constraint + historical ADR package | Historical proposal rejected as current authority; replacement P2P ADR required |
| CR-002 | Platform architecture intent vs current main implementation | PLATFORMS.md vs main tree | Keep platform strategy as scope; implementation gap remains open |
| CR-003 | Proposed planning baseline vs execution branches | CANONICAL_PLANNING_BASELINE.md + branch evidence | Planning remains Proposed; branch evidence is candidate only |
| CR-004 | Historical archives vs current repository state | archive/chatgpt-historical + current main | Preserve history; do not import without provenance/security review |
| CR-005 | Registered local artifacts vs GitHub identity | session artifact manifest | Local SHA proves observed bytes only; Git blob identity must be established separately |
| CR-006 | This 2026-10-05 canonicalization record contains a stale Main reference (`6c46bf...`) while current Git main is `46aa86b...` | current Git ref `refs/heads/main` | Treat the 2026-10-05 line as historical/stale; current snapshot in 2026-10-06 audit record is authoritative for current state |
| CR-007 | Project policy says main must not receive direct pushes, but GitHub currently reports `protected=false` and no rulesets | GitHub branch metadata + rulesets API | Policy intent remains, but technical enforcement is absent; P0 release blocker |
| CR-008 | Security verifier requires full 40-character action SHAs, while current `testlab.yml` on main uses `actions/checkout@v5`, `actions/setup-java@v5`, `gradle/actions/setup-gradle@v5` | main workflow + verifier + main CI run `37290062138` | PR #45 is the verified remediation candidate; no main promotion until merged and reverified |
| CR-009 | Current implementation/security PRs are distributed across many bases and branches; some diverge from current main | branch inventory + compare results | Do not canonicalize by branch age; reconcile/rebase/verify each line before integration |
| CR-010 | Current platform document treats Web as deferred, while PR #88 proposes an applications-only scope excluding Web | PLATFORMS.md + PR #88 | Remains unresolved until an accepted scope decision is merged; PR #88 is candidate only |

## Rule
No conflict is silently resolved by deletion or assumption. Each conflict must end in an explicit ADR, supersession record, or verified evidence disposition.
