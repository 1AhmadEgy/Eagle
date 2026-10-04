# Eagle — Identity & Trust Specification

**Status:** Draft for ADR-005 / ADR-006 / ADR-007 review  
**Branch:** `execution/identity-trust-clean-v1`  
**Baseline:** `main` @ `6c46bf53fce06ee7f7b5b2bf35c720ab5bcb7dee`  
**Owner:** Identity & Trust Engineering  
**Security posture:** fail-closed / local-first / P2P-only project requirement

> This document is an executable security baseline, not an approval of the open ADRs. It freezes interfaces, invariants, state semantics, and tests that downstream implementation can target without prematurely choosing a cryptographic library or key hierarchy.

---

## 1. Scope

This specification defines:

- logical account identity;
- per-device identity;
- identity-to-device membership binding;
- device trust states and transitions;
- P2P pairing and device linking;
- local authorization semantics;
- revocation and trust epochs;
- identity/key-change handling;
- account recovery boundary;
- platform-assurance signals;
- privacy requirements;
- security invariants and test obligations.

Out of scope:

- the final Signal/PQXDH profile;
- concrete signature/key algorithms;
- final key hierarchy and storage format;
- message-session ratchet state;
- transport selection;
- server retention or relay policy.

Those remain governed by the corresponding ADR/specification.

---

## 2. Baseline constraints

### 2.1 Existing project evidence

Corpus reconciliation establishes three different evidence classes:

- **Historical project corpus:** the archived `ADR_Decision_Pack.md` contains OI-005/006/007 and labels the corresponding ADRs as **Proposed / Open**.
- **Current canonical ADR set:** `docs/03-architecture/ADR_INDEX.md` currently exposes ADR-0007 through ADR-0014 as the active planning sequence; ADR-0008/0009/0010 are explicitly **Proposed — Decision Pending Review**.
- **Current platform baseline:** `docs/03-architecture/PLATFORMS.md` makes the Rust Security Core the cross-platform security authority and prohibits duplicating security-critical primitives in platform/UI code.

Therefore this document treats the historical ADR-005/006/007 material as **design input**, not as an already-approved current ADR state.

### 2.2 Project-level P2P constraint

For this working baseline, **P2P-only is treated as a project invariant**.

Therefore:

1. Trust establishment must not depend on a central authentication server.
2. Pairing must be executable directly between participating devices.
3. A server, if later introduced for rendezvous or delivery, must not become the authority that creates identity trust.
4. Identity and trust verification must remain valid when no service is reachable.
5. Server-mediated device attestation is an optional assurance signal only; it is not the identity root.

This creates a deliberate architecture review item against any historical document that assumes server-mediated account/device authority.

---

## 3. Identity model

The model is hierarchical in meaning, but not in secret-copying:

```text
Account Identity
      │
      ├── Device A Identity
      ├── Device B Identity
      └── Device C Identity
              │
              └── Sessions / ephemeral protocol state
```

### 3.1 Account identity

An **Account Identity** is the stable logical identity of the user-controlled Eagle account.

Required properties:

- self-certifying;
- independent of username, phone number, email, OS account, device serial, advertising ID, or IP address;
- stable across device replacement when account authority is preserved;
- represented by public information only outside secure key-management boundaries.

Canonical identifier:

```text
AccountID = H(canonical(AccountIdentityPublicKey))
```

`H` and canonical encoding are defined by the Key Management / Serialization decisions. This document does not select the final primitive.

### 3.2 Device identity

Each application installation that becomes a trusted Eagle device generates a distinct **Device Identity Key** under the approved key-management interface.

Canonical identifier:

```text
DeviceID = H(canonical(DeviceIdentityPublicKey))
```

The device identity:

- is application-generated;
- is never derived from hardware identifiers;
- must not be silently reused across unrelated installations;
- is not the same thing as a session key;
- remains stable for the intended lifetime of the device membership.

### 3.3 Membership binding

A device becomes a member of an account through an authenticated **Account Membership Statement (AMS)**.

Conceptual fields:

```text
AMS
├── protocol_version
├── account_id
├── device_id
├── device_identity_public_key
├── issued_at
├── not_before
├── not_after? 
├── trust_epoch
├── capabilities
└── account_authority_proof
```

The exact authority mechanism (single authority, delegated authority, or threshold construction) is deliberately delegated to Key Management ADRs.

No implementation may accept a device as trusted solely because it presents a matching AccountID.

---

## 4. Trust semantics

Trust is not a boolean.

Minimum state model:

```text
                    ┌──────────────┐
                    │ PROVISIONING │
                    └──────┬───────┘
                           │ successful local setup
                           ▼
                    ┌──────────────┐
                    │   PENDING    │
                    └──────┬───────┘
                           │ explicit authenticated approval
                           ▼
                    ┌──────────────┐
                    │   TRUSTED    │
                    └───┬────┬─────┘
                        │    │
              temporary │    │ administrative/security action
                        │    ▼
                        │ ┌──────────────┐
                        └►│  SUSPENDED   │
                          └──────┬───────┘
                                 │ re-approval / recovery policy
                                 ▼
                              TRUSTED

TRUSTED ─────────► REVOKED
TRUSTED ─────────► REPLACED

PENDING / SUSPENDED / REVOKED / REPLACED
must not silently escalate to TRUSTED.
```

### 4.1 Meaning of states

**PROVISIONING**  
Local setup exists but no account trust has been established.

**PENDING**  
The device has presented a valid identity and pairing transcript but has not received the required explicit trust authorization.

**TRUSTED**  
The device has a valid membership binding, has passed all applicable guards, and may receive the capabilities explicitly granted to it.

**SUSPENDED**  
Authorization is temporarily disabled without destroying the identity record.

**REVOKED**  
Future authorization is denied. Historical protocol material is not resurrected by this state.

**REPLACED**  
The device identity is retired because another device or identity has superseded it. Replacement is terminal for the retired membership.

---

## 5. Trust levels

The core security model uses two separate axes.

### 5.1 Membership trust

```text
UNKNOWN
PENDING
TRUSTED
SUSPENDED
REVOKED
REPLACED
```

### 5.2 Assurance metadata

Assurance is evidence about the environment, not proof of user identity:

```text
SOFTWARE
HARDWARE_BACKED
PLATFORM_ATTESTED
```

An implementation must never convert an assurance label directly into membership trust without satisfying the account authorization rules.

Example:

```text
PLATFORM_ATTESTED + UNKNOWN = UNKNOWN
PLATFORM_ATTESTED + PENDING = PENDING
PLATFORM_ATTESTED + explicit approval = TRUSTED
```

---

## 6. P2P pairing ceremony

### 6.1 Pairing objective

Pairing establishes:

1. a direct authenticated channel between devices;
2. explicit user intent;
3. binding of the new device identity to the intended account;
4. replay resistance;
5. transcript integrity;
6. a durable trust decision.

### 6.2 Required flow

```text
Existing trusted device
        │
        │ create pairing session
        ▼
Ephemeral pairing context
        │
        │ QR / short authenticated code
        ▼
New device
        │
        │ generate device identity
        │
        │ exchange ephemeral material
        ▼
Mutual transcript verification
        │
        │ explicit approval on trusted device
        ▼
Account Membership Statement
        │
        ▼
New device = TRUSTED
```

### 6.3 QR / code content

QR/code data may contain:

- pairing session identifier;
- protocol version;
- ephemeral public material;
- nonce/challenge;
- rendezvous hints when strictly required;
- integrity/authentication metadata.

QR/code data must **not** contain:

- account private keys;
- device private keys;
- session secrets;
- recovery secrets;
- exportable long-term secret material.

### 6.4 Anti-phishing requirement

A pairing ceremony must provide a human-verifiable binding between both endpoints.

At minimum:

- both endpoints show the same short authentication value or equivalent authenticated comparison;
- approval is explicit, not implied by opening a QR scanner;
- expiry is short;
- a pairing token is single-use;
- cancelled/expired pairing contexts cannot later be resumed.

### 6.5 Approval verification boundary

The Security Core MUST require an explicit approval-verifier interface before promoting a PENDING device to TRUSTED.

That verifier is the integration seam for the final pairing protocol and must validate the authenticated approval evidence, including endpoint binding and user authorization according to the approved protocol.

A permissive test double may exist only inside tests. Production integration MUST NOT substitute an unconditional verifier.

### 6.6 Anti-replay requirement

A completed pairing transcript must not be reusable.

The protocol layer must bind:

```text
account identity
+ both device identities
+ ephemeral session values
+ protocol version
+ pairing nonce
+ transcript hash
```

into the authenticated result.

The final cryptographic construction remains defined by the Protocol/Crypto specification.

---

## 7. Authorization rules

The Security Kernel is authoritative.

A requested action is allowed only when all applicable guards pass.

Conceptual policy:

```text
ALLOW(action, device_state, evidence, epoch, policy)
```

Examples:

| Action | Required state |
|---|---|
| Start local app session | locally enrolled device + valid local unlock policy |
| Accept inbound identity binding | authenticated protocol + known account context |
| Promote PENDING → TRUSTED | explicit authorized approval |
| Start new protected session | TRUSTED + not revoked + current trust epoch |
| Send sensitive account-management command | TRUSTED + fresh authorization |
| Use revoked device for new authorization | DENY |
| Reuse expired pairing token | DENY |
| Recover historical message keys from account login alone | DENY |

Unknown or inconsistent states fail closed.

---

## 8. Revocation

### 8.1 Revocation model

Revocation is represented as a signed state transition plus a monotonic **trust epoch**.

```text
Account Trust Epoch: 41

Device A → TRUSTED @ epoch 41
Device B → TRUSTED @ epoch 41

Device B revoked
        ↓
Account Trust Epoch: 42

Device B
  → REVOKED
  → reject future authorization
  → reject membership-based session creation
```

### 8.2 Offline semantics

In a P2P system, no device can guarantee that every peer learns about a revocation while disconnected.

Therefore:

- the revoking device enforces revocation immediately;
- every authenticated peer must apply the newest verified trust epoch it has received;
- stale epoch information must not be used to silently restore revoked authorization;
- reconnection must trigger trust-state reconciliation before new privileged operations.

This is a security boundary, not an availability optimization.

### 8.3 Contact identity changes

A remote identity-key change is not an ordinary profile update.

Required behavior:

```text
known identity
    ↓ identity change detected
QUARANTINE / REVERIFICATION
    ↓
explicit user verification
    ↓
new trusted identity binding
```

There must be no silent key replacement for a previously verified contact.

Leaving QUARANTINED requires an explicit reverification-verifier boundary. A caller may not restore VERIFIED solely by presenting a replacement identity object.

---

## 9. Device compromise

The model distinguishes:

1. **credential exposure** — an identity key or authorization secret may be exposed;
2. **application compromise** — the application process can be influenced;
3. **device compromise** — the operating system or execution environment is not trustworthy;
4. **loss/theft** — the physical device is unavailable to the owner.

A platform assurance signal can inform risk handling, but it must not be mistaken for proof that a device is uncompromised.

Containment requirement:

- compromise of one device must not automatically grant trust to a new device;
- adding a new device requires a fresh authenticated authorization path;
- revoking a compromised device must invalidate future authorization for that membership;
- recovery must not silently restore revoked device authority.

---

## 10. Recovery boundary

### 10.1 Account recovery

Account recovery means restoring the ability to establish or prove the account's current authority under a separately specified recovery policy.

### 10.2 Data recovery

Data recovery means restoring encrypted historical application data.

These are separate capabilities.

```text
Account recovery
      ≠
Data recovery
```

Required invariant:

> Successful account recovery MUST NOT imply access to historical encrypted message keys unless an independently authorized data-recovery mechanism exists.

### 10.3 Recovery and trust

Recovery events are trust-sensitive transitions.

They must:

- create an auditable state transition without recording secrets;
- invalidate stale pairing/recovery contexts;
- re-evaluate device membership;
- respect prior revocations;
- never create a hidden universal recovery key.

Exact recovery material and threshold policy belong to ADR-007 plus Key Management.

---

## 11. Platform assurance

### 11.1 Android

Android Keystore can make key material non-exportable and can bind keys to secure hardware such as TEE/StrongBox, subject to hardware and algorithm support.

For Eagle, these signals are implementation evidence for device-key protection, not the source of account identity.

### 11.2 Apple platforms

Apple Secure Enclave can protect private keys from direct plaintext handling by the application. App Attest provides app/device integrity signals through an Apple-assisted attestation flow.

Because App Attest validation is designed around server verification, it MUST NOT be a hard dependency for the core P2P trust ceremony.

### 11.3 Desktop

Desktop assurance should be represented through an adapter-defined evidence interface. Where available, TPM-backed protection may raise assurance, but TPM presence alone does not establish user identity or account membership.

### 11.4 Cross-platform contract

Platform adapters expose only normalized evidence:

```text
PlatformAssurance
├── availability
├── key_protection_level
├── integrity_signal
├── user_presence_signal
├── freshness
└── provenance
```

Rust Security Core remains the authority that decides how, or whether, these signals affect policy.

---

## 12. Privacy requirements

The following are forbidden as identity roots or stable account identifiers:

- IMEI;
- serial number;
- MAC address;
- advertising identifier;
- vendor device identifier;
- raw hardware attestation identifier exposed to application logic;
- IP address;
- network location.

Requirements:

- account identity must be self-certifying;
- device identity must be application-generated;
- public identity records must contain the minimum information needed for verification;
- contact discovery must not require global hardware identifiers;
- logs must not contain private identity keys, recovery secrets, or pairing secrets.

---

## 13. Security invariants

The implementation must satisfy all of the following:

**I-01** Login/authentication does not imply device trust.

**I-02** Unknown device state fails closed.

**I-03** PENDING cannot self-promote to TRUSTED.

**I-04** Pairing is authenticated, explicit, expiring, and single-use.

**I-05** QR/code transport never exports long-term private secrets.

**I-06** Device identity is distinct from session/ratchet keys.

**I-07** Account identity is distinct from username/profile metadata.

**I-08** Revocation blocks future authorization.

**I-09** Stale trust epochs cannot restore revoked authorization.

**I-10** Identity change requires explicit reverification.

**I-11** Account recovery cannot silently recover historical data.

**I-12** Platform attestation is additive evidence, not the trust root.

**I-13** AI or automation cannot directly grant trust or bypass the Security Kernel.

**I-14** Security-critical state transitions are auditable without recording secrets.

**I-15** No single transport/service dependency is required to preserve the meaning of identity trust.

---

## 14. Required negative tests

### Identity

- duplicate DeviceID under different account context → reject;
- malformed Account Membership Statement → reject;
- valid statement for a different account → reject;
- unknown device presenting valid-looking metadata → reject;
- altered identity public key with unchanged identifier → reject.

### Pairing

- replay of completed QR/session → reject;
- expired pairing context → reject;
- changed transcript hash → reject;
- mismatched short authentication code → reject;
- approval on a different account context → reject;
- attempted secret export during pairing → fail and record security event.

### Trust

- PENDING self-promotion → reject;
- REVOKED device requests new privileged session → reject;
- REPLACED device requests reactivation → reject;
- stale trust epoch attempts authorization → reject;
- missing authorization evidence → reject.

### Recovery

- account recovery without data-recovery material → account may recover, history remains unavailable;
- recovery attempt for a revoked device → reject;
- reuse of old recovery ceremony after trust-epoch change → reject.

### Cross-platform

- same logical flow on Android/Desktop/iOS adapters must produce equivalent Security Kernel decisions for equivalent evidence;
- platform assurance unavailable → core trust still follows explicit policy rather than silent downgrade to stronger privileges.

---

## 15. Evidence contract

Every trust-sensitive transition should emit a safe audit event:

```text
event_type
timestamp
account_id_hash
device_id_hash
previous_state
new_state
trust_epoch
policy_version
correlation_id
safe_reason_code
platform_assurance_class
```

Never log:

- private keys;
- raw session secrets;
- recovery codes;
- message plaintext;
- full pairing secrets.

---

## 16. Implementation boundary

### Rust Security Core

Owns:

- identity state machine;
- trust transitions;
- membership validation;
- membership/device binding registry;
- identity-change quarantine and explicit reverification;
- authorization policy;
- trust epoch checks;
- security event generation;
- fail-closed behavior.

### Current implementation hardening

The current branch additionally enforces two high-value pairing invariants at the Security Core boundary:

1. Pairing context is bound to the intended device identity as well as the account and trust epoch.
2. Empty pairing identifiers are rejected before a pairing context can become active.

A valid pairing context therefore cannot be reused to elevate a different pending device, even before the final protocol transcript construction is integrated. These are boundary controls; the final cryptographic binding remains delegated to the approved Protocol/Crypto implementation.

### KMP Shared Layer

Owns:

- user-facing orchestration;
- application state;
- pairing UI flow orchestration;
- non-sensitive presentation state;
- platform-neutral contracts.

### Platform adapters

Own:

- secure key-store integration;
- OS lifecycle;
- local unlock/user-presence facilities;
- platform integrity/assurance evidence;
- native transport/UI hooks.

Platform code must not reimplement trust rules.

---

## 17. Open decisions before ADOPTED

The following remain intentionally unresolved:

1. Concrete Account Identity Key type.
2. Concrete Device Identity Key type.
3. Account authority and membership-signing hierarchy.
4. Exact transcript-authentication construction.
5. Key rotation and successor proof format.
6. Maximum trusted devices per account.
7. Whether multi-device is V1 or later.
8. Exact recovery mechanism.
9. Exact trust-epoch synchronization semantics.
10. Final platform assurance scoring policy.
11. Final storage layout and secure deletion interactions.

No code may silently invent these values.

---

## 18. Acceptance gate for Identity & Trust

Identity & Trust is **NOT release-ready** until all are true:

```text
Specification approved
        +
ADR-005 / ADR-006 / ADR-007 decisions recorded
        +
Key Management interfaces frozen
        +
Protocol transcript requirements frozen
        +
Rust Security Core implementation
        +
cross-platform adapter tests
        +
negative/adversarial tests
        +
revocation/recovery evidence
        +
security review
```

Any missing item keeps the identity/trust area below the production gate.

---

## 19. References

Project references:

- `archive/chatgpt-historical/ADR_Decision_Pack.md` — historical design corpus for OI-005/006/007
- `archive/chatgpt-historical/PRIVATE_MESH_MASTER_PROJECT_REFERENCE.md` — historical architecture/reference corpus
- `docs/03-architecture/ADR_INDEX.md` — current canonical ADR sequence/status index
- `docs/03-architecture/adr/ADR-0008.md` — current protocol decision gate
- `docs/03-architecture/adr/ADR-0009.md` — current key-management decision gate
- `docs/03-architecture/adr/ADR-0010.md` — current serialization decision gate
- `docs/03-architecture/PLATFORMS.md` — current cross-platform security-boundary baseline

External authoritative references:

- Android Keystore: https://developer.android.com/privacy-and-security/keystore
- Android hardware-backed key attestation: https://developer.android.com/privacy-and-security/security-key-attestation
- Apple Secure Enclave: https://developer.apple.com/documentation/security/protecting-keys-with-the-secure-enclave
- Apple App Attest: https://developer.apple.com/documentation/DeviceCheck/establishing-your-app-s-integrity
- Signal X3DH: https://signal.org/docs/specifications/x3dh/
- Signal Double Ratchet: https://signal.org/docs/specifications/doubleratchet/
- Signal Sesame (multi-device session management reference): https://signal.org/docs/specifications/sesame/

---

## 20. Change control

Changes to:

- identity root semantics;
- trust states;
- pairing ceremony;
- revocation meaning;
- recovery authority;
- authorization guards;

require an ADR or an update to the existing identity ADRs before implementation.

This specification is deliberately conservative: **identity establishes who owns a cryptographic identity; trust establishes whether a device is authorized; platform assurance only informs that decision; session cryptography protects communications after trust has been established.**
