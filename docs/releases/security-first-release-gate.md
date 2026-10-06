# The Eagle — Security-First Release Gate

- Status: CANONICAL release policy
- Security rule: security failure blocks release

## Gate 0 — Documentation and provenance

- [ ] Canonical architecture is identified.
- [ ] Historical conflicting documents are marked historical/superseded.
- [ ] Requirements have stable IDs.
- [ ] Source/provenance is recorded.
- [ ] Unknowns are explicitly marked.

## Gate 1 — Rust Security Core

- [ ] identity tests pass
- [ ] key lifecycle tests pass
- [ ] session tests pass
- [ ] crypto tests pass
- [ ] verification tests pass
- [ ] anti-replay tests pass
- [ ] TTL/expiry tests pass
- [ ] corrupted state fails closed
- [ ] fuzz/property tests pass
- [ ] no panic crosses FFI

## Gate 2 — Android Security Boundary

- [ ] minSdk=31 enforced
- [ ] Keystore protection verified
- [ ] app starts locked where required
- [ ] lock revokes sensitive access
- [ ] process-death behavior verified
- [ ] no silent identity recreation
- [ ] no sensitive state in intents/navigation/saved state
- [ ] no uncontrolled external output

## Gate 3 — Storage

- [ ] Room contains metadata/indexes only
- [ ] object storage is ciphertext-only
- [ ] private keys absent from database/preferences/logs
- [ ] backup exclusion verified
- [ ] migrations tested
- [ ] expiry/key destruction verified

## Gate 4 — P2P/Data Plane

- [ ] application data path is P2P
- [ ] direct P2P tested
- [ ] permitted TURN fallback tested
- [ ] signaling scope verified
- [ ] no server content relay
- [ ] no server Outbox
- [ ] no server application-content store
- [ ] no hidden cloud messaging dependency

## Gate 5 — Media and Calls

- [ ] media encrypted before persistence
- [ ] bounded/chunked processing verified
- [ ] malformed/oversized input rejected
- [ ] internal playback verified
- [ ] call authentication verified
- [ ] Core-Telecom POC passed where required
- [ ] WebRTC POC passed
- [ ] TURN fallback tested
- [ ] no recording/screen sharing in current scope

## Gate 6 — Performance

- [ ] baseline benchmark stored
- [ ] startup within gate
- [ ] frame timing within gate
- [ ] memory within gate
- [ ] APK/AAB size within gate
- [ ] battery behavior reviewed
- [ ] no new security-sensitive Main Thread work
- [ ] benchmark artifacts contain no sensitive data

## Gate 7 — Supply Chain

- [ ] no forbidden runtime dependency
- [ ] dependency versions recorded
- [ ] license review passed
- [ ] vulnerability scan passed
- [ ] SBOM generated
- [ ] secret scan passed
- [ ] release AAB inspected
- [ ] release signing controls verified

## Gate 8 — Security Review

- [ ] threat model updated
- [ ] MASTG/MASVS review completed as applicable
- [ ] penetration/security assessment completed as required
- [ ] findings triaged
- [ ] critical/high findings closed or formally dispositioned
- [ ] regression tests added for closed findings

## Gate 9 — Release Provenance

Release record contains:

- version code/name
- source commit
- Rust Core version
- contract/protocol version
- database schema version
- build provenance
- checksums
- dependency/SBOM record
- benchmark report
- security sign-off
- known issues
- rollback plan

## Final decision

```text
Any security blocker
    → NO RELEASE

Any unverified critical requirement
    → NO RELEASE

Performance regression
    → investigate + evidence + gate decision

All mandatory gates PASS
    → eligible for staged release
```

Eligibility for release is not proof of perfect security; it means the defined project gates have been satisfied with evidence.
