# Architectural overview

The FHEVM architecture provides the foundation for confidential smart contracts on EVM-compatible blockchains. At its
core is FHE, a cryptographic technique enabling computations directly on encrypted data, ensuring privacy at every
stage.&#x20;

This system relies on three key types:&#x20;

- The **public key:** used by users to encrypt their inputs locally.
- The **private key:** required for decryption, securely distributed and managed through a threshold MPC-based Key
  Management System (KMS).
- The **evaluation key:** enabling homomorphic computation, used by off-chain coprocessors.

FHEVM combines these cryptographic guarantees with a hybrid architecture that balances privacy, scalability, and EVM
compatibility:

- **On-chain smart contracts** describe encrypted logic symbolically, managing access control and state via FHE handles.
- **Off-chain coprocessors** listen to emitted events, reconstruct the compute graph, and perform the actual encrypted
  computation.
- **The Gateway** acts as the protocol coordinator — verifying proofs, relaying requests, and managing data flow between
  the blockchain, the coprocessors, and the KMS.
- **The KMS** is a decentralized, threshold-secure network that handles private key operations like decryption and
  signing, without ever reconstructing the full key.

Rather than changing the EVM itself, this architecture introduces symbolic execution, turning smart contract calls into
a graph of operations to be computed off-chain. The result is a system where confidentiality, programmability, and
composability are not only compatible — they're native.

For implementation details and developer guidance, see the full [Architecture Overview](architecture_overview.md).
