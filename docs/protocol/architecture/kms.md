# KMS

The Key Management Service (KMS) is a cornerstone of the FHEVM architecture, responsible for the secure generation,
management, and usage of encryption keys needed to enable confidential smart contracts. It is implemented as a
decentralized, threshold MPC-based system, ensuring that no single party can decrypt or compromise data.

## What is the KMS?

The KMS is a decentralized network of MPC (Multi-Party Computation) nodes that:

- Generate global FHE keys
- Decrypt ciphertexts securely
- Support zero-knowledge proof infrastructure
- Manage key lifecycles with NIST compliance
- Provide public and user-targeted decryptions

It works entirely off-chain, but is orchestrated through the Gateway, which initiates and tracks all key-related
operations. This separation of powers ensures strong decentralization and auditability.

## Key Responsibilities

### FHE Key Generation

- The KMS generates a global public/private key pair used across all host chains.
- This key enables composability—encrypted data can be shared between contracts and chains.
- The private FHE key is never reconstructed; instead, it is secret-shared among the MPC nodes.

The system follows the NIST SP 800-57 key lifecycle model, managing key states such as Active, Suspended, Deactivated,
and Destroyed to ensure proper rotation and forward security.

- Threshold Decryption via MPC The KMS performs decryption using a threshold protocol—at least a minimum number of MPC
  parties (e.g., 9 out of 13) must collaborate to decrypt a value.
- This protects against compromise: no individual party has access to the full key.
- Supports both:
  - Public decryption (e.g., for smart contracts)
  - User decryption (privately returned, re-encrypted for the user)

All decryption operations are signed, and the output can be verified on-chain for full auditability.

### ZK Proof Support

The KMS generates Common Reference Strings (CRS) needed to validate Zero-Knowledge Proofs of Knowledge (ZKPoK) when
users submit encrypted values.

This ensures encrypted inputs are valid and well-formed.

## Security Architecture

### MPC-Based Key Sharing

- The KMS uses 13 MPC nodes, operated by different reputable organizations.
- Private keys are split using threshold secret sharing.
- Communications between nodes are secured using mTLS with gRPC.

### Honest Majority Assumption

- The protocol is robust against malicious actors as long as fewer than 1/3 of the nodes collude.
- Supports guaranteed output delivery even if some nodes are offline or misbehaving.

### Secure Execution Environments

Each MPC node can optionally run in Nitro Enclaves to prevent even the node operator from accessing their own key share.
This mitigates insider risks (e.g., selling shares, unauthorized reconstruction).

### Auditable via Gateway

- All operations are broadcast through the Gateway and recorded as blockchain events. -KMS responses are signed,
  allowing smart contracts and users to verify results cryptographically.

### Key Lifecycle Management

The KMS adheres to a formal key lifecycle, as per NIST SP 800-57:

| State          | Description                                                        |
| -------------- | ------------------------------------------------------------------ |
| Pre-activation | Key is created but not in use.                                     |
| Active         | Key is used for encryption and decryption.                         |
| Suspended      | Temporarily replaced during rotation. Still usable for decryption. |
| Deactivated    | Archived; only used for decryption.                                |
| Compromised    | Flagged for misuse; only decryption allowed.                       |
| Destroyed      | Key material is deleted permanently.                               |

Key Switching is supported via FHE to move ciphertexts between concrete keys during rotation, maintaining
interoperability.

### Backup & Recovery

In addition to robustness through MPC, the KMS also offers a custodial backup system:

- Each MPC party splits its key share into encrypted fragments and sends them to independent custodians.
- A quorum of custodians can help restore lost shares, enabling recovery even if multiple MPC parties are taken offline.
- This ensures business continuity and protection from mass outages .

### Workflow Example: Public Decryption

1. A smart contract requests decryption via an oracle.
2. The Gateway verifies permissions and emits an event.
3. KMS parties retrieve the ciphertext, verify it, and run the MPC decryption protocol.
4. Once a quorum agrees, the result is published (with signatures).
5. The oracle posts the plaintext back on-chain and contracts can verify the authenticity using the KMS signatures.
