# Eagle — Post-Release Security Monitoring Plan

## Activation rule

This plan is inactive until the Release Gate is fully PASS and a production artifact is authorized.

## Required controls

- Monitor security-relevant failures without logging message plaintext or key material.
- Track protocol/key-management provider advisories and exact dependency versions.
- Track P2P transport abuse indicators and repeated authentication failures.
- Track storage recovery/deletion failures.
- Track release provenance, SBOM, and artifact identity.
- Maintain vulnerability exceptions with explicit expiry.
- Re-run threat-model review after material architecture/security changes.
- Trigger rollback evaluation on confirmed security regression.

## Evidence retention

For each authorized release retain:

- artifact digest;
- source commit;
- CI workflow identity;
- provenance/attestation where configured;
- SBOM;
- test summary;
- security review record;
- release decision;
- rollback/recovery evidence.

## Privacy invariant

Monitoring must be sanitized and must not become a secondary message-content store.
