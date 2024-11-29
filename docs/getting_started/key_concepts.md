# 1. Key concepts and features

<figure><img src="../.gitbook/assets/doc_header_fhevm.png" alt=""><figcaption></figcaption></figure>

## 1.1. Introduction

The Fully Homomorphic Ethereum Virtual Machine (fhEVM) is a groundbreaking protocol enabling **confidential smart contracts** on EVM-compatible blockchains. By leveraging Fully Homomorphic Encryption (FHE), fhEVM ensures complete data privacy without sacrificing composability or usability.

## 1.2. Table of Contents

- [1. Key concepts and features](#1-key-concepts-and-features)
  - [1.1. Introduction](#11-introduction)
  - [1.2. Table of Contents](#12-table-of-contents)
  - [1.3. Core principles](#13-core-principles)
  - [1.4. Key features](#14-key-features)
    - [1.4.1. Encrypted data types](#141-encrypted-data-types)
    - [1.4.2. Casting types](#142-casting-types)
    - [1.4.3. Confidential computation](#143-confidential-computation)
    - [1.4.4. Access control mechanism](#144-access-control-mechanism)
  - [1.5. Architectural overview](#15-architectural-overview)

## 1.3. Core principles

The design of fhEVM is guided by the following principles:

1. **Preserving Security**: No impact on the underlying blockchain's security guarantees.
2. **Public Verifiability**: All computations are publicly verifiable while keeping data confidential.
3. **Developer Accessibility**: Build confidential smart contracts using familiar Solidity tooling, without requiring cryptographic expertise.
4. **Composability**: Confidential smart contracts are fully interoperable with each other and public contracts.

## 1.4. Key features

### 1.4.1. Encrypted data types

fhEVM introduces encrypted data types compatible with Solidity:

- **Booleans**: `ebool`
- **Unsigned Integers**: `euint4`, `euint8`, `euint16`, `euint32`, `euint64`, `euint128`, `euint256`
- **Addresses**: `eaddress`
- **Bytes**: `ebytes64`, `ebytes128`, `ebytes256`
- **Input**: `einput` for handling encrypted input data

Encrypted data is represented as ciphertext handles, ensuring secure computation and interaction.

For more information see [use of encrypted types](../fundamentals/first_step/types.md).

### 1.4.2. Casting types

fhEVM provides functions to cast between encrypted types:

- **Casting between encrypted types**: `TFHE.asEbool` converts encrypted integers to encrypted booleans
- **Casting to encrypted types**: `TFHE.asEuintX` converts plaintext values to encrypted types
- **Casting to encrypted addresses**: `TFHE.asEaddress` converts plaintext addresses to encrypted addresses
- **Casting to encrypted bytes**: `TFHE.asEbytesX` converts plaintext bytes to encrypted bytes

For more information see [use of encrypted types](../fundamentals/first_step/types.md).

### 1.4.3. Confidential computation

fhEVM enables symbolic execution of encrypted operations, supporting:

- Arithmetic: `TFHE.add`, `TFHE.sub`, `TFHE.mul`, `TFHE.min`, `TFHE.max`, `TFHE.neg`, `TFHE.div`, `TFHE.rem`
  - Note: `div` and `rem` operations are supported only with plaintext divisors
- Bitwise: `TFHE.and`, `TFHE.or`, `TFHE.xor`, `TFHE.not`, `TFHE.shl`, `TFHE.shr`, `TFHE.rotl`, `TFHE.rotr`
- Comparison: `TFHE.eq`, `TFHE.ne`, `TFHE.lt`, `TFHE.le`, `TFHE.gt`, `TFHE.ge`
- Advanced: `TFHE.select` for branching on encrypted conditions, `TFHE.randEuintX` for on-chain randomness.

For more information on operations, see [Operations on encrypted types](../fundamentals/first_step/operations.md).
For more information on conditional branching, see [Conditional logic in FHE](../guides/conditions.md).
For more information on random number generation, see [Generate Random Encrypted Numbers](../guides/random.md).

### 1.4.4. Access control mechanism

fhEVM enforces access control with a blockchain-based Access Control List (ACL):

- **Persistent Access**: `TFHE.allow`, `TFHE.allowThis` grants permanent permissions for ciphertexts.
- **Transient Access**: `TFHE.allowTransient` provides temporary access for specific transactions.
- **Validation**: `TFHE.isSenderAllowed` ensures that only authorized entities can interact with ciphertexts.

For more information see [ACL](../fundamentals/acl.md)

## 1.5. Architectural overview

The fhEVM architecture provides the foundation for confidential smart contracts on EVM-compatible blockchains. At its core is Fully Homomorphic Encryption (FHE), a cryptographic technique enabling computations directly on encrypted data, ensuring privacy at every stage. This system relies on three key types: the **public key** (used for encrypting data), the **private key** (used for decryption and securely managed by the Key Management System or KMS), and the **evaluation key** (enabling encrypted computations performed by the coprocessor).

The fhEVM leverages Zama's TFHE library, integrating seamlessly with blockchain environments to address transparency, composability, and scalability challenges. Its hybrid architecture combines:

- **On-chain smart contracts** for encrypted state management and access controls.
- **Off-chain coprocessors** for resource-intensive FHE computations.
- **The Gateway** to coordinate between blockchain, KMS, and coprocessors.
- **The KMS** for secure cryptographic key management and proof validation.

This architecture enables developers to write private, composable smart contracts using symbolic execution and zero-knowledge proofs, ensuring data confidentiality and computational integrity.

For a detailed exploration of the fhEVM architecture, including components, workflows, and deployment models, see [Architecture Overview](../fundamentals/architecture_overview.md).
