# Eagle — AI Contract Hardening Review

Status: Implemented in Kotlin; build verification pending
Date: 2026-10-04
Branch: ai/reverse-engineering-foundation

## Scope

This review covers the executable AI contract layer under:
app/src/main/java/com/eagle/app/ai/

Current components:
- AIProvider
- ProviderCapabilities
- ProviderPolicy
- TaskType
- ReasoningDepth
- SecurityFinding
- EvidenceRecord
- HumanApproval

This is contract/evidence infrastructure. It is not a model runtime.

## Threats addressed

### Resource exhaustion

LLM/provider output, prompt context, patch proposals, path lists, finding lists, and evidence records are attacker/provider-controlled or indirectly influenced data.

The contracts therefore enforce bounded:
- analysis context;
- provider output;
- patch size;
- test description;
- number/length of paths;
- number of findings;
- evidence records' raw output;
- regression-test identifiers;
- human approval metadata.

### Cryptographic approval bypass

The PatchProposal contract now enforces:

cryptoSensitive == true -> requiresHumanApproval == true

This invariant is enforced at construction time instead of relying only on callers to remember policy.

ProviderPolicy separately refuses automatic patches for crypto-sensitive work when human approval is required.

## Security boundary

The AI contract layer cannot itself:
- authenticate an identity;
- authorize a peer;
- access private-key bytes;
- choose runtime cryptographic primitives;
- alter replay state;
- disable rate limits;
- mark a cryptographic patch approved.

It produces proposals/evidence for downstream deterministic verification.

## Evidence boundary

EvidenceRecord deliberately stores hashes and final required provider output but no model hidden reasoning, private keys, plaintext message content, credentials, or chain-of-thought.

The record is not cryptographically signed yet. The signing key and append-only persistence mechanism remain M2 work.

## Current gaps

- no concrete provider adapter;
- no product LLM inference runtime;
- no evidence hasher implementation enforcing canonical serialization;
- no EvidenceSigner implementation;
- no append-only evidence store;
- no end-to-end proposer -> verifier orchestration;
- no deterministic merge gate implementation in the Kotlin layer.

## Verification

Unit tests now cover:
- crypto-human-approval invariant;
- bounded analysis context;
- bounded evidence output;
- bounded finding title;
- existing provider policy behavior.

Build and Gradle test execution remain pending.

## Next AI gate

Implement canonical evidence hashing/provenance verification only after the serialization format is frozen. The signer must use key material outside repository source and must not expose private signing keys through the AI provider interface.
