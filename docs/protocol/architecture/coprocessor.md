# Coprocessor

The Coprocessor is the FHEVM protocol’s off-chain computation engine. It performs the heavy cryptographic
operations—specifically, fully homomorphic encryption (FHE) computations—on behalf of smart contracts that operate on
encrypted data. Acting as a decentralized compute layer, the coprocessor bridges symbolic on-chain logic with real-world
encrypted execution.

It works in tandem with the Gateway, verifying encrypted inputs, executing FHE instructions, and maintaining
synchronization of access permissions.

## What is the Coprocessor?

The Coprocessor is an off-chain service that:

- Listens to events emitted by host chains and the Gateway.
- Executes FHE computations (`add`, `mul`, `div`, `cmp`, etc.) on ciphertexts.
- Validates encrypted inputs and ZK proofs of correctness.
- Maintains and updates a replica of the host chain’s Access Control Lists (ACLs).
- Stores and serves encrypted data for decryption or bridging.

Each coprocessor independently executes tasks and publishes verifiable results, enabling a publicly auditable and
horizontally scalable confidential compute infrastructure .

## Responsibilities of the Coprocessor

### Encrypted Input Verification

When users submit encrypted values to the Gateway, each coprocessor:

- Verifies the associated Zero-Knowledge Proof of Knowledge (ZKPoK).
- Extracts and unpacks individual ciphertexts from a packed submission.
- Stores the ciphertexts under derived handles.
- Signs the verified handles, embedding user and contract metadata.
- Sends the signed data back to the Gateway for consensus.

This ensures only valid, well-formed encrypted values enter the system .

### FHE Computation Execution

When a smart contract executes a function over encrypted values, the on-chain logic emits symbolic computation events.
Each coprocessor:

- Reads these events from the host chain node it runs.
- Fetches associated ciphertexts from its storage.
- Executes the required FHE operations using the TFHE-rs library (e.g., add, mul, select).
- Stores the resulting ciphertext under a deterministically derived handle.
- Optionally publishes a commitment (digest) of the ciphertext to the Gateway for verifiability.

This offloads expensive computation from the host chain while maintaining full determinism and auditability .

### ACL Replication

Coprocessors replicate the Access Control List (ACL) logic from host contracts. They:

- Listen to Allowed and AllowedForDecryption events.
- Push updates to the Gateway.

This ensures decentralized enforcement of access rights, enabling proper handling of decryptions, bridges, and contract
interactions .

### Ciphertext Commitment

To ensure verifiability and mitigate misbehavior, each coprocessor:

- Commits to ciphertext digests (via hash) when processing Allowed events.
- Publishes these commitments to the Gateway.
- Enables external verification of FHE computations.

This is essential for fraud-proof mechanisms and eventual slashing of malicious or faulty operators .

### Bridging & Decryption Support

Coprocessors assist in:

- Bridging encrypted values between host chains by generating new handles and signatures.
- Preparing ciphertexts for public and user decryption using operations like Switch-n-Squash to normalize ciphertexts
  for the KMS.

These roles help maintain cross-chain interoperability and enable privacy-preserving data access for users and smart
contracts .

## Security and Trust Assumptions

Coprocessors are designed to be minimally trusted and publicly verifiable. Every FHE computation or input verification
they perform is accompanied by a cryptographic commitment (hash digest) and a signature, allowing anyone to
independently verify correctness.

The protocol relies on a majority-honest assumption: as long as more than 50% of coprocessors are honest, results are
valid. The Gateway aggregates responses and accepts outputs only when a majority consensus is reached.

To enforce honest behavior, coprocessors must stake $ZAMA tokens and are subject to slashing if caught
misbehaving—either through automated checks or governance-based fraud proofs.

This model ensures correctness through transparency, resilience through decentralization, and integrity through economic
incentives.

## Architecture & Scalability

The coprocessor architecture includes:

- Event listeners for host chains and the Gateway
- A task queue for FHE and ACL update jobs
- Worker threads that process tasks in parallel
- A public storage layer (e.g., S3) for ciphertext availability

This modular setup supports horizontal scaling: adding more workers or machines increases throughput. Symbolic
computation and delayed execution also ensure low gas costs on-chain .
