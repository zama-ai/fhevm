# Gateway Contracts 🔥

**Location**: `/gateway-contracts/`
**Status**: Active Development
**Purpose**: Bridge between on-chain smart contracts and off-chain compute infrastructure

## Overview

The Gateway contracts serve as the coordination layer that manages communication between the blockchain and off-chain services. They handle ciphertext commitments, decryption requests, input verification, and access control across multiple chains.

## Key Contracts

| Contract | Purpose |
|----------|---------|
| `GatewayConfig.sol` | Central registry for KMS nodes, coprocessors, and protocol metadata |
| `Decryption.sol` | Manages public and user decryption requests with EIP712 signature validation |
| `MultichainACL.sol` | Access control for cross-chain operations and user delegations |
| `CiphertextCommits.sol` | Stores ciphertext material commitments from coprocessors |
| `InputVerification.sol` | Verifies encrypted user inputs via ZK proofs |
| `KMSGeneration.sol` | Orchestrates key generation and reshare operations |
| `ProtocolPayment.sol` | Handles protocol fee collection and distribution |

## Key Files

- `contracts/GatewayConfig.sol` - Gateway registry and configuration
- `contracts/Decryption.sol` - Decryption request handling
- `contracts/shared/Structs.sol` - Core data structures (KmsNode, Coprocessor, etc.)

## Relationships

Gateway contracts receive events from Host contracts and coordinate with the Coprocessor and KMS Connector to process FHE operations. They maintain consensus through threshold signatures from multiple coprocessors.

## Recent Development Focus (Dec 2025)

- Payment protocol implementation (`ProtocolPayment` contract)
- Multi-sig contracts based on Safe Smart Account
- LayerZero cross-chain integration for testnet/mainnet
- Monitoring events and request ID validation

## Areas for Deeper Documentation

**[TODO: Gateway consensus mechanism]** - Document the threshold-based consensus process for ciphertext commits and how multiple coprocessors agree on computation results

**[TODO: Multichain ACL flow]** - Detail how access control delegations work across different host chains

**[TODO: Payment protocol design]** - Explain the fee collection, distribution, and operator compensation mechanisms

**[TODO: KMS coordination]** - Document how Gateway orchestrates key generation, rotation, and decryption requests with the KMS

---

**Related:**
- [Host Contracts](host-contracts.md) - Emit events that Gateway processes
- [Coprocessor](coprocessor.md) - Processes Gateway-coordinated FHE operations
- [KMS Connector](kms-connector.md) - Handles Gateway key management requests
