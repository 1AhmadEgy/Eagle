# PM-SPEC-011 — Protocol Registry & Versioning

> PrivateMesh Protocol Specification

| Field | Value |
|---|---|
| Document ID | PM-SPEC-011 |
| Version | 1.1.0-draft |
| Status | IN REVIEW |
| Classification | NORMATIVE |
| Protocol Version | V1 |
| Repository | 1AhmadEgy/Eagle |

## 1. Purpose

PM-SPEC-011 defines the authoritative numeric registry and versioning rules for PrivateMesh protocol identifiers and bounded protocol parameters.

## 2. Normative rules

- Every registered identifier MUST be unique.
- A registry entry MUST have exactly one authoritative name and semantic definition.
- Reusing an identifier for a different semantic meaning is prohibited.
- Deprecated identifiers MUST NOT be assigned to a new semantic.
- Security-sensitive parameter changes MUST be versioned and reviewed.
- Implementations MUST reject unknown mandatory algorithm/security profiles.
- Protocol version negotiation MUST NOT silently downgrade security.

## 3. Registry namespaces

| Range | Purpose |
|---|---|
| 0x1000–0x1FFF | Message types |
| 0x2000–0x2FFF | Error codes |
| 0x3000–0x3FFF | State identifiers |
| 0x4000–0x4FFF | Algorithm identifiers |
| 0x5000–0x5FFF | Capability/profile identifiers |
| 0x6000–0x60FF | Protocol limits |
| 0x6100–0x61FF | Timing/freshness parameters |
| 0x6200–0x62FF | Security/session policy parameters |

## 4. Authoritative V1 security/session parameters

| ID | Name | Value | Unit | Status |
|---:|---|---:|---|---|
| 0x6201 | SIGNED_PREKEY_ROTATION | 7 | days | ACTIVE |
| 0x6202 | SIGNED_PREKEY_VERIFICATION_WINDOW | 14 | days | ACTIVE |
| 0x6203 | TIMESTAMP_ACCEPTANCE_WINDOW | ±5 | minutes | ACTIVE |
| 0x6204 | SESSION_IDLE_TIMEOUT | 30 | days | ACTIVE |
| 0x6210 | ATTESTATION_VALIDITY | 30 | days | ACTIVE |
| 0x6211 | ATTESTATION_RENEWAL_WINDOW | 7 | days | ACTIVE |
| 0x6212 | ATTESTATION_GRACE_PERIOD | 7 | days | ACTIVE |
| 0x6213 | LOCAL_AUTH_RECENCY | 5 | minutes | ACTIVE |
| 0x6214 | DEVICE_IDLE_TIMEOUT | 30 | days | ACTIVE |
| 0x6215 | OFFLINE_QUEUE_TTL | TBD | seconds | RESERVED |
| 0x6216 | QUEUE_ACK_TIMEOUT | TBD | seconds | RESERVED |
| 0x6217 | MAX_QUEUE_BYTES | TBD | bytes | RESERVED |

### 4.1 Collision rule

0x6205 is intentionally left unassigned in this baseline because prior drafts used it for a conflicting Device Idle Timeout definition. A future assignment requires an explicit registry change.

## 5. Versioning

### 5.1 Patch

A patch version MAY correct documentation errors without changing wire semantics or security behavior.

### 5.2 Minor

A minor version MAY add backward-compatible capabilities.

### 5.3 Major

A major version is required for incompatible wire, cryptographic, state-machine, or security-policy changes.

### 5.4 Security changes

Security corrections MAY use an expedited review path but MUST be recorded as security changes and MUST update affected vectors and regression tests.

## 6. Registration procedure

A new registry entry MUST include:

1. Proposed numeric ID.
2. Canonical name.
3. Type and units.
4. Semantic definition.
5. Security impact.
6. Applicable protocol version.
7. Status.
8. Compatibility impact.
9. Tests.
10. Approving ADR or change record.

## 7. Unknown values

- Unknown optional values MAY be ignored only where the containing specification explicitly permits it.
- Unknown mandatory values MUST cause deterministic rejection.
- Unknown cryptographic algorithms MUST fail closed.
- Unknown security profiles MUST NOT trigger fallback to a weaker profile.

## 8. Registry integrity checks

CI SHOULD verify:

- no duplicate IDs;
- no duplicate canonical names;
- no overlapping semantic assignments;
- all referenced IDs exist;
- deprecated IDs are not newly assigned;
- units are consistent;
- generated registry artifacts match this document.

## 9. Change control

Registry changes MUST update:

- this specification;
- Master File Registry;
- affected normative specifications;
- test vectors;
- compatibility matrix;
- implementation constants;
- changelog or ADR.

## 10. Conformance

An implementation conforms to PM-SPEC-011 only if it applies the authoritative registry without semantic aliasing or silent reassignment.

**Status:** IN REVIEW. Exact V1 values marked TBD remain release blockers.
