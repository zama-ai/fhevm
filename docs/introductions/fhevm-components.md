# Fhevm components

This document provides a detailed explanation of each major component in the FHEVM architecture, and how they interact to enable private, verifiable, and composable smart contract execution on EVM-compatible blockchains.

## Overview

The FHEVM system is composed of four core components, each responsible for a distinct part of the privacy-preserving workflow. Together, they enable developers to build confidential smart contracts without modifying the underlying EVM.

| [**Fhevm Smart Contracts**](fhevm-components.md#fhevm-smart-contracts)           | Solidity contracts managing encrypted state and symbolic execution                     | Include `FHEVMExecutor.sol`, Access Control List (`ACL.sol`), and user-defined FHE-enabled contracts |
| -------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| [**Gateway**](fhevm-components.md#gateway)                                       | Off-chain coordinator between blockchain, KMS, and coprocessors | Handles encrypted input submission, proof validation, and decryption/user decryption requests                         |
| [**Coprocessor**](fhevm-components.md#coprocessor)                               | Off-chain FHE compute engine                 | Executes encrypted operations and manages ciphertext storage        |
| [**Key Management System (KMS)**](fhevm-components.md#key-management-system-kms) | Decentralized cryptographic service           | Manages FHE keys, validates ZK proofs, and performs threshold decryption              |

<figure><img src="../../.gitbook/assets/architecture.png" alt="FHE Keys Overview"><figcaption><p>High level overview of the fhevm Architecture</p></figcaption></figure>

## **Developer workflow:**

As a developer, working with FHEVM involves two main areas:

1. **Frontend development**:\
   Encrypt user inputs using the public key and submit them to the blockchain or Gateway for processing.
2. **Smart contract development**:\
   Write Solidity contracts using the `FHE.sol` library to perform symbolic operations on encrypted values via FHE handles.

## **Fhevm smart contracts**

FHEVM smart contracts are Solidity contracts that interact with encrypted values through symbolic execution.

### **Symbolic execution in Solidity**

- **Handles**: Smart contract operations return handles (references to ciphertexts), rather than directly manipulating encrypted data.
- **Lazy Execution**: Actual computation is done off-chain by the coprocessor after the contract emits symbolic instructions.

This allows efficient, gas-minimized interaction with encrypted data, while preserving EVM compatibility.

### **Zero-Knowledge proofs of knowledge (ZKPoKs)**

FHEVM incorporates ZKPoKs to verify the correctness of encrypted inputs and outputs:

- **Validation**: ZKPoKs ensure that inputs are correctly formed and correspond to known plaintexts without revealing
  sensitive data.
- **Integrity**: They prevent misuse of ciphertexts and ensure the correctness of computations.

By combining symbolic execution and ZKPoKs, FHEVM smart contracts maintain both privacy and verifiability.

## **Coprocessor**

The coprocessor is the compute engine of FHEVM, designed to handle resource-intensive homomorphic operations.

### **Key functions**:

1. **Execution**:  Performs encrypted operations (e.g., _add_, _mul_) on ciphertexts using the evaluation key.
2. **Ciphertext management**: Stores and retrieves ciphertexts securely in an off-chain database. Only handles are returned on-chain.

## **Gateway**

The Gateway acts as the communication hub between the blockchain, the coprocessor, the KMS, and user-facing applications.

### **Key functions**:

- **API for developers**: Exposes endpoints to submit encrypted inputs, request decryption, and manage user decryption.
- **Proof validation**: Forwards ZKPoKs to the Coprocessor for verification.
- **Off-chain coordination**: Handles smart contract and user decryption workflows in a verifiable and secure manner.

The Gateway abstracts complex cryptographic flows, simplifying developer integration.

## **Key management system (KMS)**

The KMS is a decentralized threshold-MPC-based service that manages the FHE key lifecycle and cryptographic security.

### **Key functions**:

- **Threshold decryption**: Uses Multi-Party Computation (MPC) to securely decrypt ciphertexts without exposing the
  private key to any single entity.
- **Key distribution**: Maintains the global FHE keys, which include:
  - **Public key**: Used for encrypting data (accessible to the frontend and smart contracts).
  - **Private key**: Stored securely in the KMS and used for decryption.
  - **Evaluation key**: Used by the coprocessor to perform FHE computations.

The KMS ensures robust cryptographic security, preventing single points of failure and maintaining public verifiability.

In the next section, we will dive deeper into encryption, re-encryption, and decryption processes, including how they
interact with the KMS and Gateway services. For more details, see [Encryption, decryption, and computation](./d_re_ecrypt_compute.md).
