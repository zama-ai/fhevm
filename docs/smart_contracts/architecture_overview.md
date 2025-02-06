# Architectural overview

The fhEVM architecture provides the foundation for confidential smart contracts on EVM-compatible blockchains. At its core is FHE, a cryptographic technique enabling computations directly on encrypted data, ensuring privacy at every stage.&#x20;

This system relies on three key types:&#x20;

- The **public key:** used for encrypting data.
- The **private key:** used for decryption and securely managed by the Key Management System or KMS
- The **evaluation key:** enabling encrypted computations performed by the coprocessor.

The fhEVM leverages Zama's TFHE library, integrating seamlessly with blockchain environments to address transparency, composability, and scalability challenges. Its hybrid architecture combines:

- **On-chain smart contracts** for encrypted state management and access controls.
- **Off-chain coprocessors** for resource-intensive FHE computations.
- **The Gateway** to coordinate between blockchain, KMS, and coprocessors.
- **The KMS** for secure cryptographic key management and proof validation.

This architecture enables developers to write private, composable smart contracts using symbolic execution and zero-knowledge proofs, ensuring data confidentiality and computational integrity.

For a detailed exploration of the fhEVM architecture, including components, workflows, and deployment models, see [Architecture Overview](architecture_overview.md).
