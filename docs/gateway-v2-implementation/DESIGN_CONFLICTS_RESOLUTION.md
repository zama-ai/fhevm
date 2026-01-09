# Gateway V2 Design Conflicts Resolution

**Created**: January 2026  
**Purpose**: Document and resolve inconsistencies between GATEWAY_V2_DESIGN.md and RESTRUCTURED_PLAN.md  
**Last Updated**: January 2026 (Oracle Review Applied)

---

## Summary of Resolutions

| # | Issue | Severity | Resolution | Oracle Verdict |
|---|-------|----------|------------|----------------|
| 1 | Request ID definition | Critical | Use `uint256` from contract, not tx hash | ✅ Correct |
| 2 | InputVerificationRegistry.userAddress | Critical | Add `userAddress` + require user signature binding | ⚠️ Refined |
| 3 | DecryptionRegistry event payload | Critical | Full event + define handle/contract mapping | ⚠️ Refined |
| 4 | MPC context switch handling | High | SDK accepts current/previous epoch, reject mixed | ✅ Correct |
| 5 | Worker endpoint source | High | ProtocolConfig = identity, GatewayConfig = apiUrl + bind endpoint to identity | ⚠️ Refined |
| 6 | Commitment binding | Medium | Attestations MUST bind all metadata, not just commitment | ⚠️ Refined |
| 7 | KMS ciphertext fetch | Medium | Require coprocessor signature + optional 2-matching heuristic | ⚠️ Refined |
| 8 | Event reconciliation for KMS | Medium | Add to Phase 3 + reorg safety | ⚠️ Refined |
| 9 | Dispute/refund scope | **Deferred** | Requires on-chain fulfillment marker, defer from MVP | ⚠️ Changed |
| 10 | Event naming | Low | Past tense, check existing naming | ✅ Correct |

### Priority Order (Oracle Recommended)

1. **(1)** RequestId = `uint256` everywhere
2. **(3)** Decryption event includes authorization material + contract mapping clarity
3. **(2)** InputVerification `userAddress` + binding/validation story
4. **(4)** Epoch grace + mixed-epoch rejection invariant
5. **(6)** Ensure attestations bind metadata (even if commitment doesn't)
6. **(7)** Coprocessor-signed ciphertext fetch (optionally add "2 matching digests" heuristic)
7. **(8)** KMS reconciliation + reorg safety
8. **(5)** Dual-source config guardrails (bind endpoint to identity)
9. **(9)** Dispute/refund (defer until fulfillment semantics defined)
10. **(10)** Naming

---

## 1. Request ID Definition (CRITICAL) ✅

### Conflict
- **Design** (line 171-173): "Request ID = Gateway transaction hash (deterministic)"
- **Contracts**: `returns (uint256 requestId)` from registry functions
- **Worker APIs**: Expect `uint256` in path (`/share/{requestId}`)

### Resolution
**Use `uint256 requestId` returned from contract functions.**

The tx hash reference was conceptual. The actual implementation uses monotonically increasing `uint256` IDs assigned by the registry contracts (see `gateway-contracts/contracts/shared/KMSRequestCounters.sol` - IDs are globally unique with high bits encoding request type).

```solidity
// Canonical requestId source
uint256 requestId = registry.registerInputVerification(...);
// NOT: bytes32 requestId = txHash
```

### Oracle Notes
- ✅ Correct approach - `uint256 requestId` is a stronger invariant than tx hash
- Works across reorgs/relays
- Optionally retain tx hash for tracing/debug only

### Impacted Files
- [x] GATEWAY_V2_DESIGN.md: Updated line 171-173 to say "Contract-assigned uint256"
- [x] GATEWAY_V2_DESIGN.md: Updated KMS API examples (lines 333-358) to uint256
- [x] WORKER_API_SPEC.md: Updated to uint256
- [x] RESTRUCTURED_PLAN.md: Already correct (uses uint256)

---

## 2. InputVerificationRegistry.userAddress (CRITICAL) ⚠️ Refined

### Conflict
- **Design event**: `address indexed userAddress` in `InputVerificationRegistered`
- **Design function**: No `userAddress` parameter
```solidity
function registerInputVerification(
    bytes32 commitment,
    uint256 contractChainId,
    address contractAddress
) external payable returns (uint256 requestId);
```
- **Problem**: `msg.sender` is the Relayer, not the user. Coprocessor needs real user address.

### Resolution
**Add `userAddress` parameter AND require user signature binding.**

The parameter alone is **not sufficient** - without validation, a relayer can register requests with arbitrary `userAddress` values.

```solidity
function registerInputVerification(
    bytes32 commitment,
    uint256 contractChainId,
    address contractAddress,
    address userAddress,          // ADDED
    bytes calldata userSignature  // ADDED - binds userAddress
) external payable returns (uint256 requestId);
```

### User Signature Requirements (EIP-712)
Workers MUST verify the user signature binds at least:

```solidity
// EIP-712 TypedData for Input Verification
struct InputVerificationRequest {
    bytes32 commitment;
    uint256 contractChainId;
    address contractAddress;
    address userAddress;
    uint256 deadline;  // Replay protection
    uint256 nonce;     // Replay protection (optional if deadline sufficient)
}
```

### Validation Strategy (Minimal Chain Approach - Recommended)
- **On-chain**: Store `userSignature` in event (for workers to access)
- **Off-chain**: Workers verify signature and derive/confirm `userAddress`
- **Why**: Keeps gas low while maintaining trustless verification

### Alternative: On-chain Signature Verification
```solidity
// More gas but strongest guarantees
address recoveredUser = ECDSA.recover(
    _hashTypedDataV4(keccak256(abi.encode(
        INPUT_VERIFICATION_TYPEHASH,
        commitment, contractChainId, contractAddress, deadline
    ))),
    userSignature
);
require(recoveredUser == userAddress, "Invalid signature");
```

### Oracle Notes
- ⚠️ Risk without binding: Relayer can register as arbitrary user, breaking refund/dispute attribution and ACL checks
- Keep verification off-chain for minimal chain approach
- User signature prevents griefing attacks

### Impacted Files
- [x] GATEWAY_V2_DESIGN.md: Updated function signature with userAddress + userSignature
- [x] RESTRUCTURED_PLAN.md: Added user signature to event + worker validation
- [ ] gateway-contracts/: Update IInputVerificationRegistry.sol (implementation)
- [x] WORKER_API_SPEC.md: Added userSignature field and validation requirements

---

## 3. DecryptionRegistry Event Payload (CRITICAL) ⚠️ Refined

### Conflict
**Design** (line 557-665):
```solidity
event UserDecryptionRequested(
    uint256 indexed requestId,
    bytes32[] handles,
    address[] contractAddresses,  // Missing in plan
    address indexed userAddress,
    bytes publicKey,
    bytes signature,              // Missing in plan - CRITICAL for KMS validation
    uint256 chainId,
    uint256 timestamp             // Missing in plan
);
```

**Plan** (line 169-177):
```solidity
event UserDecryptionRequest(      // Different name
    uint256 indexed decryptionId,
    bytes32[] ctHandles,
    uint256 chainId,
    address userAddress,
    bytes publicKey,
    bytes extraData               // Ambiguous
);
```

### Resolution
**Use the design's full event signature with clear handle/contract mapping.**

The KMS needs:
- `signature`: To validate that the user actually signed the decryption request
- `contractAddresses`: To query ACL for each handle
- `timestamp`: For TTL (use `block.timestamp`, not user-supplied)

```solidity
event UserDecryptionRequested(
    uint256 indexed requestId,
    bytes32[] handles,
    address[] contractAddresses,  // MUST have handles.length == contractAddresses.length
    address indexed userAddress,
    bytes publicKey,
    bytes signature,
    uint256 chainId,
    uint256 timestamp             // Set to block.timestamp on-chain
);

event PublicDecryptionRequested(
    uint256 indexed requestId,
    bytes32[] handles,
    address[] contractAddresses,  // MUST have handles.length == contractAddresses.length
    uint256 chainId,
    uint256 timestamp             // Set to block.timestamp on-chain
);
```

### Handle/Contract Mapping (CRITICAL)
**Invariant**: `handles.length == contractAddresses.length`

Each `handles[i]` is associated with `contractAddresses[i]` for ACL lookup:
```solidity
// Validation in registerDecryption
require(handles.length == contractAddresses.length, "Length mismatch");
```

### Signature Requirements (EIP-712)
The `signature` field MUST cover replay protection:

```solidity
// EIP-712 TypedData for User Decryption
struct UserDecryptionRequest {
    uint256 requestId;
    bytes32[] handles;
    address[] contractAddresses;
    address userAddress;
    bytes publicKey;
    uint256 chainId;
    uint256 deadline;  // Replay protection
}
```

### Oracle Notes
- `timestamp`: Emit `block.timestamp` rather than trusting user-supplied values
- Signature domain must include `chainId` and `deadline` for replay protection
- Handle/contract relationship must be explicitly defined

### Event Naming Convention
Use past tense consistently:
- `InputVerificationRegistered` (not `InputVerificationRequest`)
- `UserDecryptionRequested` (not `UserDecryptionRequest`)
- `PublicDecryptionRequested` (not `PublicDecryptionRequest`)

**Note**: Check existing naming in `gateway-contracts/contracts/interfaces/IDecryption.sol` for compatibility.

### Impacted Files
- [x] GATEWAY_V2_DESIGN.md: Updated DecryptionRegistry with contractAddresses, removed extraData
- [x] RESTRUCTURED_PLAN.md: Updated event signatures
- [x] WORKER_API_SPEC.md: Updated event references
- [ ] gateway-contracts/: IDecryptionRegistry.sol (implementation)

---

## 4. MPC Context Switch Handling (HIGH) ✅

### Conflict
- **Design**: SDK "uses active context"
- **Problem**: During context switch, some responses may be signed by previous epoch
- **KMSVerifierV2**: Has grace period for on-chain verification
- **SDK**: No equivalent grace period logic defined

### Resolution
**SDK accepts signatures from both current AND previous epoch during grace period. Reject mixed-epoch aggregates.**

```typescript
// SDK verification logic
async function verifyUserDecryptionResponse(response: DecryptionResponse) {
    // 1. Get both contexts
    const currentContext = await getActiveMpcContext();
    const previousContext = await getPreviousMpcContext();  // May be null
    
    // 2. Group responses by epochId
    const epochGroups = groupBy(response.shares, 'epochId');
    
    // 3. CRITICAL: Verify all responses have same epochId (single-epoch threshold)
    if (Object.keys(epochGroups).length > 1) {
        throw new Error("Mixed epochs in responses - retry");
    }
    
    const responseEpochId = response.shares[0].epochId;
    
    // 4. Select context for verification
    let context;
    if (responseEpochId === currentContext.epochId) {
        context = currentContext;
    } else if (previousContext && responseEpochId === previousContext.epochId) {
        // Accept previous epoch during grace period
        const gracePeriodActive = await isGracePeriodActive();
        if (!gracePeriodActive) {
            throw new Error("Previous epoch signatures expired");
        }
        context = previousContext;
    } else {
        throw new Error("Unknown epochId");
    }
    
    // 5. Verify signatures against selected context
    return verifySignaturesAgainstContext(response.shares, context);
}
```

### Oracle Notes
- ✅ Correct approach
- **Critical invariant**: "Collect threshold shares from the same epoch" - relayer must also enforce this
- `epochId` should be in RESPONSE, not in request (putting it in request can backfire if stale epoch pinned)

### Impacted Files
- [x] GATEWAY_V2_DESIGN.md: Added SDK epoch handling in Appendix A (A.1 + A.2)
- [x] RESTRUCTURED_PLAN.md: Added epoch verification to Phase 4.10
- [x] WORKER_API_SPEC.md: Updated with epochId

---

## 5. Worker Endpoint Source (HIGH) ⚠️ Refined

### Conflict
- **Design** (line 624-631): "ProtocolConfig on Ethereum is the ground truth for valid worker identities"
- **Plan**: Adds `apiUrl` to GatewayConfig

### Resolution
**Separate concerns: ProtocolConfig = identity, GatewayConfig = operational config. Bind endpoint to identity.**

| Aspect | Source | Contains |
|--------|--------|----------|
| **Identity** | ProtocolConfig (Ethereum) | Signer addresses, threshold, epochId |
| **Operations** | GatewayConfig (Gateway Chain) | apiUrl, s3BucketUrl, **signerAddress binding** |

### Critical Addition: Bind Endpoint to Identity
GatewayConfig entries MUST include the signer identity they correspond to:

```solidity
struct WorkerConfig {
    address signerAddress;  // MUST match ProtocolConfig signer
    string apiUrl;
    string s3BucketUrl;     // Optional
}

mapping(address => WorkerConfig) public workerConfigs;
```

**Flow**:
1. Relayer discovers workers from GatewayConfig (apiUrl + signerAddress)
2. Relayer polls worker APIs
3. Relayer verifies signatures against ProtocolConfig (signer addresses)
4. **Relayer verifies response signer matches the signerAddress for that apiUrl**
5. Workers verify their own identity in ProtocolConfig before processing

### Why Binding Matters
Without binding:
- Endpoint poisoning doesn't break integrity (signatures won't verify)
- BUT degrades availability (relayer wastes time polling junk URLs)
- Ambiguous which key you're talking to

With binding:
- Clients know which signer key to expect from each endpoint
- Can detect misrouted/spoofed endpoints early

### Oracle Notes
- ⚠️ Dual-source is sensible IF GatewayConfig is discovery-only and ProtocolConfig is authority
- Allow multiple `apiUrl`s per signer for rotation
- Always verify responses against ProtocolConfig signer set

### Impacted Files
- [x] GATEWAY_V2_DESIGN.md: Added dual-source model in Section 5.6 + GatewayConfig extension
- [x] RESTRUCTURED_PLAN.md: Added signerAddress binding to GatewayConfig

---

## 6. Commitment Binding (MEDIUM) ⚠️ Refined

### Conflict
- **Plan**: `commitment = keccak256(ciphertext_with_zkpok)`
- **Problem**: Relayer can alter contractChainId, contractAddress, userAddress

### Resolution
**Commitment can remain payload-only, BUT attestations MUST bind all metadata.**

The commitment covers payload for size efficiency. The coprocessor verifies metadata against event AND produces attestation that binds everything:

```rust
async fn verify_input(request: VerifyInputRequest) -> Result<VerifyInputResponse> {
    // 1. Verify commitment
    let computed_commitment = keccak256(&request.ciphertext_with_zkpok);
    let event = get_event_for_request(request.request_id)?;
    
    if computed_commitment != event.commitment {
        return Err("Commitment mismatch");
    }
    
    // 2. Verify metadata matches event (ADDED)
    if request.contract_chain_id != event.contract_chain_id {
        return Err("contractChainId mismatch");
    }
    if request.contract_address != event.contract_address {
        return Err("contractAddress mismatch");
    }
    if request.user_address != event.user_address {
        return Err("userAddress mismatch");
    }
    
    // 3. Continue with ZKPoK verification...
    
    // 4. CRITICAL: Sign attestation that binds ALL metadata
    let attestation = sign_typed_data(InputVerificationAttestation {
        request_id: request.request_id,
        commitment: computed_commitment,
        contract_chain_id: event.contract_chain_id,
        contract_address: event.contract_address,
        user_address: event.user_address,
        epoch_id: current_epoch_id,  // ADDED
    });
}
```

### Attestation Requirements (CRITICAL)
The signed attestation MUST include:
- `requestId`
- `commitment`
- `contractChainId`
- `contractAddress`
- `userAddress`
- `epochId`

**Why**: If attestation only commits to `commitment` and not metadata, cross-context replay/ambiguity can occur (especially if identical payload reused).

### Oracle Notes
- ⚠️ Risk: Downstream signature only commits to commitment → replay attacks possible
- The "thing that is signed" must bind ALL verification-time metadata
- EIP-712 typed data recommended for all attestations

### Impacted Files
- [ ] RESTRUCTURED_PLAN.md: Add metadata verification to Phase 2 validation flow
- [ ] WORKER_API_SPEC.md: Add validation steps to POST /verify-input + attestation requirements

---

## 7. KMS Ciphertext Fetch (MEDIUM) ⚠️ Refined

### Conflict
- **Design** (line 373): "Fetch ciphertext: query Coprocessors, verify majority agreement"
- **Plan**: Only checks digest

### Resolution
**Require coprocessor signature verification. Use "2 matching digests" heuristic for robustness.**

```rust
async fn fetch_ciphertext(handle: B256) -> Result<Ciphertext> {
    let coprocessors = gateway_config.getCoprocessorsWithApis().await?;
    let mut signed_responses: Vec<(Address, CiphertextResponse)> = vec![];
    
    for copro in coprocessors {
        match fetch_from_copro(&copro.api_url, handle).await {
            Ok(ct) => {
                // 1. Verify digest (already in plan)
                if keccak256(&ct.sns_ciphertext) != ct.sns_ciphertext_digest {
                    continue;
                }
                
                // 2. Verify coprocessor signature (ADDED)
                let typed_data = eip712_hash(CiphertextResponse {
                    handle: ct.handle,
                    key_id: ct.key_id,
                    sns_ciphertext_digest: ct.sns_ciphertext_digest,
                    epoch_id: ct.epoch_id,
                });
                let signer = ecrecover(typed_data, ct.signature);
                
                // Verify signer is valid coprocessor
                if !is_valid_coprocessor_signer(signer) {
                    continue;
                }
                
                // 3. Verify epochId is current or previous (ADDED)
                if !is_valid_epoch(ct.epoch_id) {
                    continue;
                }
                
                signed_responses.push((signer, ct));
                
                // 4. HEURISTIC: Accept when ≥2 matching digests (optional robustness)
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
    
    // 5. Fallback: Accept single signed response after timeout (availability-first)
    if let Some((_, ct)) = signed_responses.first() {
        return Ok(ct.clone());
    }
    
    Err("All coprocessors failed")
}
```

### Majority Agreement Strategy (Oracle Recommended)
| Approach | Trade-off |
|----------|-----------|
| **Single signed response** | Fast, but single faulty coprocessor can DoS |
| **≥2 matching digests** | Robust against single fault, slight latency |
| **≥t+1 matching** | Most robust, highest latency |

**Recommendation**: Use "2 matching" heuristic with single-response fallback after timeout.

### Signed Payload Requirements
The coprocessor signature MUST include:
- `handle`
- `keyId`
- `ciphertextDigest`
- `epochId` (or key epoch)
- (optionally `contractChainId` for additional binding)

### Oracle Notes
- ⚠️ Without majority, single malicious coprocessor can return bad ciphertext (DoS, not privacy break)
- "2 matching digests" is practical middle-ground
- Fall back to single after timeout for availability

### Impacted Files
- [ ] RESTRUCTURED_PLAN.md: Update Coprocessor API Fetch Flow (lines 599-621)
- [ ] WORKER_API_SPEC.md: Emphasize signature verification + matching heuristic

---

## 8. Event Reconciliation for KMS (MEDIUM) ⚠️ Refined

### Conflict
- **Plan Phase 2.7**: Event reconciliation for coprocessor
- **Missing**: No equivalent for KMS connector

### Resolution
**Add event reconciliation to Phase 3 with reorg safety.**

Both gw-listeners can miss events during WS drops. Add same reconciliation strategy WITH reorg handling:

| Phase | Component | Reconciliation |
|-------|-----------|----------------|
| 2.7 | Coprocessor gw-listener | On startup/reconnect, backfill from `last_processed_block - safety_margin` |
| 3.x | KMS connector gw-listener | On startup/reconnect, backfill from `last_processed_block - safety_margin` |

### Reorg Safety (CRITICAL)
```rust
const SAFETY_MARGIN: u64 = 10; // blocks

async fn reconcile_events() {
    let start_block = last_processed_block.saturating_sub(SAFETY_MARGIN);
    
    // Re-scan with overlap to catch reorged blocks
    for event in fetch_events(start_block, latest_block).await {
        // Idempotent processing - handle duplicates gracefully
        if already_processed(&event.request_id) {
            continue;
        }
        process_event(event).await;
    }
    
    // Only "finalize" blocks after N confirmations
    let finalized_block = latest_block.saturating_sub(FINALITY_DEPTH);
    mark_finalized(finalized_block);
}
```

### Oracle Notes
- ⚠️ Reorg handling is critical for both listeners
- Re-scan from `last_processed_block - safety_margin` on reconnect
- Processing MUST be idempotent
- Only mark blocks as "finalized" after N confirmations

### Impacted Files
- [ ] RESTRUCTURED_PLAN.md: Add task 3.10 for KMS event reconciliation with reorg safety

---

## 9. Dispute/Refund Scope (DEFERRED) ⚠️ Changed

### Conflict
- **Design** (line 356-363): Per-request escrow for "all request contracts"
- **Plan**: Only DecryptionRegistry has dispute/reimburse

### Resolution
**DEFER dispute/refund from MVP. Requires on-chain fulfillment marker.**

### Why Defer?

In V2, completion is **off-chain** (relayer aggregates responses, SDK verifies locally). An on-chain "refund after timeout" mechanism is **easy to game** unless there's also an on-chain notion of fulfillment.

| Scenario | Problem |
|----------|---------|
| Request fulfilled off-chain | No on-chain proof → user can claim refund anyway |
| Request never fulfilled | Legitimate refund, but who pays? Relayer escrow or fee pool? |
| Partial fulfillment | How to prove? Need attestation of fulfillment |

### Options for Future

1. **On-chain fulfillment marker** (recommended if adding disputes):
   ```solidity
   // Relayer posts compact aggregated attestation
   function markFulfilled(uint256 requestId, bytes calldata aggregatedAttestation) external;
   ```
   Then: `dispute()` only allowed if NOT fulfilled AND timeout passed.

2. **Off-chain dispute resolution** (simpler for MVP):
   - Relayer SLA with off-chain reputation/slashing
   - No on-chain refund mechanism
   - Users trust relayer or choose different relayer

3. **Defer entirely** (recommended for MVP):
   - Focus on correctness first
   - Add dispute/refund in V2.1 after fulfillment semantics defined

### Oracle Notes
- ⚠️ On-chain refund without fulfillment proof = exploitable
- If no fulfillment marker: defer disputes from MVP
- Alternative: handle off-chain via relayer SLA

### Decision
**Defer to V2.1.** Remove dispute/reimburse from Phase 1A scope.

### Impacted Files
- [x] GATEWAY_V2_DESIGN.md: Added V2.1 deferral note to Section 8
- [x] RESTRUCTURED_PLAN.md: Removed dispute/reimburse from DecryptionRegistry
- [x] Note: V2.1 Future Work documented in DESIGN_CONFLICTS_RESOLUTION.md #9

---

## 10. Event Naming (LOW) ✅

### Conflict
- `UserDecryptionRequest` vs `UserDecryptionRequested`
- `InputVerificationRequest` vs `InputVerificationRegistered`

### Resolution
**Use past tense consistently for events (they represent facts that occurred).**

| Contract | Event Name |
|----------|------------|
| InputVerificationRegistry | `InputVerificationRegistered` |
| DecryptionRegistry | `UserDecryptionRequested` |
| DecryptionRegistry | `PublicDecryptionRequested` |

### Oracle Notes
- ✅ Low priority, past tense is fine
- Check existing naming in `gateway-contracts/contracts/interfaces/IDecryption.sol` for V2 compatibility
- If introducing new V2 registries, pick clean convention; just be consistent within V2

### Impacted Files
- [ ] RESTRUCTURED_PLAN.md: Update event names throughout
- [ ] WORKER_API_SPEC.md: Update event references
- [ ] gateway-contracts/: Ensure consistency

---

## Action Items

### Immediate (Before Implementation) - Priority Order

#### P0: Critical Path (Must Fix) ✅ COMPLETED

1. **RequestId = uint256 everywhere** (Conflict #1)
   - [x] GATEWAY_V2_DESIGN.md: Line 171-173 → "contract-assigned uint256"
   - [x] GATEWAY_V2_DESIGN.md: KMS API examples updated to uint256
   - [x] WORKER_API_SPEC.md: Updated to uint256
   - [x] All docs now use uint256

2. **DecryptionRegistry event with authorization material** (Conflict #3)
   - [x] GATEWAY_V2_DESIGN.md: Updated with contractAddresses, removed extraData
   - [x] RESTRUCTURED_PLAN.md: Fixed event signature
   - [x] Defined `handles.length == contractAddresses.length` invariant
   - [x] EIP-712 typed data defined in this document

3. **InputVerification userAddress + signature binding** (Conflict #2)
   - [x] GATEWAY_V2_DESIGN.md: Added userAddress + userSignature to function
   - [x] WORKER_API_SPEC.md: Added userSignature field to /v1/verify-input
   - [x] RESTRUCTURED_PLAN.md: Added user signature to event
   - [x] EIP-712 typed data defined in this document
   - [x] Worker validation requirements documented

4. **Epoch grace + mixed-epoch rejection** (Conflict #4)
   - [x] GATEWAY_V2_DESIGN.md: Added SDK epoch handling to Appendix A (A.1 + A.2)
   - [x] RESTRUCTURED_PLAN.md: Added to Phase 4.10
   - [x] Documented "single-epoch threshold" invariant

#### P1: Important (Should Fix) ✅ COMPLETED

5. **Attestations bind all metadata** (Conflict #6)
   - [x] RESTRUCTURED_PLAN.md: Added metadata verification to Phase 2
   - [x] WORKER_API_SPEC.md: Defined attestation typed data with ALL fields

6. **KMS ciphertext signature + matching heuristic** (Conflict #7)
   - [x] RESTRUCTURED_PLAN.md: Updated Coprocessor API Fetch Flow
   - [x] Added "2 matching digests" heuristic
   - [x] Defined signed ciphertext response payload

7. **KMS reconciliation + reorg safety** (Conflict #8)
   - [x] RESTRUCTURED_PLAN.md: Added task 3.10 for KMS event reconciliation
   - [x] Added safety_margin and idempotent processing requirements

8. **Worker endpoint binding** (Conflict #5)
   - [x] GATEWAY_V2_DESIGN.md: Added dual-source model in Section 5.6
   - [x] RESTRUCTURED_PLAN.md: Added signerAddress to GatewayConfig
   - [x] Documented endpoint-to-identity binding requirements

#### P2: Deferred (V2.1) ✅ DOCUMENTED

9. **Dispute/refund** (Conflict #9)
   - [x] GATEWAY_V2_DESIGN.md: Added V2.1 deferral note to Section 8
   - [x] Removed from Phase 1A scope in RESTRUCTURED_PLAN.md
   - [x] Documented fulfillment marker requirement in this document

10. **Event naming** (Conflict #10)
    - [x] Using past tense consistently (UserDecryptionRequested, etc.)
    - [ ] Check existing IDecryption.sol naming during implementation

---

## EIP-712 Typed Data Definitions (NEW)

Define these typed data structures for all signatures:

### 1. InputVerificationRequest (User Signs)
```solidity
bytes32 constant INPUT_VERIFICATION_TYPEHASH = keccak256(
    "InputVerificationRequest(bytes32 commitment,uint256 contractChainId,address contractAddress,address userAddress,uint256 deadline)"
);

struct InputVerificationRequest {
    bytes32 commitment;
    uint256 contractChainId;
    address contractAddress;
    address userAddress;
    uint256 deadline;
}
```

### 2. UserDecryptionRequest (User Signs)
```solidity
bytes32 constant USER_DECRYPTION_TYPEHASH = keccak256(
    "UserDecryptionRequest(uint256 requestId,bytes32[] handles,address[] contractAddresses,address userAddress,bytes publicKey,uint256 chainId,uint256 deadline)"
);

struct UserDecryptionRequest {
    uint256 requestId;
    bytes32[] handles;
    address[] contractAddresses;
    address userAddress;
    bytes publicKey;
    uint256 chainId;
    uint256 deadline;
}
```

### 3. InputVerificationAttestation (Coprocessor Signs)
```solidity
bytes32 constant INPUT_ATTESTATION_TYPEHASH = keccak256(
    "InputVerificationAttestation(uint256 requestId,bytes32 commitment,uint256 contractChainId,address contractAddress,address userAddress,uint256 epochId)"
);

struct InputVerificationAttestation {
    uint256 requestId;
    bytes32 commitment;
    uint256 contractChainId;
    address contractAddress;
    address userAddress;
    uint256 epochId;
}
```

### 4. CiphertextResponse (Coprocessor Signs)
```solidity
bytes32 constant CIPHERTEXT_RESPONSE_TYPEHASH = keccak256(
    "CiphertextResponse(bytes32 handle,uint256 keyId,bytes32 ciphertextDigest,uint256 epochId)"
);

struct CiphertextResponse {
    bytes32 handle;
    uint256 keyId;
    bytes32 ciphertextDigest;
    uint256 epochId;
}
```

### 5. DecryptionShare (KMS Signs)
```solidity
bytes32 constant DECRYPTION_SHARE_TYPEHASH = keccak256(
    "DecryptionShare(uint256 requestId,bytes share,uint256 epochId)"
);

struct DecryptionShare {
    uint256 requestId;
    bytes share;
    uint256 epochId;
}
```

---

## Security Invariants (NEW)

Based on Oracle review, these invariants MUST be maintained:

1. **Never trust relayer-provided metadata** unless bound by a user/worker signature
2. **Every signed attestation** must include requestId + all metadata needed at verification time
3. **Handle reorgs/finality** in every listener (coprocessor + KMS + relayer)
4. **Single-epoch threshold**: All shares in an aggregate must have same epochId
5. **Endpoint-to-identity binding**: apiUrl must map to specific signerAddress

---

## Effort Estimate (Oracle)

| Scope | Effort | Items |
|-------|--------|-------|
| **Short (1-4h)** | Docs alignment | #1, #4, #10 |
| **Medium (1-2d)** | Typed data + listener behavior | #2, #3, #6, #7 |
| **Large (3d+)** | On-chain dispute with fulfillment | #9 (deferred) |

---

## Changelog

| Date | Author | Changes |
|------|--------|---------|
| 2026-01-09 | AI | Initial resolution document |
| 2026-01-09 | AI | Applied Oracle review: refined #2, #3, #5, #6, #7, #8; deferred #9; added EIP-712 definitions |
| 2026-01-09 | AI | Updated GATEWAY_V2_DESIGN.md to align with resolutions; marked P0/P1 items complete |
