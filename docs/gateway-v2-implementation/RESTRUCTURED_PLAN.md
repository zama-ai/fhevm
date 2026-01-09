# Gateway V2: Component-First Implementation Plan

**Objective**: Restructure the Gateway V2 migration into atomic, component-scoped phases that are independently testable.

**Key Principles**:
1. **Delete legacy first** — Force explicit breakage rather than silent misuse
2. **Component isolation** — Each phase touches ONE component/codebase
3. **Tests bundled with implementation** — No separate "testing phase"
4. **Deprecation embedded** — Remove V1 code as part of component updates

**Related Documents**:
- [WORKER_API_SPEC.md](./WORKER_API_SPEC.md) — Detailed HTTP API specifications for KMS and Coprocessor
- [E2E_CLI_PLAN.md](./E2E_CLI_PLAN.md) — E2E testing CLI plan with caching and debugging optimizations
- [GATEWAY_V2_DESIGN.md](../../GATEWAY_V2_DESIGN.md) — Original design specification
- [DESIGN_CONFLICTS_RESOLUTION.md](./DESIGN_CONFLICTS_RESOLUTION.md) — Resolution of design/plan inconsistencies (includes EIP-712 typed data definitions)

**Last Updated**: January 2026 (Oracle Review Applied, Codebase State Review Applied)

---

## Plan Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           GATEWAY V2 MIGRATION                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  PHASE 1: CONTRACTS                                                          │
│  ├── 1A: Gateway Contracts (V2 + delete V1)                                 │
│  └── 1B: Host Contracts (KMSVerifierV2 only)                                │
│                                                                              │
│  PHASE 2: COPROCESSOR                                                        │
│  └── API server + V2 events + remove V1 tx-sender logic                     │
│                                                                              │
│  PHASE 3: KMS CONNECTOR                                                      │
│  └── API server + V2 events + direct ACL + Coprocessor API fetch            │
│                                                                              │
│  PHASE 4: RELAYER                                                            │
│  └── V2 contracts + worker polling + response aggregation                   │
│                                                                              │
│  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─  │
│                                                                              │
│  PHASE 5: COLD PATH (OUT OF SCOPE)                                           │
│  └── DecryptionFallback contract + KMS Host listener + SDK polling          │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Current State vs Target State

### System Architecture

```
CURRENT (V1)                                    TARGET (V2)
────────────────────────────────────────────────────────────────────────────────

User/SDK                                        User/SDK
    │                                               │
    ▼                                               ▼
┌─────────┐                                    ┌─────────┐
│ Relayer │                                    │ Relayer │
└────┬────┘                                    └────┬────┘
     │                                              │
     │ ① Full ciphertext+ZKPoK                     │ ① Commitment only
     │   on-chain (~15KB)                          │   on-chain (~100B)
     ▼                                              ▼
┌──────────────────────┐                       ┌──────────────────────┐
│    GATEWAY CHAIN     │                       │    GATEWAY CHAIN     │
│                      │                       │                      │
│ • InputVerification  │                       │ • InputVerification- │
│   (full payload)     │                       │   Registry (commit)  │
│ • Decryption         │                       │ • DecryptionRegistry │
│   (with responses)   │                       │   (no responses)     │
│ • CiphertextCommits  │ ──── DELETED ────────>│                      │
│ • MultichainACL      │ ──── DELETED ────────>│                      │
└──────────┬───────────┘                       └──────────┬───────────┘
           │                                              │
     ② Workers post                                 ② Workers expose
       responses on-chain                             results via API
           │                                              │
           ▼                                              ▼
┌──────────────────────┐                       ┌──────────────────────┐
│      WORKERS         │                       │      WORKERS         │
│                      │                       │                      │
│ Coprocessor:         │                       │ Coprocessor:         │
│ • tx-sender → GW     │ ──── REMOVED ────────>│ • HTTP API server    │
│ • S3 for ciphertexts │                       │ • S3 for ciphertexts │
│                      │                       │   (exposed via API)  │
│                      │                       │                      │
│ KMS Connector:       │                       │ KMS Connector:       │
│ • tx-sender → GW     │ ──── REMOVED ────────>│ • HTTP API server    │
│ • Query MultichainACL│ ──── REPLACED ───────>│ • Direct Host ACL    │
│ • Fetch from S3/     │ ──── REPLACED ───────>│ • Fetch from Copro   │
│   CiphertextCommits  │                       │   HTTP API           │
└──────────────────────┘                       └──────────────────────┘
           │                                              │
     ③ Relayer listens                            ③ Relayer polls
       to GW events                                  worker APIs
           │                                              │
           ▼                                              ▼
┌──────────────────────┐                       ┌──────────────────────┐
│    HOST CHAIN        │                       │    HOST CHAIN        │
│                      │                       │                      │
│ • KMSVerifier        │ ──── UPGRADED ───────>│ • KMSVerifierV2      │
│   (no grace period)  │                       │   (epoch grace)      │
│ • InputVerifier      │                       │ • InputVerifier      │
│   (unchanged)        │                       │   (unchanged)        │
└──────────────────────┘                       └──────────────────────┘
```

---

# PHASE 1: CONTRACTS

## Phase 1A: Gateway Contracts

**Duration**: 1-2 weeks
**Codebase**: `gateway-contracts/`
**Outcome**: V2 contracts deployed, V1 contracts deleted, bindings regenerated

### Before → After

```
gateway-contracts/contracts/
────────────────────────────────────────────────────────────────────────────────

BEFORE                                          AFTER
├── InputVerification.sol      ─── DELETE ────> (removed)
├── Decryption.sol             ─── DELETE ────> (removed)
├── CiphertextCommits.sol      ─── DELETE ────> (removed)
├── MultichainACL.sol          ─── DELETE ────> (removed)
│
├── InputVerificationRegistry.sol ─ KEEP ─────> InputVerificationRegistry.sol
├── DecryptionRegistry.sol     ─── NEW ───────> DecryptionRegistry.sol
├── GatewayConfig.sol          ─── MODIFY ────> GatewayConfig.sol (+ apiUrl)
├── ProtocolPayment.sol        ─── KEEP ─────> ProtocolPayment.sol
└── KMSGeneration.sol          ─── KEEP ─────> KMSGeneration.sol

interfaces/
├── IInputVerification.sol     ─── DELETE ────> (removed)
├── IDecryption.sol            ─── DELETE ────> (removed)
├── ICiphertextCommits.sol     ─── DELETE ────> (removed)
├── IMultichainACL.sol         ─── DELETE ────> (removed)
│
├── IInputVerificationRegistry.sol ─ KEEP ───> IInputVerificationRegistry.sol
├── IDecryptionRegistry.sol    ─── NEW ──────> IDecryptionRegistry.sol
└── IGatewayConfig.sol         ─── MODIFY ───> IGatewayConfig.sol
```

### Tasks

| ID | Task | Files |
|----|------|-------|
| 1A.1 | Delete V1 contracts | `InputVerification.sol`, `Decryption.sol`, `CiphertextCommits.sol`, `MultichainACL.sol` |
| 1A.2 | Delete V1 interfaces | `IInputVerification.sol`, `IDecryption.sol`, `ICiphertextCommits.sol`, `IMultichainACL.sol` |
| 1A.3 | Delete V1 mocks | `mocks/InputVerificationMock.sol`, etc. |
| 1A.4 | Create DecryptionRegistry | New contract without response handling |
| 1A.5 | Update InputVerificationRegistry | Add userAddress + userSignature params (see DESIGN_CONFLICTS_RESOLUTION.md #2) |
| 1A.6 | Update GatewayConfig | Add `apiUrl` to KmsNode and Coprocessor structs |
| 1A.7 | Update shared imports | Remove references to deleted contracts in `shared/` |
| 1A.8 | Update Hardhat tasks | Point to V2 contracts, update deployment scripts |
| 1A.9 | Regenerate Rust bindings | `python3 scripts/bindings_update.py update` |
| 1A.10 | Write Foundry tests | Unit tests for V2 contracts |
| 1A.11 | Update upgrade tests | Remove V1 references in `test/upgrades/upgrades.ts` |

### Phase Boundary Check (1A)
- No changes outside `gateway-contracts/`
- `InputVerificationRegistry` changes included in this phase (do not defer to Phase 2)
- V2 contract tests added and passing

### Contract Changes Detail

#### DecryptionRegistry.sol (NEW)

```solidity
// Key differences from V1 Decryption.sol:
// 1. NO response handling functions (userDecryptionResponse, publicDecryptionResponse)
// 2. Events emit handles only (not SnsCiphertextMaterial[])
// 3. NO dependency on CiphertextCommits
// 4. NO dependency on MultichainACL (KMS validates on Host Chain)

interface IDecryptionRegistry {
    // UPDATED per Oracle review - includes authorization material
    event UserDecryptionRequested(               // Past tense naming
        uint256 indexed requestId,
        bytes32[] handles,
        address[] contractAddresses,             // ADDED: for per-handle ACL lookup
        address indexed userAddress,
        bytes publicKey,
        bytes signature,                         // ADDED: EIP-712 user signature
        uint256 chainId,
        uint256 timestamp                        // ADDED: set to block.timestamp
    );

    event PublicDecryptionRequested(             // Past tense naming
        uint256 indexed requestId,
        bytes32[] handles,
        address[] contractAddresses,             // ADDED: for per-handle ACL lookup
        uint256 chainId,
        uint256 timestamp                        // ADDED: set to block.timestamp
    );

    // INVARIANT: handles.length == contractAddresses.length
    // Each handles[i] is associated with contractAddresses[i] for ACL lookup

    function requestUserDecryption(
        bytes32[] calldata handles,
        address[] calldata contractAddresses,
        bytes calldata publicKey,
        bytes calldata signature                 // EIP-712 signed request
    ) external payable returns (uint256);

    function requestPublicDecryption(
        bytes32[] calldata handles,
        address[] calldata contractAddresses
    ) external payable returns (uint256);

    // Dispute/refund DEFERRED to V2.1 - requires on-chain fulfillment marker
    // See DESIGN_CONFLICTS_RESOLUTION.md #9 for rationale
}
```

#### GatewayConfig.sol (MODIFIED)

```solidity
// Add apiUrl field to both structs
// UPDATED per Oracle review - bind endpoint to identity
struct KmsNode {
    address txSenderAddress;
    address signerAddress;          // Identity binding - must match ProtocolConfig
    string ipAddress;
    string storageUrl;
    string apiUrl;                  // NEW: HTTP API endpoint bound to signerAddress
}

struct Coprocessor {
    address txSenderAddress;
    address signerAddress;          // Identity binding - must match ProtocolConfig
    string s3BucketUrl;
    string apiUrl;                  // NEW: HTTP API endpoint bound to signerAddress
}

// New getter functions
function getKmsNodesWithApis() external view returns (KmsNode[] memory);
function getCoprocessorsWithApis() external view returns (Coprocessor[] memory);

// Dual-source model:
// - ProtocolConfig (Ethereum): Source of truth for signer identities and threshold
// - GatewayConfig (Gateway Chain): Operational config (apiUrl, s3BucketUrl)
// Relayer MUST verify response signer matches the signerAddress for that apiUrl
```

### Testing

| Test Type | Description |
|-----------|-------------|
| Unit (Foundry) | DecryptionRegistry request registration |
| Unit (Foundry) | DecryptionRegistry handles.length == contractAddresses.length validation |
| Unit (Foundry) | InputVerificationRegistry commitment verification |
| Unit (Foundry) | InputVerificationRegistry userSignature validation (if on-chain) |
| Unit (Foundry) | GatewayConfig apiUrl storage and retrieval |
| Integration | Deploy sequence with Hardhat tasks |

> **Note**: Dispute/reimburse tests deferred to V2.1 per Oracle review (see DESIGN_CONFLICTS_RESOLUTION.md #9)

### Acceptance Criteria

- [ ] V1 contracts deleted (compilation fails if anyone imports them)
- [ ] DecryptionRegistry deploys and emits correct events
- [ ] InputVerificationRegistry works with commitment-only flow
- [ ] GatewayConfig stores and returns apiUrl for workers
- [ ] Rust bindings regenerated successfully
- [ ] All Foundry tests pass
- [ ] Hardhat deployment tasks work

---

## Phase 1B: Host Contracts (KMSVerifierV2)

**Duration**: 1 week
**Codebase**: `host-contracts/`
**Outcome**: KMSVerifierV2 with epoch grace period support

### Before → After

```
host-contracts/contracts/
────────────────────────────────────────────────────────────────────────────────

BEFORE                                          AFTER
├── KMSVerifier.sol            ─── DELETE ─────> REMOVED
├── KMSVerifierV2.sol          ─── ENHANCE ──> KMSVerifierV2.sol (grace period)
├── InputVerifier.sol          ─── KEEP ─────> InputVerifier.sol
├── ACL.sol                    ─── KEEP ─────> ACL.sol
└── FHEVMExecutor.sol          ─── KEEP ─────> FHEVMExecutor.sol
```

### KMSVerifierV2 Epoch Grace Period

```
┌────────────────┐   defineNewContext()   ┌─────────────────────┐
│ EPOCH_N_ACTIVE │───────────────────────>│ TRANSITION_PERIOD   │
│                │                        │ (grace period)      │
│ signers: [A,B] │                        │                     │
│ threshold: 2   │                        │ current: [C,D]      │
└────────────────┘                        │ previous: [A,B]     │
                                          │ BOTH valid          │
                                          └──────────┬──────────┘
                                                     │
                                                     │ grace period expires
                                                     ▼
                                          ┌─────────────────────┐
                                          │ EPOCH_N+1_ACTIVE    │
                                          │                     │
                                          │ signers: [C,D]      │
                                          │ threshold: 2        │
                                          │ previous cleared    │
                                          └─────────────────────┘
```

### Tasks

| ID | Task | Files |
|----|------|-------|
| 1B.1 | Implement grace period state machine | `KMSVerifierV2.sol` |
| 1B.2 | Add `isValidSigner()` dual-check | `KMSVerifierV2.sol` |
| 1B.3 | Add `getEffectiveThreshold()` | `KMSVerifierV2.sol` |
| 1B.4 | Add epoch tracking | `KMSVerifierV2.sol` |
| 1B.5 | Update deployment tasks | `tasks/taskDeploy.ts` |
| 1B.6 | Write Foundry tests | Grace period scenarios |

### Testing

| Test Type | Description |
|-----------|-------------|
| Unit (Foundry) | Signature valid during grace period (both epochs) |
| Unit (Foundry) | Signature invalid after grace period (old epoch) |
| Unit (Foundry) | Threshold calculation during transition |
| Unit (Foundry) | Multiple rapid context switches |

### Acceptance Criteria

- [ ] KMSVerifierV2 accepts signatures from previous epoch during grace period
- [ ] KMSVerifierV2 rejects old signatures after grace period
- [ ] Effective threshold uses minimum during transition
- [ ] Epoch ID increments correctly
- [ ] All Foundry tests pass

---

# PHASE 2: COPROCESSOR

**Duration**: 2-3 weeks
**Codebase**: `coprocessor/fhevm-engine/`
**Outcome**: HTTP API server, V2 event subscription, V1 tx-sender disabled

### Before → After

```
coprocessor/fhevm-engine/
────────────────────────────────────────────────────────────────────────────────

COMPONENT         BEFORE                           AFTER
─────────────────────────────────────────────────────────────────────────────────
gw-listener       • Subscribes to V1 events        • Subscribes to V2 events
                  • VerifyProofRequest event       • InputVerificationRegistered
                    (from InputVerification.sol)     (from InputVerificationRegistry.sol)
                  • Has axum server for            • Extends axum server with
                    /healthz, /liveness only         functional V2 endpoints:
                                                   • POST /v1/verify-input
                                                   • GET /v1/ciphertext/{handle}
                                                   • GET /v1/health

transaction-      • verify_proof.rs active         • verify_proof.rs → NO-OP
sender            • Sends verifyProofResponse()    • (V2: responses via HTTP)
                  • On-chain response txs          • Other ops unchanged

zkproof-worker    • (unchanged)                    • (unchanged)
```

### Architecture Change

```
BEFORE (V1)                                    AFTER (V2)
────────────────────────────────────────────────────────────────────────────────

Gateway                                        Gateway
   │                                              │
   │ VerifyProofRequest event                     │ InputVerificationRegistered
   │ (full ciphertext+ZKPoK)                      │ (commitment only)
   ▼                                              ▼
┌─────────────────┐                          ┌─────────────────┐
│   gw-listener   │                          │   gw-listener   │
│                 │                          │                 │
│ Parse event     │                          │ Parse event     │
│ Store in DB     │                          │ Store in DB     │
│                 │                          │ HTTP API:       │
│                 │                          │  • /verify-input│
│                 │                          │  • /ciphertext  │
└────────┬────────┘                          └────────┬────────┘
         │                                            │
         │ pg_notify                                  │ pg_notify
         ▼                                            ▼
┌─────────────────┐                          ┌─────────────────┐
│  zkproof-worker │                          │  zkproof-worker │
│                 │                          │                 │
│ Verify ZKPoK    │                          │ Verify ZKPoK    │
│ Generate handles│                          │ Generate handles│
│ Update DB       │                          │ Update DB       │
└────────┬────────┘                          └─────────────────┘
         │
         │ DB polling                        Relayer polls
         ▼                                   HTTP API directly
┌─────────────────┐                          ┌─────────────────┐
│ transaction-    │                          │ transaction-    │
│ sender          │                          │ sender          │
│                 │                          │                 │
│ verifyProof-    │ ─── DISABLED ──────────> │ (no-op for     │
│ Response() TX   │                          │  verify_proof)  │
└─────────────────┘                          └─────────────────┘
```

### Tasks

| ID | Task | Files |
|----|------|-------|
| 2.1 | **Extend** existing axum HTTP server with V2 endpoints | `gw-listener/src/http_server.rs` (existing), `gw-listener/src/api/` (new handlers) |
| 2.2 | Implement POST /v1/verify-input | `gw-listener/src/api/handlers.rs` |
| 2.3 | Implement GET /v1/ciphertext/{handle} | `gw-listener/src/api/handlers.rs` |
| 2.4 | Implement GET /v1/health | `gw-listener/src/api/handlers.rs` |
| 2.5 | Add EIP-712 signing for responses (include epochId) | `gw-listener/src/api/` |
| 2.6 | Update event subscription to V2 | `gw-listener/src/gw_listener.rs` |
| 2.7 | **Add event reconciliation on startup** | `gw-listener/src/` (Oracle finding) |
| 2.8 | Disable verify_proof in tx-sender (comment out + WARN log) | `transaction-sender/src/ops/verify_proof.rs` |
| 2.9 | Add DB migration for V2 input verification fields | `db-migration/migrations/` (commitment + user_signature + epoch_id as needed) |
| 2.10 | Write API unit tests | `gw-listener/src/api/tests/` |
| 2.11 | Phase boundary check | Ensure Phase 2 touches only `coprocessor/fhevm-engine/` and does not add `gateway-contracts/` changes |

> **Note on 2.1**: The coprocessor's `gw-listener` already has an axum server (`http_server.rs`) exposing `/healthz` and `/liveness` for monitoring. Task 2.1 extends this existing server with the V2 functional endpoints, not creating a new server from scratch.

### API Endpoints

> **Full API specification**: See [WORKER_API_SPEC.md](./WORKER_API_SPEC.md#4-coprocessor-api) for complete endpoint definitions, request/response schemas, error codes, and EIP-712 signature formats.

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/v1/verify-input` | POST | Receive input verification payload, validate commitment, return signed handles |
| `/v1/ciphertext/{handle}` | GET | Retrieve ciphertext material for KMS nodes during decryption |
| `/v1/health` | GET | Health check for load balancers and monitoring |

**Key Implementation Notes**:
1. Validate `request_id` exists in DB (from Gateway event)
2. Verify commitment = `keccak256(ciphertext_with_zkpok)` matches on-chain
3. Include `epochId` in all signed responses (see WORKER_API_SPEC.md Section 4.2)
4. Trigger zkproof-worker via `pg_notify`

### Disabling tx-sender verify_proof (Task 2.8)

The `verify_proof.rs` module should be disabled with a WARN log, not deleted, to preserve code for reference:

```rust
// transaction-sender/src/ops/verify_proof.rs

pub async fn process_verify_proof_responses(...) -> Result<()> {
    // V2: Disabled - responses now served via HTTP API
    // See RESTRUCTURED_PLAN.md Phase 2
    warn!("verify_proof tx-sender DISABLED in V2 - responses served via Coprocessor HTTP API");
    
    // Original implementation commented out below for reference:
    // let pending = get_pending_verify_proofs(&db_pool).await?;
    // for proof in pending {
    //     send_verify_proof_response(&provider, proof).await?;
    // }
    
    Ok(())
}
```

**Why comment out instead of delete**: Preserves the code for rollback reference and helps future developers understand what changed.

### Testing

| Test Type | Description |
|-----------|-------------|
| Unit | API handler logic with mocked DB |
| Unit | Commitment verification |
| Unit | EIP-712 signature generation |
| Integration | Full flow: Gateway event → API → zkproof-worker |

### Acceptance Criteria

- [ ] HTTP API server starts on configured port
- [ ] POST /verify-input validates commitment against Gateway
- [ ] GET /ciphertext returns signed ciphertext data
- [ ] V2 events (InputVerificationRegistered) are subscribed
- [ ] verify_proof tx-sender operation is disabled
- [ ] All unit tests pass
- [ ] No cross-component changes (Phase 2 is coprocessor-only)

---

# PHASE 3: KMS CONNECTOR

**Duration**: 2-3 weeks
**Codebase**: `kms-connector/`
**Outcome**: HTTP API server, V2 events, direct ACL, Coprocessor API fetch

### Before → After

```
kms-connector/crates/
────────────────────────────────────────────────────────────────────────────────

COMPONENT         BEFORE                           AFTER
─────────────────────────────────────────────────────────────────────────────────
gw-listener       • Subscribes to V1 events        • Subscribes to V2 events
                  • UserDecryptionRequest with     • UserDecryptionRequested with
                    SnsCiphertextMaterial[]          bytes32[] handles only
                  • PublicDecryptionRequest        • PublicDecryptionRequested
                    (from Decryption.sol)            (from DecryptionRegistry.sol)

kms-worker        • Fetches CT from S3/            • Fetches CT from Coprocessor
                    CiphertextCommits                HTTP API
                  • Queries MultichainACL          • Queries Host Chain ACL
                  • No HTTP API                    • HTTP API server
                                                   • GET /v1/share/{requestId}
                                                   • GET /v1/health

tx-sender         • Posts responses on-chain       • DISABLED (no-op)
                  • userDecryptionResponse()       • Responses via HTTP API
                  • publicDecryptionResponse()
```

### Current State Clarification

**V1 Events (Current - from `Decryption.sol`)**:
```solidity
// Current events include full SnsCiphertextMaterial[] - heavy payload
event UserDecryptionRequest(
    uint256 indexed decryptionId,
    SnsCiphertextMaterial[] snsCtMaterials,  // Full ciphertext material on-chain
    address userAddress,
    bytes publicKey,
    bytes extraData
);

event PublicDecryptionRequest(
    uint256 indexed decryptionId,
    SnsCiphertextMaterial[] snsCtMaterials,  // Full ciphertext material on-chain
    bytes extraData
);
```

**V2 Events (Target - from new `DecryptionRegistry.sol`)**:
```solidity
// V2 events use handles only - KMS fetches ciphertext from Coprocessor API
event UserDecryptionRequested(
    uint256 indexed requestId,
    bytes32[] handles,                        // Just handles - lightweight
    address[] contractAddresses,
    address indexed userAddress,
    bytes publicKey,
    bytes signature,
    uint256 chainId,
    uint256 timestamp
);

event PublicDecryptionRequested(
    uint256 indexed requestId,
    bytes32[] handles,                        // Just handles - lightweight
    address[] contractAddresses,
    uint256 chainId,
    uint256 timestamp
);
```

### Architecture Change

```
BEFORE (V1)                                    AFTER (V2)
────────────────────────────────────────────────────────────────────────────────

Gateway (Decryption.sol)                       Gateway (DecryptionRegistry.sol)
   │                                              │
   │ UserDecryptionRequest event                  │ UserDecryptionRequested event
   │ (with SnsCiphertextMaterial[])               │ (with bytes32[] handles only)
   ▼                                              ▼
┌─────────────────┐                          ┌─────────────────┐
│   gw-listener   │                          │   gw-listener   │
│                 │                          │                 │
│ Parse materials │                          │ Parse handles   │
│ Store in DB     │                          │ Store in DB     │
└────────┬────────┘                          └────────┬────────┘
         │                                            │
         ▼                                            ▼
┌─────────────────┐                          ┌─────────────────┐
│   kms-worker    │                          │   kms-worker    │
│                 │                          │                 │
│ Query Gateway   │                          │ Query Host      │
│ MultichainACL   │ ─── REPLACED ──────────> │ Chain ACL       │
│                 │                          │                 │
│ Fetch from S3/  │                          │ Fetch from      │
│ CiphertextCommits│ ─── REPLACED ──────────>│ Coprocessor API │
│                 │                          │                 │
│ Compute share   │                          │ Compute share   │
│ Store in DB     │                          │ Store in DB     │
│                 │                          │                 │
│                 │                          │ HTTP API:       │
│                 │                          │  • /share/{id}  │
│                 │                          │  • /health      │
└────────┬────────┘                          └─────────────────┘
         │
         │ DB polling                        Relayer polls
         ▼                                   HTTP API directly
┌─────────────────┐                          ┌─────────────────┐
│   tx-sender     │                          │   tx-sender     │
│                 │                          │                 │
│ user/public-    │ ─── DISABLED ──────────> │ (no-op for     │
│ DecryptResponse │                          │  decrypt ops)   │
└─────────────────┘                          └─────────────────┘
```

### Tasks

| ID | Task | Files |
|----|------|-------|
| 3.1 | **Extend** existing actix-web server with V2 endpoints | `kms-worker/src/bin/kms_worker.rs` (existing monitoring server), `kms-worker/src/api/` (new handlers) |
| 3.2 | Implement GET /v1/share/{requestId} | `kms-worker/src/api/handlers.rs` |
| 3.3 | Implement GET /v1/health | `kms-worker/src/api/handlers.rs` |
| 3.4 | Update event subscription to V2 | `gw-listener/src/core/gw_listener.rs` |
| 3.5 | Implement direct Host Chain ACL query | `kms-worker/src/core/event_processor/acl.rs` (new) |
| 3.6 | Implement Coprocessor API ciphertext fetch | `kms-worker/src/core/event_processor/coprocessor_api.rs` |
| 3.7 | Remove S3/CiphertextCommits fetch | `kms-worker/src/core/event_processor/s3.rs` |
| 3.8 | Disable tx-sender decryption responses (comment out + WARN log) | `tx-sender/src/core/tx_sender.rs` |
| 3.9 | Write API unit tests | `kms-worker/src/api/tests/` |
| 3.10 | **Add event reconciliation with reorg safety** | `gw-listener/src/` (Oracle finding - see DESIGN_CONFLICTS_RESOLUTION.md #8) |
| 3.11 | Add DB migration for V2 decryption requests | `connector-db/migrations/` (handles + contract_addresses + chain_id + timestamp + signature + epoch_id + rejection metadata; drop legacy columns) |
| 3.12 | Retire legacy S3 decryption tests | `kms-worker/tests/` (remove/update V1 S3-focused tests and mocks) |
| 3.13 | Phase boundary check | Ensure Phase 3 touches only `kms-connector/` |

> **Note on 3.1**: The KMS connector's `kms-worker` already has an actix-web server exposing `/metrics`, `/healthz`, `/liveness`, and `/version` for monitoring. Task 3.1 extends this existing server with the V2 functional endpoints.

### API Endpoints

> **Full API specification**: See [WORKER_API_SPEC.md](./WORKER_API_SPEC.md#3-kms-node-api) for complete endpoint definitions, request/response schemas, error codes, and EIP-712 signature formats.

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/v1/share/{requestId}` | GET | Retrieve decryption share (user or public) |
| `/v1/health` | GET | Health check for load balancers and monitoring |

**Key Implementation Notes**:
1. Include `epochId` in all signed responses (see WORKER_API_SPEC.md Section 3.2)
2. Return `status: pending` if still processing
3. Return `status: rejected` with reason if ACL check fails
4. Signature format differs for user vs public decryption

### Disabling tx-sender Decryption Responses (Task 3.8)

The decryption response functions should be disabled with WARN logs:

```rust
// tx-sender/src/core/tx_sender.rs

pub async fn send_public_decryption_response(...) -> Result<()> {
    // V2: Disabled - responses now served via KMS HTTP API
    // See RESTRUCTURED_PLAN.md Phase 3
    warn!("send_public_decryption_response DISABLED in V2 - responses served via KMS HTTP API");
    Ok(())
}

pub async fn send_user_decryption_response(...) -> Result<()> {
    // V2: Disabled - responses now served via KMS HTTP API
    warn!("send_user_decryption_response DISABLED in V2 - responses served via KMS HTTP API");
    Ok(())
}

// NOTE: Keep keygen responses ACTIVE - those still go on-chain
pub async fn send_keygen_response(...) -> Result<()> {
    // This remains active in V2
    // ... existing implementation ...
}
```

**Important**: Only disable decryption responses. Keygen responses (`send_keygen_response`) remain active as they still go on-chain.

### Direct ACL Query Flow

```rust
// Instead of querying Gateway MultichainACL:
// Query Host Chain ACL directly

async fn check_acl(handle: B256, requester: Address, host_chain_id: u64) -> bool {
    let host_rpc = get_host_chain_rpc(host_chain_id);
    let acl_contract = ACL::new(acl_address, host_rpc);

    // Query at finalized block
    acl_contract.isAllowed(handle, requester).call().await
}
```

### Coprocessor API Fetch Flow

```rust
// Instead of S3/CiphertextCommits:
// Fetch from Coprocessor HTTP API with signature verification
// UPDATED per Oracle review - see DESIGN_CONFLICTS_RESOLUTION.md #7

async fn fetch_ciphertext(handle: B256) -> Result<Ciphertext> {
    let coprocessors = gateway_config.getCoprocessorsWithApis().await?;
    let mut signed_responses: Vec<(Address, CiphertextResponse)> = vec![];

    for copro in coprocessors {
        match fetch_from_copro(&copro.api_url, handle).await {
            Ok(ct) => {
                // 1. Verify digest matches
                if keccak256(&ct.sns_ciphertext) != ct.sns_ciphertext_digest {
                    continue;
                }

                // 2. Verify coprocessor signature (REQUIRED)
                let typed_data = eip712_hash(CiphertextResponse {
                    handle: ct.handle,
                    key_id: ct.key_id,
                    sns_ciphertext_digest: ct.sns_ciphertext_digest,
                    epoch_id: ct.epoch_id,
                });
                let signer = ecrecover(typed_data, ct.signature);

                // Verify signer is valid coprocessor from ProtocolConfig
                if !is_valid_coprocessor_signer(signer) {
                    continue;
                }

                // 3. Verify epochId is current or previous
                if !is_valid_epoch(ct.epoch_id) {
                    continue;
                }

                signed_responses.push((signer, ct));

                // 4. HEURISTIC: Accept when ≥2 matching digests (robustness)
                let matching = signed_responses.iter()
                    .filter(|(_, r)| r.sns_ciphertext_digest == ct.sns_ciphertext_digest)
                    .count();
                if matching >= 2 {
                    return Ok(ct);
                }
            }
            Err(_) => continue,
        }
    }

    // 5. Fallback: Accept single signed response (availability-first)
    if let Some((_, ct)) = signed_responses.first() {
        return Ok(ct.clone());
    }

    Err("All coprocessors failed")
}
```

### Testing

| Test Type | Description |
|-----------|-------------|
| Unit | API handler logic with mocked DB |
| Unit | Direct ACL query |
| Unit | Coprocessor API fetch with digest verification |
| Integration | Full flow: Gateway event → ACL check → CT fetch → share |

### Acceptance Criteria

- [ ] HTTP API server starts on configured port
- [ ] GET /share returns signed share data
- [ ] V2 events (handles-only) are subscribed
- [ ] ACL checks query Host Chain directly
- [ ] Ciphertext fetched from Coprocessor API
- [ ] tx-sender decryption responses disabled
- [ ] All unit tests pass
- [ ] DB schema updated for V2 decryption requests
- [ ] Legacy S3 tests removed/updated
- [ ] No cross-component changes (Phase 3 is kms-connector-only)

---

# PHASE 4: RELAYER

**Duration**: 2-3 weeks
**Codebase**: `console/apps/relayer/` (external repo)
**Outcome**: V2 contracts, worker polling, response aggregation

### Before → After

```
console/apps/relayer/src/gateway/
────────────────────────────────────────────────────────────────────────────────

COMPONENT              BEFORE                        AFTER
─────────────────────────────────────────────────────────────────────────────────
input_handlers.rs      • Calls InputVerification     • Calls InputVerification-
                         .verifyProofRequest()         Registry.register...()
                       • Full payload on-chain       • Commitment only on-chain
                       • Listens for response        • Polls Coprocessor API
                         events

user_decrypt_handler   • Calls Decryption contract   • Calls DecryptionRegistry
                       • Listens for response        • Polls KMS API
                         events

public_decrypt_handler • Calls Decryption contract   • Calls DecryptionRegistry
                       • Listens for response        • Polls KMS API
                         events

worker_polling/        • (doesn't exist)             • NEW: Polling module
                                                     • Backoff strategy
                                                     • Threshold aggregation
```

### Architecture Change

```
BEFORE (V1)                                    AFTER (V2)
────────────────────────────────────────────────────────────────────────────────

User                                           User
  │                                              │
  │ POST /v1/input-proof                         │ POST /v1/input-proof
  ▼                                              ▼
┌─────────────────┐                          ┌─────────────────┐
│     Relayer     │                          │     Relayer     │
│                 │                          │                 │
│ ① Call Gateway  │                          │ ① Compute       │
│   InputVerif-   │                          │   commitment    │
│   ication.      │                          │                 │
│   verifyProof-  │                          │ ② Call Gateway  │
│   Request()     │                          │   InputVerif-   │
│   (full payload)│                          │   icationReg.   │
│                 │                          │   register()    │
│ ② Listen for    │                          │   (commit only) │
│   VerifyProof-  │                          │                 │
│   Response      │                          │ ③ Broadcast     │
│   event         │                          │   payload to    │
│                 │                          │   Coprocessors  │
│                 │                          │                 │
│                 │                          │ ④ Poll Copro    │
│                 │                          │   APIs until    │
│                 │                          │   threshold     │
│                 │                          │                 │
│ ③ Return to     │                          │ ⑤ Aggregate +   │
│   user          │                          │   return        │
└─────────────────┘                          └─────────────────┘
```

### Tasks

| ID | Task | Files |
|----|------|-------|
| 4.1 | Update contract bindings import | Use V2 bindings |
| 4.2 | Implement worker polling module | `worker_polling/` |
| 4.3 | Update input_handlers.rs | V2 flow with commitment |
| 4.4 | Update user_decrypt_handler.rs | Poll KMS API |
| 4.5 | Update public_decrypt_handler.rs | Poll KMS API |
| 4.6 | Remove V1 event listening | Remove response event handlers |
| 4.7 | Implement response aggregator | Threshold logic |
| 4.8 | Add V2 feature flag | Config-based V1/V2 switch |
| 4.9 | **Skip workers with empty `apiUrl`** | `worker_polling/` (Oracle finding) |
| 4.10 | **Verify epochId consistency in responses** | `worker_polling/` (Oracle finding) |
| 4.11 | Write integration tests | Full flow tests |

### Worker Polling Module

```rust
// worker_polling/poller.rs

pub struct WorkerPoller {
    config: PollingConfig,
}

pub struct PollingConfig {
    initial_delay_ms: u64,    // 100ms
    max_delay_ms: u64,        // 5000ms
    backoff_factor: f64,      // 1.5
    timeout_secs: u64,        // 60s
    jitter_percent: u64,      // 20%
}

impl WorkerPoller {
    pub async fn poll_until_threshold<T>(
        &self,
        workers: &[WorkerEndpoint],
        request_id: &str,
        threshold: usize,
    ) -> Result<Vec<T>> {
        let mut delay = self.config.initial_delay_ms;
        let deadline = Instant::now() + Duration::from_secs(self.config.timeout_secs);

        loop {
            // Poll all workers in parallel
            let responses = join_all(
                workers.iter().map(|w| self.poll_worker(w, request_id))
            ).await;

            let ready: Vec<_> = responses
                .into_iter()
                .filter_map(|r| r.ok())
                .filter(|r| r.status == "ready")
                .collect();

            if ready.len() >= threshold {
                return Ok(ready);
            }

            if Instant::now() > deadline {
                return Err("Timeout waiting for threshold");
            }

            // Backoff with jitter
            let jitter = rand::thread_rng().gen_range(0..self.config.jitter_percent);
            sleep(Duration::from_millis(delay * (100 + jitter) / 100)).await;
            delay = (delay as f64 * self.config.backoff_factor) as u64;
            delay = delay.min(self.config.max_delay_ms);
        }
    }
}
```

### Testing

| Test Type | Description |
|-----------|-------------|
| Unit | Polling backoff calculation |
| Unit | Threshold aggregation |
| Unit | Commitment computation |
| Integration | Full V2 input verification flow |
| Integration | Full V2 decryption flow |

### Acceptance Criteria

- [ ] Relayer uses V2 contract bindings
- [ ] Input verification uses commitment-only flow
- [ ] Decryption handlers poll worker APIs
- [ ] Polling implements exponential backoff with jitter
- [ ] Threshold aggregation works correctly
- [ ] V1 event listening removed
- [ ] All tests pass

---

# PHASE 5: COLD PATH (OUT OF SCOPE)

> **Status**: OUT OF SCOPE for current implementation. To be planned separately.

**Duration**: TBD
**Codebase**: `host-contracts/`, `kms-connector/`, `relayer-sdk/`
**Outcome**: Users can bypass Gateway/Relayer entirely for decryption requests

### Overview

The Cold Path allows users to submit decryption requests directly to the Host Chain, bypassing the Gateway and Relayer. This provides a trustless fallback when:
- Relayer is unavailable or censoring
- User wants maximum decentralization
- Gateway chain is congested

### Architecture (Future)

```
COLD PATH FLOW (Future Implementation)
────────────────────────────────────────────────────────────────────────────────

User/SDK
    │
    │ ① Submit directly to Host Chain
    │   (pay in native token)
    ▼
┌──────────────────────┐
│    HOST CHAIN        │
│                      │
│ DecryptionFallback   │◄── NEW CONTRACT
│ • requestUser-       │
│   Decryption()       │
│ • requestPublic-     │
│   Decryption()       │
│ • Native token fee   │
└──────────┬───────────┘
           │
           │ ② KMS listens to Host Chain events
           │   (in addition to Gateway events)
           ▼
┌──────────────────────┐
│    KMS CONNECTOR     │
│                      │
│ • Dual listener:     │
│   - Gateway events   │
│   - Host Chain events│◄── NEW LISTENER
│                      │
│ • Same processing    │
│ • Same API output    │
└──────────────────────┘
           │
           │ ③ User polls KMS directly
           │   (no Relayer involvement)
           ▼
┌──────────────────────┐
│      USER/SDK        │
│                      │
│ • Discover KMS URLs  │
│   from GatewayConfig │
│ • Poll GET /share/   │
│ • Aggregate locally  │
│ • Verify signatures  │
└──────────────────────┘
```

### Components Required

| Component | Work Required |
|-----------|---------------|
| **DecryptionFallback.sol** | New Host Chain contract for direct request submission |
| **KMS gw-listener** | Add Host Chain event listener alongside Gateway listener |
| **SDK** | Add endpoint discovery + direct worker polling |
| **GatewayConfig** | Already has apiUrl (from Phase 1A) |

### Why Out of Scope

1. **Hot path is primary**: 99%+ of requests will use the Relayer (hot path)
2. **Complexity**: Requires SDK changes for endpoint discovery and polling
3. **Testing burden**: Requires full E2E testing of alternative flow
4. **Lower priority**: Users have workaround (run own Relayer) if needed

### Future Implementation Notes

When implementing Cold Path:
- DecryptionFallback should use native token (ETH) for fees
- Request ID format should use distinct prefix (e.g., `0x03`) to avoid collision
- KMS must treat cold path requests identically to Gateway requests
- SDK needs `GatewayConfig.getKmsNodesWithApis()` for endpoint discovery

---

# Dependency Graph

```
PHASE 1A ──────────────────────────────────────────────────────────────────────
Gateway Contracts
  │
  ├── Rust bindings regenerated
  │
  ▼
PHASE 1B ──────────────────────────────────────────────────────────────────────
Host Contracts (KMSVerifierV2)
  │
  │ (independent of 1A, can run in parallel)
  │
  ▼
PHASE 2 ───────────────────────────────────────────────────────────────────────
Coprocessor
  │
  ├── Depends on: V2 Gateway contracts deployed
  ├── Depends on: Rust bindings from 1A
  │
  ▼
PHASE 3 ───────────────────────────────────────────────────────────────────────
KMS Connector
  │
  ├── Depends on: V2 Gateway contracts deployed
  ├── Depends on: Rust bindings from 1A
  ├── Depends on: Coprocessor API running (for CT fetch)
  │
  ▼
PHASE 4 ───────────────────────────────────────────────────────────────────────
Relayer
  │
  ├── Depends on: V2 Gateway contracts deployed
  ├── Depends on: Coprocessor API running
  ├── Depends on: KMS API running
  │
  ▼
PHASE 5 (OUT OF SCOPE) ────────────────────────────────────────────────────────
Cold Path
  │
  ├── Depends on: All previous phases complete
  ├── New: DecryptionFallback contract on Host Chain
  ├── New: KMS Host Chain event listener
  ├── New: SDK endpoint discovery + direct polling
  │
  ▼
COMPLETE ──────────────────────────────────────────────────────────────────────
```

---

# Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Binding regeneration fails | Medium | High | Manual binding updates documented |
| V1 deletion breaks unexpected consumers | Medium | Medium | Search codebase for imports before deletion |
| Worker API latency issues | Low | Medium | Configurable timeouts, backoff strategy |
| Grace period edge cases | Low | High | Comprehensive Foundry tests |
| Relayer SDK compatibility | Medium | High | Keep /v1/ API routes, internal V2 logic |

---

# Timeline Estimate

| Phase | Duration | Dependencies |
|-------|----------|--------------|
| 1A: Gateway Contracts | 1-2 weeks | None |
| 1B: Host Contracts | 1 week | None (parallel with 1A) |
| 2: Coprocessor | 2-3 weeks | 1A complete |
| 3: KMS Connector | 2-3 weeks | 1A, 2 complete |
| 4: Relayer | 2-3 weeks | 1A, 2, 3 complete |
| 5: Cold Path | TBD | OUT OF SCOPE |
| **Total (Phases 1-4)** | **8-12 weeks** | |

---

# Lessons Learned & Safeguards

> **Source**: Issues documented in `CHANGELOG.md` from previous implementation attempts.

This section captures critical mistakes made during prior V2 implementation work and the explicit safeguards built into this plan to prevent recurrence.

## 1. Rust Bindings Synchronization

### Problem Encountered
- Missing `apiUrl` field in Coprocessor struct caused runtime errors: `"type check failed for 'offset (usize)'"`
- V1 contract bindings remained after V1 contracts were deleted
- `forge bind` failed due to version mismatches, leaving bindings outdated

### Safeguards in This Plan

| Phase | Checkpoint | Command |
|-------|------------|---------|
| 1A.9 | Regenerate bindings after contract changes | `python3 scripts/bindings_update.py update` |
| 1A.9 | **Verify** bindings match contract structs | `cargo build` in `rust_bindings/` |
| 1A.9 | If `forge bind` fails, document manual fix | Update field manually, document in PR |
| 2, 3, 4 | Each phase: verify bindings import before coding | `use fhevm_gateway_bindings::...` compiles |

**MANDATORY**: After ANY contract struct change (add/remove field), regenerate bindings AND verify downstream Rust crates compile.

---

## 2. Event Subscription Mismatches

### Problem Encountered
- Relayer emitted V1 `VerifyProofRequest` while coprocessor gw-listener expected V2 `InputVerificationRegistered`
- KMS connector subscribed to wrong event types
- Logs showed: `Looking for topic: VerifyProofRequest(...)` when should be `InputVerificationRegistered(...)`

### Safeguards in This Plan

| Phase | Checkpoint | Verification |
|-------|------------|--------------|
| 1A | Delete V1 contracts FIRST | V1 events no longer exist |
| 2.6 | Update coprocessor event subscription | `grep -r "VerifyProofRequest" coprocessor/` returns 0 |
| 3.4 | Update KMS event subscription | `grep -r "Decryption::" kms-connector/` returns 0 |
| 4.6 | Remove V1 event listening from relayer | `grep -r "VerifyProofResponse" relayer/` returns 0 |

**VERIFICATION SCRIPT** (run after each phase):
```bash
# Should return NO matches after V2 migration
grep -rn "VerifyProofRequest\|VerifyProofResponse\|RejectProofResponse" \
  coprocessor/ kms-connector/ --include="*.rs"

grep -rn "Decryption::\(public\|user\)DecryptionResponse" \
  kms-connector/ --include="*.rs"
```

---

## 3. V1 Readiness Checks Still Called

### Problem Encountered
- Handlers called `self.check_readiness()` which queries `CiphertextCommits.isCiphertextMaterialAdded()` and `MultichainACL.isAccountAllowed()`
- These contracts don't exist in V2, causing HTTP 503 errors

### Safeguards in This Plan

| Phase | Action | Files |
|-------|--------|-------|
| 1A | Delete `CiphertextCommits.sol` | Compilation fails if referenced |
| 1A | Delete `MultichainACL.sol` | Compilation fails if referenced |
| 4 | Remove `check_readiness()` calls | `user_decrypt_handler.rs`, `public_decrypt_handler.rs` |
| 4 | Add comment explaining V2 flow | KMS validates ACL on Host Chain directly |

**MANDATORY**: After deleting contracts, search for ANY remaining references:
```bash
grep -rn "CiphertextCommits\|MultichainACL\|check_readiness" \
  --include="*.rs" --include="*.sol" --include="*.ts"
```

---

## 4. Database Schema Mismatches

### Problem Encountered
- `tx-sender` used queries expecting `under_process` column, but V2 migration changed to `status` enum
- Error: `column "under_process" does not exist`
- SQLX cache outdated, needed regeneration

### Safeguards in This Plan

| Phase | Action | Command |
|-------|--------|---------|
| 2.8 | Add DB migration for new columns | Check migration files exist |
| 2, 3 | Regenerate SQLX cache after schema changes | `cargo sqlx prepare --workspace` |
| 2, 3 | Test with fresh database | `docker compose down -v && ./fhevm-cli deploy` |

**MANDATORY**: After ANY schema change:
```bash
# Regenerate SQLX offline cache
cd coprocessor/fhevm-engine && DATABASE_URL="postgres://..." cargo sqlx prepare --workspace
cd kms-connector && DATABASE_URL="postgres://..." cargo sqlx prepare --workspace
```

---

## 5. Hardhat Task Idempotency

### Problem Encountered
- `addHostChain` task didn't check if chain already registered
- Deployment failed with `HostChainAlreadyRegistered` revert on re-runs

### Safeguards in This Plan

| Phase | Action | Pattern |
|-------|--------|---------|
| 1A.8 | Review all Hardhat tasks for idempotency | Check before mutate |

**PATTERN** for idempotent tasks:
```typescript
// GOOD: Check before calling
const isRegistered = await gatewayConfig.isHostChainRegistered(chainId);
if (isRegistered) {
  console.log(`Chain ${chainId} already registered, skipping...`);
  return;
}
await gatewayConfig.addHostChain(...);

// BAD: Call unconditionally
await gatewayConfig.addHostChain(...);  // Will revert on re-run
```

---

## 6. Separate V2 Handler Architecture Mistake

### Problem Encountered
- Created separate `/v2/` route handlers (`user_decrypt_handler_v2.rs`, etc.)
- SDK hardcodes `/v1/` endpoints, so V2 handlers were never called
- Logic duplication between V1 and V2 handlers

### Safeguards in This Plan

| Rule | Implementation |
|------|----------------|
| Keep HTTP routes as `/v1/` | SDK compatibility, no SDK changes needed |
| Modify existing handlers internally | V1 handlers use V2 logic under the hood |
| Delete separate V2 handler files | No `*_v2.rs` files in final state |

**ARCHITECTURE PRINCIPLE**: Gateway V2 is a **backend optimization**, not an API version change.

```
WRONG:                              CORRECT:
/v1/user-decrypt → V1 handler       /v1/user-decrypt → V1 handler (uses V2 logic)
/v2/user-decrypt → V2 handler       (no /v2/ routes)
```

---

## 7. Docker/Deployment Issues

### Problems Encountered
- BuildKit provenance hang: builds stuck at "resolving provenance"
- Missing `relayer_db` database before migrations
- Service reference bugs: `no such service: coprocessor-and-kms-db`
- Missing environment variables during manual service startup

### Safeguards in This Plan

| Issue | Safeguard |
|-------|-----------|
| Provenance hang | Set `BUILDX_NO_DEFAULT_ATTESTATIONS=1` |
| Missing database | Run `CREATE DATABASE` before migrations |
| Service references | Use service names, not container names |
| Missing env vars | Always source env file before docker-compose |

**DEPLOYMENT CHECKLIST** (for each phase):
```bash
# 1. Set build env
export BUILDX_NO_DEFAULT_ATTESTATIONS=1

# 2. Create databases if needed
docker exec coprocessor-and-kms-db psql -U postgres -c "CREATE DATABASE IF NOT EXISTS ..."

# 3. Run migrations BEFORE starting services
./fhevm-cli db migrate coprocessor
./fhevm-cli db migrate kms
./fhevm-cli db migrate relayer

# 4. Verify services start
./fhevm-cli status
```

---

## 8. Contract Dependency Removal Order

### Problem Encountered
- Removing `CiphertextCommits` required changing `DecryptionRegistry` event signatures
- Removing `MultichainACL` required removing ACL check calls from contracts
- Breaking changes cascaded through multiple files

### Safeguards in This Plan

**REMOVAL ORDER** (Phase 1A):
```
1. Delete V1 contracts (InputVerification, Decryption V1)
2. Update V2 contracts to NOT import deleted contracts
3. Delete dependency contracts (CiphertextCommits, MultichainACL)
4. Update remaining contracts to NOT import deleted dependencies
5. Regenerate bindings
6. Fix ALL compilation errors before proceeding
```

**VERIFICATION**:
```bash
# After deletions, ensure clean compilation
cd gateway-contracts && forge build

# Ensure no dangling imports
grep -rn "import.*CiphertextCommits\|import.*MultichainACL" contracts/
```

---

## Oracle Review Findings (January 2026)

> **Source**: Oracle agent review comparing plan against GATEWAY_V2_DESIGN.md

### Gaps Identified and Addressed

| Gap | Design Reference | Resolution |
|-----|------------------|------------|
| **Relayer skip workers with empty `apiUrl`** | GATEWAY_V2_DESIGN.md:647-649 | Added to Phase 4 tasks |
| **`epochId` in signed payloads** | GATEWAY_V2_DESIGN.md:816-844 | Added to Phase 2, 3 signing requirements |
| **Event delivery robustness** | Operational concern | Added reconciliation guidance |
| **Migration mixed-mode safety** | GATEWAY_V2_DESIGN.md:647-649 | Added pre-deletion gate |

### Additional Safeguards Added

#### 1. Relayer Must Skip Workers Without `apiUrl`

**Phase 4 Addition** (worker_polling/poller.rs):
```rust
// Filter workers with empty apiUrl (v1-only, not yet migrated)
let v2_workers: Vec<_> = workers
    .iter()
    .filter(|w| !w.api_url.is_empty())
    .collect();

if v2_workers.len() < threshold {
    warn!("Insufficient V2 workers: {} available, {} required",
          v2_workers.len(), threshold);
    // Fall back to v1 flow or return error
}
```

**Rationale**: Design explicitly states existing deployments may have empty `apiUrl` fields and relayer should skip them (GATEWAY_V2_DESIGN.md:647-649).

#### 2. Epoch Binding in Signatures

**Phase 2/3 Addition** (API response signing):
```rust
// EIP-712 typed data MUST include epochId
struct VerificationResponse {
    request_id: U256,
    handles: Vec<B256>,
    epoch_id: U256,        // REQUIRED: Current MPC context epoch
    // ...
}

// Sign with epochId to prevent cross-epoch replay
let typed_data = eip712_hash(domain, VerificationResponse { epoch_id, ... });
```

**Phase 4 Addition** (response verification):
```rust
// Relayer MUST verify epochId consistency
fn verify_responses(responses: &[WorkerResponse]) -> Result<()> {
    let epochs: HashSet<_> = responses.iter().map(|r| r.epoch_id).collect();
    if epochs.len() > 1 {
        return Err("Mixed epochs in responses - context switch in progress");
    }
    Ok(())
}
```

**Rationale**: Design requires epochId in extraData to prevent cross-epoch replay (GATEWAY_V2_DESIGN.md:816-844).

#### 3. Pre-Deletion Gate for V1 Contracts

**Phase 1A Gate** (before deleting V1 contracts from deployed environments):

| Condition | Verification |
|-----------|--------------|
| All workers have `apiUrl` populated | `GatewayConfig.getKmsNodesWithApis()` returns non-empty URLs |
| Worker APIs are healthy | `GET /v1/health` returns 200 for all workers |
| Relayer has V2 polling implemented | Phase 4 complete or in parallel |

**Note**: V1 contract *code* can be deleted immediately (forces binding updates). V1 contract *deployment* deprecation should wait for worker readiness.

#### 4. Event Delivery Reconciliation

**Phase 2/3 Addition** (gw-listener robustness):
```rust
// On startup or WS reconnect, backfill missed events
async fn reconcile_events(last_processed_block: u64) -> Result<()> {
    let current_block = provider.get_block_number().await?;
    if current_block > last_processed_block + 1 {
        warn!("Gap detected: {} to {}", last_processed_block, current_block);
        // Query historical events and process
        let events = contract
            .event::<InputVerificationRegistered>()
            .from_block(last_processed_block + 1)
            .to_block(current_block)
            .query()
            .await?;
        for event in events {
            process_event(event).await?;
        }
    }
    Ok(())
}
```

**Rationale**: WS drops can cause missed events; reconciliation prevents request stalls.

#### 5. Attestation Metadata Binding (NEW - January 2026)

**All signed attestations MUST bind metadata, not just commitment.**

```rust
// WRONG: Only binding commitment
struct InputVerificationAttestation {
    request_id: U256,
    commitment: B256,
    // Missing: contractChainId, contractAddress, userAddress, epochId
}

// CORRECT: Binding ALL metadata
struct InputVerificationAttestation {
    request_id: U256,
    commitment: B256,
    contract_chain_id: U256,
    contract_address: Address,
    user_address: Address,
    epoch_id: U256,
}
```

**Rationale**: If attestation only commits to commitment, cross-context replay attacks are possible when identical payloads are reused. See DESIGN_CONFLICTS_RESOLUTION.md #6.

---

## Security Invariants (NEW - Oracle Review January 2026)

These invariants MUST be maintained throughout implementation:

| Invariant | Enforcement |
|-----------|-------------|
| **Never trust relayer-provided metadata** | User signature MUST bind userAddress + context |
| **Every attestation binds full context** | EIP-712 typed data includes requestId + all metadata |
| **Handle reorgs in all listeners** | Re-scan from `last_block - safety_margin`, idempotent processing |
| **Single-epoch threshold** | All shares in aggregate must have same epochId |
| **Endpoint-to-identity binding** | apiUrl must map to specific signerAddress |
| **handles.length == contractAddresses.length** | Validated in DecryptionRegistry |

See [DESIGN_CONFLICTS_RESOLUTION.md](./DESIGN_CONFLICTS_RESOLUTION.md) for detailed EIP-712 typed data definitions.

---

## Phase Completion Checklist

Each phase MUST complete these checks before marking done:

### Universal Checks
- [ ] `forge build` passes (gateway-contracts)
- [ ] `cargo build` passes (all Rust crates touching this phase)
- [ ] `grep` for V1 references returns 0 matches
- [ ] Unit tests pass
- [ ] Bindings regenerated if contracts changed

### Phase 1A Specific
- [ ] V1 contracts deleted
- [ ] V1 interfaces deleted
- [ ] V1 mocks deleted
- [ ] Rust bindings regenerated
- [ ] No `import.*InputVerification` in codebase (except registry)
- [ ] No `import.*CiphertextCommits` in codebase
- [ ] No `import.*MultichainACL` in codebase

### Phase 2/3 Specific
- [ ] SQLX cache regenerated
- [ ] Event subscriptions updated to V2
- [ ] tx-sender V1 operations disabled
- [ ] API endpoints documented and tested

### Phase 4 Specific
- [ ] `/v1/` routes maintained (no `/v2/` routes)
- [ ] `check_readiness()` removed
- [ ] Worker polling implemented
- [ ] V1 event listening removed

---

# Notes

1. **Cold Path**: Explicitly out of scope. Added as Phase 5 placeholder for future planning.
2. **SDK Updates**: No changes required to `@zama-fhe/relayer-sdk` for Phases 1-4.
3. **Dual-Run Validation**: To be determined later. Per-component testing for now.
4. **Rollback Strategy**: None. V1 contracts will be deleted; no rollback path.
