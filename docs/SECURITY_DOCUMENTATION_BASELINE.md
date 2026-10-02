# Eagle — Security Documentation Baseline

## Security-first requirements

Documentation must not leak project secrets or sensitive operational data.

### Never commit

- API keys
- access tokens
- passwords
- private SSH/GPG keys
- session cookies
- cloud credentials
- database credentials
- recovery codes
- unredacted sensitive personal data

### Required controls

- secret scanning before release;
- dependency vulnerability review;
- least-privilege repository access;
- branch protection/review controls where available;
- reproducible verification of release artifacts;
- documented incident and rollback procedure;
- provenance for imported external artifacts.

### Security evidence record

For each security review record scope, date, tool/check, version, findings, severity, remediation, residual risk, reviewer, and evidence location.

Security status must be based on documented checks and evidence, not on absence of a reported problem.
