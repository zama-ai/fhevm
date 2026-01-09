# Gateway v2: High-Throughput Architecture

## Executive Summary

This document proposes a redesigned Gateway architecture that increases request
throughput to 200+ decryptions or input verifications per second by moving
response aggregation off-chain while maintaining trustless fallback paths.

**Key distinction**: Today's ~300/sec limit counts all Gateway transactions
(requests + responses mixed). In v2, responses move off-chain, so Gateway only
handles requests.

**Core Principle**: The Gateway chain becomes a minimal payment and request
registration layer. All heavy processing (consensus, aggregation) moves
off-chain to the Relayer, with final verification happening either in the SDK or
on the Host Chain.

---

## Table of Contents

1. [Problem Statement](#1-problem-statement)
2. [Design Principles](#2-design-principles)
3. [Unified Communication Model](#3-unified-communication-model)
4. [Architecture Overview](#4-architecture-overview)
5. [Component Specifications](#5-component-specifications)
6. [Workflow Specifications](#6-workflow-specifications)
7. [Payment Model](#7-payment-model)
8. [Dispute & Refund Protocol](#8-dispute--refund-protocol)
9. [Trust Model](#9-trust-model)
10. [Migration Path](#10-migration-path)
11. [Open Questions](#11-open-questions)

---

## 1. Problem Statement

### Current Bottleneck

The Gateway chain (Arbitrum L3 via Conduit) is bottlenecked by calldata
throughput:

| Metric                              | Current Value                                |
| ----------------------------------- | -------------------------------------------- |
| Gateway max throughput (minimal tx) | ~1,000 tx/sec                                |
| User decryption response size       | ~1.3 KB per KMS response                     |
| KMS responses per decryption        | 9 (for threshold 2t+1 with t=4)              |
| Total calldata per decryption       | ~11.7 KB                                     |
| **Effective decryption throughput** | **~330 responses/sec** - ~25 decryptions/sec |

### Root Cause

Each worker node (KMS or Coprocessor) posts its response as a separate
transaction to the Gateway chain. The Gateway then performs consensus
aggregation on-chain. This design creates a calldata bottleneck as usage scales.

### Target

| Metric                 | Target                                     |
| ---------------------- | ------------------------------------------ |
| Throughput             | 200 decryptions or input verifications/sec |
| Horizontal scalability | Add capacity without architectural changes |

---

## 2. Design Principles

### 2.1 Gateway as Payment Layer, Not Consensus Layer

The Gateway chain should:

- Collect payments
- Register requests (emit events for workers to observe)
- NOT aggregate responses
- NOT perform consensus

### 2.2 Two Paths: Hot and Cold

| Path          | Use Case                 | Trust                              | Performance            |
| ------------- | ------------------------ | ---------------------------------- | ---------------------- |
| **Hot Path**  | 99% of requests          | Trust Gateway/Relayer availability | Fast, cheap            |
| **Cold Path** | Fallback, paranoid users | Trustless (only worker threshold)  | Slower, more expensive |

### 2.3 Workers as Trust Anchors

Worker nodes (KMS and Coprocessors) are the security foundation. They must:

- Verify permissions independently (query Host Chain directly)
- Fetch data independently (from each other or Host Chain)
- Respond via API (not on-chain)

### 2.4 Final Verification at Point of Use

Signature verification happens at the point where results are used:

- **Host Chain contracts** for operations that affect on-chain state
- **SDK** for operations that only affect the user locally

---

## 3. Unified Communication Model

All workflows in Gateway v2 follow the same abstract pattern. This section
defines the common model before detailing specific workflows.

### 3.1 Abstract Request-Response Pattern

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                          UNIFIED COMMUNICATION MODEL                        │
│                                                                             │
│  Every workflow follows these 5 phases:                                     │
│                                                                             │
│  ┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐    ┌───────────┐  │
│  │ SUBMIT  │───>│ REGISTER│───>│ PROCESS │───>│ COLLECT │───>│  VERIFY   │  │
│  └─────────┘    └─────────┘    └─────────┘    └─────────┘    └───────────┘  │
│                                                                             │
│  User/SDK       Gateway        Workers        Relayer        Host Chain     │
│  submits to     registers      observe        polls          or SDK         │
│  Relayer        + payment      event,         worker         verifies       │
│                 + emit event   compute,       APIs,          signatures     │
│                               store result   aggregates                     │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 3.2 The Five Phases

#### Phase 1: SUBMIT

User submits request to Relayer via HTTP API.

| Aspect       | Details                                                      |
| ------------ | ------------------------------------------------------------ |
| **Actor**    | User / SDK                                                   |
| **Target**   | Relayer HTTP API                                             |
| **Contains** | Full request payload (handles, signatures, ciphertext, etc.) |
| **Trust**    | Relayer trusted for availability only                        |

#### Phase 2: REGISTER

Relayer registers request on Gateway, triggering payment and event emission.

| Aspect       | Details                                                           |
| ------------ | ----------------------------------------------------------------- |
| **Actor**    | Relayer                                                           |
| **Target**   | Gateway Chain                                                     |
| **Contains** | Commitment (hash) OR full request, depending on workflow and size |
| **Output**   | Event emitted for workers to observe                              |
| **Payment**  | Collected from user's account                                     |

#### Phase 3: PROCESS

Workers observe Gateway event, validate, compute, and store results locally.

| Aspect         | Details                                                    |
| -------------- | ---------------------------------------------------------- |
| **Actor**      | Workers (KMS or Coprocessors)                              |
| **Trigger**    | Gateway event observed                                     |
| **Validation** | Workers verify independently (ACL, commitment match, etc.) |
| **Output**     | Signed result stored locally, exposed via API              |
| **On-chain**   | **Nothing** - no response transactions                     |

#### Phase 4: COLLECT

Relayer polls worker APIs to collect signed responses.

| Aspect          | Details                                  |
| --------------- | ---------------------------------------- |
| **Actor**       | Relayer                                  |
| **Target**      | Worker APIs                              |
| **Method**      | Polling (GET /result/{requestId})        |
| **Request ID**  | Contract-assigned uint256 (from registry counter) |
| **Aggregation** | Relayer collects until threshold reached |
| **Output**      | Aggregated response returned to User/SDK |

#### Phase 5: VERIFY

Signatures verified at point of use.

| Aspect           | Details                              |
| ---------------- | ------------------------------------ |
| **Actor**        | Host Chain contract OR SDK           |
| **Input**        | Aggregated response with signatures  |
| **Verification** | Threshold of valid worker signatures |
| **Trust**        | Cryptographic - cannot be forged     |

### 3.3 Pattern Application by Workflow

| Workflow               | REGISTER contains | Workers      | VERIFY location            |
| ---------------------- | ----------------- | ------------ | -------------------------- |
| **Input Verification** | Commitment only   | Coprocessors | Host Chain (InputVerifier) |
| **User Decryption**    | Full request      | KMS Nodes    | SDK                        |
| **Public Decryption**  | Full request      | KMS Nodes    | Host Chain (KMSVerifier)   |

### 3.4 Why This Pattern Works

1. **Bottleneck eliminated**: Workers respond via API, not on-chain transactions
2. **Trust preserved**: Signatures verified cryptographically at point of use
3. **Gateway simplified**: Only payment + event emission, no consensus logic
4. **Horizontal scaling**: Add Relayer instances, workers process in parallel
5. **Trustless cold path**: Users can bypass Relayer, poll workers directly and
   pay their requests on Host Chain directly.

---

## 4. Architecture Overview

### 4.1 High-Level Architecture

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                              USER / DAPP                                     │
│                                                                             │
│                    ┌─────────────────────────────────┐                      │
│                    │     fhevm SDK / Frontend        │                      │
│                    │  • Submits requests to Relayer  │                      │
│                    │  • Verifies signatures (user decrypt)                  │
│                    └─────────────────────────────────┘                      │
└─────────────────────────────────────────────────────────────────────────────┘
                          │                    │
                          │ HOT PATH           │ COLD PATH
                          │ (default)          │ (fallback)
                          ▼                    ▼
┌──────────────────────────────────┐    ┌─────────────────────────────────────┐
│           RELAYER                │    │          HOST CHAIN                 │
│                                  │    │                                     │
│  • HTTP API for users            │    │  • Direct request submission        │
│  • Registers requests on Gateway │    │  • Payment in native token          │
│  • Sends payloads to workers     │    │  • User polls workers directly      │
│  • Polls worker APIs             │    │                                     │
│  • Aggregates responses          │    │                                     │
└──────────────────────────────────┘    └─────────────────────────────────────┘
          │                                        │
          ▼                                        │
┌─────────────────────────────────────────────────────────────────────────────┐
│                          GATEWAY CHAIN (Minimal Role)                       │
│                                                                             │
│  • Receives requests with payment                                           │
│  • Emits events for workers to observe                                      │
│  • Does NOT receive worker responses                                        │
│  • Does NOT perform consensus                                               │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ Workers listen to Gateway events
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              WORKERS                                        │
│                                                                             │
│  ┌─────────────────────────────┐    ┌─────────────────────────────────┐     │
│  │      KMS NODES (n=3t+1)     │    │     COPROCESSOR NODES           │     │
│  │                             │    │                                 │     │
│  │  • Listen to decrypt events │    │  • Listen to input verif events │     │
│  │  • Verify ACL on Host Chain │    │  • Verify ZKPoK                 │     │
│  │  • Fetch CT from Copros     │    │  • Store ciphertext in DA       │     │
│  │  • Compute decrypt share    │    │  • Sign attestation             │     │
│  │  • Expose via API           │    │  • Expose via API               │     │
│  └─────────────────────────────┘    └─────────────────────────────────┘     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ Final verification
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                             HOST CHAIN                                      │
│                                                                             │
│  • InputVerifier: validates Coprocessor signatures                          │
│  • KMSVerifier: validates KMS signatures (public decrypt)                   │
│  • ACL: source of truth for permissions                                     │
│  • DecryptionFallback: cold path request submission                         │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 4.2 What Moves Where (v1 → v2)

| Function                         | v1 Location        | v2 Location                         | Rationale                     |
| -------------------------------- | ------------------ | ----------------------------------- | ----------------------------- |
| **Input verification request**   | Gateway            | Gateway (payment + commitment only) | Reduced calldata              |
| **Input verification response**  | Gateway (on-chain) | Coprocessor API                     | Eliminate bottleneck          |
| **Input verification consensus** | Gateway contract   | Relayer (off-chain)                 | Eliminate bottleneck          |
| **Decryption request**           | Gateway            | Gateway (unchanged)                 | Still need event for KMS      |
| **Decryption response**          | Gateway (on-chain) | KMS API                             | Eliminate bottleneck          |
| **Decryption consensus**         | Gateway contract   | Relayer (off-chain)                 | Eliminate bottleneck          |
| **Ciphertext commits**           | Gateway            | **Eliminated**                      | KMS fetches from Coprocessors |
| **Multichain ACL**               | Gateway            | **Eliminated**                      | KMS queries Host Chain        |
| **Final verification**           | Gateway            | Host Chain or SDK                   | Trust at point of use         |

### 4.3 Gateway Contracts - Before and After

#### Eliminated Contracts/Functions

| Contract            | Function                     | Status         |
| ------------------- | ---------------------------- | -------------- |
| `InputVerification` | `verifyProofResponse()`      | **ELIMINATED** |
| `Decryption`        | `userDecryptionResponse()`   | **ELIMINATED** |
| `Decryption`        | `publicDecryptionResponse()` | **ELIMINATED** |
| `CiphertextCommits` | All                          | **ELIMINATED** |
| `MultichainACL`     | All                          | **ELIMINATED** |

#### Retained/Modified Contracts

| Contract                    | Function                      | Change                            |
| --------------------------- | ----------------------------- | --------------------------------- |
| `DecryptionRegistry`        | `requestUserDecryption()`     | Simplified - no response handling |
| `DecryptionRegistry`        | `requestPublicDecryption()`   | Simplified - no response handling |
| `InputVerificationRegistry` | `registerInputVerification()` | New - commitment + payment only   |
| `GatewayConfig`             | Configuration + worker apiUrl | Extended (see below)              |
| `ProtocolPayment`           | Fee collection                | Retained                          |

#### GatewayConfig Extension: Worker API Endpoints

GatewayConfig is extended with `apiUrl` fields for worker discovery:

```solidity
struct KmsNode {
    address txSenderAddress;
    address signerAddress;   // Identity - MUST match ProtocolConfig
    string ipAddress;
    string storageUrl;
    string apiUrl;           // NEW: HTTP API endpoint bound to signerAddress
}

struct Coprocessor {
    address txSenderAddress;
    address signerAddress;   // Identity - MUST match ProtocolConfig
    string s3BucketUrl;
    string apiUrl;           // NEW: HTTP API endpoint bound to signerAddress
}
```

**Dual-Source Model**:
- **ProtocolConfig (Ethereum)**: Source of truth for signer identities and threshold
- **GatewayConfig (Gateway Chain)**: Operational config (apiUrl, s3BucketUrl)

The `signerAddress` in GatewayConfig MUST match the corresponding entry in
ProtocolConfig. This binding ensures:
1. Relayer knows which signer key to expect from each endpoint
2. Misrouted/spoofed endpoints can be detected early
3. apiUrl can be updated on Gateway without Ethereum transaction

---

## 5. Component Specifications

### 5.1 KMS Node

#### Responsibilities

| Current (v1)                              | Proposed (v2)                                |
| ----------------------------------------- | -------------------------------------------- |
| Listen to Gateway decrypt events          | Listen to Gateway decrypt events (unchanged) |
| Verify via Gateway's MultichainACL        | Verify by querying Host Chain ACL directly   |
| Fetch CT from Gateway's CiphertextCommits | Fetch CT from Coprocessors directly          |
| Post response to Gateway chain            | Expose response via API                      |

#### Request Discovery

KMS nodes discover requests by listening to Gateway events:

- `DecryptionRequested` event from Gateway
- `DecryptionRequested` event from Host Chain (cold path)

#### API Endpoints

**GET /share/{requestId}**:

Returns the computed decryption share for a request.

```text
Response (ready):
{
  "status": "ready",
  "requestId": "uint256",         // Contract-assigned ID (not tx hash)
  "shareIndex": "uint256",
  "encryptedShare": "bytes",      // User decryption
  "decryptedValue": "bytes",      // Public decryption
  "epochId": "uint256",           // MPC context epoch for replay protection
  "signature": "bytes",
  "kmsNodeAddress": "address"
}

Response (pending):
{
  "status": "pending",
  "requestId": "uint256"
}

Response (rejected):
{
  "status": "rejected",
  "requestId": "uint256",
  "reason": "string"
}
```

**GET /health**:

Health check endpoint.

#### Processing Flow

```text
1. Observe DecryptionRequested event (Gateway or Host Chain)
2. Validate request signature
3. Verify ACL: query Host Chain ACL.isAllowed(handle, requester)
4. Fetch ciphertext: query Coprocessors, verify majority agreement
5. Compute decryption share
6. Re-encrypt under user's public key (user decryption only)
7. Sign: EIP-712(requestId, shareIndex, share)
8. Store locally with TTL
9. Expose via GET /share/{requestId}
```

#### Dynamic Node Addition

KMS node changes happen via **context switches** (see
[Section 5.6](#56-node-identity-management)):

1. Operators create new MPC context with updated node set
2. Key resharing runs between old and new node sets (if nodes changed)
3. New context activates after delay period
4. New requests use new context; in-flight requests complete with old context
5. Old context deactivates after grace period

### 5.2 Coprocessor Node

#### Responsibilities

| Current (v1)                         | Proposed (v2)                                    |
| ------------------------------------ | ------------------------------------------------ |
| Listen to Gateway input verif events | Listen to Gateway input verif events (unchanged) |
| Post response to Gateway chain       | Expose response via API                          |
| Store in CiphertextCommits           | Store in S3, expose via HTTP API for KMS         |

#### Request Discovery

Coprocessors discover requests by listening to:

- `InputVerificationRegistered` event from Gateway

#### API Endpoints

**POST /verify-input**:

Receives full input verification payload from Relayer.

```text
Request:
{
  "requestId": "uint256",
  "ciphertextWithZkpok": "bytes",
  "contractChainId": "uint256",
  "contractAddress": "address",
  "userAddress": "address"
}

Response (success):
{
  "status": "verified",
  "requestId": "uint256",
  "handles": ["bytes32"],
  "signature": "bytes",
  "coprocessorAddress": "address"
}

Response (failure):
{
  "status": "rejected",
  "requestId": "uint256",
  "reason": "string"
}
```

**GET /ciphertext/{handle}**:

Returns ciphertext material for KMS nodes.

```text
Response:
{
  "handle": "bytes32",
  "keyId": "uint256",
  "snsCiphertext": "bytes",
  "snsCiphertextDigest": "bytes32",
  "timestamp": "uint256",
  "signature": "bytes"
}
```

#### Processing Flow (Input Verification)

```text
1. Observe InputVerificationRegistered event (Gateway)
   - Extract: requestId, commitment
2. Receive HTTP request from Relayer (POST /verify-input)
   - Extract: full payload (ciphertext + ZKPoK)
3. Verify commitment: hash(payload) == commitment from event
4. If mismatch: reject (Relayer tampering detected)
5. Verify ZKPoK
6. Store ciphertext in S3
7. Derive handles
8. Sign attestation: EIP-712(requestId, handles)
9. Return signed handles to Relayer
```

### 5.3 Relayer

#### Responsibilities

| Current (v1)                           | Proposed (v2)                          |
| -------------------------------------- | -------------------------------------- |
| HTTP API for users                     | HTTP API for users (unchanged)         |
| Submit requests to Gateway             | Submit requests to Gateway (unchanged) |
| Listen to Gateway events for responses | Poll worker APIs for responses         |
| Forward aggregated responses           | Aggregate responses locally            |

#### Unified Request Handler

All workflows use the same abstract handler:

```text
async function handleRequest(workflow, userRequest):
    // Phase 1: SUBMIT (already done - we received HTTP request)

    // Phase 2: REGISTER
    requestId = await registerOnGateway(workflow, userRequest)

    // Phase 3: PROCESS (workers do this autonomously)
    // For input verification: also send payload to workers
    if workflow == INPUT_VERIFICATION:
        await broadcastToWorkers(workflow, requestId, userRequest.payload)

    // Phase 4: COLLECT
    responses = await pollWorkersUntilThreshold(workflow, requestId)

    // Phase 5: Return to user (VERIFY happens later)
    return aggregateResponses(responses)
```

#### Workflow-Specific Details

| Workflow           | REGISTER calldata         | Worker broadcast        | Workers      |
| ------------------ | ------------------------- | ----------------------- | ------------ |
| Input Verification | Commitment (~100 bytes)   | Yes (full payload)      | Coprocessors |
| User Decryption    | Full request (~500 bytes) | No (workers read event) | KMS          |
| Public Decryption  | Full request (~500 bytes) | No (workers read event) | KMS          |

### 5.4 Gateway Chain

#### Minimal Responsibilities

The Gateway chain is reduced to:

- Payment collection
- Request registration (emit events)
- Configuration storage

#### New Contract: InputVerificationRegistry

```solidity
interface IInputVerificationRegistry {
    event InputVerificationRegistered(
        uint256 indexed requestId,
        bytes32 commitment,          // hash(ciphertext + ZKPoK)
        address indexed userAddress,
        uint256 contractChainId,
        address contractAddress,
        bytes userSignature,         // EIP-712 signature binding userAddress
        uint256 timestamp
    );

    /// @notice Register an input verification request
    /// @param commitment Hash of the ciphertext + ZKPoK payload
    /// @param contractChainId Target host chain ID
    /// @param contractAddress Target contract address
    /// @param userAddress User who owns the input (NOT msg.sender, which is Relayer)
    /// @param userSignature EIP-712 signature from user binding (commitment, chainId, contract, userAddress)
    /// @dev userSignature prevents Relayer from registering with arbitrary userAddress
    function registerInputVerification(
        bytes32 commitment,
        uint256 contractChainId,
        address contractAddress,
        address userAddress,
        bytes calldata userSignature
    ) external payable returns (uint256 requestId);

    function getRequest(uint256 requestId) external view returns (
        bytes32 commitment,
        address userAddress,
        uint256 fee,
        uint256 timestamp
    );
}
```

**User Signature Binding**: The `userSignature` ensures the Relayer cannot register
requests with arbitrary `userAddress` values. Workers verify this signature to
confirm the user actually authorized the request. See
[DESIGN_CONFLICTS_RESOLUTION.md](docs/gateway-v2-implementation/DESIGN_CONFLICTS_RESOLUTION.md#2-inputverificationregistryuseraddress-critical-%EF%B8%8F-refined)
for EIP-712 typed data definition.

#### Modified Contract: DecryptionRegistry

```solidity
interface IDecryptionRegistry {
    /// @notice Emitted when a user requests decryption of ciphertext handles
    /// @dev handles.length MUST equal contractAddresses.length (parallel arrays)
    event UserDecryptionRequested(
        uint256 indexed requestId,
        bytes32[] handles,
        address[] contractAddresses,  // contractAddresses[i] is ACL source for handles[i]
        address indexed userAddress,
        bytes publicKey,
        bytes signature,              // EIP-712 signature from user
        uint256 chainId,
        uint256 timestamp             // block.timestamp at registration
    );

    /// @notice Emitted when a contract requests public decryption
    /// @dev handles.length MUST equal contractAddresses.length (parallel arrays)
    event PublicDecryptionRequested(
        uint256 indexed requestId,
        bytes32[] handles,
        address[] contractAddresses,  // ADDED: for per-handle ACL lookup
        uint256 chainId,
        uint256 timestamp             // block.timestamp at registration
    );

    /// @notice Request decryption for user (result re-encrypted to user's pubkey)
    /// @param handles Ciphertext handles to decrypt
    /// @param contractAddresses Contract address for each handle's ACL check
    /// @param publicKey User's public key for re-encryption
    /// @param signature EIP-712 signature from user authorizing this request
    /// @dev Requires handles.length == contractAddresses.length
    function requestUserDecryption(
        bytes32[] calldata handles,
        address[] calldata contractAddresses,
        bytes calldata publicKey,
        bytes calldata signature
    ) external payable returns (uint256 requestId);

    /// @notice Request public decryption (result returned as plaintext)
    /// @param handles Ciphertext handles to decrypt
    /// @param contractAddresses Contract address for each handle's ACL check
    /// @dev Requires handles.length == contractAddresses.length
    function requestPublicDecryption(
        bytes32[] calldata handles,
        address[] calldata contractAddresses
    ) external payable returns (uint256 requestId);
}
```

**Key Changes from v1**:
- No `*Response()` functions - workers respond via API only
- `extraData` removed - not needed in v2 (epochId is in worker responses)
- `contractAddresses` added to public decryption for per-handle ACL lookup
- `timestamp` set to `block.timestamp` (not user-supplied)

### 5.5 Host Chain

#### Existing Contracts (unchanged)

- `FHEVMExecutor`: Symbolic FHE execution
- `ACL`: Access control (source of truth)
- `InputVerifier`: Validates Coprocessor signatures when handles are used
- `KMSVerifier`: Validates KMS signatures for public decryption results

#### New Contract: DecryptionFallback (Cold Path)

```solidity
interface IDecryptionFallback {
    event DecryptionRequested(
        uint256 indexed requestId,
        bytes32[] handles,
        address indexed requester,
        bytes publicKey,
        uint256 timestamp
    );

    function requestDecryption(
        bytes32[] calldata handles,
        address[] calldata contractAddresses,
        bytes calldata publicKey,
        bytes calldata signature
    ) external payable returns (uint256 requestId);
}
```

### 5.6 Node Identity Management

#### Dual-Source Model

Worker configuration uses two sources with distinct responsibilities:

| Source | Location | Purpose | Contains |
| ------ | -------- | ------- | -------- |
| **ProtocolConfig** | Ethereum | Identity (ground truth) | Signer addresses, threshold, epochId |
| **GatewayConfig** | Gateway Chain | Operations | apiUrl, s3BucketUrl, bound to signerAddress |

**Why two sources?**
- ProtocolConfig updates require Ethereum transactions (expensive, slow)
- GatewayConfig updates are cheap (L3 transaction)
- API endpoints may change more frequently than identity
- Identity MUST be on Ethereum for cross-chain trust

#### Endpoint-to-Identity Binding

Each GatewayConfig entry MUST bind `apiUrl` to a specific `signerAddress`:

```solidity
struct KmsNode {
    address signerAddress;   // MUST match ProtocolConfig
    string apiUrl;           // Bound to this signerAddress
    // ...
}
```

This binding ensures:
1. Relayer knows which signer key to expect from each endpoint
2. Misrouted/spoofed endpoints are detected (signature won't match)
3. Multiple apiUrls per signer are allowed (for endpoint rotation)

#### Verification Flow

1. Relayer discovers workers from **GatewayConfig** (apiUrl + signerAddress)
2. Relayer polls worker APIs
3. Relayer verifies response signer matches expected signerAddress for that apiUrl
4. Relayer verifies signer is valid in **ProtocolConfig** (threshold check)

See
[tech-spec/architecture/config.md](https://github.com/zama-ai/tech-spec/blob/main/architecture/config.md)
and
[tech-spec/architecture/completed_intercomponent_flows/context.md](https://github.com/zama-ai/tech-spec/blob/main/architecture/completed_intercomponent_flows/context.md)
for details on context management and lifecycle.

#### How v2 Uses This

Gateway v2 does not change the Context system. The only difference is **where**
signature verification happens:

| Workflow           | v1 Verification | v2 Verification            |
| ------------------ | --------------- | -------------------------- |
| Input Verification | Gateway chain   | Host Chain (InputVerifier) |
| User Decryption    | Gateway chain   | SDK                        |
| Public Decryption  | Gateway chain   | Host Chain (KMSVerifier)   |

The SDK and Host Chain verifiers query ProtocolConfig (or synced copies) to get
valid signer addresses and thresholds.

---

## 6. Workflow Specifications

### 6.1 Input Verification

#### Hot Path Flow

```text
┌────────┐     ┌─────────┐     ┌─────────┐     ┌────────────┐     ┌───────────┐
│  User  │     │ Relayer │     │ Gateway │     │ Coprocessor│     │ Host Chain│
└───┬────┘     └────┬────┘     └────┬────┘     └─────┬──────┘     └─────┬─────┘
    │               │               │                │                  │
    │ 1. POST /input-proof          │                │                  │
    │   {ciphertext, ZKPoK}         │                │                  │
    │──────────────>│               │                │                  │
    │               │               │                │                  │
    │               │ 2. registerInputVerification   │                  │
    │               │   (commitment, fee)            │                  │
    │               │──────────────>│                │                  │
    │               │               │                │                  │
    │               │               │ 3. emit InputVerificationRegistered
    │               │               │───────────────>│                  │
    │               │               │                │                  │
    │               │ 4. POST /verify-input          │                  │
    │               │   {full payload}               │                  │
    │               │───────────────────────────────>│                  │
    │               │               │                │                  │
    │               │               │                │ 5. Verify:       │
    │               │               │                │  hash(payload)   │
    │               │               │                │  == commitment   │
    │               │               │                │                  │
    │               │               │                │ 6. Verify ZKPoK  │
    │               │               │                │    Store CT      │
    │               │               │                │    Sign handles  │
    │               │               │                │                  │
    │               │<──────────────────────────────│                  │
    │               │ 7. Return {handles, signature} │                  │
    │               │               │                │                  │
    │               │ (repeat for threshold)         │                  │
    │               │               │                │                  │
    │<──────────────│               │                │                  │
    │ 8. Return aggregated          │                │                  │
    │    {handles, signatures}      │                │                  │
    │               │               │                │                  │
    │ 9. User calls contract        │                │                  │
    │    with handles + proof       │                │                  │
    │─────────────────────────────────────────────────────────────────>│
    │               │               │                │                  │
    │               │               │                │    10. InputVerifier
    │               │               │                │    validates sigs│
    │               │               │                │                  │
```

#### Calldata Analysis

| v1                                    | v2                                    |
| ------------------------------------- | ------------------------------------- |
| Request: ~10KB (ciphertext + ZKPoK)   | Request: ~100 bytes (commitment only) |
| Response: ~500 bytes × N coprocessors | Response: 0 (via API)                 |
| **Total: ~15KB per input**            | **Total: ~100 bytes per input**       |

### 6.2 User Decryption

#### Hot Path Flow

```text
┌────────┐     ┌─────────┐     ┌─────────┐     ┌─────────┐     ┌────────────┐
│  User  │     │ Relayer │     │ Gateway │     │   KMS   │     │ Coprocessor│
│  +SDK  │     │         │     │         │     │         │     │            │
└───┬────┘     └────┬────┘     └────┬────┘     └────┬────┘     └─────┬──────┘
    │               │               │               │                 │
    │ 1. POST /user-decrypt         │               │                 │
    │   {handles, pubKey, sig}      │               │                 │
    │──────────────>│               │               │                 │
    │               │               │               │                 │
    │               │ 2. requestUserDecryption      │                 │
    │               │   (full request + fee)        │                 │
    │               │──────────────>│               │                 │
    │               │               │               │                 │
    │               │               │ 3. emit UserDecryptionRequested │
    │               │               │──────────────>│                 │
    │               │               │               │                 │
    │               │               │               │ 4. Verify ACL   │
    │               │               │               │   (Host Chain)  │
    │               │               │               │                 │
    │               │               │               │ 5. GET /ciphertext
    │               │               │               │────────────────>│
    │               │               │               │<────────────────│
    │               │               │               │                 │
    │               │               │               │ 6. Compute share│
    │               │               │               │    Re-encrypt   │
    │               │               │               │    Sign         │
    │               │               │               │                 │
    │               │ 7. GET /share/{requestId}     │                 │
    │               │──────────────────────────────>│                 │
    │               │<──────────────────────────────│                 │
    │               │               │               │                 │
    │               │ (repeat for threshold)        │                 │
    │               │               │               │                 │
    │<──────────────│               │               │                 │
    │ 8. Return {shares, signatures}│               │                 │
    │               │               │               │                 │
    │ 9. SDK verifies KMS signatures│               │                 │
    │    (against MPC Context)      │               │                 │
    │               │               │               │                 │
    │ 10. SDK decrypts shares       │               │                 │
    │     with user's private key   │               │                 │
    │               │               │               │                 │
```

#### Calldata Analysis

| v1                               | v2                                |
| -------------------------------- | --------------------------------- |
| Request: ~500 bytes              | Request: ~500 bytes (unchanged)   |
| Response: ~1.3KB × 9 KMS = ~12KB | Response: 0 (via API)             |
| **Total: ~12.5KB per decrypt**   | **Total: ~500 bytes per decrypt** |

### 6.3 Public Decryption

#### Hot Path Flow

Similar to User Decryption, except:

- KMS returns plaintext instead of re-encrypted share
- Final verification happens on Host Chain (KMSVerifier) when result is used
- SDK does NOT need to verify signatures (Host Chain does)

### 6.4 Cold Path (All Workflows)

For users who want to bypass Gateway and Relayer entirely:

```text
┌────────┐     ┌───────────┐     ┌─────────┐
│  User  │     │ Host Chain│     │ Workers │
└───┬────┘     └─────┬─────┘     └────┬────┘
    │                │                │
    │ 1. Submit request directly      │
    │   (pay in native token)         │
    │───────────────>│                │
    │                │                │
    │                │ 2. emit event  │
    │                │───────────────>│
    │                │                │
    │                │                │ 3. Process
    │                │                │    (same as hot path)
    │                │                │
    │ 4. Poll each worker directly    │
    │    GET /share/{requestId}       │
    │────────────────────────────────>│
    │<────────────────────────────────│
    │                │                │
    │ 5. Aggregate locally            │
    │                │                │
```

---

## 7. Payment Model

### 7.1 Overview

Apps (dApps) sponsor protocol fees on behalf of their users. The payment flow:

1. **App acquires ZAMA tokens** on Ethereum or another chain
2. **App bridges ZAMA to Gateway** chain
3. **App grants allowance** to a Relayer (via `approve` or `permit`)
4. **Relayer spends** from app's allowance when registering requests

### 7.2 Payment Flow

```text
┌─────────┐     ┌─────────┐     ┌─────────────┐     ┌──────────────────┐
│   App   │     │ Relayer │     │   Gateway   │     │ Request Contract │
└────┬────┘     └────┬────┘     └──────┬──────┘     └────────┬─────────┘
     │               │                 │                     │
     │ 1. approve(relayer, amount)     │                     │
     │─────────────────────────────────>                     │
     │               │                 │                     │
     │               │ 2. registerRequest(app, ...)          │
     │               │────────────────────────────────────────>
     │               │                 │                     │
     │               │                 │ 3. transferFrom(app, protocol, fee)
     │               │                 │<────────────────────│
     │               │                 │                     │
     │               │                 │ 4. Store escrow:    │
     │               │                 │    {requestId, payer: app, fee, timestamp}
     │               │                 │                     │
```

### 7.3 Key Points

| Aspect               | Details                                                            |
| -------------------- | ------------------------------------------------------------------ |
| **Payer**            | The app (dApp), not the end user                                   |
| **Token**            | ZAMA token bridged to Gateway                                      |
| **Spending**         | Relayer spends via allowance granted by app                        |
| **Escrow**           | Per-request state stored in request contract (not ProtocolPayment) |
| **Refund recipient** | The app that paid (see [Section 8](#8-dispute--refund-protocol))   |

### 7.4 Per-Request Escrow State

Each request contract (e.g., `DecryptionRegistry`, `InputVerificationRegistry`)
stores:

```solidity
struct RequestEscrow {
    address payer;      // App that sponsored this request
    uint256 fee;        // Amount escrowed
    uint256 timestamp;  // Block timestamp at registration
    RequestStatus status; // Registered, Disputed, Reimbursed
}

enum RequestStatus {
    Registered,
    Disputed,
    Reimbursed
}
```

---

## 8. Dispute & Refund Protocol

> **⚠️ DEFERRED TO V2.1**: The dispute/refund mechanism described in this section
> requires an on-chain fulfillment marker to be secure. Without it, users can
> claim refunds even after receiving valid off-chain responses. This section
> documents the intended design for V2.1; **V2.0 MVP will not include dispute/refund**.
> See [DESIGN_CONFLICTS_RESOLUTION.md](docs/gateway-v2-implementation/DESIGN_CONFLICTS_RESOLUTION.md#9-disputerefund-scope-deferred-%EF%B8%8F-changed)
> for detailed rationale.

### 8.1 Problem

What happens if workers never respond? Without on-chain fulfillment markers
(which would reintroduce the bottleneck), we need a dispute mechanism to refund
apps.

### 8.2 Design Choice: Dispute Protocol (not Fulfillment Markers)

**Rejected alternative**: Mark requests as "fulfilled" on-chain when workers
respond.

- This would require workers to post signatures on-chain, reintroducing the
  calldata bottleneck.

**Chosen approach**: Dispute protocol with timeout-based refunds.

- Hot path has no on-chain fulfillment markers
- Refunds available via dispute after timeout

### 8.3 Timeout Parameter T

Each request is eligible for dispute after `timestamp + T`, where:

- `timestamp` = `block.timestamp` when request was registered
- `T` = hardcoded timeout in the contract (e.g., 1 hour)

### 8.4 Dispute Flow (v1: Refund-Only)

```text
┌─────────┐     ┌──────────────────┐     ┌─────────┐
│ Anyone  │     │ Request Contract │     │   App   │
└────┬────┘     └────────┬─────────┘     └────┬────┘
     │                   │                    │
     │ 1. dispute(requestId)                  │
     │   (after timestamp + T)                │
     │   (pay small fee)                      │
     │──────────────────>│                    │
     │                   │                    │
     │                   │ 2. Verify: block.timestamp > request.timestamp + T
     │                   │    Verify: status == Registered
     │                   │                    │
     │                   │ 3. status = Disputed
     │                   │    Start resolution window (D blocks)
     │                   │                    │
     │                   │    ... D blocks pass with no resolution ...
     │                   │                    │
     │ 4. reimburse(requestId)                │
     │──────────────────>│                    │
     │                   │                    │
     │                   │ 5. transfer(app, fee)
     │                   │───────────────────>│
     │                   │                    │
     │                   │ 6. status = Reimbursed
     │                   │    Refund dispute fee to caller
     │                   │                    │
```

### 8.5 Dispute Resolution (v2: With Slashing - TBD)

In a future version, KMS nodes can contest disputes by proving they served the
request:

1. **KMS nodes check peers**: Query other KMS nodes via the same API endpoint
   (`GET /share/{requestId}`) that relayers use
2. **Consensus on non-participants**: Threshold of KMS nodes post on-chain which
   nodes did NOT serve the request
3. **Slashing**: Non-participating nodes are slashed (mechanism TBD)
4. **Dispute rejected**: If threshold of nodes served, dispute fails and
   disputer loses fee

**Note**: Slashing mechanism details (stake location, slash amount, enforcement)
are out of scope for v1.

### 8.6 Contract Interface

```solidity
interface IRequestDispute {
    event DisputeOpened(uint256 indexed requestId, address disputer, uint256 timestamp);
    event RequestReimbursed(uint256 indexed requestId, address payer, uint256 amount);

    /// @notice Open a dispute for an unfulfilled request
    /// @dev Requires block.timestamp > request.timestamp + T
    /// @param requestId The request to dispute
    function dispute(uint256 requestId) external payable;

    /// @notice Reimburse the payer after dispute resolution window
    /// @dev Requires status == Disputed and resolution window passed
    /// @param requestId The request to reimburse
    function reimburse(uint256 requestId) external;

    /// @notice Timeout parameter (hardcoded)
    function DISPUTE_TIMEOUT() external view returns (uint256);

    /// @notice Resolution window after dispute opened
    function RESOLUTION_WINDOW() external view returns (uint256);
}
```

### 8.7 Spam Protection

| Measure         | Details                                                                    |
| --------------- | -------------------------------------------------------------------------- |
| **Dispute fee** | Small fee required to open dispute                                         |
| **Fee refund**  | Fee refunded if dispute is valid (reimbursement happens)                   |
| **Fee burned**  | Fee burned/kept if dispute is invalid (v2: when workers prove they served) |

---

## 9. Trust Model

### 9.1 Unified Trust Model

The same trust model applies to all workflows:

| Layer                         | Trust Assumption          | Violation Impact      |
| ----------------------------- | ------------------------- | --------------------- |
| **Workers (KMS/Coprocessor)** | Threshold honest          | Core security breach  |
| **Relayer**                   | Availability only         | Use cold path         |
| **Gateway**                   | Availability for hot path | Use cold path         |
| **SDK**                       | Correct implementation    | User's responsibility |
| **Host Chain**                | Standard blockchain       | Foundation assumption |

### 9.2 Signature Verification by Workflow

| Workflow           | Verifier      | Location    | When                     |
| ------------------ | ------------- | ----------- | ------------------------ |
| Input Verification | InputVerifier | Host Chain  | When handles are used    |
| User Decryption    | SDK           | Client-side | Before decrypting shares |
| Public Decryption  | KMSVerifier   | Host Chain  | When result is used      |

### 9.3 What Relayer Can and Cannot Do

| Can Do                       | Cannot Do                       |
| ---------------------------- | ------------------------------- |
| Delay/drop requests (DoS)    | Forge worker signatures         |
| Return incomplete responses  | Modify signed data              |
| See request metadata         | Access plaintext (user decrypt) |
| Charge service fees          | Bypass ACL checks               |
| **Censor requests**          | Prevent cold path usage         |
| Withhold responses from user | Prevent user from self-relaying |

### 9.4 Relayer Censorship Risk

**Important**: The Relayer can censor requests by refusing to register them on
Gateway or by withholding worker responses from users.

**Mitigations**:

| Risk                                | Mitigation                                                         |
| ----------------------------------- | ------------------------------------------------------------------ |
| Relayer refuses to register request | App uses cold path (submit directly to Host Chain)                 |
| Relayer withholds worker responses  | App polls workers directly using API endpoints from ProtocolConfig |
| Relayer is unavailable              | App runs its own Relayer instance                                  |

**Recommendation for apps requiring liveness guarantees**: Run your own Relayer
instance or implement direct worker polling as fallback. The protocol is
designed to allow this — Relayer is a convenience layer, not a trust
requirement.

### 9.5 Commitment Integrity (Input Verification)

For Input Verification, the Relayer sends the full payload directly to
Coprocessors. To prevent tampering:

1. Relayer posts `commitment = hash(payload)` to Gateway
2. Gateway emits event with commitment
3. Coprocessors observe event, receive payload from Relayer
4. Coprocessors verify `hash(received_payload) == commitment`
5. If mismatch → reject (Relayer tampering detected)

This ensures all Coprocessors process the same input.

### 9.6 Worker Request Sourcing

**Workers only process requests that originate from on-chain events.**

Workers do NOT accept direct API requests to initiate new work. The flow is
always:

1. Request registered on-chain (Gateway or Host Chain)
2. Event emitted
3. Worker observes event and begins processing
4. Worker exposes result via API

This ensures:

- All requests are paid for
- Request ordering is deterministic
- Audit trail exists on-chain

---

## 10. Migration Path

### Phase 1: Parallel Infrastructure

- Update ProtocolConfig contexts to include API endpoints for workers
- Add API endpoints to KMS nodes (expose /share/{requestId})
- Add API endpoints to Coprocessors (expose /verify-input, /ciphertext/{handle})
- Deploy new Gateway contracts (InputVerificationRegistry, DecryptionRegistry)
- Deploy DecryptionFallback on Host Chains

### Phase 2: Relayer Update

- Update Relayer to use new Gateway contracts
- Update Relayer to poll worker APIs
- Implement local aggregation logic
- Run in parallel with v1 Relayer

### Phase 3: Deprecate v1

- Stop workers from posting responses on-chain
- Deprecate old Gateway contracts
- Remove CiphertextCommits, MultichainACL

### Phase 4: Optimization (Ongoing)

- Implement caching
- Add batch operations
- Optimize polling strategies

---

## 11. Open Questions

### 12.1 Polling vs Push

**Question**: Should Relayer poll workers, or should workers push responses?

**Options**:

- Polling (current design): Simple, Relayer controls timing
- WebSocket push: Lower latency, more complex
- Hybrid: Poll with webhook notification

**Recommendation**: Start with polling, optimize to push later if needed

### 12.2 Cold Path Payment

**Question**: What token for cold path payments on Host Chain?

**Options**:

- Native token (ETH) with price oracle
- Bridged ZAMA token
- Stablecoin

**Recommendation**: Defer, abstract payment interface

### 12.3 Request Expiry

**Question**: How long should workers retain computed responses?

**Options**:

- Fixed TTL (e.g., 1 hour)
- Until explicitly cleared
- Based on Gateway event finality

**Recommendation**: Fixed TTL of 1 hour, configurable

### 12.4 Batch Operations

**Question**: Should we support batching multiple operations in one request?

**Benefit**: Amortize Gateway transaction costs **Complexity**: More complex
aggregation logic

**Recommendation**: Design for it, implement in Phase 4

---

## Appendix A: SDK Requirements

### A.1 Mandatory Signature Verification (User Decryption)

The SDK MUST verify KMS signatures before using decryption results:

```javascript
function verifyUserDecryptionResponse(response, requestParams):
    // 1. Fetch active AND previous MPC context (for epoch grace period)
    currentContext = fetchActiveMpcContext()
    previousContext = fetchPreviousMpcContext()  // May be null
    
    // 2. Verify all responses have same epochId (single-epoch threshold invariant)
    epochIds = new Set(response.shares.map(s => s.epochId))
    if epochIds.size > 1:
        return ERROR("mixed epochs in responses - retry")
    
    responseEpochId = response.shares[0].epochId
    
    // 3. Select context based on epochId
    let context, kmsNodes, threshold
    if responseEpochId == currentContext.epochId:
        context = currentContext
    else if previousContext && responseEpochId == previousContext.epochId:
        // Accept previous epoch during grace period
        if !isGracePeriodActive():
            return ERROR("previous epoch signatures expired")
        context = previousContext
    else:
        return ERROR("unknown epochId")
    
    kmsNodes = context.nodes.map(n => n.evmAddress)
    threshold = context.threshold

    // 4. Verify each signature (includes epochId in typed data)
    validShares = []
    for share in response.shares:
        typedDataHash = EIP712Hash(
            domain: {name: "FHEVM", chainId: requestParams.chainId},
            message: {requestId, shareIndex, encryptedShare, epochId: share.epochId}
        )
        signer = ecrecover(typedDataHash, share.signature)

        if signer in kmsNodes and signer not in validShares.signers:
            validShares.append(share)

    // 5. Verify threshold
    if validShares.length < threshold:
        return ERROR("insufficient valid signatures")

    return SUCCESS(validShares)
```

### A.2 Epoch Handling During Context Switches

During MPC context switches (key rotation, node changes), both old and new epochs
are valid for a grace period:

```
┌─────────────────┐  defineNewContext()  ┌───────────────────────┐
│  EPOCH_N_ACTIVE │─────────────────────>│   GRACE PERIOD        │
│                 │                      │                       │
│ signers: [A,B]  │                      │ current: [C,D] epoch=N+1
│ epoch: N        │                      │ previous: [A,B] epoch=N
└─────────────────┘                      │ BOTH VALID            │
                                         └───────────┬───────────┘
                                                     │ grace expires
                                                     ▼
                                         ┌───────────────────────┐
                                         │  EPOCH_N+1_ACTIVE     │
                                         │                       │
                                         │ signers: [C,D]        │
                                         │ previous: cleared     │
                                         └───────────────────────┘
```

**Critical Invariant**: All shares in an aggregated response MUST have the same
`epochId`. The Relayer MUST NOT mix shares from different epochs.

### A.2 No Verification Required (Input Verification, Public Decryption)

For these workflows, signature verification happens on-chain:

- **Input Verification**: InputVerifier on Host Chain
- **Public Decryption**: KMSVerifier on Host Chain

The SDK can trust the Relayer's response, as invalid signatures would fail
on-chain verification when used.

---

## Appendix B: Glossary

| Term           | Definition                                                              |
| -------------- | ----------------------------------------------------------------------- |
| **Hot Path**   | Optimized path through Gateway and Relayer                              |
| **Cold Path**  | Trustless fallback through Host Chain directly                          |
| **Worker**     | Generic term for KMS Node or Coprocessor (the FHE infrastructure nodes) |
| **Commitment** | Hash of data, posted on-chain for integrity verification                |
| **Threshold**  | Minimum signatures required (2t+1 for n=3t+1 nodes)                     |
| **Share**      | Partial decryption result from one KMS node                             |

---

_Document Version: 2.1_ _Last Updated: January 2026_

_Changes in 2.1 (Oracle Review):_

- _Request ID changed from tx hash to contract-assigned uint256_
- _Added userAddress + userSignature to InputVerificationRegistry_
- _Added contractAddresses to PublicDecryptionRequested event_
- _Removed extraData from DecryptionRegistry functions_
- _Added epochId to all worker API responses_
- _Added SDK epoch/grace period handling (Appendix A.2)_
- _Added dual-source model (ProtocolConfig + GatewayConfig) with endpoint binding_
- _Deferred dispute/refund to V2.1 (requires on-chain fulfillment marker)_

_Changes in 2.0:_

- _Unified communication model across all workflows_
- _Input Verification moved off-chain (commitment only on Gateway)_
- _Eliminated CiphertextCommits, MultichainACL contracts_
- _Added dynamic KMS node addition_
- _Clarified cold path for all workflows_
