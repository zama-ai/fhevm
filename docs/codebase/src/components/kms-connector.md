# KMS Connector 🔥

**Location**: `/kms-connector/`
**Status**: Active Development
**Purpose**: Interface between the Gateway and Key Management System (KMS Core)

## Overview

The KMS Connector bridges the Gateway contracts with the external KMS Core service that manages encryption keys using multi-party computation (MPC). This ensures no single party ever holds the complete decryption key.

## Key Crates

| Crate | Purpose |
|-------|---------|
| `gw-listener` | Monitors Gateway for key-related events |
| `kms-worker` | Forwards requests to KMS Core service |
| `transaction-sender` | Submits signed responses back to Gateway |
| `utils` | Shared utilities and types |

## Supported Key Operations

- **Key generation** - Initial setup of encryption keys
- **Preprocessing keygen** - Pre-generated key material for faster operations
- **Key reshare (rotation)** - Distribute key shares to new node set
- **CRS generation** - Common Reference String for cryptographic protocols
- **Decryption signing** - Threshold signatures for decryption results

## Architecture

```
Gateway Events → gw-listener → kms-worker → KMS Core (external)
                                               ↓
Gateway Contracts ← transaction-sender ← Signed Response
```

**Flow:**
1. Gateway emits key operation event (e.g., DecryptionRequest)
2. `gw-listener` detects event and creates job
3. `kms-worker` forwards request to external KMS Core
4. KMS Core performs MPC protocol across threshold nodes
5. KMS Core returns EIP712-signed response
6. `transaction-sender` submits signed result to Gateway contract

## Key Files

- `gw-listener/src/main.rs` - Event listener entry point
- `kms-worker/src/main.rs` - KMS request handler
- `Cargo.toml` - Workspace dependencies

## Relationships

KMS Connector listens to `KMSGeneration` and `Decryption` events from Gateway contracts. It forwards requests to the external KMS Core (not in this repo) and submits EIP712-signed responses back to the chain.

## Recent Development Focus (Dec 2025)

- **Garbage collection**: Implementation of key material cleanup
- **Database management**: Transaction handling and retry logic
- **Polling improvements**: Enhanced listener reliability
- **Nonce manager**: Recoverable nonce patterns for transaction submission
- **Configuration updates**: WebSocket to HTTP migration

## Areas for Deeper Documentation

**[TODO: KMS integration flow]** - Document the complete flow from decryption request to signed response. Include sequence diagrams and error handling patterns.

**[TODO: Threshold signature scheme]** - Explain the MPC-based threshold signature mechanism for key security. Detail the trust model and security guarantees.

**[TODO: Key lifecycle]** - Document key generation, rotation, and retirement processes. Explain preprocessing and performance optimizations.

**[TODO: EIP712 signing]** - Detail the structured data signing format used for Gateway responses and verification.

**[TODO: External KMS Core integration]** - Document the interface contract with external KMS Core service, API expectations, and deployment patterns.

---

**Related:**
- [Gateway Contracts](gateway-contracts.md) - Emits events that trigger KMS operations
- [Key Concepts](../key-concepts.md) - Understanding the role of KMS in FHEVM
- [Workflows: Decryption Pipeline](../workflows/decryption-pipeline.md) - How KMS fits in decryption flow
