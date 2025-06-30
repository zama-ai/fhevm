# KMS

This document explains one of the key components of the Zama Protocol - The Key Management Service (KMS), responsible for the secure generation, management, and usage of FHE keys needed to enable confidential smart contracts.

## What is the KMS?

The KMS is a decentralized network of several nodes (also called "parties") that run an MPC (Multi-Party Computation) protocol:

- Securely generate global FHE keys
- Decrypt ciphertexts securely for public and user-targeted decryptions
- Support zero-knowledge proof infrastructure
- Manage key lifecycles with NIST compliance

It works entirely off-chain, but is orchestrated through the Gateway, which initiates and tracks all key-related operations. This separation of powers ensures strong decentralization and auditability.

## Key responsibilities

### FHE threshold key generation

- The KMS securely generates a global public/private key pair used across all host chains.
- This key enables composability — encrypted data can be shared between contracts and chains.
- The private FHE key is never directly accessible by a single party; instead, it is secret-shared among the MPC nodes.

The system follows the NIST SP 800-57 key lifecycle model, managing key states such as Active, Suspended, Deactivated,and Destroyed to ensure proper rotation and forward security.

### Threshold Decryption via MPC

The KMS performs decryption using a threshold decryption protocol — at least a minimum number of MPC parties (e.g., 9 out of 13) must participate in the protocol to robustly decrypt a value.

- This protects against compromise: no individual party has access to the full key. And adversary would need to control more than the threshold of KMS nodes to influence the system.
- The protocol supports both:
  - Public decryption (e.g., for smart contracts)
  - User decryption (privately returned, re-encrypted only for the user to access)

All decryption operation outputs are signed by each node and the output can be verified on-chain for full auditability.

### ZK Proof support

The KMS generates Common Reference Strings (CRS) needed to validate Zero-Knowledge Proofs of Knowledge (ZKPoK) when users submit encrypted values.

This ensures encrypted inputs are valid and well-formed, and that a user has knowledge of the plaintext contained in the submitted input ciphertext.

## Security architecture

### MPC-based key sharing

- The KMS currently uses 13 MPC nodes, operated by different reputable organizations.
- Private keys are split using threshold secret sharing.
- Communication between nodes are secured using mTLS with gRPC.

### Honest majority assumption

- The protocol is robust against malicious actors as long as at most 1/3 of the nodes act maliciously.
- It supports guaranteed output delivery even if some nodes are offline or misbehaving.

### Secure execution environments

Each MPC node runs by default inside an AWS Nitro Enclave, a secure execution environment that prevents even node operators from accessing their own key shares.
This design mitigates insider risks, such as unauthorized key reconstruction or selling of shares.

### Auditable via gateway

- All operations are broadcast through the Gateway and recorded as blockchain events.
- KMS responses are signed, allowing smart contracts and users to verify results cryptographically.

### Key lifecycle management

The KMS adheres to a formal key lifecycle, as per NIST SP 800-57:

| State          | Description                                                        |
| -------------- | ------------------------------------------------------------------ |
| Pre-activation | Key is created but not in use.                                     |
| Active         | Key is used for encryption and decryption.                         |
| Suspended      | Temporarily replaced during rotation. Still usable for decryption. |
| Deactivated    | Archived; only used for decryption.                                |
| Compromised    | Flagged for misuse; only decryption allowed.                       |
| Destroyed      | Key material is deleted permanently.                               |

The KMS supports key switching using FHE, allowing ciphertexts to be securely transferred between keys during rotation. This maintains interoperability across key updates.

### Backup & recovery

In addition to robustness through MPC, the KMS also offers a custodial backup system:

- Each MPC node splits its key share into encrypted fragments, distributing them to independent custodians.
- If a share is lost, a quorum of custodians can collaboratively restore it, ensuring recovery even if several MPC nodes are offline.
- This approach guarantees business continuity and resilience against outages.
- All recovery operations require a quorum of operators and are fully auditable on-chain.

### Workflow example: Public decryption

1. A smart contract requests decryption via an oracle.
2. The Gateway verifies permissions (i.e. that the contract is allowed to decrypt the ciphertext) and emits an event.
3. KMS parties retrieve the ciphertext, verify it, and run the MPC decryption protocol to jointly compute the plaintext and sign their result.
4. Once a quorum agrees on the plaintext result, it is published (with signatures).
5. The oracle posts the plaintext back on-chain and contracts can verify the authenticity using the KMS signatures.
