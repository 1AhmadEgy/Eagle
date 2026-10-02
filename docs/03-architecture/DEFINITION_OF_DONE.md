# Eagle Definition of Done

**Status:** Proposed planning standard

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

## Dual review
Two independent reviewers are required before merge for identity, crypto, key-management, mesh-security, and other explicitly security-sensitive changes.

## Evidence rule
Completion is demonstrated by observable repository evidence; unchecked items are not implied complete.