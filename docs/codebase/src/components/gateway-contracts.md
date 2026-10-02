# Gateway Contracts 🔥

**Location**: `/gateway-contracts/`
**Status**: Active Development
**Purpose**: Bridge between on-chain smart contracts and off-chain compute infrastructure

## Overview

The Gateway contracts serve as the coordination layer that manages communication between the blockchain and off-chain services. They handle ciphertext commitments, decryption requests, input verification, and access control across multiple chains.

## Key Contracts

| Contract | Purpose |
|----------|---------|
| `GatewayConfig.sol` | Central registry for KMS nodes, coprocessors, and protocol metadata |
| `Decryption.sol` | Manages public and user decryption requests with EIP712 signature validation |
| `MultichainACL.sol` | Access control for cross-chain operations and user delegations |
| `CiphertextCommits.sol` | Stores ciphertext material commitments from coprocessors |
| `InputVerification.sol` | Verifies encrypted user inputs via ZK proofs |
| `KMSGeneration.sol` | Orchestrates key generation and reshare operations |
| `ProtocolPayment.sol` | Handles protocol fee collection and distribution |

## Key Files

- `contracts/GatewayConfig.sol` - Gateway registry and configuration
- `contracts/Decryption.sol` - Decryption request handling
- `contracts/shared/Structs.sol` - Core data structures (KmsNode, Coprocessor, etc.)

## Relationships

Gateway contracts receive events from Host contracts and coordinate with the Coprocessor and KMS Connector to process FHE operations. They maintain consensus through threshold signatures from multiple coprocessors.

## Recent Development Focus (Dec 2025)

- Payment protocol implementation (`ProtocolPayment` contract)
- Multi-sig contracts based on Safe Smart Account
- LayerZero cross-chain integration for testnet/mainnet
- Monitoring events and request ID validation

## Areas for Deeper Documentation

### Gateway Consensus Mechanism

The Gateway implements a **threshold-based consensus system** for ciphertext commits, ensuring Byzantine-fault-tolerant agreement among multiple independent coprocessors on FHE computation results.

**Architecture** (`contracts/CiphertextCommits.sol`):

The consensus mechanism revolves around the `addCiphertextMaterial()` function (lines 121-189), which aggregates votes from coprocessors. Each coprocessor submits ciphertext metadata including:
- `ctHandle` - unique ciphertext identifier
- `keyId` - encryption key reference
- `ciphertextDigest` - hash of the encrypted data
- `snsCiphertextDigest` - hash of the Switch-and-Squash transformed version

**Vote Aggregation Process**:

1. **Hash-Based Voting**: All parameters are hashed together to create a unique `addCiphertextHash`. Coprocessors can only reach consensus if they provide identical data for all parameters.

2. **Deduplication**: The mapping `alreadyAddedCoprocessorTxSenders[ctHandle][msg.sender]` prevents any coprocessor from voting twice on the same ciphertext handle.

3. **Counter Increment**: Each vote increments `addCiphertextHashCounters[addCiphertextHash]`, tracking how many coprocessors agree on this exact data combination.

4. **Threshold Check**: Consensus is reached when `counter >= GATEWAY_CONFIG.getCoprocessorMajorityThreshold()`. This threshold is configured in `contracts/GatewayConfig.sol` (lines 790-805) and must be ≥1 and ≤ total coprocessors.

**Consensus Finalization**:

When the threshold is reached, the contract:
- Permanently stores the ciphertext digests and keyId
- Marks the handle as committed via `isCiphertextMaterialAdded[ctHandle] = true`
- Records the consensus hash for audit trails
- Emits `AddCiphertextMaterialConsensus` event with the list of participating coprocessors

**Byzantine Fault Tolerance**:

The system handles malicious or faulty coprocessors through data isolation. If Coprocessor A submits different data than Coprocessors B and C, A's vote creates a different hash and counts separately. Consensus is reached on the data agreed upon by the threshold majority, effectively excluding dishonest participants from the final commitment.

**Late Arrivals**: Coprocessors that respond after consensus has been reached have their votes recorded but don't re-trigger consensus events. This ensures all valid responses are acknowledged without duplicate processing.

**Data Retrieval**: The `getCiphertextMaterials()` function (lines 202-234) returns committed ciphertext metadata along with the complete list of coprocessors that participated in consensus, enabling full auditability of the agreement process.

### Multichain ACL Flow

The `MultichainACL` contract (`contracts/MultichainACL.sol`) provides a sophisticated cross-chain access control system for user decryption delegations, allowing users on one chain to delegate decryption rights to others across multiple host chains.

**Delegation Model**:

Delegations are stored in a four-dimensional mapping indexed by `(chainId, delegator, delegate, contractAddress)`, with each delegation containing:
- `expirationDate` (uint64) - UNIX timestamp when delegation expires
- `delegationCounter` (uint64) - monotonically increasing counter for ordering

**Delegation Lifecycle** (lines 233-310):

1. **Initiation**: Coprocessors call `delegateUserDecryption()` with delegation parameters including the target chain ID, delegator, delegate, contract address, counter, and expiration date.

2. **Hash-Based Agreement**: All parameters (including the expiration date) are hashed together using `DELEGATE_USER_DECRYPTION_DOMAIN_SEPARATOR_HASH`. This ensures coprocessors can only reach consensus if they agree on the complete delegation specification.

3. **Consensus Accumulation**: Each coprocessor's vote is recorded via `alreadyDelegatedUserDecryptionCoprocessors[hash][msg.sender]`, preventing double-voting. The system tracks participating coprocessors in `delegateUserDecryptionTxSenders[hash]`.

4. **Finalization**: When the threshold is reached (via `GATEWAY_CONFIG.getCoprocessorMajorityThreshold()`), the delegation is finalized only if its counter is strictly greater than the previous counter for that (delegator, delegate, contract) triple. This prevents replay attacks and enforces chronological ordering.

5. **Event Emission**: The `DelegateUserDecryptionConsensus` event is emitted with both old and new expiration dates, enabling off-chain systems to track delegation state changes.

**Cross-Chain Synchronization**:

The system doesn't rely on cross-chain messaging. Instead, each host chain's ACL state is mirrored in the Gateway's storage, with coprocessors acting as oracles that monitor host chain events and report them to the Gateway. The consensus mechanism ensures that multiple independent coprocessors agree on the chain state before it's accepted.

**Revocation Flow** (lines 313-389):

Revocations follow a similar pattern but set `expirationDate = 0`. Critically, a coprocessor cannot vote for both delegation and revocation of the same hash - the contract enforces mutual exclusivity via `_checkAlreadyDelegatedOrRevokedUserDecryptionDelegation()` (lines 580-621).

**Counter-Based Ordering**:

The monotonic counter requirement solves several problems:
- **Out-of-order prevention**: Even if network delays cause operations to arrive out of sequence, only higher counters are accepted
- **Revocation protection**: An old delegation can't override a newer revocation
- **Replay prevention**: Previously used counters can't be reused

**State Queries**:

- `isUserDecryptionDelegated()` checks if a delegation is currently active (counter exists and expiration > block.timestamp)
- `getDelegateUserDecryptionConsensusTxSenders()` returns the list of coprocessors that participated in a specific delegation consensus

**Integration with Decryption**:

When processing user decryption requests, the `Decryption` contract uses `MultichainACLChecks._checkIsUserDecryptionDelegated()` to verify that the delegate has valid permission from the delegator for the specific contract context, respecting per-chain delegation boundaries.

### Payment Protocol Design

The `ProtocolPayment` contract (`contracts/ProtocolPayment.sol`) implements a modular fee collection system for Gateway operations, denominated in the $ZAMA token (ERC20 with 18 decimals).

**Fee Structure**:

Three distinct fee types are managed independently:
- **Input Verification Fee** - Charged when users submit encrypted inputs via `InputVerification.sol`
- **Public Decryption Fee** - Charged when contracts request public decryption via `Decryption.sol`
- **User Decryption Fee** - Charged when users request private reencryption via `Decryption.sol`

Each fee is stored in the `ProtocolPaymentStorage` structure (ERC-7201 pattern) and can be updated independently by the Gateway Owner via `setInputVerificationPrice()`, `setPublicDecryptionPrice()`, and `setUserDecryptionPrice()`.

**Collection Mechanism**:

The protocol follows a caller-pays model. When a user or contract initiates an operation:

1. The `InputVerification` or `Decryption` contract calls the appropriate fee collection method (`collectInputVerificationFee()`, `collectPublicDecryptionFee()`, or `collectUserDecryptionFee()`)

2. Authorization is enforced via `onlyInputVerificationContract` or `onlyDecryptionContract` modifiers, ensuring only legitimate protocol contracts can trigger fee collection

3. The contract uses ERC20 `transferFrom()` to move $ZAMA tokens from the transaction sender to the `FeesSenderToBurner` address

4. If the transfer fails (insufficient balance or allowance), the entire operation reverts

**Integration Pattern**:

Gateway contracts inherit from `ProtocolPaymentUtils` (`contracts/shared/ProtocolPaymentUtils.sol`), which provides convenience wrappers like `_collectInputVerificationFee(address txSender)`. This abstraction allows easy integration without tight coupling to the payment implementation.

**Operator Compensation**:

The current design transfers fees to `FeesSenderToBurner`, which acts as an aggregation point. The actual distribution to operators (coprocessors, KMS nodes, relay services) happens through an external system not directly managed by these contracts. This separation allows for flexible compensation models without requiring protocol upgrades.

**Upgradeability**:

The contract uses the UUPS (Universal Upgradeable Proxy Standard) pattern, allowing the Gateway Owner to upgrade pricing logic, add new fee types, or modify the distribution mechanism without disrupting existing operations. Version tracking (currently v0.1.0) ensures compatibility during upgrades.

### KMS Coordination

The Gateway orchestrates cryptographic key lifecycle management through the `KMSGeneration` contract (`contracts/KMSGeneration.sol`), implementing a consensus-based protocol for key generation and activation.

**KMS Node Structure** (from `contracts/shared/Structs.sol`):

Each registered KMS node has:
- `txSenderAddress` - Transaction submission address
- `signerAddress` - EIP712 signature address
- `ipAddress` - Network location
- `storageUrl` - Key material storage endpoint

**Two-Phase Key Generation Protocol**:

The protocol separates key preprocessing from actual key generation, enabling efficient multi-party computation:

**Phase 1: Preprocessing Keygen**
1. Gateway Owner calls `keygen(paramsType)` specifying FHE parameters (Default or Test)
2. Contract generates a unique `prepKeygenId` and paired `keyId` using counters with distinct prefixes (0x03... and 0x04...)
3. `PrepKeygenRequest` event is emitted
4. KMS nodes sign the `prepKeygenId` with EIP712 (`PrepKeygenVerification` typed data)
5. Each node calls `prepKeygenResponse(prepKeygenId, signature)` with their signature
6. Signatures are validated using ECDSA recovery, checking both that the signer is authorized (`isKmsSigner()`) and matches the transaction sender
7. When consensus threshold is reached (via `GATEWAY_CONFIG.getKmsGenThreshold()`), `KeygenRequest` event is emitted

**Phase 2: Actual Keygen**
1. KMS nodes perform off-chain MPC to generate key material
2. Each node signs the `keyId` and key digests with EIP712 (`KeygenVerification` typed data)
3. Key digests include both server keys (type 0) and public keys (type 1)
4. Nodes call `keygenResponse(keyId, keyDigests, signature)`
5. After consensus, the contract stores key digests and emits `ActivateKey` with storage URLs from all participating KMS nodes
6. The key becomes the active key via `setActiveKeyId(keyId)`

**CRS Generation**:

Common Reference String (CRS) generation follows a similar consensus pattern:
1. Gateway Owner calls `crsgenRequest(maxBitLength, paramsType)`
2. KMS nodes respond with `crsgenResponse(crsId, crsDigest, signature)`
3. Upon consensus, `ActivateCrs` is emitted with storage URLs and digest
4. CRS is activated via `setActiveCrsId(crsId)`

**Request ID Uniqueness**:

The system uses counter bases defined in `contracts/shared/KMSRequestCounters.sol`:
- `0x0300...` for prep keygen
- `0x0400...` for keygen
- `0x0500...` for CRS generation
- `0x0600...` for key reshare

This ensures globally unique request IDs across all operation types.

**Consensus Storage**:

The `KMSGenerationStorage` structure tracks:
- `kmsHasSignedForResponse[requestId][kmsSigner]` - Prevents double-signing
- `isRequestDone[requestId]` - Marks completed requests
- `consensusTxSenderAddresses[requestId][digest]` - Lists participants for each digest
- `consensusDigest[requestId]` - Final agreed-upon digest
- `keygenIdPairs[id]` - Bidirectional mapping between prepKeygenId ↔ keyId
- `activeKeyId` and `activeCrsId` - Currently active cryptographic material

**Integration with Decryption**:

When the `Decryption` contract emits `PublicDecryptionRequest` or `UserDecryptionRequest` events, these reference the `activeKeyId`. KMS nodes:
1. Query `getActiveKeyId()` to identify which key to use
2. Call `getKeyMaterials(keyId)` to retrieve storage URLs and digests
3. Fetch key material from their storage endpoints
4. Perform decryption operations off-chain
5. Submit results back to the Decryption contract with signatures

**Storage URL Coordination**:

When consensus is reached, the contract queries `GATEWAY_CONFIG.getKmsNode(address)` for each participating KMS transaction sender and aggregates their storage URLs. This ensures clients can retrieve key material from multiple redundant sources, improving availability and fault tolerance.

**EIP712 Security**:

All KMS responses use typed structured data signatures (EIP712), providing:
- Human-readable signature content for hardware wallet users
- Type safety preventing signature malleability
- Domain separation preventing cross-contract replay attacks
- Clear accountability for each KMS node's responses

---

**Related:**
- [Host Contracts](host-contracts.md) - Emit events that Gateway processes
- [Coprocessor](coprocessor.md) - Processes Gateway-coordinated FHE operations
- [KMS Connector](kms-connector.md) - Handles Gateway key management requests
