# Errors

## Error Hierarchy

All SDK errors extend a common `ErrorBase` class that provides structured metadata.

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

## Error Types

### Base Errors

| Error | Module | Description |
|-------|--------|-------------|
| `InvalidTypeError` | `base/errors` | Value does not match the expected type |
| `FheTypeError` | `base/errors` | Invalid FHE type name or ID |
| `AddressError` | `base/errors` | Invalid Ethereum address format |
| `ChecksummedAddressError` | `base/errors` | Address fails EIP-55 checksum validation |
| `InvalidPropertyError` | `base/errors` | Object property has invalid value |
| `FetchError` | `base/errors` | Generic HTTP fetch failure |
| `InternalError` | `base/errors` | SDK internal bug (should not happen) |

### Domain Errors

| Error | Module | Description |
|-------|--------|-------------|
| `ACLError` | `core/errors` | ACL permission denied (handle not allowed for user/contract) |
| `EncryptionError` | `core/errors` | Encryption operation failed |
| `FhevmConfigError` | `core/errors` | Invalid chain or runtime configuration |
| `FhevmHandleError` | `core/errors` | Malformed or invalid FhevmHandle |
| `InputProofError` | `core/errors` | Input proof validation failed |
| `TFHEError` | `core/errors` | TFHE WASM module error |
| `ZkProofError` | `core/errors` | ZK proof generation failed |
| `SignersError` | `core/errors` | Signature verification failed (KMS or coprocessor) |

### Relayer Errors

| Error | Module | Description |
|-------|--------|-------------|
| `RelayerAbortError` | `core/errors` | Request was aborted (e.g., via AbortController) |
| `RelayerFetchError` | `core/errors` | Network-level fetch failure |
| `RelayerMaxRetryError` | `core/errors` | Maximum retry attempts exceeded |
| `RelayerStateError` | `core/errors` | Relayer returned unexpected state |
| `RelayerTimeoutError` | `core/errors` | Request timed out |
| `RelayerResponseApiError` | `core/errors` | Relayer returned an API-level error |
| `RelayerResponseInputProofRejectedError` | `core/errors` | Relayer rejected the input proof |
| `RelayerResponseInvalidBodyError` | `core/errors` | Relayer response body is malformed |
| `RelayerResponseStatusError` | `core/errors` | Relayer returned non-OK HTTP status |

## Common Error Scenarios

### ACL Permission Denied

```
ACLError: User 0xAbc... is not allowed to decrypt handle 0x123...
```

The user or contract does not have permission on the ACL contract to decrypt the given handle. Ensure the smart contract has called `TFHE.allow()` or `TFHE.allowForDecryption()`.

### 2048-Bit Limit Exceeded

Decryption requests are limited to 2048 total encrypted bits. For example, you can decrypt at most:
- 8 `euint256` handles (8 x 256 = 2048 bits)
- 64 `euint32` handles (64 x 32 = 2048 bits)
- 1024 `ebool` handles (1024 x 2 = 2048 bits)

### Handle Chain ID Mismatch

All handles in a single request must belong to the same chain. Handles encode the chain ID in their bytes — mixing handles from different chains will throw.

### Invalid Checksummed Address

```
ChecksummedAddressError: Address "0xabcdef..." is not a valid EIP-55 checksummed address
```

All addresses in the SDK must be EIP-55 checksummed. Use `assertIsChecksummedAddress()` to validate, or use `ethers.getAddress()` to compute the checksum.

### Relayer Timeout / Retry Exhausted

Encryption and decryption involve HTTP calls to the relayer. If the relayer is unreachable or slow, you'll see `RelayerTimeoutError` or `RelayerMaxRetryError`. Check your network connection and the relayer URL in your chain config.

## Error Handling Pattern

```ts
try {
  const result = await client.publicDecrypt({ handles, extraData: "0x" });
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
