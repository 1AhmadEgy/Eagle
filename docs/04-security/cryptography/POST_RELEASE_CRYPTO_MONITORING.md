# Eagle — Post-Release Cryptography Monitoring

This control becomes active only after the cryptography release gate G9 is passed.

## Continuous signals

- New vulnerabilities or advisories affecting the selected crypto provider.
- Protocol specification revisions and errata.
- Library releases, API changes, and provenance changes.
- NIST/FIPS updates and cryptographic deprecation guidance.
- Platform security changes affecting Keystore, Keychain, Secure Enclave, StrongBox or desktop protected stores.
- Newly discovered downgrade, identity, replay, rollback or state-machine failures.
- Cryptographic review findings and external researcher reports.

## Triggered re-entry

Any material change to protocol, dependency, key storage, recovery, serialization, platform capability, or threat assumptions re-enters the 20-stage lifecycle beginning at Inventory.

## Fail-safe response

When a critical cryptographic issue is confirmed:
1. stop new affected sessions where technically possible;
2. prevent silent downgrade;
3. rotate/revoke affected keys according to the protocol;
4. freeze the affected release channel;
5. produce a reproducible incident evidence package;
6. re-run the security and verification gates before re-enabling the feature.

No post-release monitoring event may silently change a cryptographic protocol or key lifecycle.
