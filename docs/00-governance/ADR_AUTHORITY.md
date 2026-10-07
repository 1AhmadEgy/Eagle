# ADR Authority and Acceptance Governance

## Status
**Proposed governance baseline — not an authorization to approve ADRs**

## Purpose
This document separates proposing a decision, technical/security review, acceptance authority, repository enforcement, and implementation verification.

## Authority model
Until a broader governance decision is ratified, a single designated project decision owner (BDFL/technical owner) may accept or reject an ADR after required review.

This authority is temporary and MUST be re-evaluated after the next N=10 accepted ADRs, when the core engineering team materially expands, or when an ADR changes the security trust boundary, cryptographic protocol, key-management authority, recovery authority, or release authority.

## Required review
Security-sensitive ADRs MUST receive security review before acceptance. An ADR affecting an accepted ADR MUST explicitly identify the dependency and reconciliation decision.

An ADR is not accepted merely because it exists, CI passes, implementation exists, or a historical artifact calls it final.

## Acceptance record
An accepted ADR MUST record the decision owner, reviewers, acceptance timestamp, exact commit, conflicts/superseded ADRs, and implementation/verification follow-ups.

## Enforcement boundary
CI, CODEOWNERS, branch protection, required reviews, and evidence retention enforce accepted decisions. They do not independently create architectural authority. ADR-0013 remains responsible for enforcement.

## Current state
No ADR is accepted by this document automatically. ADR-0015 remains Proposed until its substantive gaps and governance prerequisites are explicitly closed.
