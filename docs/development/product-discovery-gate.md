# Product Discovery Gate

## Purpose

This document defines the first product-facing gate after the secure CI/Test Lab foundation.

The repository must not invent product behavior, requirements, protocols, cryptographic claims, or tests merely to make Test Lab categories turn green.

## Deterministic discovery

CI runs `scripts/ci/discover-product-surface.py`.

The scanner records:

- implementation-file presence;
- common source directories;
- test directories;
- common dependency manifests;
- common entrypoints;
- the resulting discovery state.

The machine-readable result is written to `.ci/testlab/product-surface.json`.

## Current interpretation

At the time this gate was introduced, the repository tree is dominated by:

- CI/security workflows;
- OpenCode agent policies;
- governance/provenance/legal documentation;
- CI verification scripts.

No authoritative application capability surface was identified from which a real product requirements set could safely be inferred.

Therefore:

- product-specific Test Lab categories remain **PENDING**;
- no application architecture is invented;
- no cryptographic/protocol behavior is invented;
- no release eligibility is inferred.

## Transition criteria

The next product phase can begin when authoritative product material exists in the repository or is supplied as a durable project source.

The sequence is:

1. Requirements extraction.
2. Architecture and threat-model derivation.
3. Test Matrix mapping requirements to executable tests.
4. Unit/integration/security/protocol/crypto tests as actually required.
5. Regression and property/fuzz coverage where applicable.
6. Independent evidence review.
7. Release artifact, SBOM, provenance and attestation controls when real release artifacts exist.

## Safety invariant

A missing product specification is a **blocking evidence condition**, not a permission to fabricate implementation details.
