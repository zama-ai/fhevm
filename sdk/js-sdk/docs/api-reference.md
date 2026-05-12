# API reference

This is the complete reference for every function the SDK exports. For guided usage, see [Getting started](getting-started.md), [Encryption](encryption.md), or [Decryption](decryption.md) instead.

Client factories and runtime config are importable from `@fhevm/sdk/ethers` or `@fhevm/sdk/viem` (identical APIs). Standalone action functions are available from their respective entry points under `@fhevm/sdk/actions/*`.

---

## Client factories

### `setFhevmRuntimeConfig(config)`

Configures the global runtime. Must be called before creating any clients.

```ts
setFhevmRuntimeConfig(config: FhevmRuntimeConfig): void
```

| Parameter                | Type                    | Description                        |
| ------------------------ | ----------------------- | ---------------------------------- |
| `config.locateFile`      | `(file: string) => URL` | Custom WASM file locator           |
| `config.logger`          | `Logger`                | Logger instance `{ debug, error }` |
| `config.singleThread`    | `boolean`               | Force single-threaded WASM         |
| `config.numberOfThreads` | `number`                | Number of WASM worker threads      |

### `createFhevmClient(parameters)`

Full client with encrypt, decrypt, and base modules.

```ts
createFhevmClient<chain, provider>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
  readonly options?: FhevmOptions;
}): FhevmClient<chain, WithAll, provider>
```

### `createFhevmEncryptClient(parameters)`

Encrypt-only client (no TKMS WASM loaded).

```ts
createFhevmEncryptClient<chain, provider>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
  readonly options?: FhevmOptions;
}): FhevmEncryptClient<chain, WithEncrypt, provider>
```

### `createFhevmDecryptClient(parameters)`

Decrypt-only client (no TFHE WASM loaded).

```ts
createFhevmDecryptClient<chain, provider>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
  readonly options?: FhevmOptions;
}): FhevmDecryptClient<chain, WithDecrypt, provider>
```

### `createFhevmBaseClient(parameters)`

Empty base client — extend with `encryptActions`, `decryptActions`, or both.

```ts
createFhevmBaseClient<chain, provider>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
  readonly options?: FhevmOptions;
}): FhevmBaseClient<chain, FhevmRuntime, provider>
```

---

## Encryption actions

Import from `@fhevm/sdk/actions/encrypt`.

### `encrypt(fhevm, parameters)`

Encrypts values and returns encrypted handles with a verified input proof. The FHE public encryption key is automatically fetched and cached on first use.

```ts
// Single value
encrypt(fhevm, parameters: EncryptSingleParameters): Promise<EncryptSingleReturnType>

// Multiple values
encrypt(fhevm, parameters: EncryptMultipleParameters): Promise<EncryptMultipleReturnType>
```

| Parameter         | Type                                          | Description                         |
| ----------------- | --------------------------------------------- | ----------------------------------- |
| `contractAddress` | `string`                                      | Target contract address             |
| `userAddress`     | `string`                                      | User's Ethereum address             |
| `values`          | `TypedValueLike \| readonly TypedValueLike[]` | Values to encrypt (single or array) |
| `options?`        | `RelayerInputProofOptions`                    | Optional relayer options            |

**Single value return** (`EncryptSingleReturnType`):

| Field                    | Type                     | Description                     |
| ------------------------ | ------------------------ | ------------------------------- |
| `externalEncryptedValue` | `ExternalEncryptedValue` | The encrypted handle            |
| `inputProof`             | `BytesHex`               | Proof bytes to pass to contract |

**Multiple values return** (`EncryptMultipleReturnType`):

| Field                     | Type                                | Description                       |
| ------------------------- | ----------------------------------- | --------------------------------- |
| `externalEncryptedValues` | `readonly ExternalEncryptedValue[]` | Encrypted handles in input order  |
| `inputProof`              | `BytesHex`                          | Shared proof bytes for all values |

### `generateZkProof(fhevm, parameters)`

Generates a ZK proof of correct encryption (CPU-intensive TFHE WASM).

```ts
generateZkProof(fhevm, parameters: GenerateZkProofParameters): Promise<ZkProof>
```

---

## Base actions

Import from `@fhevm/sdk/actions/base`. Also available as client methods on all client types.

### `publicDecrypt(fhevm, parameters)` / `client.publicDecrypt(parameters)`

Decrypts encrypted values that are publicly decryptable on-chain.

```ts
publicDecrypt(fhevm, parameters: PublicDecryptParameters): Promise<PublicDecryptionProof>
```

| Parameter         | Type                            | Description                                        |
| ----------------- | ------------------------------- | -------------------------------------------------- |
| `encryptedValues` | `readonly EncryptedValueLike[]` | Encrypted values to decrypt (min 1, max 2048 bits) |
| `options?`        | `RelayerPublicDecryptOptions`   | Optional relayer options                           |

### `fetchVerifiedInputProof(fhevm, parameters)`

Sends a ZK proof to the relayer, returns a verified input proof with coprocessor signatures.

```ts
fetchVerifiedInputProof(fhevm, parameters: FetchVerifiedInputProofParameters): Promise<VerifiedInputProof>
```

### `fetchKmsSignedcryptedShares(fhevm, parameters)`

Fetches KMS signcrypted shares for decryption.

```ts
fetchKmsSignedcryptedShares(fhevm, parameters: FetchKmsSignedcryptedSharesParameters): Promise<KmsSigncryptedShares>
```

### `isAllowedForDecryption(fhevm, parameters)`

Checks if encrypted values are allowed for decryption on the ACL contract.

```ts
isAllowedForDecryption(fhevm, parameters): Promise<boolean | boolean[]>
```

### `checkAllowedForDecryption(fhevm, parameters)`

Same as `isAllowedForDecryption`, but throws if any value is not allowed.

```ts
checkAllowedForDecryption(fhevm, parameters: CheckAllowedForDecryptionParameters): Promise<void>
```

---

## Decrypt actions

Import from `@fhevm/sdk/actions/decrypt`.

### `decrypt(fhevm, parameters)` / `client.decrypt(parameters)`

Decrypts encrypted values using a transport key pair and signed permit.

```ts
decrypt(fhevm, parameters: DecryptParameters): Promise<readonly ClearValue[]>
```

| Parameter          | Type                                                              | Description                                              |
| ------------------ | ----------------------------------------------------------------- | -------------------------------------------------------- |
| `encryptedValues`  | `EncryptedValueEntry \| readonly EncryptedValueEntry[]`           | Encrypted values with their contract addresses           |
| `signedPermit`     | `SignedSelfDecryptionPermit \| SignedDelegatedDecryptionPermit`   | Signed permit from `signDecryptionPermit()`              |
| `transportKeyPair` | `TransportKeyPair`                                                | E2E transport key pair from `generateTransportKeyPair()` |
| `options?`         | `RelayerUserDecryptOptions \| RelayerDelegatedUserDecryptOptions` | Optional relayer options                                 |

Each `EncryptedValueEntry` has `{ encryptedValue: EncryptedValueLike, contractAddress: ChecksummedAddress }`.

### `generateTransportKeyPair(fhevm)` / `client.generateTransportKeyPair()`

Generates a new E2E transport key pair for decryption.

```ts
generateTransportKeyPair(fhevm): Promise<TransportKeyPair>
```

### `decryptKmsSignedcryptedShares(fhevm, parameters)`

Lower-level: decrypts KMS signcrypted shares locally using TKMS WASM.

```ts
decryptKmsSignedcryptedShares(fhevm, parameters: DecryptKmsSignedcryptedSharesParameters): Promise<ClearValue[]>
```

---

## Chain actions

Import from `@fhevm/sdk/actions/chain`. These act on the chain definition alone — no provider needed.

### `signDecryptionPermit(fhevm, parameters)` / `client.signDecryptionPermit(parameters)`

Creates and signs a decryption permit in a single step. Constructs the EIP-712 typed data internally and signs it with the provided signer.

```ts
// Self decryption
signDecryptionPermit(fhevm, parameters: SignSelfDecryptionPermitParameters): Promise<SignedSelfDecryptionPermit>

// Delegated decryption (onBehalfOf)
signDecryptionPermit(fhevm, parameters: SignDelegatedDecryptionPermitParameters): Promise<SignedDelegatedDecryptionPermit>
```

| Parameter           | Type                | Description                                |
| ------------------- | ------------------- | ------------------------------------------ |
| `contractAddresses` | `readonly string[]` | Allowed contracts (max 10)                 |
| `startTimestamp`    | `number`            | Unix timestamp (seconds)                   |
| `durationDays`      | `number`            | Validity period (max 365)                  |
| `signerAddress`     | `string`            | Address of the signer                      |
| `signer`            | `NativeSigner`      | Ethers Signer or viem WalletClient         |
| `transportKeyPair`  | `TransportKeyPair`  | Transport key pair                         |
| `onBehalfOf?`       | `string`            | Optional — address to decrypt on behalf of |

### `createKmsUserDecryptEIP712(fhevm, parameters)` / `client.createUserDecryptEIP712(parameters)`

Lower-level: constructs EIP-712 typed data for a decrypt permit without signing. Use `signDecryptionPermit` for the common case.

```ts
createKmsUserDecryptEIP712(fhevm, parameters: CreateKmsUserDecryptEIP712Parameters): Promise<CreateKmsUserDecryptEIP712ReturnType>
```

### `createKmsDelegatedUserDecryptEIP712(fhevm, parameters)` / `client.createDelegatedUserDecryptEIP712(parameters)`

Lower-level: constructs EIP-712 typed data for a delegated decrypt permit without signing.

```ts
createKmsDelegatedUserDecryptEIP712(fhevm, parameters: CreateKmsDelegatedUserDecryptEIP712Parameters): Promise<CreateKmsDelegatedUserDecryptEIP712ReturnType>
```

### `verifyKmsUserDecryptEIP712(fhevm, parameters)`

Verifies a decrypt permit EIP-712 signature. Throws on invalid signature.

```ts
verifyKmsUserDecryptEIP712(fhevm, parameters: VerifyKmsUserDecryptEIP712Parameters): Promise<void>
```

### `parseTransportKeyPair(fhevm, parameters)` / `client.parseTransportKeyPair(parameters)`

Restores a key pair from serialized bytes.

```ts
parseTransportKeyPair(fhevm, parameters: ParseTransportKeyPairParameters): Promise<TransportKeyPair>
```

### `serializeTransportKeyPair(fhevm, parameters)` / `client.serializeTransportKeyPair(parameters)`

Serializes a key pair for storage/persistence.

```ts
serializeTransportKeyPair(fhevm, parameters: SerializeTransportKeyPairParameters): SerializeTransportKeyPairReturnType
```

### `fetchFheEncryptionKeyBytes(fhevm, parameters?)` / `client.fetchFheEncryptionKeyBytes(parameters?)`

Fetches the ~50MB FHE public encryption key from the relayer and caches it.

```ts
fetchFheEncryptionKeyBytes(fhevm, parameters?: FetchFheEncryptionKeyBytesParameters): Promise<FetchFheEncryptionKeyBytesReturnType>
```

### `createKmsEIP712Domain(fhevm)`

Creates the EIP-712 domain for KMS operations.

```ts
createKmsEIP712Domain(fhevm): Promise<CreateKmsEIP712DomainReturnType>
```

### `createCoprocessorEIP712Domain(fhevm)`

Creates the EIP-712 domain for coprocessor operations.

```ts
createCoprocessorEIP712Domain(fhevm): Promise<CreateCoprocessorEIP712DomainReturnType>
```

---

## Host contract actions

Import from `@fhevm/sdk/actions/host`. These read on-chain contract data. They take any `Fhevm` instance — no `FhevmChain` required.

### `resolveFhevmConfig(fhevm, parameters)`

Resolves complete FHEVM configuration by reading multiple host contracts.

```ts
resolveFhevmConfig(fhevm, parameters: ResolveFhevmConfigParameters): Promise<ResolveFhevmConfigReturnType>
```

### `readFhevmExecutorContractData(fhevm, parameters)`

```ts
readFhevmExecutorContractData(fhevm, parameters: ReadFhevmExecutorContractDataParameters): Promise<ReadFhevmExecutorContractDataReturnType>
```

### `readInputVerifierContractData(fhevm, parameters)`

```ts
readInputVerifierContractData(fhevm, parameters: ReadInputVerifierContractDataParameters): Promise<ReadInputVerifierContractDataReturnType>
```

### `readKmsVerifierContractData(fhevm, parameters)`

```ts
readKmsVerifierContractData(fhevm, parameters: ReadKmsVerifierContractDataParameters): Promise<ReadKmsVerifierContractDataReturnType>
```

### Other host actions

| Function                                 | Description                             |
| ---------------------------------------- | --------------------------------------- |
| `getACLAddress(fhevm, params)`           | Gets the ACL contract address           |
| `getFHEVMExecutorAddress(fhevm, params)` | Gets the FhevmExecutor contract address |
| `getInputVerifierAddress(fhevm, params)` | Gets the InputVerifier contract address |
| `getKmsSigners(fhevm, params)`           | Gets KMS signer addresses               |
| `getCoprocessorSigners(fhevm, params)`   | Gets coprocessor signer addresses       |
| `getThreshold(fhevm, params)`            | Gets the signature threshold            |
| `getHandleVersion(fhevm, params)`        | Gets the handle version                 |
| `resolveChainId(fhevm, params)`          | Resolves the chain ID                   |
| `isAllowedForDecryption(fhevm, params)`  | Checks ACL decryption permission        |
| `persistAllowed(fhevm, params)`          | Checks persisted ACL permissions        |
| `eip712Domain(fhevm, params)`            | Reads EIP-712 domain from contract      |

---

## Type exports

**Client:** `FhevmClient`, `FhevmEncryptClient`, `FhevmDecryptClient`, `FhevmBaseClient`, `Fhevm`, `FhevmRuntime`, `FhevmRuntimeConfig`, `FhevmOptions`, `WithEncrypt`, `WithDecrypt`, `WithAll`

**Encrypted values:** `EncryptedValue`, `ComputedEncryptedValue`, `ExternalEncryptedValue`, `EncryptedValueLike`, `Handle` (alias), `InputHandle` (alias), `Ebool`, `Euint8`, `Euint16`, `Euint32`, `Euint64`, `Euint128`, `Euint256`, `Eaddress`, `ExternalEbool`, `ExternalEuint8`, ... `ExternalEaddress`

**Clear values:** `ClearValue`, `ClearValueOfType`, `ClearBool`, `ClearUint8`, `ClearUint16`, `ClearUint32`, `ClearUint64`, `ClearUint128`, `ClearUint256`, `ClearAddress`

**FHE:** `FheType`, `FheTypeId`

**Primitives:** `ChecksummedAddress`, `Address`, `BytesHex`, `Bytes32Hex`, `Bytes65Hex`, `Uint8Number`, `Uint16Number`, `Uint32Number`, `Uint64BigInt`, `Uint128BigInt`, `Uint256BigInt`, `TypedValue`, `TypedValueLike`

**Proofs:** `VerifiedInputProof`, `InputProof`, `ZkProof`, `PublicDecryptionProof`

**Permits:** `SignedSelfDecryptionPermit`, `SignedDelegatedDecryptionPermit`, `SignedDecryptionPermit`, `KmsUserDecryptEIP712`, `KmsDelegatedUserDecryptEIP712`, `KmsEIP712Domain`, `TransportKeyPair`

**Chains:** `FhevmChain`

**Convention:** Every action exports `FunctionNameParameters` and `FunctionNameReturnType` (e.g., `EncryptParameters`, `EncryptReturnType`).
