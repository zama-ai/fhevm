# Architecture Overview

FHEVM follows a three-layer architecture separating on-chain coordination from off-chain computation:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         SMART CONTRACT LAYER                            │
│                           (On-Chain / EVM)                              │
│  ┌────────────────┐  ┌────────────────┐  ┌────────────────────────┐    │
│  │     Host       │  │    Gateway     │  │   Library (Developer)  │    │
│  │   Contracts    │◄─┤   Contracts    │  │   FHE.sol + Types      │    │
│  │                │  │                │  │                        │    │
│  │ FHEVMExecutor  │  │ GatewayConfig  │  │ euint8, euint256,      │    │
│  │ ACL, HCULimit  │  │ Decryption     │  │ ebool, eaddress...     │    │
│  │ KMSVerifier    │  │ MultichainACL  │  │                        │    │
│  └───────┬────────┘  └───────┬────────┘  └────────────────────────┘    │
└──────────┼───────────────────┼─────────────────────────────────────────┘
           │                   │
           │  Events/Requests  │
           ▼                   ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           COMPUTE LAYER                                 │
│                         (Off-Chain / Rust)                              │
│  ┌───────────────────────────────────────────────────────────────────┐ │
│  │                    Coprocessor (fhevm-engine)                      │ │
│  │                                                                    │ │
│  │  ┌─────────────┐  ┌───────────┐  ┌──────────────┐  ┌───────────┐ │ │
│  │  │ tfhe-worker │  │ scheduler │  │ zkproof-     │  │ listeners │ │ │
│  │  │ (FHE ops)   │  │           │  │ worker       │  │ (host/gw) │ │ │
│  │  └─────────────┘  └───────────┘  └──────────────┘  └───────────┘ │ │
│  └───────────────────────────────────────────────────────────────────┘ │
└──────────────────────────────────┬──────────────────────────────────────┘
                                   │
                                   │  Key Operations
                                   ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                        KMS CONNECTIVITY LAYER                           │
│                         (Off-Chain / Rust)                              │
│  ┌───────────────────────────────────────────────────────────────────┐ │
│  │                        KMS Connector                               │ │
│  │                                                                    │ │
│  │  ┌─────────────┐  ┌────────────┐  ┌────────────────────────────┐ │ │
│  │  │ gw-listener │  │ kms-worker │  │ transaction-sender         │ │ │
│  │  │             │  │ (MPC keys) │  │                            │ │ │
│  │  └─────────────┘  └────────────┘  └────────────────────────────┘ │ │
│  └───────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────────┘
```

## Layer Breakdown

### Layer 1: Smart Contract Layer (On-Chain)

**Components:**
- **Host Contracts**: Symbolic FHE execution on each EVM chain
- **Gateway Contracts**: Cross-chain coordination and off-chain communication
- **Library**: Developer-facing API (FHE.sol with encrypted types)

**Purpose:** Fast, deterministic on-chain operations that generate ciphertext handles and coordinate with off-chain services.

### Layer 2: Compute Layer (Off-Chain)

**Components:**
- **Coprocessor (fhevm-engine)**: Rust-based FHE computation engine
  - `tfhe-worker`: Actual FHE operations using TFHE-rs
  - `scheduler`: Job orchestration
  - `zkproof-worker`: Zero-knowledge proof generation
  - `listeners`: Monitor blockchain events

**Purpose:** Perform expensive FHE computations asynchronously and submit verified results back to chain.

### Layer 3: KMS Connectivity Layer (Off-Chain)

**Components:**
- **KMS Connector**: Interface to Key Management System
  - `gw-listener`: Monitor gateway events
  - `kms-worker`: Coordinate with external KMS Core
  - `transaction-sender`: Submit signed results

**Purpose:** Manage encryption keys via multi-party computation (MPC), ensuring no single party holds complete keys.

## Data Flow Summary

1. **Developer** writes smart contract using `FHE.sol` library with encrypted types
2. **User** submits transaction with encrypted inputs (proven via ZK proofs)
3. **Host Contracts** execute operations symbolically, generating ciphertext handles
4. **Gateway Contracts** coordinate between chain and off-chain components
5. **Coprocessor** performs actual FHE computations asynchronously
6. **KMS** manages encryption keys via multi-party computation (MPC)
7. Results are verified and committed back to chain state

## Key Architectural Principles

### Separation of Concerns
- **Consensus** (on-chain) is separate from **computation** (off-chain)
- Smart contracts never touch raw ciphertexts, only handles
- Heavy crypto happens asynchronously without blocking transactions

### Eventual Consistency
- Operations complete immediately on-chain (handle generation)
- Results become available later (seconds to minutes)
- System guarantees eventual consistency across all layers

### Threshold Security
- No single point of trust or failure
- Multiple coprocessors can provide redundancy
- KMS uses threshold signatures (MPC) for key operations

### Multi-Chain Design
- Single gateway can coordinate multiple host chains
- Host chains operate independently
- Gateway provides cross-chain ACL and coordination

---

**Next:** Check [Component Health & Activity](component-health.md) to see which areas are actively evolving →
