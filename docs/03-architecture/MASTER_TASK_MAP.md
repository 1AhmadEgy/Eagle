# Eagle Master Task Map

**Planning baseline:** Canonical Planning Baseline v1 (proposed)  
**Baseline date:** 2026-10-02  
**Status:** Proposed — not approved until ARCH-001 / ADR-0007 gates pass

## Purpose

This document is the durable task map for the architecture-planning work captured during the 2026-10-02 project coordination.

It separates:
- architecture decisions (ADRs),
- implementation / verification tasks (Issues),
- module ownership,
- dependency direction,
- security gates.

## Foundation

- ARCH-001 — Architecture & Task Baseline
- ARCH-002 — Core Contracts
- ARCH-003 — Dependency Rules
- ARCH-004 — Architecture Tests
- ARCH-005 — ADR Framework
- ADR-0007 — Architecture & Task Baseline
- ADR-0008 — Cryptographic Protocol
- ADR-0009 — Key Management
- ADR-0010 — Serialization
- ADR-0011 — Local Storage
- ADR-0012 — Transport Architecture
- ADR-0013 — Architecture Enforcement
- ADR-0014 — Observability

## Identity

- IDENTITY-001 — Device Identity
- IDENTITY-002 — Keystore
- IDENTITY-003 — Public Identity
- IDENTITY-004 — Fingerprint
- IDENTITY-005 — Signing
- IDENTITY-006 — Lifecycle
- IDENTITY-007 — Revocation
- IDENTITY-008 — Recovery / Migration
- IDENTITY-009 — Security Tests

## Crypto

- CRYPTO-001 — Primitives
- CRYPTO-002 — Session Establishment
- CRYPTO-003 — AEAD
- CRYPTO-004 — Authentication
- CRYPTO-005 — Key Rotation
- CRYPTO-006 — Replay Protection
- CRYPTO-007 — Forward Secrecy
- CRYPTO-008 — Invalid Packet Handling
- CRYPTO-009 — Fuzz / Property Tests
- CRYPTO-010 — Known-Answer Tests

## Protocol

- PROTO-001 — Message Envelope
- PROTO-002 — Packet Framing
- PROTO-003 — Serialization
- PROTO-004 — Versioning
- PROTO-005 — Compatibility Tests

## Storage

- STORAGE-001 — Schema
- STORAGE-002 — Encrypted Storage
- STORAGE-003 — Message Repository
- STORAGE-004 — Session Repository
- STORAGE-005 — Retention / Secure Deletion
- STORAGE-006 — Backup
- STORAGE-007 — Migration Tests
- STORAGE-008 — Corruption Recovery

## Mesh

- MESH-001 — Peer Abstraction
- MESH-002 — Discovery
- MESH-003 — Transport Abstraction
- MESH-004 — BLE
- MESH-005 — Local Transport
- MESH-006 — Internet Transport
- MESH-007 — Routing
- MESH-008 — Store-and-Forward
- MESH-009 — TTL / Replay
- MESH-010 — Peer Authentication
- MESH-011 — Failure Recovery
- MESH-012 — Two-Device Integration
- MESH-013 — Chunking / Reassembly
- MESH-014 — Queue / Priority
- MESH-015 — Battery / Background Strategy

## UI

- UI-001 — Identity Creation
- UI-002 — Identity Verification
- UI-003 — QR / Fingerprint Exchange
- UI-004 — Conversation List
- UI-005 — Conversation
- UI-006 — Encryption State
- UI-007 — Connectivity State
- UI-008 — Failure States
- UI-009 — RTL
- UI-010 — Accessibility

## Integration

- INTEG-001 — Dependency Injection / Wiring
- INTEG-002 — Contract Test Matrix
- INTEG-003 — Two-Device E2E
- INTEG-004 — Full E2E

## Security

- SEC-001 — Threat Model
- SEC-002 — Attack Surface
- SEC-003 — Trust Boundaries
- SEC-004 — Key Management Review
- SEC-005 — Authentication Review
- SEC-006 — Replay / Downgrade Review
- SEC-007 — Storage Leakage Review
- SEC-008 — Logging Review
- SEC-009 — Supply Chain
- SEC-010 — Static Analysis
- SEC-011 — Fuzzing
- SEC-012 — Release Security Gate

## Observability

- OBS-001 — Structured Logging
- OBS-002 — Crash Reporting
- OBS-003 — Performance Metrics
- OBS-004 — Network Diagnostics
- OBS-005 — Developer Diagnostics

## Documentation

- DOC-001 — Architecture
- DOC-002 — ADR Index
- DOC-003 — API Contracts
- DOC-004 — Security Model
- DOC-005 — Operations
- DOC-006 — Release Evidence

## Critical dependency spine

`ARCH-001 → ADR-0007 → ADR-0008/0009/0010 → core contracts → implementation tracks → integration → security gate → release`

No concrete cryptographic, serialization, storage, or transport technology is approved by this map alone.
