# Eagle — P2P Cryptographic Deployment Profile

## Security position

Eagle's product constraint is **P2P-only**. Therefore no central service is allowed to become a cryptographic trust root.

## Required boundary

A connectivity/discovery/rendezvous component may exist only if separately approved and restricted to public/ephemeral routing data. It must never receive:

- plaintext;
- private identity keys;
- session secrets;
- message keys;
- recovery secrets.

## Prekey constraint

PQXDH is specified for asynchronous communication with published prekey material. The standard model assumes a service can make that public material available. Eagle must not silently add such a service and then describe the product as "serverless P2P".

Before PQXDH interoperability is declared, Eagle must freeze one of these security-approved profiles:

1. **Direct authenticated pairing:** prekey material is transferred directly between already authenticated peers; no server publication.
2. **User-controlled rendezvous directory:** a user-controlled or independently trusted directory publishes only approved public/prekey material and has no access to secret state.
3. **Strict online-only first contact:** no asynchronous first message is accepted until the peer is directly reachable.

A profile that changes the PQXDH message semantics or security assumptions requires a dedicated protocol specification and conformance suite; it must not be presented as stock PQXDH.

## Failure policy

- no direct peer reachability + no approved rendezvous profile → fail closed;
- stale/expired prekey material → reject or refresh according to the frozen profile;
- identity change without re-verification → reject/hold;
- unsupported protocol/profile version → reject;
- missing crypto provider → reject.

## Release condition

P2P transport is not a cryptographic substitute for E2EE. The P2P claim is accepted only when the crypto boundary, metadata boundary, and asynchronous/prekey profile are all independently evidenced.
