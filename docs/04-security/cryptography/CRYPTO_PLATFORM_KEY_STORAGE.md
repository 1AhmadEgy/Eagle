# Eagle — Platform Key Storage Contract

## Android

Primary boundary:
- Android Keystore.
- Hardware-backed protection when supported.
- StrongBox as an optional stronger isolation tier.
- Key Attestation only with full certificate-chain and policy validation.

Rules:
1. Never claim all protocol keys are hardware-backed.
2. Record actual key security level.
3. Treat StrongBox unavailability as a capability result.
4. Encrypt application protocol state at rest.
5. Test key invalidation and restore behavior.
6. Keep attestation separate from user identity authentication.

## Apple

Primary boundary:
- Keychain for persistent key and secret storage.
- Secure Enclave only for supported key types and operations.

Rules:
1. Never claim Secure-Enclave residency unless the exact key algorithm is supported.
2. Protect protocol state at rest.
3. Use Secure Enclave for compatible device-protection/wrapping/authentication roles where justified.
4. Test restore, migration, and key invalidation.

## Desktop

Use native protected credential/key facilities for each supported OS.

Fallback:
- encrypted application state;
- OS/device-bound protection when available;
- no plaintext persistence;
- explicit lower-assurance classification when hardware backing is absent.

## Crypto-core boundary

The crypto core consumes abstract key handles/contracts and does not directly depend on UI, mesh, database internals, or platform APIs.

UI → Keystore is forbidden.  
Mesh → plaintext is forbidden.  
Storage → private keys is forbidden.  
Transport → crypto internals is forbidden.

## Assurance labels

HW_BACKED — hardware-backed protection verified.  
OS_PROTECTED — OS protected but hardware backing not established.  
APP_ENCRYPTED — application-level at-rest encryption only.  
UNTRUSTED — protection requirement not met; fail closed where stronger assurance is required.
