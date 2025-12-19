# FHEVM Codebase Overview

> **Version**: 1.0 | **Last Updated**: December 2024
> **Purpose**: High-level architectural overview for developers working with or rebuilding the fhevm codebase

---

## Executive Summary

**FHEVM** is the core framework of the Zama Confidential Blockchain Protocol. It enables **confidential smart contracts on EVM-compatible blockchains** by leveraging Fully Homomorphic Encryption (FHE), allowing encrypted data to be processed directly on-chain without ever being decrypted.

### Key Guarantees

- **End-to-end encryption**: Transaction data and state remain encrypted at all times
- **Composability**: Encrypted state coexists with public state, enabling complex DeFi and application logic
- **No impact on existing dApps**: Confidential features are additive; existing applications continue to function

### Core Innovation

FHEVM uses **symbolic execution with asynchronous computation**:
1. FHE operations execute **symbolically on-chain** (fast, deterministic, cheap)
2. Actual FHE computation happens **asynchronously off-chain** via the coprocessor
3. Results are verified and committed back to the chain

This architecture separates the slow cryptographic work from blockchain consensus, enabling practical FHE on Ethereum-compatible chains.

---

## Key Concepts

Before diving into the architecture, these concepts are essential to understanding FHEVM:

### Ciphertext Handles

A **ciphertext handle** is a 32-byte identifier (`bytes32`) that references encrypted data. Think of it like a pointer or database key:
- The **handle** is stored on-chain (small, cheap)
- The actual **ciphertext** (encrypted data) is stored off-chain in the coprocessor
- Smart contracts operate on handles; the coprocessor operates on ciphertexts

### Symbolic Execution

**Symbolic execution** means the on-chain contracts don't perform actual FHE computation. Instead, they:

1. **Generate deterministic handles**: Given inputs `handle_a` and `handle_b` for an `add` operation, compute `handle_result = hash(handle_a, handle_b, "add", counter)`
2. **Emit events**: Log the operation details for the off-chain coprocessor
3. **Return immediately**: The transaction completes without waiting for FHE computation

**Example**: When a contract calls `FHE.add(a, b)`:
```
On-chain (FHEVMExecutor):              Off-chain (Coprocessor):
1. Validate inputs
2. Generate handle_c = hash(...)
3. Emit event(ADD, a, b, c)      -->   4. Listen for event
4. Return handle_c                     5. Load ciphertexts a, b
                                       6. Compute c = TFHE.add(a, b)
                                       7. Store ciphertext c
                                       8. Commit to CiphertextCommits
```

The on-chain transaction completes in step 4. Steps 5-8 happen asynchronously seconds/minutes later.

### Asynchronous Computation Model

Because FHE operations are slow (seconds to minutes), FHEVM uses an **eventual consistency** model:

- **Writes are immediate**: `balances[user] = FHE.add(balances[user], amount)` completes immediately (new handle stored)
- **Results arrive later**: The actual encrypted value is computed and stored asynchronously
- **Reads use latest state**: Subsequent operations on the same handle will use the computed ciphertext once available

**For decryption**: Contracts must request decryption explicitly and receive results via callback:
```solidity
// Request decryption (async)
Gateway.requestDecryption(handle, callbackSelector);

// Receive result later via callback
function onDecrypt(uint256 plaintext) external onlyGateway {
    // Use decrypted value
}
```

### Host Chain vs Gateway Chain

FHEVM supports **multiple host chains** (any EVM-compatible chain) coordinated by a single **gateway chain**:

- **Host Chain**: Where your dApp runs. Each host chain has its own FHEVMExecutor, ACL, etc.
- **Gateway Chain**: Central coordination point. Stores ciphertext commitments, manages cross-chain ACLs, coordinates with coprocessors and KMS.

In simple deployments, these may be the same chain. In multichain deployments, the gateway is a separate chain that coordinates FHE operations across all hosts.

### Use Cases

- Confidential token transfers (private balances without mixers)
- Blind auctions (hidden bids until reveal)
- On-chain games (hidden cards, moves, selections)
- Encrypted DIDs and attestations
- Confidential voting (anti-bribery, anti-coercion)

---

## Architecture Overview

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

### Data Flow Summary

1. **Developer** writes smart contract using `FHE.sol` library with encrypted types
2. **User** submits transaction with encrypted inputs (proven via ZK proofs)
3. **Host Contracts** execute operations symbolically, generating ciphertext handles
4. **Gateway Contracts** coordinate between chain and off-chain components
5. **Coprocessor** performs actual FHE computations asynchronously
6. **KMS** manages encryption keys via multi-party computation (MPC)
7. Results are verified and committed back to chain state

---

## Component Health & Freshness

> **Last analyzed**: December 2024 | Based on 6-month git history

| Component | Status | 6-mo Commits | Focus Areas |
|-----------|--------|--------------|-------------|
| `coprocessor/` | 🔥 Active | 1,718 | GPU optimization, metrics, health checks |
| `kms-connector/` | 🔥 Active | 1,110 | Garbage collection, polling, nonce management |
| `gateway-contracts/` | 🔥 Active | 1,071 | Payment protocol, multi-sig, cross-chain |
| `protocol-contracts/` | 🔥 Active | 748 | Staking, delegation, fee management |
| `host-contracts/` | ✅ Stable | 455 | ACL enhancements, operator pricing |
| `library-solidity/` | ✅ Stable | 410 | Codegen consolidation, type improvements |
| `test-suite/` | 🔥 Active | 975 | E2E tests, version tracking |
| `charts/` | 📦 Infra | 138 | K8s deployment emerging |
| `sdk/` | 📦 Infra | 91 | Maintenance mode |

**Legend**: 🔥 Active development | ✅ Stable/maintained | 📦 Infrastructure (minimal changes)

**Recent removals** (avoid documenting these deprecated items):
- `ProtocolOperatorRegistry` - removed from protocol-contracts, replaced by `OperatorStaking`
- Distributed codegen - consolidated to `/library-solidity/codegen/`
- Safe-specific tasks - removed from gateway-contracts

---

## 3. Core Components

### 3.1 Gateway Contracts 🔥

**Location**: `/gateway-contracts/`
**Purpose**: Bridge between on-chain smart contracts and off-chain compute infrastructure

The Gateway contracts serve as the coordination layer that manages communication between the blockchain and off-chain services. They handle ciphertext commitments, decryption requests, input verification, and access control across multiple chains.

**Key Contracts**:

| Contract | Purpose |
|----------|---------|
| `GatewayConfig.sol` | Central registry for KMS nodes, coprocessors, and protocol metadata |
| `Decryption.sol` | Manages public and user decryption requests with EIP712 signature validation |
| `MultichainACL.sol` | Access control for cross-chain operations and user delegations |
| `CiphertextCommits.sol` | Stores ciphertext material commitments from coprocessors |
| `InputVerification.sol` | Verifies encrypted user inputs via ZK proofs |
| `KMSGeneration.sol` | Orchestrates key generation and reshare operations |
| `ProtocolPayment.sol` | Handles protocol fee collection and distribution |

**Key Files**:
- `contracts/GatewayConfig.sol` - Gateway registry and configuration
- `contracts/Decryption.sol` - Decryption request handling
- `contracts/shared/Structs.sol` - Core data structures (KmsNode, Coprocessor, etc.)

**Relationships**: Gateway contracts receive events from Host contracts and coordinate with the Coprocessor and KMS Connector to process FHE operations. They maintain consensus through threshold signatures from multiple coprocessors.

**Recent Development Focus** (as of Dec 2024):
- Payment protocol implementation (`ProtocolPayment` contract)
- Multi-sig contracts based on Safe Smart Account
- LayerZero cross-chain integration for testnet/mainnet
- Monitoring events and request ID validation

[TODO: Gateway consensus mechanism - Document the threshold-based consensus process for ciphertext commits and how multiple coprocessors agree on computation results]

[TODO: Multichain ACL flow - Detail how access control delegations work across different host chains]

---

### 3.2 Host Contracts ✅

**Location**: `/host-contracts/`
**Purpose**: On-chain symbolic execution of FHE workflows on the host EVM chain

Host contracts are deployed on each supported EVM chain and provide the core FHE execution interface. They execute operations symbolically (generating deterministic handles) while the actual encrypted computation happens off-chain.

**Key Contracts**:

| Contract | Purpose |
|----------|---------|
| `FHEVMExecutor.sol` | Symbolic execution engine with 20+ FHE operators |
| `ACL.sol` | Access control for encrypted data handles |
| `HCULimit.sol` | Enforces Homomorphic Complexity Unit limits per transaction |
| `KMSVerifier.sol` | Verifies KMS-signed decryption results |
| `InputVerifier.sol` | Verifies encrypted user input proofs (host-side) |

> **Note**: Both Gateway (`InputVerification.sol`) and Host (`InputVerifier.sol`) have input verification. Gateway handles cross-chain verification; Host handles local chain verification. In single-chain deployments, Host's InputVerifier is the primary entry point.

**Key Files**:
- `contracts/FHEVMExecutor.sol` - Core symbolic execution (fheAdd, fheMul, fheEq, etc.)
- `contracts/ACL.sol` - Permission management for ciphertext handles
- `contracts/shared/FheType.sol` - Encrypted type definitions

**Relationships**: Host contracts receive calls from user smart contracts via the FHE library. They emit events that the Coprocessor's host-listener picks up. Results from the Coprocessor are verified via KMSVerifier before being accepted.

[TODO: FHEVMExecutor operators - Document all 20+ FHE operators (fheAdd, fheSub, fheMul, fheDiv, fheEq, fheLt, fheGt, fheBitAnd, fheBitOr, fheBitXor, fheShl, fheShr, fheRotl, fheRotr, etc.) and their symbolic execution semantics]

[TODO: ACL permission model - Detail the allowList, denyList, and delegation mechanisms for controlling access to encrypted values]

[TODO: HCU limit enforcement - Explain the 20M HCU/tx and 5M depth limits and how they prevent DoS attacks]

---

### 3.3 Solidity Library ✅

**Location**: `/library-solidity/`
**Purpose**: Developer-facing FHE primitives for writing confidential smart contracts

The Solidity library provides the API that smart contract developers use to work with encrypted types. It abstracts away the complexity of FHE operations behind familiar Solidity syntax.

**Key Components**:

| File | Purpose |
|------|---------|
| `lib/FHE.sol` | Main developer API - import this to use FHE |
| `lib/Impl.sol` | Implementation details, delegates to precompiles |
| `lib/FheType.sol` | Encrypted type enum definitions |

**Encrypted Types**:
- **Boolean**: `ebool`
- **Unsigned integers**: `euint4`, `euint8`, `euint16`, `euint32`, `euint64`, `euint128`, `euint256`, up to `euint2048`
- **Signed integers**: `eint8`, `eint16`, `eint32`, `eint64`, `eint128`, `eint256`
- **Special**: `eaddress`, `AsciiString`

**Example Usage**:
```solidity
import {FHE, euint64} from "fhevm/lib/FHE.sol";

contract ConfidentialToken {
    mapping(address => euint64) private balances;

    function transfer(address to, euint64 amount) external {
        balances[msg.sender] = FHE.sub(balances[msg.sender], amount);
        balances[to] = FHE.add(balances[to], amount);
    }
}
```

**Key Files**:
- `lib/FHE.sol` - Primary import for developers
- `examples/EncryptedERC20.sol` - Reference implementation
- `codegen/` - Code generation for operator overloads

[TODO: Encrypted type system - Document all supported types, their bit sizes, and conversion rules between types]

[TODO: Codegen system - Explain how operator overloads are generated and how to extend the library]

---

### 3.4 Coprocessor 🔥

**Location**: `/coprocessor/`
**Purpose**: Rust-based asynchronous FHE computation engine

The Coprocessor is the off-chain component that performs actual FHE computations. It listens to events from Host and Gateway contracts, executes the expensive cryptographic operations, and submits results back to the chain.

**Key Crates** (in `/coprocessor/fhevm-engine/`):

| Crate | Purpose |
|-------|---------|
| `tfhe-worker` | Core FHE computation engine using TFHE-rs |
| `scheduler` | Job orchestration and work distribution |
| `zkproof-worker` | Zero-knowledge proof generation |
| `sns-worker` | Switch and Squash optimization for ciphertexts |
| `host-listener` | Monitors host chain events |
| `gw-listener` | Monitors gateway chain events |
| `transaction-sender` | Broadcasts results back to chain |

**Architecture**:
- **Event-driven**: Listeners pick up on-chain events and create jobs
- **Database-backed**: PostgreSQL stores job state and ciphertext data
- **Async processing**: Workers process jobs concurrently via scheduler
- **Threshold consensus**: Multiple coprocessors can run for redundancy

**Key Files**:
- `fhevm-engine/tfhe-worker/` - TFHE computation implementation
- `fhevm-engine/scheduler/` - Job queue and distribution
- `Cargo.toml` - Workspace manifest

**Relationships**: The Coprocessor receives work from Gateway contract events, processes FHE operations, and submits results via transaction-sender. It coordinates with the KMS Connector for key material.

**Recent Development Focus** (as of Dec 2024):
- GPU scheduler improvements and GPU memory management
- Metrics collection (SNS latency, ZK verify latency, tfhe-per-txn timing)
- Health checking in tfhe-worker and sns-worker
- Database optimization (indices on ciphertext_digest, schedule order)
- Compression for large ciphertexts
- Off-chain execution optimization

[TODO: Worker architecture - Detail the tfhe-worker, zkproof-worker, and sns-worker implementations and their processing pipelines]

[TODO: Scheduler and job orchestration - Document the job lifecycle from event reception to result submission]

---

### 3.5 KMS Connector 🔥

**Location**: `/kms-connector/`
**Purpose**: Interface between the Gateway and Key Management System (KMS Core)

The KMS Connector bridges the Gateway contracts with the external KMS Core service that manages encryption keys using multi-party computation (MPC). This ensures no single party ever holds the complete decryption key.

**Key Crates**:

| Crate | Purpose |
|-------|---------|
| `gw-listener` | Monitors Gateway for key-related events |
| `kms-worker` | Forwards requests to KMS Core service |
| `transaction-sender` | Submits signed responses back to Gateway |
| `utils` | Shared utilities and types |

**Supported Key Operations**:
- Key generation (initial setup)
- Preprocessing keygen
- Key reshare (rotation)
- CRS (Common Reference String) generation
- Decryption signing (threshold signatures)

**Key Files**:
- `gw-listener/src/main.rs` - Event listener entry point
- `kms-worker/src/main.rs` - KMS request handler
- `Cargo.toml` - Workspace dependencies

**Relationships**: KMS Connector listens to KMSGeneration and Decryption events from Gateway contracts. It forwards requests to the external KMS Core (not in this repo) and submits EIP712-signed responses back to the chain.

**Recent Development Focus** (as of Dec 2024):
- Garbage collection implementation
- Database transaction management and retry logic
- Polling mechanisms and listener improvements
- Nonce manager with recoverable patterns
- Configuration updates (WebSocket to HTTP migration)

[TODO: KMS integration flow - Document the complete flow from decryption request to signed response]

[TODO: Threshold signature scheme - Explain the MPC-based threshold signature mechanism for key security]

---

### 3.6 Protocol Contracts 🔥

**Location**: `/protocol-contracts/`
**Purpose**: Protocol-level infrastructure including token, staking, and governance

Protocol contracts implement the economic and governance layer of the FHEVM ecosystem. They are organized into domain-specific subdirectories.

**Submodules**:

| Directory | Purpose |
|-----------|---------|
| `token/` | ZAMA ERC20 token and OFT (Omnichain Fungible Token) for cross-chain |
| `staking/` | Node operator staking mechanisms |
| `governance/` | DAO voting and protocol governance |
| `confidential-wrapper/` | Wraps public tokens for confidential transfers |
| `feesBurner/` | Fee collection and token burning |
| `safe/` | Safe module for protocol administration |

**Key Files**:
- `token/ZamaERC20.sol` - Protocol token
- `confidential-wrapper/Wrapper.sol` - Public-to-confidential token bridge
- `staking/OperatorStaking.sol` - Operator staking

**Recent Development Focus** (as of Dec 2024):
- Staking/delegating contracts (`OperatorStaking`, `Rewarder`)
- Fee management and burner implementation
- Governance improvements (Safe ownership, admin modules)
- ERC1363 integration
- UUPS upgradeability patterns

> ⚠️ **Note**: `ProtocolOperatorRegistry` has been removed. Use `OperatorStaking` for staking functionality.

[TODO: Confidential wrapper pattern - Document how public ERC20 tokens are wrapped for confidential use and unwrapped back]

---

### 3.7 Supporting Infrastructure 📦

**Location**: Various
**Purpose**: Deployment, testing, and operational tooling

**Directories**:

| Directory | Purpose |
|-----------|---------|
| `/charts/` | Helm charts for Kubernetes deployment |
| `/test-suite/` | E2E integration tests with docker-compose |
| `/golden-container-images/` | Base Docker images for Node.js and Rust |
| `/docs/` | Gitbook documentation source |
| `/sdk/` | Rust SDK for building applications |
| `/ci/` | CI/CD pipeline configurations |

**Testing Infrastructure**:
- **Hardhat tests**: Unit/integration tests in `host-contracts/test/`, `library-solidity/test/`
- **Foundry tests**: Solidity-native tests via `forge test`
- **E2E tests**: Full-stack tests in `test-suite/` using docker-compose
- **Mock FHE**: SQLite-backed mocking for fast testing without real FHE

**Key Files**:
- `test-suite/docker-compose.yml` - Full stack orchestration
- `host-contracts/hardhat.config.ts` - Hardhat configuration
- `.github/workflows/` - CI pipeline definitions

[TODO: Testing infrastructure deep-dive - Document the mock FHE system, test fixtures, and E2E testing patterns]

[TODO: Deployment guide - Document the Helm charts and Kubernetes deployment process]

---

## 4. Key Workflows

### Symbolic Execution Pattern

When a smart contract calls an FHE operation like `FHE.add(a, b)`:

1. **On-chain (fast)**: `FHEVMExecutor` generates a deterministic ciphertext handle
2. **Event emission**: Operation details emitted as an event
3. **Off-chain (async)**: Coprocessor picks up event, performs actual FHE computation
4. **Result commitment**: Coprocessor submits result ciphertext to `CiphertextCommits`
5. **Consensus**: Multiple coprocessors agree on result via threshold signatures

This pattern allows smart contracts to execute quickly on-chain while heavy crypto happens off-chain.

### Decryption Pipeline

When a contract requests decryption:

1. **Request**: Contract calls decryption via Gateway's `Decryption` contract
2. **ACL check**: `MultichainACL` verifies requester has permission
3. **KMS notification**: `KMSGeneration` event triggers KMS Connector
4. **MPC decryption**: KMS nodes perform threshold decryption
5. **Signature**: Result signed by threshold of KMS nodes
6. **Verification**: `KMSVerifier` validates signatures on host chain
7. **Callback**: Decrypted value delivered to requesting contract

### Input Verification

When a user submits encrypted input:

1. **Client-side**: User encrypts data and generates ZK proof
2. **Submission**: Transaction includes ciphertext + proof
3. **Verification**: `InputVerifier` validates ZK proof
4. **Registration**: Valid ciphertext registered in `CiphertextCommits`
5. **Handle creation**: Ciphertext handle returned to smart contract

---

## 5. Technology Stack

| Layer | Technologies | Key Files |
|-------|-------------|-----------|
| **Smart Contracts** | Solidity 0.8+, OpenZeppelin v5.2, UUPS Proxies | `*.sol` |
| **Dev Tooling** | Hardhat, Foundry, TypeChain, Prettier | `hardhat.config.ts`, `foundry.toml` |
| **Coprocessor** | Rust, TFHE-rs, Tokio, SQLx, PostgreSQL | `Cargo.toml`, `*.rs` |
| **Communication** | gRPC, Protobuf, EIP712 signatures | `*.proto` |
| **Deployment** | Docker, Kubernetes, Helm | `charts/`, `docker-compose.yml` |
| **CI/CD** | GitHub Actions | `.github/workflows/` |

---

## 6. Documentation Roadmap

The following areas require deeper documentation, **prioritized by component activity level**. Each item is tagged for agent-based expansion:

### High Priority (🔥 Active Components)

1. **[TODO: Worker architecture]** - Detail tfhe-worker, zkproof-worker, sns-worker implementations (Coprocessor 🔥)
2. **[TODO: Scheduler and job orchestration]** - Job lifecycle from event reception to result submission (Coprocessor 🔥)
3. **[TODO: Gateway consensus mechanism]** - Threshold-based consensus for ciphertext commits (Gateway 🔥)
4. **[TODO: Multichain ACL flow]** - Access control delegations across host chains (Gateway 🔥)
5. **[TODO: KMS integration flow]** - Complete flow from decryption request to signed response (KMS 🔥)
6. **[TODO: Threshold signature scheme]** - MPC-based threshold signature mechanism (KMS 🔥)
7. **[TODO: Staking/delegation contracts]** - OperatorStaking, Rewarder implementation (Protocol 🔥)

### Medium Priority (✅ Stable Components)

8. **[TODO: FHEVMExecutor operators]** - Document all 20+ FHE operators and symbolic execution (Host ✅)
9. **[TODO: ACL permission model]** - allowList, denyList, and delegation mechanisms (Host ✅)
10. **[TODO: HCU limit enforcement]** - Resource limits and DoS prevention (Host ✅)
11. **[TODO: Encrypted type system]** - All supported types and conversion rules (Library ✅)
12. **[TODO: Codegen system]** - Operator overload generation process (Library ✅)

### Lower Priority (📦 Infrastructure)

13. **[TODO: Confidential wrapper pattern]** - Public-to-confidential token bridge
14. **[TODO: Testing infrastructure deep-dive]** - Mock FHE system and E2E patterns
15. **[TODO: Deployment guide]** - Helm charts and Kubernetes deployment

---

## Quick Reference

### Directory Map

```
fhevm/
├── gateway-contracts/     # Bridge to off-chain (Solidity)
├── host-contracts/        # On-chain FHE execution (Solidity)
├── library-solidity/      # Developer FHE library (Solidity)
├── protocol-contracts/    # Token, staking, governance (Solidity)
├── coprocessor/           # FHE compute engine (Rust)
├── kms-connector/         # KMS bridge (Rust)
├── charts/                # Kubernetes Helm charts
├── test-suite/            # E2E integration tests
├── docs/                  # Documentation source
└── sdk/                   # Rust SDK
```

### Entry Points for Developers

- **Writing contracts**: Start with `/library-solidity/lib/FHE.sol`
- **Understanding execution**: Read `/host-contracts/contracts/FHEVMExecutor.sol`
- **Understanding coordination**: Read `/gateway-contracts/contracts/GatewayConfig.sol`
- **Running tests**: See `/host-contracts/package.json` scripts
- **Deploying**: See `/charts/` Helm configurations

---

## Glossary

| Term | Definition |
|------|------------|
| **Ciphertext** | Encrypted data that can be operated on using FHE |
| **Ciphertext Handle** | A 32-byte on-chain identifier referencing off-chain encrypted data |
| **Coprocessor** | Off-chain Rust service that performs actual FHE computation |
| **EIP712** | Ethereum standard for typed structured data signing |
| **FHE** | Fully Homomorphic Encryption - allows computation on encrypted data |
| **Gateway Chain** | Central chain coordinating FHE operations across multiple hosts |
| **HCU** | Homomorphic Complexity Unit - measure of FHE computation cost |
| **Host Chain** | EVM chain where user dApps run |
| **KMS** | Key Management System - manages encryption keys via MPC |
| **MPC** | Multi-Party Computation - distributes trust across multiple parties |
| **Symbolic Execution** | On-chain execution that generates handles without computing ciphertexts |
| **TFHE-rs** | Rust library implementing the TFHE FHE scheme (used by coprocessor) |
| **UUPS** | Universal Upgradeable Proxy Standard - pattern for upgradeable contracts |

---

*This document is the foundation for iterative expansion. Each [TODO] marker indicates an area that can be expanded into its own detailed section.*
