# Errors

When something goes wrong, the SDK throws structured errors that tell you exactly what happened and why. This page covers all the error types and the most common scenarios you'll encounter.

## Error hierarchy

All SDK errors extend `ErrorBase` with structured metadata:

```ts
class ErrorBase extends Error {
  readonly name: string;
  readonly message: string;
  readonly details?: string;
  readonly docsUrl?: string;
  readonly version?: string;
  readonly cause?: Error;
}
```

## Error types

### Base errors

| Error | Description |
| --- | --- |
| `InvalidTypeError` | Value does not match expected type |
| `FheTypeError` | Invalid FHE type name or ID |
| `AddressError` | Invalid Ethereum address format |
| `ChecksummedAddressError` | Address fails EIP-55 checksum validation |
| `InvalidPropertyError` | Object property has invalid value |
| `FetchError` | Generic HTTP fetch failure |
| `InternalError` | SDK internal bug (should not happen) |

### Domain errors

| Error | Description |
| --- | --- |
| `ACLError` | ACL permission denied (encrypted value not allowed for user/contract) |
| `EncryptionError` | Encryption operation failed |
| `FhevmConfigError` | Invalid chain or runtime configuration |
| `FhevmHandleError` | Malformed or invalid encrypted value handle |
| `InputProofError` | Input proof validation failed |
| `TFHEError` | TFHE WASM module error |
| `ZkProofError` | ZK proof generation failed |
| `SignersError` | Signature verification failed (KMS or coprocessor) |

### Relayer errors

| Error | Description |
| --- | --- |
| `RelayerAbortError` | Request aborted (e.g., via AbortController) |
| `RelayerFetchError` | Network-level fetch failure |
| `RelayerMaxRetryError` | Maximum retry attempts exceeded |
| `RelayerStateError` | Relayer returned unexpected state |
| `RelayerTimeoutError` | Request timed out |
| `RelayerResponseApiError` | Relayer returned an API-level error |
| `RelayerResponseInputProofRejectedError` | Relayer rejected the input proof |
| `RelayerResponseInvalidBodyError` | Relayer response body is malformed |
| `RelayerResponseStatusError` | Relayer returned non-OK HTTP status |

## Common scenarios

**ACL Permission Denied:**
The user or contract does not have permission on the ACL contract. Ensure the smart contract has called `FHE.allow()` or `FHE.allowForDecryption()`.

**2048-Bit Limit Exceeded:**
Requests are limited to 2048 total encrypted bits. Examples: 8x `euint256` (2048), 64x `euint32` (2048), 1024x `ebool` (2048).

**Chain ID mismatch:**
All encrypted values in a single request must belong to the same chain.

**Invalid Checksummed Address:**
All addresses must be EIP-55 checksummed. Validate with `assertIsChecksummedAddress()`.

**Relayer Timeout / Retry Exhausted:**
Encryption and decryption involve HTTP calls to the relayer. Check network connectivity and the `relayerUrl` in your chain config.

**ExtraData Validation Failed:**
The `extraData` field is auto-fetched in most operations. If you're using standalone functions, pass `"0x00"` for standard operations.

## Error handling

```ts
try {
  const result = await client.publicDecrypt({ encryptedValues: handles });
} catch (error) {
  if (error instanceof ACLError) {
    // Handle permission issues
  } else if (error instanceof RelayerTimeoutError) {
    // Retry or notify user
  } else if (error instanceof FhevmHandleError) {
    // Invalid handle format
  }
  throw error;
}
```

All SDK errors include a `cause` chain — check `error.cause` for the underlying error when debugging relayer or RPC failures.
