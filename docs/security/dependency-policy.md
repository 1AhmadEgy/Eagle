# The Eagle — Dependency and Supply-Chain Security Policy

- Status: CANONICAL
- Priority: Security first
- Enforcement: CI + release gate

## 1. Runtime independence rule

The production application must not require:

- Firebase
- FCM
- Google Play Services
- Play Integrity
- external analytics
- external crash/telemetry runtime services
- third-party messaging backends
- third-party application-content relays

Google Play is distribution/update infrastructure only.

## 2. Dependency admission

Every dependency requires a documented record containing:

- dependency name
- exact version
- direct/transitive role
- purpose
- license
- maintenance status
- known vulnerabilities
- transitive dependency impact
- binary-size impact
- startup/performance impact
- runtime network behavior
- security owner/reviewer

## 3. Cryptography rule

Do not introduce custom cryptography.

Do not add a cryptographic library solely because it is popular or convenient.

Any cryptographic dependency must have:

- explicit protocol justification
- ownership/integration boundary
- test strategy
- security review
- license review
- fallback/upgrade strategy

## 4. Build/release enforcement

CI must run:

- dependency inventory
- vulnerability scanning
- license review checks
- secret scanning
- forbidden-dependency checks
- SBOM generation/review
- release artifact inspection

## 5. Network-behavior enforcement

A dependency that opens an unexpected runtime network path is a release blocker until explicitly reviewed.

## 6. Historical dependencies

Dependencies mentioned in earlier documents but superseded by the current baseline remain historical evidence only.

Examples:

- Firebase/FCM
- Google runtime services
- server-content persistence components
- server Outbox implementation dependencies

They must not be silently restored by copy/paste from an older branch or document.

## 7. Evidence

The release package should record:

- dependency manifest
- lockfile state
- SBOM
- scan results
- exceptions/waivers
- reviewer/approval
