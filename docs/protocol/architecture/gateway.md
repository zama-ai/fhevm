# Gateway

The Gateway is a central orchestrator within Zama’s FHEVM protocol, playing a critical role in enabling secure,
composable, and confidential smart contracts across multiple blockchains. It coordinates interactions between users,
host chains, coprocessors, and the Key Management Service (KMS), ensuring that encrypted data flows securely and
correctly through the system.

## What is the Gateway?

The Gateway is a specialized blockchain component (implemented as an Arbitrum rollup) responsible for managing:

- Validation of encrypted inputs from users and applications.
- Bridging of encrypted ciphertexts across different blockchains.
- Decryption orchestration via KMS nodes.
- Consensus enforcement among decentralized coprocessors.
- Staking and reward distribution to operators participating in FHE computations.

It is designed to be trust-minimized: computations are independently verifiable, and no sensitive data or decryption
keys are stored on the Gateway itself.

## Responsibilities of the Gateway

### Encrypted Input Validation

The Gateway ensures that encrypted values provided by users are well-formed and valid. It does this by:

- Accepting encrypted inputs along with Zero-Knowledge Proofs of Knowledge (ZKPoKs).
- Emitting verification events for coprocessors to validate.
- Aggregating signatures from a majority of coprocessors to generate attestations, which can then be used on-chain as
  trusted external values.

### Access Control Coordination

The Gateway maintains a synchronized copy of Access Control Lists (ACLs) from host chains, enabling it to independently
determine if decryption or computation rights should be granted for a ciphertext. This helps enforce:

- Access permissions (allow)
- Public decryption permissions (allowForDecryption)

These ACL updates are replicated by coprocessors and pushed to the Gateway for verification and enforcement.

### Decryption Orchestration

When a smart contract or user requests the decryption of an encrypted value:

1. The Gateway verifies ACL permissions.
2. It then triggers the KMS to decrypt (either publicly or privately).
3. Once the KMS returns signed results, the Gateway emits events that can be picked up by an oracle (for smart contract
   decryption) or returned to the user (for private decryption).

This ensures asynchronous, secure, and auditable decryption without the Gateway itself knowing the plaintext.

### Cross-Chain Bridging

The Gateway also handles bridging of encrypted handles between host chains. It:

- Verifies access rights on the source chain using its ACL copy.
- Requests the coprocessors to compute new handles for the target chain.
- Collects signatures from coprocessors.

Issues attestations allowing these handles to be used on the destination chain.

### Consensus and Slashing Enforcement

The Gateway enforces consensus across decentralized coprocessors and KMS nodes. If discrepancies occur:

- Coprocessors must provide commitments to ciphertexts.
- Fraudulent or incorrect behavior can be challenged and slashed.
- Governance mechanisms can be triggered for off-chain verification when necessary.

### Protocol Administration

The Gateway runs smart contracts that administer:

- Operator and participant registration (coprocessors, KMS nodes, host chains)
- Key management and rotation
- Bridging logic
- Input validation and decryption workflows

## Security and Trust Assumptions

The Gateway is designed to operate without requiring trust:

- It does not perform any computation itself—it merely orchestrates and validates.
- All actions are signed, and cryptographic verification is built into every step.

The protocol assumes no trust in the Gateway for security guarantees—it can be fully audited and replaced if necessary.
