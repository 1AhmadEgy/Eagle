# Eagle / PrivateMesh — Architecture Baseline

Status: **NORMATIVE PLANNING BASELINE — NOT A PRODUCTION SECURITY CLAIM**

This directory is the canonical architecture index for the staged Eagle / PrivateMesh development plan.

## Layers

1. Client applications
2. Platform adapters
3. Shared core
4. Security kernel
5. Cryptographic boundary
6. Transport adapters
7. Encrypted storage and recovery

## Security rule

Data must flow through authentication, authorization, validation and cryptographic verification before application acceptance. Transport is untrusted infrastructure and must not become a trust shortcut.

## Scope by release

| Release | Scope |
|---|---|
| V0.1 | Foundation, state machine, policy, CI, evidence and test harness |
| V1.0 | Private 1:1 identity, E2E session, transport, storage, recovery, deletion |
| V1.1 | Hardening, fuzzing, interoperability, failure injection and independent verification |
| V2.0 | Optional group/MLS and scale extensions only after separate ADR/threat-model approval |

Open decisions OI-001..OI-008 remain authoritative blockers where not closed by an approved ADR.
