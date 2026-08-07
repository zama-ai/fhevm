# Core Components

FHEVM consists of seven major components, organized into three deployment layers:

## On-Chain Components (Solidity)

### 1. [Gateway Contracts](gateway-contracts.md) 🔥
Bridge between on-chain smart contracts and off-chain compute infrastructure. Manages ciphertext commitments, decryption requests, cross-chain ACL, and KMS coordination.

**Key contracts:** `GatewayConfig`, `Decryption`, `MultichainACL`, `CiphertextCommits`

### 2. [Host Contracts](host-contracts.md) ✅
On-chain symbolic execution of FHE workflows. Provides the core FHE execution interface for each supported EVM chain.

**Key contracts:** `FHEVMExecutor`, `ACL`, `HCULimit`, `KMSVerifier`

### 3. [Solidity Library](library-solidity.md) ✅
Developer-facing FHE primitives for writing confidential smart contracts. Provides encrypted types and FHE operation API.

**Key exports:** `FHE.sol`, encrypted types (`euint8`, `euint256`, `ebool`, `eaddress`)

## Off-Chain Components (Rust)

### 4. [Coprocessor](coprocessor.md) 🔥
Rust-based asynchronous FHE computation engine. Performs actual TFHE operations off-chain and submits verified results.

**Key crates:** `tfhe-worker`, `scheduler`, `zkproof-worker`, `host-listener`, `gw-listener`

### 5. [KMS Connector](kms-connector.md) 🔥
Interface between Gateway and Key Management System (KMS Core). Manages key generation, rotation, and decryption via MPC.

**Key crates:** `gw-listener`, `kms-worker`, `transaction-sender`

## Protocol Layer (Solidity)

### 6. [Protocol Contracts](protocol-contracts.md) 🔥
Protocol-level infrastructure including token, staking, and governance.

**Key modules:** `token/`, `staking/`, `governance/`, `confidential-wrapper/`

## Supporting Infrastructure

### 7. [Supporting Infrastructure](infrastructure.md) 📦
Deployment, testing, and operational tooling.

**Key directories:** `charts/`, `test-suite/`, `docs/`, `sdk/`

---

## Component Relationships

```
Developer Smart Contract
         ↓
   Solidity Library (FHE.sol)
         ↓
   Host Contracts (FHEVMExecutor)
         ↓
   Gateway Contracts
         ↓
    ┌────┴────┐
    ↓         ↓
Coprocessor  KMS Connector
```

**Flow:**
1. Developer uses Library to write contract with encrypted types
2. Contract calls Host Contracts for symbolic FHE operations
3. Host Contracts emit events picked up by Gateway
4. Gateway coordinates with Coprocessor (FHE compute) and KMS (key management)
5. Results flow back through Gateway → Host → Contract

---

**Status Legend:**
- 🔥 Active development
- ✅ Stable/maintained
- 📦 Infrastructure

Choose a component to explore its detailed documentation.
