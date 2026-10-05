# PrivateMesh — Security Review Checklist

Status: REQUIRED GATE

## Boundary isolation

- [ ] Mesh never receives application plaintext.
- [ ] Mesh never reads private keys or crypto internals.
- [ ] Relay cannot decrypt or authenticate application content on behalf of the endpoint.
- [ ] Storage internals are inaccessible from Mesh.
- [ ] No Crypto↔Mesh dependency cycle exists.

## Untrusted network input

- [ ] All lengths, versions, IDs, candidates, routes, and metadata are validated.
- [ ] Validation occurs before state mutation.
- [ ] Malformed inputs cannot cause unbounded allocation or panic-based denial of service.
- [ ] Unknown mandatory values fail closed.
- [ ] Unknown cryptographic/security profiles never trigger weaker fallback.

## Connectivity and path security

- [ ] Direct path is preferred by policy.
- [ ] Relay is fallback only.
- [ ] Path switching does not change cryptographic trust state.
- [ ] Network/interface changes cannot bypass authentication.
- [ ] Simultaneous connection attempts converge deterministically.
- [ ] Route loops terminate under a hard bound.

## Resilience / DoS

- [ ] Candidate, peer, route, queue, retry, timer, and telemetry limits are explicit.
- [ ] Backoff is bounded and jittered where appropriate.
- [ ] Reconnection cannot create resource amplification.
- [ ] Relay abuse is rate/lease bounded.
- [ ] Sleep/wake and network changes cannot produce retry storms.

## Privacy

- [ ] Telemetry excludes plaintext, keys, tokens, full payloads, and unnecessary endpoint metadata.
- [ ] Discovery retention and expiry are bounded.
- [ ] Candidate/address exposure is minimized by policy.
- [ ] Relay use does not silently broaden metadata collection.

## Verification

- [ ] NET-001..020 are executable against the approved implementation.
- [ ] Negative tests cover malformed, oversized, unknown-version, downgrade, loop, poisoning, and auth-failure cases.
- [ ] Loss, reorder, duplication, timeout, NAT change, interface change, sleep/wake, direct/relay transition are tested.
- [ ] Resource-bound tests demonstrate graceful rejection.
- [ ] Independent verification has reviewed the evidence.

## Release decision

Any unchecked critical boundary control => NOT RELEASE-READY.
