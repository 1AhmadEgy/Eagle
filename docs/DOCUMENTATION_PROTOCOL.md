# Eagle — Documentation & Change Control Protocol

## Objective

Move durable project knowledge from transient conversations into GitHub while preserving provenance, history, decisions, tests, security evidence, and release state.

## Rules

1. GitHub is canonical after migration.
2. Never silently overwrite historical versions.
3. Every substantive change must be traceable to a requirement/reason, affected artifacts, implementation, tests, security impact, and rollback/recovery considerations.
4. Secrets never enter documentation or source control.
5. Do not claim verified, secure, complete, or canonical without evidence.

## Change record

- Change ID
- Date
- Requirement/source
- Affected paths
- Previous canonical version
- New canonical version
- Rationale
- Tests
- Security review
- Reviewer
- Commit
- PR
- Rollback plan

## Release Gate

A release is eligible only when registry updates are complete, canonical artifacts are identified, required tests pass, security checks are recorded, unresolved blockers are listed, rollback/recovery is documented, and the final commit/PR is traceable.
