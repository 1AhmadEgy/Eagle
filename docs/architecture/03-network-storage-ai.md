# 03 — Network, Storage and AI/MCP

## Network

```mermaid
flowchart LR
  A[Device A] -->|Preferred| P2P[Direct P2P]
  P2P --> B[Device B]
  A -->|Fallback| RELAY[Authenticated Relay]
  RELAY --> B
  RELAY -. ciphertext only .-> RELAY
```

## Storage and deletion

```mermaid
flowchart TB
  APP[Application] --> SM[Storage Manager]
  SM --> DB[Durable DB]
  SM --> C[Cache]
  SM --> WAL[WAL]
  SM --> TMP[Temporary]
  SM --> EXP[Export]
  SM --> BK[Backup]
  DB --> ENC[Encrypted Data]
  C --> ENC
  WAL --> ENC
  TMP --> ENC
  EXP --> ENC
  BK --> ENC
  DEL[Deletion Manager] --> DB
  DEL --> C
  DEL --> WAL
  DEL --> TMP
  DEL --> EXP
  DEL --> BK
```

Deletion is not complete until all persistence paths covered by the approved V1 contract are handled.

## AI/MCP

```mermaid
flowchart TD
  REQ[AI / MCP Request] --> AUTH[Authentication]
  AUTH --> AZ[Authorization]
  AZ --> AL[Tool Allowlist]
  AL --> SV[Schema Validation]
  SV --> AV[Argument Validation]
  AV --> SB[Sandbox]
  SB --> RL[Rate Limit]
  RL --> AUD[Audit]
  AUD --> EX[Execution]
```

AI must not bypass the security kernel or directly access protected resources.
