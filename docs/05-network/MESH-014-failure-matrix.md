# MESH-014 — Failure Injection and Network Test Matrix
Status: SPECIFIED / VERIFICATION-GATED

## Mandatory scenarios
- empty candidate set
- peer disappearance
- NAT mapping change
- direct path failure
- relay outage
- relay recovery
- packet loss
- reordering
- duplication
- malformed frame
- oversized frame
- unknown version
- connection timeout
- rapid interface changes
- sleep/wake
- simultaneous connections
- route loop
- discovery poisoning
- peer authentication failure

## Evidence
Each scenario records TEST-ID, commit, environment, stimulus, expected/observed behavior, artifact/hash, reviewer, status.
