# Glossary

Comprehensive terminology reference for the FHEVM codebase.

## Core Concepts

**Ciphertext**
Encrypted data that can be operated on using Fully Homomorphic Encryption (FHE). The actual encrypted bytes stored off-chain in the coprocessor's database.

**Ciphertext Handle**
A 32-byte (`bytes32`) identifier that references off-chain encrypted data. On-chain contracts store and manipulate handles, not ciphertexts themselves. Think of it like a pointer or database key.

**Coprocessor**
Off-chain Rust service that performs actual FHE computation. Listens to blockchain events, executes TFHE operations asynchronously, and submits verified results back to the chain.

**Symbolic Execution**
On-chain execution model where contracts generate deterministic handles for operations without performing actual FHE computation. The "symbolic" part means it manipulates symbols (handles) representing values rather than the values themselves.

## System Architecture

**FHE (Fully Homomorphic Encryption)**
Cryptographic scheme that allows arbitrary computation on encrypted data without decrypting it. FHEVM uses the TFHE scheme implemented in TFHE-rs.

**TFHE (Torus Fully Homomorphic Encryption)**
Specific FHE scheme used by FHEVM. Optimized for boolean and small integer operations. Implemented in the TFHE-rs Rust library.

**Gateway Chain**
Central EVM chain that coordinates FHE operations across multiple host chains. Stores ciphertext commitments, manages cross-chain ACLs, and coordinates with coprocessors and KMS.

**Host Chain**
Any EVM-compatible blockchain where confidential dApps run. Each host chain has its own FHEVMExecutor, ACL, and other host contracts.

**KMS (Key Management System)**
System that manages encryption keys using multi-party computation (MPC). Ensures no single party holds complete decryption keys. FHEVM uses external KMS Core for key operations.

**KMS Core**
External service (not in this repository) that implements the threshold key management protocol. KMS Connector interfaces with KMS Core.

## Blockchain Components

**ACL (Access Control List)**
Smart contract that manages permissions for encrypted data handles. Controls which contracts and users can access specific ciphertexts.

**CiphertextCommits**
Gateway contract that stores commitments to ciphertexts computed by the coprocessor. Acts as proof that ciphertext exists and matches the handle.

**FHEVMExecutor**
Host contract that provides symbolic execution of FHE operations. Implements 20+ operators like `fheAdd`, `fheMul`, `fheEq`, etc.

**GatewayConfig**
Central registry on the gateway chain for KMS nodes, coprocessor instances, and protocol metadata.

**HCU (Homomorphic Complexity Unit)**
Measure of computational cost for FHE operations. Used to limit transaction complexity and prevent DoS. Each operation has an HCU cost (e.g., `fheAdd` costs 10 HCU).

**InputVerifier**
Contract that verifies zero-knowledge proofs for user-submitted encrypted inputs. Ensures ciphertexts are well-formed before accepting them.

## Off-Chain Components

**gw-listener**
Component that monitors gateway chain for relevant events (decryption requests, key operations, etc.). Part of both Coprocessor and KMS Connector.

**host-listener**
Component that monitors host chain for FHE operation events. Part of the Coprocessor.

**Scheduler**
Coprocessor component that orchestrates job distribution to worker threads. Manages priority queues and retry logic.

**tfhe-worker**
Coprocessor component that performs actual TFHE operations using the TFHE-rs library. CPU and GPU-accelerated.

**zkproof-worker**
Coprocessor component that generates zero-knowledge proofs for certain operations.

**sns-worker**
Coprocessor component that performs "Switch and Squash" optimizations on ciphertexts to reduce size and improve performance.

**transaction-sender**
Component that submits results from off-chain computation back to blockchain. Handles nonce management, gas estimation, and retries.

## Cryptographic Operations

**EIP712**
Ethereum standard for typed structured data hashing and signing. Used for KMS signatures on decryption results and other off-chain-generated data.

**MPC (Multi-Party Computation)**
Cryptographic protocol where multiple parties jointly compute a function while keeping inputs private. Used by KMS for threshold key operations.

**Threshold Signature**
Signature scheme where t-of-n parties must cooperate to create a valid signature. Used by KMS to sign decryption results (e.g., 3-of-5 KMS nodes must agree).

**Zero-Knowledge Proof (ZK Proof)**
Cryptographic proof that a statement is true without revealing why it's true. Used to prove encrypted inputs are well-formed without revealing plaintext.

## Protocol & Economics

**OFT (Omnichain Fungible Token)**
LayerZero standard for tokens that can transfer across chains. Used by ZAMA token for cross-chain transfers.

**OperatorStaking**
Protocol contract for node operators to stake ZAMA tokens. Provides economic security and Sybil resistance.

**ProtocolPayment**
Gateway contract that handles fee collection and distribution to operators. Implements the economic incentive layer.

**Rewarder**
Protocol contract that distributes rewards to stakers and delegators based on operator performance.

**UUPS (Universal Upgradeable Proxy Standard)**
Pattern for upgradeable smart contracts. Implementation stored separately from proxy, allowing logic updates.

## Development & Testing

**Hardhat**
Ethereum development environment for compiling, testing, and deploying smart contracts. Uses TypeScript for tests.

**Foundry**
Rust-based Ethereum development toolkit. Uses Solidity for tests (`forge test`).

**Mock FHE**
SQLite-backed fake FHE system for testing. Provides same API as real FHE but returns deterministic results instantly.

**TypeChain**
Tool that generates TypeScript bindings from smart contract ABIs. Enables type-safe contract interaction in tests.

## Encrypted Types

**ebool**
Encrypted boolean type. Represents encrypted true/false values.

**euint8, euint16, euint32, euint64, euint128, euint256**
Encrypted unsigned integer types of various bit sizes. Main types for confidential numeric operations.

**eint8, eint16, eint32, eint64, eint128, eint256**
Encrypted signed integer types. Support negative values.

**eaddress**
Encrypted Ethereum address type. Enables confidential address storage and comparison.

**AsciiString**
Encrypted ASCII string type. Enables confidential string operations.

## Operations

**Arithmetic Operations**
`add`, `sub`, `mul`, `div`, `rem`, `min`, `max` - Standard math operations on encrypted values.

**Comparison Operations**
`eq`, `ne`, `lt`, `le`, `gt`, `ge` - Comparison operations returning encrypted booleans.

**Bitwise Operations**
`and`, `or`, `xor`, `not`, `shl`, `shr`, `rotl`, `rotr` - Bit manipulation operations.

**Control Flow**
`select` - Encrypted ternary operator: `condition ? a : b`. Enables conditional logic without revealing which branch executes.

## Deployment

**Helm**
Kubernetes package manager. FHEVM components deployed via Helm charts in `/charts/`.

**Kubernetes (K8s)**
Container orchestration platform. Used for production deployment of coprocessor, KMS connector, and other services.

**docker-compose**
Tool for defining multi-container applications. Used in test-suite for local development and E2E testing.

## Miscellaneous

**Handle Generation**
Process of creating deterministic ciphertext handles. Uses cryptographic hash of inputs, operation type, and counter.

**Eventual Consistency**
Design pattern where on-chain state (handles) is immediately consistent, but off-chain ciphertexts become available later.

**Async Computation Model**
FHEVM's model where expensive operations happen asynchronously off-chain while on-chain transactions complete quickly.

**Multichain Deployment**
Configuration where single gateway chain coordinates FHE operations across multiple host chains.

**Single-chain Deployment**
Simple configuration where gateway and host are the same chain. Suitable for testing and simple deployments.

---

**Related:**
- [Key Concepts](../key-concepts.md) - Detailed explanation of core concepts
- [Quick Reference](quick-reference.md) - Fast lookup guide
- [Technology Stack](tech-stack.md) - Technologies used
