# API Reference

## Client Factories

All factories are imported from `@fhevm/sdk/ethers`.

### `setFhevmRuntimeConfig(config)`

Configures the global FHEVM runtime. Must be called before creating any clients.

```ts
setFhevmRuntimeConfig(config: FhevmRuntimeConfig): void
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `config.locateFile` | `(file: string) => URL` | Custom WASM file locator |
| `config.logger` | `Logger` | Logger instance |
| `config.singleThread` | `boolean` | Force single-threaded WASM |
| `config.numberOfThreads` | `number` | Number of WASM worker threads |

### `createFhevmClient(parameters)`

Creates a full-featured client with encrypt, decrypt, and relayer modules.

```ts
createFhevmClient<chain, provider>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
}): FhevmClient<chain, WithAll, provider>
```

### `createFhevmEncryptClient(parameters)`

Creates an encrypt-only client (no TKMS WASM loaded).

```ts
createFhevmEncryptClient<chain, provider>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
}): FhevmEncryptClient<chain, WithEncryptAndRelayer, provider>
```

### `createFhevmDecryptClient(parameters)`

Creates a decrypt-only client (no TFHE WASM loaded).

```ts
createFhevmDecryptClient<chain, provider>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
}): FhevmDecryptClient<chain, WithDecryptAndRelayer, provider>
```

### `createFhevmHostClient(parameters)`

Creates a minimal client with no modules — only host contract reads.

```ts
createFhevmHostClient<chain, provider>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
}): Fhevm<chain, FhevmRuntime, provider>
```

---

## Encryption Actions

### `encrypt(fhevm, parameters)`

Encrypts values and returns a verified input proof. Combines `generateZkProof` + `fetchVerifiedInputProof`.

```ts
encrypt(
  fhevm: Fhevm<FhevmChain, WithEncryptAndRelayer>,
  parameters: EncryptParameters,
): Promise<VerifiedInputProof>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `globalFhePublicEncryptionParams` | `GlobalFhePkeParams` | Public encryption parameters |
| `contractAddress` | `string` | Target contract address |
| `userAddress` | `string` | User's Ethereum address |
| `values` | `readonly TypedValueLike[]` | Values to encrypt |
| `extraData` | `BytesHex` | Extra data (typically `"0x"`) |
| `options` | `RelayerFetchOptions` | Optional fetch options |

### `generateZkProof(fhevm, parameters)`

Generates a zero-knowledge proof of correct encryption (CPU-intensive TFHE WASM operation).

```ts
generateZkProof(
  fhevm: Fhevm<FhevmChain, WithEncrypt>,
  parameters: GenerateZkProofParameters,
): Promise<ZkProof>
```

### `fetchVerifiedInputProof(fhevm, parameters)`

Sends a ZK proof to the relayer and returns verified input proof with coprocessor signatures.

```ts
fetchVerifiedInputProof(
  fhevm: Fhevm<FhevmChain, WithEncryptAndRelayer>,
  parameters: FetchVerifiedInputProofParameters,
): Promise<VerifiedInputProof>
```

---

## Decryption Actions

### `publicDecrypt(fhevm, parameters)`

Decrypts handles that are publicly decryptable on-chain.

```ts
publicDecrypt(
  fhevm: Fhevm<FhevmChain, WithRelayer>,
  parameters: PublicDecryptParameters,
): Promise<PublicDecryptionProof>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `handles` | `readonly FhevmHandle[]` | Handles to decrypt (min 1, max 2048 bits total) |
| `extraData` | `BytesHex` | Extra data (typically `"0x"`) |
| `options` | `RelayerFetchOptions` | Optional fetch options |

### `userDecrypt(fhevm, parameters)`

Decrypts handles using a user's KMS private key and signed EIP-712 permit.

```ts
userDecrypt(
  fhevm: Fhevm<FhevmChain, WithRelayer>,
  parameters: UserDecryptParameters,
): Promise<readonly DecryptedFhevmHandle[]>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `decryptionKey` | `FhevmDecryptionKey` | User's decryption key |
| `handleContractPairs` | `ReadonlyArray<{ handle, contractAddress }>` | Handles with their contract addresses |
| `userDecryptEIP712Signer` | `ChecksummedAddress` | Address that signed the permit |
| `userDecryptEIP712Message` | `KmsUserDecryptEIP712Message` | EIP-712 message content |
| `userDecryptEIP712Signature` | `Bytes65Hex` | EIP-712 signature |
| `options` | `RelayerFetchOptions` | Optional fetch options |

### `createUserDecryptEIP712(fhevm, parameters)`

Constructs the EIP-712 typed data for a user decryption permit.

```ts
createUserDecryptEIP712(
  fhevm: Fhevm<FhevmChain>,
  parameters: CreateUserDecryptEIP712Parameters,
): KmsUserDecryptEIP712
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `publicKey` | `BytesHex` | KMS public key hex |
| `contractAddresses` | `readonly string[]` | Allowed contracts (max 10) |
| `startTimestamp` | `number` | Unix timestamp (seconds) |
| `durationDays` | `number` | Validity period (max 365) |
| `extraData` | `BytesHex` | Extra data (typically `"0x"`) |

### `createDelegatedUserDecryptEIP712(fhevm, parameters)`

Constructs EIP-712 typed data for a delegated user decryption permit.

```ts
createDelegatedUserDecryptEIP712(
  fhevm: Fhevm<FhevmChain>,
  parameters: CreateDelegatedUserDecryptEIP712Parameters,
): KmsDelegatedUserDecryptEIP712
```

Additional parameter:
| Parameter | Type | Description |
|-----------|------|-------------|
| `delegatedAccount` | `ChecksummedAddress` | Authorized delegate address |

---

## Key Management Actions

### `fetchGlobalFhePkeParams(fhevm, parameters?)`

Fetches and deserializes the global FHE public encryption parameters. Results are cached by relayer URL.

```ts
fetchGlobalFhePkeParams(
  fhevm: Fhevm<FhevmChain, WithEncryptAndRelayer>,
  parameters?: FetchGlobalFhePkeParamsParameters,
): Promise<GlobalFhePkeParams>
```

### `fetchGlobalFhePkeParamsBytes(fhevm, parameters?)`

Fetches the raw bytes of the global FHE public encryption parameters.

```ts
fetchGlobalFhePkeParamsBytes(
  fhevm: Fhevm<FhevmChain, WithRelayer>,
  parameters?: FetchGlobalFhePkeParamsBytesParameters,
): Promise<GlobalFhePkeParamsBytes>
```

### Cache Management

```ts
clearGlobalFhePkeParamsCache(): void
deleteGlobalFhePkeParamsCache(relayerUrl: string): void
```

---

## Serialization Actions

### `serializeGlobalFhePkeParams(fhevm, parameters)`

Serializes `GlobalFhePkeParams` to `GlobalFhePkeParamsBytes`.

### `serializeGlobalFhePkeParamsToHex(fhevm, parameters)`

Serializes `GlobalFhePkeParams` to `GlobalFhePkeParamsBytesHex`.

### `deserializeGlobalFhePkeParams(fhevm, parameters)`

Deserializes `GlobalFhePkeParamsBytes` to `GlobalFhePkeParams`.

### `deserializeGlobalFhePkeParamsFromHex(fhevm, parameters)`

Deserializes `GlobalFhePkeParamsBytesHex` to `GlobalFhePkeParams`.

---

## Host Contract Actions

All imported from `@fhevm/sdk/ethers`.

### `readFhevmExecutorContractData(fhevm, parameters)`

Reads FHEVM Executor contract data (ACL address, handle version, etc.).

```ts
readFhevmExecutorContractData(
  fhevm: Fhevm,
  parameters: { address: ChecksummedAddress },
): Promise<FhevmExecutorContractData>
```

### `readInputVerifierContractData(fhevm, parameters)`

Reads Input Verifier contract data (coprocessor signers, threshold, EIP-712 domain).

```ts
readInputVerifierContractData(
  fhevm: Fhevm,
  parameters: { address: ChecksummedAddress },
): Promise<InputVerifierContractData>
```

### `readKmsVerifierContractData(fhevm, parameters)`

Reads KMS Verifier contract data (KMS signers, threshold, gateway chain ID).

```ts
readKmsVerifierContractData(
  fhevm: Fhevm,
  parameters: { address: ChecksummedAddress },
): Promise<KmsVerifierContractData>
```

### `resolveFhevmConfig(fhevm, parameters)`

Resolves complete FHEVM configuration by reading multiple host contracts.

```ts
resolveFhevmConfig(
  fhevm: Fhevm,
  parameters: ResolveFhevmConfigParameters,
): Promise<ResolveFhevmConfigReturnType>
```

---

## Chain Verification

### `verifyKmsUserDecryptEIP712(fhevm, parameters)`

Verifies a KMS user decrypt EIP-712 signature on-chain.

```ts
verifyKmsUserDecryptEIP712(
  fhevm: Fhevm,
  parameters: VerifyKmsUserDecryptEIP712Parameters,
): Promise<ChecksummedAddress>  // Returns recovered signer address
```

---

## Address Utilities

### `assertIsChecksummedAddress(address, options)`

Validates that a string is a valid EIP-55 checksummed Ethereum address. Throws `ChecksummedAddressError` if invalid.

```ts
import { assertIsChecksummedAddress } from "@fhevm/sdk/ethers";

assertIsChecksummedAddress("0xAbCdEf...", {});
```

---

## Type Exports

All types are available from `@fhevm/sdk`:

### Client Types
- `FhevmClient`, `FhevmEncryptClient`, `FhevmDecryptClient`
- `FhevmRuntime`, `FhevmRuntimeConfig`
- `WithEncrypt`, `WithDecrypt`, `WithRelayer`, `WithAll`
- `WithEncryptModule`, `WithDecryptModule`, `WithRelayerModule`

### Handle Types
- `FhevmHandle`, `FhevmHandleLike`, `ExternalFhevmHandle`
- `Ebool`, `Euint8`, `Euint16`, `Euint32`, `Euint64`, `Euint128`, `Euint256`, `Eaddress`

### Decrypted Types
- `DecryptedFhevmHandle`, `DecryptedFhevmHandleOfType`
- `DecryptedEbool`, `DecryptedEuint8`, ... `DecryptedEaddress`

### FHE Types
- `FheType`, `FheTypeId`, `EncryptionBits`

### Primitive Types
- `ChecksummedAddress`, `Address`
- `BytesHex`, `Bytes32Hex`, `Bytes65Hex`
- `Uint8Number`, `Uint16Number`, `Uint32Number`
- `Uint64BigInt`, `Uint128BigInt`, `Uint256BigInt`
- `TypedValue`, `TypedValueLike`

### Proof Types
- `VerifiedInputProof`, `ZkProof`, `PublicDecryptionProof`

### KMS Types
- `KmsUserDecryptEIP712`, `KmsUserDecryptEIP712Message`
- `KmsDelegatedUserDecryptEIP712`, `KmsDelegatedUserDecryptEIP712Message`
- `KmsPublicDecryptEIP712`, `KmsEIP712Domain`
- `KmsVerifierContractData`
- `FhevmDecryptionKey`

### Chain Types
- `FhevmChain`
- `FhevmExecutorContractData`, `InputVerifierContractData`

### Global PKE Params Types
- `GlobalFhePkeParams`, `GlobalFhePkeParamsBytes`, `GlobalFhePkeParamsBytesHex`

### Parameter/Return Types

Every action function exports its parameter and return types following the convention:
- `FunctionNameParameters` (e.g., `EncryptParameters`, `PublicDecryptParameters`)
- `FunctionNameReturnType` (e.g., `EncryptReturnType`, `PublicDecryptReturnType`)
