---
date: 2026-01-13
type: exploration
depth: deep
focus: full codebase
commit: 8b6af23a
---

# FHEVM Codebase Exploration

## Executive Summary

**FHEVM** is the core framework of the Zama Confidential Blockchain Protocol. It enables confidential smart contracts on EVM-compatible blockchains by leveraging **Fully Homomorphic Encryption (FHE)**, allowing encrypted data to be processed directly on-chain without ever being decrypted.

### Key Guarantees

- **End-to-end encryption**: Transaction data is encrypted and never visible to anyone
- **Composability**: States update while remaining encrypted at all times
- **Backward compatibility**: Encrypted state co-exists with public state without impacting existing dApps

---

## Project Structure

The repository is a **monorepo** containing 7 major components:

```
fhevm/
├── library-solidity/     # Main Solidity library for dApp developers
├── host-contracts/       # FHE workflow orchestration contracts
├── gateway-contracts/    # Cross-chain gateway contracts
├── coprocessor/          # Rust FHE computation engine
├── kms-connector/        # Key Management Service bridge
├── protocol-contracts/   # Governance, staking, token contracts
├── test-suite/           # E2E testing infrastructure
├── charts/               # Helm deployment configurations
└── docs/                 # Documentation assets
```

---

## Component Deep Dives

### 1. library-solidity - Developer API

**Location:** `/library-solidity/`

This is the **primary interface for smart contract developers**. It provides a Solidity library that makes working with encrypted values as natural as working with regular integers.

#### Core Files

| File | Lines | Purpose |
|------|-------|---------|
| `lib/FHE.sol` | 9,524 | Main API - all FHE operations |
| `lib/Impl.sol` | 916 | Implementation layer |
| `lib/FheType.sol` | 89 | Type definitions |

#### Encrypted Types

All encrypted types wrap a `bytes32` handle that represents the encrypted value:

```solidity
// Boolean
ebool

// Unsigned integers
euint8, euint16, euint32, euint64, euint128, euint256

// Address type
eaddress

// External input types (for user-provided encrypted data)
externalEuint8, externalEuint16, ... externalEuint256
```

#### Supported Operations

The library provides **30+ FHE operations**:

| Category | Operations |
|----------|------------|
| Arithmetic | `add`, `sub`, `mul`, `div`, `rem` |
| Bitwise | `and`, `or`, `xor`, `not`, `shl`, `shr`, `rotl`, `rotr` |
| Comparison | `eq`, `ne`, `ge`, `gt`, `le`, `lt` |
| Selection | `min`, `max`, `select` (ternary if-then-else) |
| Utility | `neg`, `rand`, `randBounded`, `cast` |

#### Usage Pattern

```solidity
import {FHE, euint64, ebool} from "fhevm/lib/FHE.sol";

contract ConfidentialVault {
    mapping(address => euint64) private balances;

    function initialize() external {
        // 1. Setup coprocessor (once per contract)
        FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
    }

    function deposit(
        externalEuint64 encryptedAmount,
        bytes calldata inputProof
    ) external {
        // 2. Verify and convert external input
        euint64 amount = FHE.fromExternal(encryptedAmount, inputProof);

        // 3. Perform encrypted operations
        euint64 newBalance = FHE.add(balances[msg.sender], amount);

        // 4. Grant access permissions (CRITICAL!)
        FHE.allowThis(newBalance);           // Allow this contract
        FHE.allow(newBalance, msg.sender);   // Allow the user

        // 5. Store result
        balances[msg.sender] = newBalance;
    }
}
```

#### Examples Included

- `examples/EncryptedERC20.sol` - Confidential token with hidden balances
- `examples/HeadsOrTails.sol` - Coin flip game with encrypted randomness
- `examples/Rand.sol` - Encrypted random number generation
- `examples/MultiSig/` - Multi-signature wallet examples

---

### 2. host-contracts - On-Chain Orchestration

**Location:** `/host-contracts/`

These contracts run on the **host blockchain** (the EVM chain where dApps deploy) and orchestrate FHE operations through **symbolic execution**.

#### How It Works

1. Smart contract calls `FHE.add(a, b)`
2. The executor creates a **deterministic handle** for the result
3. The actual FHE computation is **offloaded to the coprocessor**
4. The handle can be used immediately for further operations

#### Core Contracts

##### ACL.sol (Access Control List) - v0.2.0

Manages **who can access which encrypted values**:

```solidity
// Persistent permission (survives across transactions)
function allow(bytes32 handle, address account) external;

// Transient permission (current transaction only)
function allowTransient(bytes32 handle, address account) external;

// Check permission
function isAllowed(bytes32 handle, address account) view returns (bool);

// Mark for public decryption
function allowForDecryption(bytes32[] memory handles) external;

// Delegation for user decryption
function delegateForUserDecryption(
    address delegate,
    address contractAddress,
    uint64 expirationDate
) external;
```

##### FHEVMExecutor.sol - v0.1.0

The **symbolic execution engine** that processes FHE operations:

- Validates input types and permissions
- Generates deterministic handles for results
- Integrates with ACL for permission checks
- Enforces HCU (Homomorphic Computation Unit) limits

**Handle Format:**
```
[hash: 20 bytes][index: 1 byte][chainId: 8 bytes][type: 1 byte][version: 1 byte]
```

##### InputVerifier.sol - v0.2.0

Verifies **user-provided encrypted inputs** using:
- Multi-signature threshold verification
- EIP-712 typed signatures
- Cross-chain signature support (from Gateway)

##### HCULimit.sol - v0.1.0

**Gas metering for FHE operations** to prevent DoS:

| Limit | Value |
|-------|-------|
| Per transaction | 20 million HCU |
| Sequential depth | 5 million HCU |

---

### 3. gateway-contracts - Cross-Chain Gateway

**Location:** `/gateway-contracts/`

The Gateway is a **separate blockchain** that coordinates:
- KMS (Key Management Service) nodes
- Coprocessor instances
- Decryption requests
- Cross-chain messaging

#### Core Contracts

| Contract | Purpose |
|----------|---------|
| `GatewayConfig.sol` | Central registry for all actors (KMS nodes, coprocessors, chains) |
| `MultichainACL.sol` | Access control across multiple host chains |
| `Decryption.sol` | Orchestrates decryption requests (public/user/delegated) |
| `KMSGeneration.sol` | FHE key and CRS (Common Reference String) generation |
| `InputVerification.sol` | Zero-knowledge proof validation |
| `CiphertextCommits.sol` | Registry of ciphertext commitments |
| `ProtocolPayment.sol` | Fee collection in $ZAMA tokens |

#### Key Patterns

1. **EIP-712 Signatures**: All off-chain operations use typed signatures
2. **Threshold Consensus**: KMS nodes vote on decryption/key operations
3. **Multi-chain Design**: Single gateway serves multiple host chains
4. **Emergency Pause**: Role-based pause functionality

#### Decryption Flow

```
1. Contract marks handle as decryptable
         ↓
2. Request emitted to Gateway
         ↓
3. KMS nodes vote (threshold required)
         ↓
4. If threshold met, decrypt
         ↓
5. Result sent back to requesting contract
```

---

### 4. coprocessor - FHE Computation Engine

**Location:** `/coprocessor/`

A **Rust microservices architecture** that performs the actual FHE computations off-chain.

#### Architecture

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Host Blockchain│────▶│  host-listener  │────▶│   PostgreSQL    │
└─────────────────┘     └─────────────────┘     └────────┬────────┘
                                                         │
                                                         ▼
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│    Blockchain   │◀────│transaction-sender│◀────│   tfhe-worker   │
└─────────────────┘     └─────────────────┘     └────────┬────────┘
                                                         │
                                                         ▼
                                                ┌─────────────────┐
                                                │    scheduler    │
                                                └─────────────────┘
```

#### Workspace Components

| Crate | Lines | Purpose |
|-------|-------|---------|
| `fhevm-engine-common` | 8,323 | Core types, FHE operations |
| `tfhe-worker` | - | gRPC server + background worker |
| `scheduler` | - | DAG scheduler for dependency graphs |
| `host-listener` | - | Blockchain event ingestion |
| `transaction-sender` | - | Result publishing |
| `gw-listener` | - | Gateway integration |
| `zkproof-worker` | - | Zero-knowledge proof generation |
| `sns-worker` | - | S3/SNS for large ciphertexts |

#### Database Schema

PostgreSQL tables:
- `computations` - FHE operations and their dependencies
- `ciphertexts` - Encrypted computation results
- `tenants` - Multi-tenant key management

Uses `NOTIFY/LISTEN` for event-driven work distribution.

#### Key File

`fhevm-engine-common/src/tfhe_ops.rs` (3,414 lines) contains all FHE operation implementations including GPU memory management support.

---

### 5. kms-connector - Key Management Bridge

**Location:** `/kms-connector/`

Bridges the **Gateway smart contracts** with the **KMS Core** (Key Management Service).

#### Three Microservices

1. **GatewayListener**
   - Listens to Gateway blockchain events
   - Multiple instances for high availability
   - WebSocket-based event streaming

2. **KmsWorker**
   - Forwards events to KMS Core via gRPC
   - Multi-shard KMS support
   - S3 ciphertext retrieval

3. **TransactionSender**
   - Single instance (nonce management)
   - AWS KMS or private key signing
   - Gas multiplier and retry logic

#### KMS Operations

- Public decryption requests
- User decryption requests
- Key generation (preprocessing + full)
- CRS generation
- PRSS initialization
- Key resharing

---

### 6. protocol-contracts - Protocol Infrastructure

**Location:** `/protocol-contracts/`

Contains 10 sub-projects implementing protocol-level functionality:

| Sub-project | Purpose |
|-------------|---------|
| `token/` | $ZAMA ERC20 with LayerZero OFT bridges |
| `governance/` | Cross-chain governance (Ethereum ↔ Gateway) |
| `staking/` | Protocol staking with rewards |
| `safe/` | Admin module for Safe smart accounts |
| `confidential-wrapper/` | ERC7984 wrapper for confidential tokens |
| `feesBurner/` | Fee collection and burning |
| `fhevm-cli/` | Hardhat tasks for FHE operations |
| `deployment-cli/` | Orchestrated deployment tool |

#### Governance Flow

```
Aragon DAO (Ethereum)
        ↓
GovernanceOAppSender
        ↓
   LayerZero V2
        ↓
GovernanceOAppReceiver (Gateway)
        ↓
    AdminModule
        ↓
   Safe Proxy
        ↓
 Target Contracts
```

---

### 7. test-suite - E2E Testing

**Location:** `/test-suite/`

Docker-based integration testing infrastructure.

#### CLI Commands

```bash
# Deploy entire stack
./fhevm-cli deploy

# Run specific tests
./fhevm-cli test input-proof
./fhevm-cli test user-decryption
./fhevm-cli test erc20

# Upgrade a service
./fhevm-cli upgrade coprocessor

# View logs
./fhevm-cli logs relayer

# Clean up
./fhevm-cli clean
```

#### KMS Modes

- **Centralized**: Single KMS node (development)
- **Threshold**: Multiple KMS nodes with consensus (production)

---

## Development Conventions

### Solidity

| Convention | Details |
|------------|---------|
| Version | `^0.8.24` |
| Upgrades | UUPS proxy pattern |
| Storage | ERC-7201 namespaced storage |
| Errors | Custom errors (gas efficient) |
| Signatures | EIP-712 typed data |

### Rust

| Convention | Details |
|------------|---------|
| Edition | 2021 |
| Async | Tokio runtime |
| Database | sqlx with PostgreSQL |
| gRPC | Tonic |

### Testing

| Level | Tool |
|-------|------|
| Unit | Hardhat (Solidity), Cargo test (Rust) |
| Integration | Docker Compose |
| E2E | `fhevm-cli` commands |

---

## Quick Start for Developers

### Writing a Confidential Smart Contract

1. **Import the library**
   ```solidity
   import {FHE, euint64, ebool} from "fhevm/lib/FHE.sol";
   ```

2. **Initialize coprocessor** (in constructor or initializer)
   ```solidity
   FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
   ```

3. **Accept encrypted inputs**
   ```solidity
   euint64 value = FHE.fromExternal(externalValue, proof);
   ```

4. **Perform operations**
   ```solidity
   euint64 result = FHE.add(a, b);
   ebool isGreater = FHE.gt(a, b);
   euint64 selected = FHE.select(isGreater, a, b);
   ```

5. **Always grant access** (this is critical!)
   ```solidity
   FHE.allowThis(result);  // Allow contract to use
   FHE.allow(result, recipient);  // Allow user to use
   ```

### Running the Test Suite

```bash
cd test-suite/fhevm

# Deploy the full stack
./fhevm-cli deploy

# Run tests
./fhevm-cli test erc20
./fhevm-cli test user-decryption

# View logs if something fails
./fhevm-cli logs coprocessor
```

---

## Architecture Diagram

```
┌──────────────────────────────────────────────────────────────────────┐
│                         USER / DAPP                                   │
└────────────────────────────────┬─────────────────────────────────────┘
                                 │
                    ┌────────────▼────────────┐
                    │   Smart Contract        │
                    │   (uses FHE library)    │
                    └────────────┬────────────┘
                                 │
          ┌──────────────────────┼──────────────────────┐
          │                      │                      │
          ▼                      ▼                      ▼
┌─────────────────┐   ┌─────────────────┐   ┌─────────────────┐
│   ACL.sol       │   │ FHEVMExecutor   │   │ InputVerifier   │
│ (permissions)   │   │ (symbolic exec) │   │ (proof verify)  │
└─────────────────┘   └────────┬────────┘   └─────────────────┘
                               │
                    ┌──────────▼──────────┐
                    │   HOST BLOCKCHAIN   │
                    └──────────┬──────────┘
                               │ Events
                    ┌──────────▼──────────┐
                    │   host-listener     │
                    │   (Rust service)    │
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │     PostgreSQL      │
                    │  (job queue + data) │
                    └──────────┬──────────┘
                               │
          ┌────────────────────┼────────────────────┐
          │                    │                    │
          ▼                    ▼                    ▼
┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
│  tfhe-worker    │ │   scheduler     │ │ zkproof-worker  │
│  (FHE compute)  │ │ (DAG ordering)  │ │ (ZK proofs)     │
└────────┬────────┘ └─────────────────┘ └─────────────────┘
         │
         ▼
┌─────────────────┐
│transaction-sender│───────────▶ HOST BLOCKCHAIN
└─────────────────┘


                        ┌─────────────────┐
                        │  GATEWAY CHAIN  │
                        └────────┬────────┘
                                 │
        ┌────────────────────────┼────────────────────────┐
        │                        │                        │
        ▼                        ▼                        ▼
┌───────────────┐     ┌───────────────┐     ┌───────────────┐
│ GatewayConfig │     │  Decryption   │     │ KMSGeneration │
└───────────────┘     └───────┬───────┘     └───────────────┘
                              │
                    ┌─────────▼─────────┐
                    │   kms-connector   │
                    │  (Rust services)  │
                    └─────────┬─────────┘
                              │
                    ┌─────────▼─────────┐
                    │     KMS CORE      │
                    │  (Key Management) │
                    └───────────────────┘
```

---

## References

### Documentation
- [Official Documentation](https://docs.zama.ai/protocol)
- [Whitepaper](./fhevm-whitepaper.pdf)
- [Examples](https://docs.zama.ai/protocol/examples)

### Key Files
| File | Description |
|------|-------------|
| `library-solidity/lib/FHE.sol` | Main developer API |
| `library-solidity/lib/Impl.sol` | Implementation layer |
| `host-contracts/contracts/ACL.sol` | Access control |
| `host-contracts/contracts/FHEVMExecutor.sol` | Symbolic execution |
| `gateway-contracts/contracts/Decryption.sol` | Decryption orchestration |
| `coprocessor/fhevm-engine/fhevm-engine-common/src/tfhe_ops.rs` | FHE operations |
| `test-suite/README.md` | E2E testing guide |

### Example Contracts
| Example | Location |
|---------|----------|
| Encrypted ERC20 | `library-solidity/examples/EncryptedERC20.sol` |
| Coin Flip Game | `library-solidity/examples/HeadsOrTails.sol` |
| Random Numbers | `library-solidity/examples/Rand.sol` |
