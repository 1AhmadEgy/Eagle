# Eagle AI Layer — M3 Routing and Verification

Status: IMPLEMENTED

## Scope

M3 adds deterministic orchestration contracts without introducing a concrete remote model provider.

Implemented:
- ProviderRegistry
- ProviderRequirements
- TaskRouter
- FindingVerifier
- VerificationPipeline
- unit tests covering capability routing and proposer/verifier separation

## Routing invariants

1. Provider IDs are metadata, not routing preferences.
2. Task eligibility comes from ProviderPolicy.allowedTaskTypes.
3. Technical suitability comes from declared ProviderCapabilities.
4. Requirements can demand tool use, on-premise execution, telemetry-free operation, code-only mode, crypto-review support, reasoning depth, and context capacity.
5. Ties are deterministic and use provider ID only as a final stable ordering key.

## Proposer / verifier separation

VerificationPipeline selects:
1. a proposer for the requested task;
2. a different provider satisfying the same task and verifier requirements;
3. an independent FindingVerifier boundary.

The verifier receives the finding, target commit, and selected verifier provider. It does not receive hidden proposer reasoning.

A finding becomes VERIFIED only from the verifier outcome. A failed verification becomes DISPUTED.

## Patch boundary

M3 does not apply patches, merge changes, or bypass human approval. Existing PatchProposal invariants remain authoritative, including the crypto-sensitive human-approval requirement.

## Test strategy

Fake providers are used only in unit tests. No network model adapter is introduced.

The test suite proves:
- capability requirements influence selection;
- policy can exclude otherwise capable providers;
- proposer and verifier are different providers;
- verifier output controls VERIFIED/DISPUTED normalization.

## Remaining

M4 is concrete provider adapters. It must preserve this abstraction and must not make a provider name an authority or trust boundary.
