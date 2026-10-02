# Eagle Definition of Done

**Status:** Proposed planning standard  
**Applies to:** architecture, implementation, test, security and documentation work

A task is not complete merely because code or documents exist. Completion requires evidence appropriate to the task.

## Standard checklist

- [ ] Implementation / documentation changes complete
- [ ] Unit tests where applicable
- [ ] Negative / failure-path tests where applicable
- [ ] Contract tests where interfaces cross module boundaries
- [ ] Integration tests where behavior crosses modules
- [ ] Security review where security impact exists
- [ ] Threat-model impact review where trust boundaries or secrets change
- [ ] Static analysis
- [ ] Secrets scan
- [ ] Dependency / supply-chain check where applicable
- [ ] Architecture check
- [ ] Documentation updated
- [ ] Code / review evidence recorded

## Mandatory dual review

Two independent reviewers are required before merge for changes classified as:

- identity
- crypto
- key-management
- mesh-security
- other explicitly security-sensitive work

## Evidence rule

Unchecked boxes are not implied to be complete.

Each completed item should point to observable evidence: test output, CI run, review, document change, or other repository artifact.
