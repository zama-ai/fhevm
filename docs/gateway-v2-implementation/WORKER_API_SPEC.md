# Worker API Specification

**Version**: 1.2  
**Status**: Draft  
**Last Updated**: January 2026 (Oracle Review Applied)

> **This is the authoritative API reference** for KMS and Coprocessor HTTP endpoints in Gateway V2.
> Referenced by [RESTRUCTURED_PLAN.md](./RESTRUCTURED_PLAN.md) phases 2, 3, and 4.

This document specifies the HTTP APIs that KMS nodes and Coprocessors expose in
Gateway v2. These APIs replace on-chain response transactions.

---

## Table of Contents

1. [Overview](#1-overview)
2. [Common Conventions](#2-common-conventions)
3. [KMS Node API](#3-kms-node-api)
4. [Coprocessor API](#4-coprocessor-api)
5. [Error Handling](#5-error-handling)
6. [Rate Limiting](#6-rate-limiting)
7. [Security Considerations](#7-security-considerations)

---

## 1. Overview

### 1.1 Design Principles

| Principle | Description |
|-----------|-------------|
| **Event-driven processing** | Workers only process requests registered on-chain (Gateway or Host Chain events) |
| **Stateless API** | APIs are read-only; workers compute and cache results internally |
| **Deterministic RequestIds** | RequestId = contract-assigned uint256 (from registry counter) |
| **Signature-based trust** | All responses include EIP-712 signatures for verification |

### 1.2 API Discovery

Worker API endpoints are discoverable via `GatewayConfig.sol`:

| Worker Type | Field | Example |
|-------------|-------|---------|
| KMS Node | `KmsNode.apiUrl` | `https://kms-1.zama.ai/api` |
| Coprocessor | `Coprocessor.apiUrl` | `https://coprocessor-1.zama.ai/api` |

See [Section 5.4.1 of GATEWAY_V2_DESIGN.md](../../GATEWAY_V2_DESIGN.md) for
GatewayConfig extension details.

---

## 2. Common Conventions

### 2.1 Base URL

All endpoints are prefixed with `/v1/`. Future breaking changes will use `/v2/`.

### 2.2 Content Types

| Direction | Content-Type |
|-----------|--------------|
| Request | `application/json` |
| Response | `application/json` |

### 2.3 Common Headers

| Header | Required | Description |
|--------|----------|-------------|
| `X-Request-ID` | Optional | Client-provided correlation ID for tracing |
| `X-Worker-Address` | Response | Worker's signer address (for debugging) |

### 2.4 Response Status

All responses include a `status` field:

| Status | Meaning |
|--------|---------|
| `ready` | Result computed and available |
| `pending` | Request observed, processing in progress |
| `not_found` | Request not observed or expired |
| `rejected` | Request invalid (bad signature, ACL denied, etc.) |

### 2.5 Timestamps

All timestamps are Unix epoch seconds (uint64).

### 2.6 Hex Encoding

All byte arrays are hex-encoded with `0x` prefix:
- `bytes32`: 66 characters (`0x` + 64 hex chars)
- `address`: 42 characters (`0x` + 40 hex chars)
- Variable-length `bytes`: `0x` + even number of hex chars

---

## 3. KMS Node API

KMS nodes expose APIs for retrieving decryption shares after observing
`DecryptionRequested` events.

### 3.1 GET /v1/share/{requestId}

Retrieves the decryption share for a specific request.

#### Request

```
GET /v1/share/{requestId}
```

| Parameter | Type | Location | Description |
|-----------|------|----------|-------------|
| `requestId` | `uint256` | Path | The request ID (contract-assigned) |

#### Response: Ready (User Decryption)

```json
{
  "status": "ready",
  "requestId": "0x1234...5678",
  "requestType": "user_decryption",
  "shareIndex": 0,
  "encryptedShare": "0xabcd...ef00",
  "epochId": 1,
  "signature": "0x...",
  "signerAddress": "0x...",
  "anchorBlock": {
    "number": 12345678,
    "hash": "0x..."
  },
  "timestamp": 1704067200,
  "ttl": 3600
}
```

#### Response: Ready (Public Decryption)

```json
{
  "status": "ready",
  "requestId": "0x1234...5678",
  "requestType": "public_decryption",
  "decryptedValue": "0x000000000000000000000000000000000000000000000000000000000000002a",
  "epochId": 1,
  "signature": "0x...",
  "signerAddress": "0x...",
  "anchorBlock": {
    "number": 12345678,
    "hash": "0x..."
  },
  "timestamp": 1704067200,
  "ttl": 3600
}
```

#### Response: Pending

```json
{
  "status": "pending",
  "requestId": "0x1234...5678",
  "observedAt": 1704067190,
  "estimatedReadyAt": 1704067200
}
```

#### Response: Not Found

```json
{
  "status": "not_found",
  "requestId": "0x1234...5678",
  "reason": "Request not observed or expired"
}
```

#### Response: Rejected

```json
{
  "status": "rejected",
  "requestId": "0x1234...5678",
  "reason": "ACL check failed: requester not allowed",
  "errorCode": "ACL_DENIED"
}
```

#### Response Fields

| Field | Type | Description |
|-------|------|-------------|
| `status` | `string` | One of: `ready`, `pending`, `not_found`, `rejected` |
| `requestId` | `uint256` | Echo of the request ID |
| `requestType` | `string` | `user_decryption` or `public_decryption` |
| `shareIndex` | `uint256` | Index of this KMS node in the signer set (user decryption only) |
| `encryptedShare` | `bytes` | Re-encrypted share for user's public key (user decryption only) |
| `decryptedValue` | `bytes` | ABI-encoded plaintext value (public decryption only) |
| `epochId` | `uint256` | **REQUIRED**: MPC context epoch when share was computed (for replay protection) |
| `signature` | `bytes` | EIP-712 signature over the result (includes epochId) |
| `signerAddress` | `address` | KMS node's signer address |
| `anchorBlock.number` | `uint256` | Block number when request was observed |
| `anchorBlock.hash` | `bytes32` | Block hash for reorg detection |
| `timestamp` | `uint64` | Unix timestamp when result was computed |
| `ttl` | `uint64` | Seconds until result expires |
| `estimatedReadyAt` | `uint64` | Estimated Unix timestamp when result will be ready |
| `reason` | `string` | Human-readable error description |
| `errorCode` | `string` | Machine-readable error code |

### 3.2 Signature Format (User Decryption)

KMS nodes sign user decryption results using EIP-712:

```
Domain:
{
  name: "FHEVM",
  version: "1",
  chainId: <gateway_chain_id>,
  verifyingContract: <decryption_contract_address>
}

Type:
UserDecryptionShare(
  uint256 requestId,
  uint256 shareIndex,
  bytes encryptedShare,
  uint256 epochId,        // REQUIRED: Current MPC context epoch for replay protection
  bytes extraData
)
```

> **IMPORTANT (V2)**: The `epochId` field MUST be included to prevent cross-epoch replay attacks.
> The Relayer MUST verify that all collected shares have the same `epochId` before aggregating.

### 3.3 Signature Format (Public Decryption)

Public decryption uses the same format as V1 (compatible with KMSVerifier):

```
Domain:
{
  name: "Decryption",
  version: "1",
  chainId: <gateway_chain_id>,
  verifyingContract: <decryption_contract_address>
}

Type:
PublicDecryptVerification(
  bytes32[] ctHandles,
  bytes decryptedResult,
  bytes extraData
)
```

### 3.4 GET /v1/health

Health check endpoint for load balancers and monitoring.

#### Request

```
GET /v1/health
```

#### Response

```json
{
  "status": "healthy",
  "version": "1.0.0",
  "signerAddress": "0x...",
  "uptime": 86400,
  "lastBlockProcessed": 12345678,
  "pendingRequests": 42
}
```

| Field | Type | Description |
|-------|------|-------------|
| `status` | `string` | `healthy`, `degraded`, or `unhealthy` |
| `version` | `string` | API version |
| `signerAddress` | `address` | This KMS node's signer address |
| `uptime` | `uint64` | Seconds since service started |
| `lastBlockProcessed` | `uint256` | Latest block number processed |
| `pendingRequests` | `uint64` | Number of requests being processed |

---

## 4. Coprocessor API

Coprocessors expose APIs for input verification and ciphertext retrieval.

### 4.1 POST /v1/verify-input

Receives input verification payload from Relayer after Gateway registration.

**Note**: Coprocessors only process this request if they have observed a matching
`InputVerificationRegistered` event with the correct commitment.

#### Request

```
POST /v1/verify-input
Content-Type: application/json
```

```json
{
  "requestId": "0x1234...5678",
  "ciphertextWithZkpok": "0x...",
  "contractChainId": 1,
  "contractAddress": "0x...",
  "userAddress": "0x...",
  "userSignature": "0x...",
  "commitment": "0x..."
}
```

| Field | Type | Description |
|-------|------|-------------|
| `requestId` | `uint256` | Request ID from Gateway event |
| `ciphertextWithZkpok` | `bytes` | Encrypted input with zero-knowledge proof |
| `contractChainId` | `uint256` | Target host chain ID |
| `contractAddress` | `address` | Target contract address |
| `userAddress` | `address` | User who submitted the input |
| `userSignature` | `bytes` | **REQUIRED**: EIP-712 signature from user binding the request |
| `commitment` | `bytes32` | Commitment from Gateway event (for validation) |

**User Signature Validation**: Coprocessors MUST verify `userSignature` binds
`(commitment, contractChainId, contractAddress, userAddress, deadline)`. This
prevents the Relayer from registering requests with arbitrary `userAddress`.
The signature can be fetched from the Gateway event or passed in the request.

#### Response: Verified

```json
{
  "status": "verified",
  "requestId": "0x1234...5678",
  "handles": [
    "0xabc...123",
    "0xdef...456"
  ],
  "epochId": 1,
  "signature": "0x...",
  "signerAddress": "0x...",
  "timestamp": 1704067200
}
```

#### Response: Rejected

```json
{
  "status": "rejected",
  "requestId": "0x1234...5678",
  "reason": "ZKPoK verification failed",
  "errorCode": "ZKPOK_INVALID"
}
```

#### Response: Commitment Mismatch

```json
{
  "status": "rejected",
  "requestId": "0x1234...5678",
  "reason": "Payload hash does not match on-chain commitment",
  "errorCode": "COMMITMENT_MISMATCH"
}
```

#### Response Fields

| Field | Type | Description |
|-------|------|-------------|
| `status` | `string` | `verified`, `pending`, `rejected` |
| `requestId` | `uint256` | Echo of the request ID |
| `handles` | `bytes32[]` | Derived ciphertext handles |
| `epochId` | `uint256` | **REQUIRED**: MPC context epoch when verification was performed |
| `signature` | `bytes` | EIP-712 signature over handles (includes epochId) |
| `signerAddress` | `address` | Coprocessor's signer address |
| `timestamp` | `uint64` | Unix timestamp of verification |
| `reason` | `string` | Error description (on rejection) |
| `errorCode` | `string` | Machine-readable error code |

### 4.2 Signature Format (Input Verification)

Coprocessors sign input verification results using EIP-712:

```
Domain:
{
  name: "FHEVM",
  version: "1",
  chainId: <gateway_chain_id>,
  verifyingContract: <input_verification_contract_address>
}

Type:
InputVerificationAttestation(
  uint256 requestId,
  bytes32 commitment,          // ADDED: Bind commitment
  bytes32[] handles,
  uint256 contractChainId,
  address contractAddress,
  address userAddress,
  uint256 epochId              // REQUIRED: Current epoch for replay protection
)
```

> **IMPORTANT (V2 - Oracle Review)**:
> 1. The signature MUST bind ALL metadata fields, not just handles. This prevents cross-context replay attacks.
> 2. The `epochId` field MUST be included to bind the verification to a specific MPC context epoch.
> 3. See [DESIGN_CONFLICTS_RESOLUTION.md](./DESIGN_CONFLICTS_RESOLUTION.md) Section 6 for rationale.

### 4.3 GET /v1/ciphertext/{handle}

Retrieves ciphertext material for KMS nodes during decryption.

#### Request

```
GET /v1/ciphertext/{handle}
```

| Parameter | Type | Location | Description |
|-----------|------|----------|-------------|
| `handle` | `bytes32` | Path | Ciphertext handle (hex-encoded) |

#### Response: Found

```json
{
  "status": "found",
  "handle": "0x1234...5678",
  "keyId": 1,
  "snsCiphertext": "0x...",
  "snsCiphertextDigest": "0x...",
  "epochId": 1,
  "timestamp": 1704067200,
  "signature": "0x...",
  "signerAddress": "0x..."
}
```

#### Response: Not Found

```json
{
  "status": "not_found",
  "handle": "0x1234...5678",
  "reason": "Ciphertext not stored by this coprocessor"
}
```

#### Response Fields

| Field | Type | Description |
|-------|------|-------------|
| `status` | `string` | `found` or `not_found` |
| `handle` | `bytes32` | Echo of the handle |
| `keyId` | `uint256` | FHE key version used for encryption |
| `snsCiphertext` | `bytes` | Switch-and-squash ciphertext material |
| `snsCiphertextDigest` | `bytes32` | Keccak256 hash of snsCiphertext |
| `epochId` | `uint256` | **REQUIRED**: MPC context epoch (for signature verification) |
| `timestamp` | `uint64` | When ciphertext was stored |
| `signature` | `bytes` | Signature over the ciphertext data (see format below) |
| `signerAddress` | `address` | Coprocessor's signer address |

### 4.5 Signature Format (Ciphertext Response)

KMS nodes MUST verify the coprocessor signature when fetching ciphertexts:

```
Type:
CiphertextResponse(
  bytes32 handle,
  uint256 keyId,
  bytes32 ciphertextDigest,
  uint256 epochId
)
```

> **KMS Verification (Oracle Review)**:
> 1. Verify `snsCiphertextDigest == keccak256(snsCiphertext)`
> 2. Verify signature over CiphertextResponse typed data
> 3. Verify `signerAddress` is valid coprocessor from ProtocolConfig
> 4. Verify `epochId` is current or previous (during grace period)
> 5. **Heuristic**: Accept when ≥2 coprocessors return matching digests (robustness)
>
> See [DESIGN_CONFLICTS_RESOLUTION.md](./DESIGN_CONFLICTS_RESOLUTION.md) Section 7 for details.

### 4.4 GET /v1/health

Health check endpoint for load balancers and monitoring.

#### Request

```
GET /v1/health
```

#### Response

```json
{
  "status": "healthy",
  "version": "1.0.0",
  "signerAddress": "0x...",
  "uptime": 86400,
  "lastBlockProcessed": 12345678,
  "storedCiphertexts": 1000000,
  "pendingVerifications": 5
}
```

---

## 5. Error Handling

### 5.1 HTTP Status Codes

| Code | Meaning | When Used |
|------|---------|-----------|
| `200` | OK | Successful request (including `rejected` status) |
| `400` | Bad Request | Malformed request body or parameters |
| `404` | Not Found | Invalid endpoint |
| `429` | Too Many Requests | Rate limit exceeded |
| `500` | Internal Server Error | Unexpected server error |
| `503` | Service Unavailable | Worker not ready (starting up, syncing) |

### 5.2 Error Response Format

All error responses follow this format:

```json
{
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Too many requests. Retry after 60 seconds.",
    "retryAfter": 60
  }
}
```

### 5.3 Error Codes

| Code | Description |
|------|-------------|
| `INVALID_REQUEST_ID` | Request ID format is invalid |
| `INVALID_HANDLE` | Ciphertext handle format is invalid |
| `COMMITMENT_MISMATCH` | Payload hash doesn't match on-chain commitment |
| `ZKPOK_INVALID` | Zero-knowledge proof verification failed |
| `ACL_DENIED` | Requester not allowed to decrypt this ciphertext |
| `SIGNATURE_INVALID` | Request signature verification failed |
| `REQUEST_EXPIRED` | Request TTL has passed |
| `RATE_LIMIT_EXCEEDED` | Too many requests from this source |
| `SERVICE_UNAVAILABLE` | Worker not ready to process requests |
| `INTERNAL_ERROR` | Unexpected internal error |

---

## 6. Rate Limiting

### 6.1 Limits

| Endpoint | Limit | Window |
|----------|-------|--------|
| `GET /v1/share/{requestId}` | 100 req | Per minute per IP |
| `POST /v1/verify-input` | 50 req | Per minute per IP |
| `GET /v1/ciphertext/{handle}` | 200 req | Per minute per IP |
| `GET /v1/health` | 60 req | Per minute per IP |

### 6.2 Rate Limit Headers

| Header | Description |
|--------|-------------|
| `X-RateLimit-Limit` | Maximum requests in window |
| `X-RateLimit-Remaining` | Requests remaining in window |
| `X-RateLimit-Reset` | Unix timestamp when window resets |
| `Retry-After` | Seconds to wait (when rate limited) |

### 6.3 Retry Strategy

Clients should implement exponential backoff:

1. Initial retry after `Retry-After` header value (or 1 second)
2. Double wait time on each subsequent 429 response
3. Maximum wait time: 60 seconds
4. Maximum retries: 5

---

## 7. Security Considerations

### 7.1 Request Origin Validation

Workers MUST validate that requests correspond to on-chain events:

1. **Input Verification**: Only process `POST /v1/verify-input` after observing
   `InputVerificationRegistered` event with matching `requestId` and `commitment`

2. **Decryption**: Only compute shares after observing `DecryptionRequested`
   event with matching `requestId`

### 7.2 TLS Requirements

All API endpoints MUST be served over HTTPS with:
- TLS 1.2 or higher
- Strong cipher suites only
- Valid certificates from trusted CAs

### 7.3 Signature Verification

Clients MUST verify signatures before trusting results:
- Check signer address is in the active KMS/Coprocessor set
- Verify EIP-712 signature matches the response data
- Collect threshold signatures before using results

### 7.4 Replay Protection

- `requestId` is unique per request (derived from on-chain counter)
- Workers cache results by `requestId` and return identical responses
- Results expire after TTL (default 1 hour)

### 7.5 Block Anchoring

Responses include `anchorBlock` to enable reorg detection:
- Clients should verify `anchorBlock.hash` matches the chain
- If block was reorged, discard the result and re-request

### 7.6 Metadata Binding (Oracle Review - CRITICAL)

All signed attestations MUST bind ALL metadata fields, not just the primary data:

| Attestation | MUST Include |
|-------------|--------------|
| **InputVerificationAttestation** | requestId, commitment, handles, contractChainId, contractAddress, userAddress, epochId |
| **CiphertextResponse** | handle, keyId, ciphertextDigest, epochId |
| **DecryptionShare** | requestId, share/value, epochId |

**Why**: If signatures only bind primary data (e.g., handles), cross-context replay attacks are possible when identical payloads are reused across different requests.

### 7.7 Epoch Consistency (Oracle Review - CRITICAL)

Relayer MUST verify epoch consistency before aggregating responses:

```typescript
function verifyResponses(responses: WorkerResponse[]): void {
    const epochs = new Set(responses.map(r => r.epochId));
    if (epochs.size > 1) {
        throw new Error("Mixed epochs in responses - context switch in progress, retry");
    }
}
```

**Invariant**: All shares in an aggregate MUST have the same `epochId`.

---

## Appendix A: OpenAPI Specification

A complete OpenAPI 3.0 specification is available at:
`docs/gateway-v2-implementation/openapi/worker-api.yaml` (TBD)

---

## Appendix B: Example Flows

### B.1 User Decryption Flow

```
1. SDK → Relayer: POST /user-decrypt {handles, pubKey, sig}
2. Relayer → Gateway: requestUserDecryption(...)
3. Gateway: emit UserDecryptionRequested(requestId, ...)
4. KMS nodes observe event, compute shares
5. Relayer polls: GET /v1/share/{requestId} on each KMS node
6. Relayer aggregates responses until threshold reached
7. Relayer → SDK: {shares, signatures}
8. SDK verifies signatures against MPC context
9. SDK decrypts shares with user's private key
```

### B.2 Input Verification Flow

```
1. SDK → Relayer: POST /input-proof {ciphertext, ZKPoK}
2. Relayer computes commitment = keccak256(ciphertext || ZKPoK)
3. Relayer → Gateway: registerInputVerification(commitment, ...)
4. Gateway: emit InputVerificationRegistered(requestId, commitment, ...)
5. Relayer → Coprocessors: POST /v1/verify-input {requestId, payload}
6. Coprocessors verify commitment matches event, verify ZKPoK
7. Coprocessors store ciphertext, return signed handles
8. Relayer aggregates until threshold reached
9. Relayer → SDK: {handles, signatures}
10. SDK submits to Host Chain contract with proof
11. Host Chain InputVerifier validates signatures
```

---

_Document Version: 1.2_  
_Last Updated: January 2026 (Oracle Review Applied)_

---

## Appendix C: Related Documents

- [DESIGN_CONFLICTS_RESOLUTION.md](./DESIGN_CONFLICTS_RESOLUTION.md) — EIP-712 typed data definitions and security invariants
- [RESTRUCTURED_PLAN.md](./RESTRUCTURED_PLAN.md) — Implementation plan with phase details
- [GATEWAY_V2_DESIGN.md](../../GATEWAY_V2_DESIGN.md) — Original design specification
