# fhEVM components

This document gives an detail explanantion of each components of fhEVM and illustrate how they work together to perform compuations.&#x20;

## Overview

The fhEVM architecture is built around four primary components, each contributing to the system's functionality and performance. These components work together to enable the development and execution of private, composable smart contracts on EVM-compatible blockchains. Below is an overview of these components and their responsibilities:

| [**fhEVM Smart Contracts**](fhevm-components.md#fhevm-smart-contracts)           | Smart contracts deployed on the blockchain to manage encrypted data and interactions.                     | Includes the Access Control List (ACL) contract, `TFHE.sol` Solidity library, `Gateway.sol` and other FHE-enabled smart contracts. |
| -------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| [**Gateway**](fhevm-components.md#gateway)                                       | An off-chain service that bridges the blockchain with the cryptographic systems like KMS and coprocessor. | Acts as an intermediary to forward the necessary requests and results between the blockchain, the KMS, and users.                  |
| [**Coprocessor**](fhevm-components.md#coprocessor)                               | An off-chain computational engine designed to execute resource-intensive FHE operations.                  | Executes symbolic FHE operations, manages ciphertext storage, and ensures efficient computation handling.                          |
| [**Key Management System (KMS)**](fhevm-components.md#key-management-system-kms) | A decentralized cryptographic service that securely manages FHE keys and validates operations.            | Manages the global FHE key (public, private, evaluation), performs threshold decryption, and validates ZKPoKs.                     |

<figure><img src="../../.gitbook/assets/architecture.png" alt="FHE Keys Overview"><figcaption><p>High level overview of the fhEVM Architecture</p></figcaption></figure>

## **Developer workflow:**

As a developer working with fhEVM, your workflow typically involves two key elements:

1. **Frontend development**:\
   You create a frontend interface for users to interact with your confidential application. This includes encrypting inputs using the public FHE key and submitting them to the blockchain.
2. **Smart contract development**:\
   You write Solidity contracts deployed on the same blockchain as the fhEVM smart contracts. These contracts leverage the `TFHE.sol` library to perform operations on encrypted data. Below, we explore the major components involved.

## **fhEVM smart contracts**

fhEVM smart contracts include the Access Control List (ACL) contract, `TFHE.sol` library, and related FHE-enabled contracts.

### **Symbolic execution in Solidity**

fhEVM implements **symbolic execution** to optimize FHE computations:

- **Handles**: Operations on encrypted data return "handles" (references to ciphertexts) instead of immediate results.
- **Lazy Execution**: Actual computations are performed asynchronously, offloading resource-intensive tasks to the coprocessor.

This approach ensures high throughput and flexibility in managing encrypted data.

### **Zero-Knowledge proofs of knowledge (ZKPoKs)**

fhEVM incorporates ZKPoKs to verify the correctness of encrypted inputs and outputs:

- **Validation**: ZKPoKs ensure that inputs are correctly formed and correspond to known plaintexts without revealing sensitive data.
- **Integrity**: They prevent misuse of ciphertexts and ensure the correctness of computations.

By combining symbolic execution and ZKPoKs, fhEVM smart contracts maintain both privacy and verifiability.

## **Coprocessor**

The coprocessor is the backbone for handling computationally intensive FHE tasks.

### **Key functions**:

1. **Execution**: Performs operations such as addition, multiplication, and comparison on encrypted data.
2. **Ciphertext management**: Stores encrypted inputs, states, and outputs securely, either off-chain or in a dedicated on-chain database.

## **Gateway**

The Gateway acts as the bridge between the blockchain, coprocessor, and KMS.

### **Key functions**:

- **API for developers**: Exposes endpoints for submitting encrypted inputs, retrieving outputs, and managing ciphertexts.
- **Proof validation**: Forwards ZKPoKs to the KMS for verification.
- **Off-chain coordination**: Relays encrypted data and computation results between on-chain and off-chain systems.

The Gateway simplifies the development process by abstracting the complexity of cryptographic operations.

## **Key management system (KMS)**

The KMS securely manages the cryptographic backbone of fhEVM by maintaining and distributing the global FHE keys.

### **Key functions**:

- **Threshold decryption**: Uses Multi-Party Computation (MPC) to securely decrypt ciphertexts without exposing the private key to any single entity.
- **ZKPoK validation**: Verifies proofs of plaintext knowledge to ensure that encrypted inputs are valid.
- **Key distribution**: Maintains the global FHE keys, which include:
  - **Public key**: Used for encrypting data (accessible to the frontend and smart contracts).
  - **Private key**: Stored securely in the KMS and used for decryption.
  - **Evaluation key**: Used by the coprocessor to perform FHE computations.

The KMS ensures robust cryptographic security, preventing single points of failure and maintaining public verifiability.

In the next section, we will dive deeper into encryption, re-encryption, and decryption processes, including how they interact with the KMS and Gateway services. For more details, see [Decrypt and re-encrypt](../d_re_ecrypt_compute.md).
