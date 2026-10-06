# Eagle - Developer & Designer Execution Pack v1.0

**Date:** 2026-10-06  
**Purpose:** implementation handoff baseline for developers, security engineers, QA, architecture, product, and UI/UX.

## Canonical project constraints

1. **Security first.**
2. **P2P only** for message transport and device-to-device communication.
3. **No mandatory operational external runtime services.**
4. App stores are **distribution channels only**, not runtime dependencies.
5. No mandatory Firebase/FCM, APNs, Google Play Services, hosted authentication, hosted discovery/rendezvous, cloud message history, hosted backup, or mandatory hosted telemetry.
6. Rust Security Core owns security-critical cryptographic primitives and protocol-sensitive operations.
7. KMP Shared Layer owns reusable domain/application orchestration.
8. Android/Desktop/iOS use platform adapters; platform UI does not own security-critical primitives.
9. Performance, stability, correctness, resource efficiency, and package size are explicit engineering constraints.
10. No custom cryptography unless a formal ADR and independent review explicitly justify it.

## Current authority state

The repository already contains architecture, platform, security, requirements-traceability, verification, readiness, and execution documents. This handoff pack consolidates those baselines for execution.

Important distinction:

- Accepted/established project constraints must be treated as implementation constraints.
- Technical choices that remain **Proposed/Pending** must not be presented as final architecture.
- In particular, the current protocol/key-management ADR chain must be closed with tests and evidence before production.

## Reading order

- [Project reference](../00-reference/PROJECT_REFERENCE.md)
- [Architecture](../03-architecture/ARCHITECTURE.md)
- [Platform strategy](../03-architecture/PLATFORMS.md)
- [Security baseline](../04-security/SECURITY_BASELINE.md)
- [Security/identity threat model](../06-security/IDENTITY_TRUST_THREAT_MODEL.md)
- [Requirements traceability](../09-requirements/REQUIREMENTS_TRACEABILITY.md)
- [Implementation readiness](../12-readiness/IMPLEMENTATION_READINESS.md)
- [Execution continuation plan](../13-execution/EXECUTION_CONTINUATION_PLAN.md)
- [Research/security baseline](../01-research/RESEARCH_SECURITY_ARCHITECTURE_BASELINE.md)

## Companion package contents

The downloadable execution pack contains:

- project charter and canonical constraints;
- layered system architecture;
- security architecture and threat model;
- cryptography/protocol decision framework;
- P2P transport/discovery rules;
- data storage and synchronization policy;
- Android/Desktop/iOS platform boundary;
- UI/UX security-state specification;
- engineering/PR/dependency workflow;
- QA, fuzzing, performance, and release gates;
- requirements/ADR/evidence/exception templates;
- team work packages;
- repository/reference index;
- DOT, SVG, and PNG architecture diagrams;
- DOCX and PDF handoff editions.

## Diagram set

1. System context / P2P topology
2. Layered architecture
3. Trust boundaries
4. Message lifecycle
5. Engineering/security gate
6. Platform boundary

## Non-negotiable implementation rule

A feature is not accepted merely because it works locally. Each production feature must map:

**Requirement -> Architecture/ADR -> Implementation -> Tests -> Security Gate -> Review -> Evidence -> Release**

