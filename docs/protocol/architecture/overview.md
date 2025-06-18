# FHE on blockchain

This page provides an overview of FHE and its implementation in the blockchain context via FHEVM. It introduces the
essential cryptographic concepts and architecture needed to start building confidential smart contracts.

## **FHE overview**

Fully Homomorphic Encryption (FHE) is an advanced cryptographic technique that allows computations to be performed
directly on encrypted data, without requiring decryption. This enables data to remain confidential throughout its entire
lifecycle — at rest, in transit, and during computation.

With FHE:

- Data can be securely encrypted and still usable for meaningful computation.
- Computation results are also encrypted, maintaining end-to-end confidentiality.

FHE relies on three core cryptographic keys, each with a distinct role:

### **Private key**

- **Purpose**: Decrypts ciphertexts to recover the original plaintext.
- **Usage in FHEVM**: Managed by the Key Management System (KMS) using threshold MPC, ensuring no single party ever
  holds the complete key.

### **Public key**

- **Purpose**: Encrypts plaintext inputs.
- **Usage in FHEVM**: Globally available to encrypt data from frontend apps and users.

### **Evaluation key**

- **Purpose**: Enables homomorphic operations (e.g., _add_, _mul_) on encrypted data.
- **Usage in FHEVM**: Stored on coprocessors to perform encrypted computations without exposing data.

These keys are the foundation of FHEVM’s secure computation model, enabling a system where confidentiality,
programmability, and composability can coexist.

<figure><img src="../../.gitbook/assets/keys_fhe.png" alt="FHE Keys Overview"><figcaption><p>Overview of FHE Keys and their roles</p></figcaption></figure>

## **FHE to Blockchain: From library to FHEVM**

### **Building on Zama's FHE library**

At its core, FHEVM is powered by Zama’s high-performance FHE library, **TFHE-rs** — a Rust implementation of the Torus
Fully Homomorphic Encryption (TFHE) scheme. TFHE-rs is optimized for fast, secure computation on encrypted data, forming
the cryptographic backbone of FHEVM.

> **Info**: For detailed documentation and implementation examples on the `tfhe-rs` library, visit the
> [TFHE-rs documentation](https://docs.zama.ai/tfhe-rs).

However, integrating a standalone FHE library like TFHE-rs into a blockchain environment involves unique challenges.
Blockchain systems demand efficient processing, public verifiability, and seamless interoperability, all while
preserving their decentralized nature. To address these requirements, Zama designed the FHEVM, a system that bridges the
computational power of TFHE-rs with the transparency and scalability of blockchain technology.

### **Challenges in blockchain integration**

Integrating FHE into blockchain required solving several key problems:

1. **Transparency and privacy**: Blockchains are public by default. FHE enables privacy by keeping data encrypted
   throughout contract execution.
2. **Public verifiability**: Operations must be verifiable without revealing inputs. FHEVM uses symbolic execution and
   proofs to maintain trust.
3. **Composability**: Encrypted smart contracts must interoperate like standard ones. FHEVM allows encrypted state to be
   shared, composed, and reused across contracts.
4. **Performance and scalability**: FHE is computationally heavy. To keep contracts efficient, compute is offloaded to
   specialized nodes — coprocessors.

## Architectural overview

FHEVM combines these cryptographic guarantees with a hybrid architecture that balances privacy, scalability, and EVM
compatibility:

- **On-chain smart contracts** describe encrypted logic symbolically, managing access control and state via FHE handles.
- **Off-chain coprocessors** listen to emitted events, reconstruct the compute graph, and perform the actual encrypted
  computation.
- **The Relayer** acts as the protocol coordinator — verifying proofs, relaying requests, and managing data flow between
  the blockchain, the coprocessors, and the KMS.
- **The KMS** is a decentralized, threshold-secure network that handles private key operations like decryption and
  signing, without ever reconstructing the full key.

Rather than changing the EVM itself, this architecture introduces symbolic execution, turning smart contract calls into
a graph of operations to be computed off-chain. The result is a system where confidentiality, programmability, and
composability are not only compatible — they're native.
