# 01 — System Architecture

```mermaid
flowchart TB
  U[Users / Devices] --> APP[Application Layer]
  APP --> PA[Platform Adapter]
  PA --> CORE[Shared Core]
  CORE --> SK[Security Kernel]
  SK --> CRYPTO[Crypto Boundary]
  SK --> TA[Transport Adapter]
  CRYPTO --> STORE[Encrypted Storage]
  TA --> NET[Direct P2P / Authenticated Relay]
  NET --> PEER[Peer Device]
  STORE --> REC[Recovery / Deletion Manager]
  AI[AI / MCP] --> AUTH[AuthN + AuthZ + Allowlist + Validation]
  AUTH --> SK
```

### Trust boundaries

- The network is untrusted.
- Relay infrastructure may forward ciphertext but must not receive E2E plaintext or private keys.
- The security kernel is the authorization boundary.
- Persistent state is encrypted and subject to deletion guarantees.

### Implementation target

```text
apps/
core/
crypto/
transport/
storage/
api/
shared/
test-vectors/
integration-tests/
fuzz/
security/
docs/
scripts/
```
