# Eagle — Platform Key Storage Contract

## Android
- Primary boundary: Android Keystore.
- Prefer hardware-backed protection where supported.
- StrongBox is an optional higher-assurance tier.
- Key Attestation requires certificate-chain and policy validation.
- Never claim all protocol keys are hardware-backed without device/API evidence.
- Test key invalidation, restore, and migration.

## Apple
- Primary boundary: Keychain.
- Secure Enclave only for supported algorithms and operations.
- Never claim Secure-Enclave residency without exact platform evidence.
- Test restore, migration, and invalidation.

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
