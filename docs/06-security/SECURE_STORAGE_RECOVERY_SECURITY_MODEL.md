# Eagle — Secure Storage & Recovery Security Model

**Scope:** local secret storage and recovery envelope only.

## Security boundary
- Local secrets are encrypted with AES-256-GCM.
- The long-lived local storage key is generated and retained by Android Keystore.
- StrongBox is preferred when the platform advertises StrongBox KeyStore support; the implementation falls back to Android Keystore when StrongBox is unavailable.
- Recovery exports are encrypted with a fresh random salt and nonce and authenticated with AES-GCM.
- Recovery key derivation uses PBKDF2-HMAC-SHA256 with 600,000 iterations.
- Recovery associated data can bind an envelope to account, device, trust epoch, or another caller-controlled security context.

## Persistence invariants
1. Plaintext values are never written to the secure-store file.
2. Secure-store ciphertext is authenticated; modification is rejected.
3. Writes use a temporary file followed by a rename after file-descriptor synchronization.
4. Storage file names are restricted to the application's files directory.
5. Backup and device-transfer rules currently exclude the application's root data set, preventing implicit transfer of storage that depends on an Android Keystore key.
6. Recovery is independent from local storage and does not itself restore historical message keys.

## Recovery invariants
1. A wrong recovery secret fails authentication.
2. Tampering fails authentication.
3. Changing associated data fails authentication.
4. Unsupported envelope versions are rejected.
5. Recovery input and output sizes are bounded.
6. Recovery secrets are accepted as character arrays so the caller can clear them after use.

## Rollback boundary
The current local file format provides authenticated integrity but does not claim trusted-monotonic rollback protection for a complete rollback of the application's private filesystem. Detecting rollback of the entire application data set requires a monotonic external trust anchor or platform primitive that is not yet selected by the project's key-management ADR.

Accordingly, storage rollback protection remains **PENDING** rather than PASS.

## Platform guidance
Android documents StrongBox as a hardware-backed security boundary available on supported devices and exposes FEATURE_STRONGBOX_KEYSTORE plus setIsStrongBoxBacked(true) for key generation. The platform also documents that StrongBox is slower and more resource constrained than the general Keystore path.

## Review status
- Implementation branch: security/secure-storage-recovery-v1
- Release status: draft / not production-approved
- Independent security review: pending
- End-to-end recovery integration: pending
- Trusted rollback anchor: pending
