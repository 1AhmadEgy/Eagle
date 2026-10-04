# Eagle AI Layer — Evidence and Signing

Status: M2 IMPLEMENTATION

## 1. Scope

M2 establishes cryptographic evidence primitives for binding AI-assisted findings to a source snapshot and verification history.

It intentionally does not add persistent JSON/JSONL storage, Room, remote AI providers, automatic merge, or an Android Keystore implementation.

## 2. Canonical record hash

EvidenceHasher computes SHA-256 over a deterministic, type-aware, length-delimited binary representation.

The representation includes the domain/version marker, every EvidenceRecord field required for provenance, nested SecurityFinding and HumanApproval fields when present, ordered regression-test IDs, sorted members of set-valued fields, and explicit null/presence markers.

recordHash and signature are excluded because they are derived values.

The framing is a canonicalization and ambiguity defense. It should not be described as a standalone fix for SHA-256 length-extension attacks; that concern depends on the construction in which a hash is used. Eagle uses the digest as a content commitment and signs that digest separately.

## 3. Hash chain

EvidenceChain.verifyChain starts from the genesis condition previousHash == null.

Every subsequent record must point to the immediately preceding recordHash.

Verification stops at the first broken link or mismatching recomputed hash and returns isValid, firstBrokenIndex, and reason.

## 4. Signatures

EvidenceSigner signs and verifies the canonical recordHash. Key management is intentionally outside the interface so Android Keystore and CI-managed tooling can be added later without changing the evidence contract.

Current test implementation:
- JVM-only InMemoryTestSigner
- Ed25519 test key material exists only in test sources
- the test key is not a production credential

Production key-source adapters are deferred to a later phase. A production private key must never be committed to the repository.

## 5. Storage decision

M2 deliberately does not choose JSON or Room.

The logical evidence contract and verification semantics should stabilize before they are coupled to persistence. Eagle currently has no database module, and evidence may ultimately belong to CI/development tooling rather than the Android runtime.

A later storage adapter must preserve append-only semantics and retain record order, recordHash, previousHash, signature, and verification metadata.

## 6. Security invariants

1. AI output is evidence/advice, not authority.
2. Cryptographic patches require human approval.
3. Model chain-of-thought is never persisted as evidence.
4. Secrets, private keys, and plaintext payloads are excluded.
5. Hashing is deterministic for the defined byte encoding.
6. Changing a bound field changes the record hash with overwhelming cryptographic probability.
7. Changing a recordHash invalidates its signature.
8. Modifying a middle record is reported at the first affected index.

## 7. M2 acceptance

Implemented:
- EvidenceHasher
- EvidenceChain
- EvidenceSigner
- JVM InMemoryTestSigner
- EvidenceRecord.withComputedHash
- M2 evidence documentation
- unit tests for hash stability, bound-field changes, chain integrity, tamper localization, and signature verification

Android Test Lab remains an external gate. The current failure is the Android SDK package installation, not an evidence-test result.