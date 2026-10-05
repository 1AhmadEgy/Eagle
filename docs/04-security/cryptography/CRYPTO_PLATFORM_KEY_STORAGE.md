# Eagle — Platform Key Storage Contract

## Android

- Primary boundary: Android Keystore.
- Prefer hardware-backed protection where supported and verified.
- StrongBox is a higher-assurance tier, not a universal algorithm target; availability and algorithm support must be checked at runtime.
- For Android 10+ evidence, record the key security level from KeyInfo rather than assuming hardware backing.
- Key Attestation requires certificate-chain validation and an explicit policy decision.
- Never claim all protocol keys are hardware-backed without device/API evidence.
- Test invalidation, backup/restore, migration, StrongBox absence, StrongBox algorithm incompatibility, and downgrade/fallback behavior.

Android documents distinct software, trusted-environment, and StrongBox security levels. StrongBox provides stronger isolation but has tighter algorithm/resource constraints. citeturn668861search0turn668861search4

## Apple

- Primary boundary: Keychain.
- Secure Enclave only for supported algorithms and operations.
- Treat Secure Enclave as a hardware execution/custody boundary, not a generic PQ key store.
- Never claim Secure-Enclave residency without exact platform evidence.
- Test restore, migration, invalidation, device binding, and supported-algorithm failures.

Apple documents that Secure Enclave-protected private-key material is not handled in plaintext by the application and that the documented Secure Enclave key flow is restricted to supported algorithms, including P-256. citeturn668861search5

## Desktop

Use native protected credential/key facilities for each supported OS.
Fallback to encrypted application state is lower assurance and must be labeled.

## Forbidden crossings

UI -> private keys: forbidden.
Mesh -> plaintext: forbidden.
Storage -> identity private keys: forbidden.
Transport -> protocol secrets: forbidden.

## Assurance

HW_BACKED = hardware-backed protection verified.
OS_PROTECTED = OS protection verified; hardware backing not established.
APP_ENCRYPTED = application encryption only.
UNTRUSTED = required protection is not met; fail closed where required.

---
