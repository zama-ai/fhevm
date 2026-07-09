# API reference

Complete reference for the public surface of `@fhevm/sdk`. Signatures are written
in TypeScript. Guides for each area are linked at the point of use.

## Entry points

| Import path                  | Provides                                                        |
| ---------------------------- | -------------------------------------------------------------- |
| `@fhevm/sdk/ethers`          | Client factories + runtime config, ethers v6 adapter           |
| `@fhevm/sdk/viem`            | Client factories + runtime config, viem adapter                |
| `@fhevm/sdk/chains`          | Built-in chains and `defineFhevmChain`                         |
| `@fhevm/sdk/types`           | Public type exports and encrypted-value helpers                |
| `@fhevm/sdk/actions/base`    | Public-decrypt and proof-fetch actions                         |
| `@fhevm/sdk/actions/encrypt` | Encrypt actions                                                |
| `@fhevm/sdk/actions/decrypt` | Private-decrypt actions                                        |
| `@fhevm/sdk/actions/chain`   | Permit / key / serialization actions                           |
| `@fhevm/sdk/actions/host`    | Host-contract read actions                                     |

The ethers and viem entry points export the same symbols; only the native
connection object differs (`provider` vs `publicClient`).

## Runtime config

Exported from both `@fhevm/sdk/ethers` and `@fhevm/sdk/viem`.

```ts
function setFhevmRuntimeConfig(config: FhevmRuntimeConfig): void;
function hasFhevmRuntimeConfig(): boolean;
```

`setFhevmRuntimeConfig` is idempotent with identical config and throws if re-set
with a different config. See [Runtime configuration](runtime-configuration.md).

```ts
type FhevmRuntimeConfig = {
  readonly numberOfThreads?: number | undefined;
  readonly singleThread?: boolean | undefined;
  readonly wasmAssetLoadMode?: WasmAssetLoadMode | undefined;
  readonly locateFile?: ((file: string) => URL) | undefined;
  readonly moduleVersions?: FhevmModuleVersions | undefined;
  readonly logger?: Logger | undefined;
  readonly auth?: Auth | undefined;
};

type WasmAssetLoadMode =
  | 'embedded-base64'
  | 'verified-blob'
  | 'precheck-direct-url'
  | 'trusted-direct-url'
  | 'auto';

type FhevmModuleVersions =
  | 'auto'
  | {
      readonly tfhe?: '1.5.3' | '1.6.2' | undefined;
      readonly kms?: '0.13.10' | '0.13.20-0' | undefined;
      readonly checkCompatibility?: 'throw' | 'warn' | 'off' | undefined;
    };
```

### Runtime init helpers

```ts
function initFhevmRuntime(): Promise<void>;
function initFhevmEncryptRuntime(): Promise<void>;
function initFhevmDecryptRuntime(): Promise<void>;
```

## Client factories

Exported from both adapter entry points. The ethers variants take `provider`
(`ethers.ContractRunner`); the viem variants take `publicClient`
(`viem.PublicClient`). Shapes otherwise match.

```ts
// ethers
function createFhevmClient(parameters: {
  readonly provider: ContractRunner;
  readonly chain: FhevmChain;
  readonly options?: FhevmOptions | undefined;
}): FhevmClient;

function createFhevmEncryptClient(parameters: {
  readonly provider: ContractRunner;
  readonly chain: FhevmChain;
  readonly options?: FhevmEncryptOptions | undefined;
}): FhevmEncryptClient;

function createFhevmDecryptClient(parameters: {
  readonly provider: ContractRunner;
  readonly chain: FhevmChain;
  readonly options?: FhevmDecryptOptions | undefined;
}): FhevmDecryptClient;

function createFhevmBaseClient(parameters: {
  readonly provider: ContractRunner;
  readonly chain: FhevmChain;
  readonly options?: FhevmOptions | undefined;
}): FhevmBaseClient;
```

The viem forms replace `provider: ContractRunner` with
`publicClient: PublicClient`. See [Clients](clients.md).

### Factory options

```ts
type FhevmBaseOptions = {
  readonly batchRpcCalls?: boolean | undefined;
};

type FhevmOptions = FhevmBaseOptions & {
  readonly fheEncryptionKey?: FheEncryptionKeyBytes | undefined;
  readonly moduleVersions?: FhevmModuleVersions | undefined;
};

type FhevmEncryptOptions = FhevmBaseOptions & {
  readonly fheEncryptionKey?: FheEncryptionKeyBytes | undefined;
  readonly moduleVersions?: FhevmEncryptModuleVersions | undefined;
};

type FhevmDecryptOptions = FhevmBaseOptions & {
  readonly moduleVersions?: FhevmDecryptModuleVersions | undefined;
};
```

### Client lifecycle members

Present on every client:

```ts
init(): Promise<void>;         // preload + compile WASM
readonly ready: Promise<void>; // resolves when usable
readonly uid: string;
readonly chain: FhevmChain;
readonly protocolVersion: ProtocolVersionResolution;
```

### Method availability

| Method                              | Base | Encrypt | Decrypt | Full |
| ----------------------------------- | :--: | :-----: | :-----: | :--: |
| `encryptValue` / `encryptValues`    |      |   ✅    |         |  ✅  |
| `decryptValue` / `decryptValues` / `decryptValuesFromPairs` |  |  |  ✅  | ✅ |
| `generateTransportKeyPair`          |      |         |   ✅    |  ✅  |
| `decryptPublicValue(s)`             |  ✅  |   ✅    |   ✅    |  ✅  |
| `decryptPublicValuesWithSignatures` |  ✅  |   ✅    |   ✅    |  ✅  |
| `signDecryptionPermit`              |  ✅  |   ✅    |   ✅    |  ✅  |
| `serialize`/`parse` permit + key    |  ✅  |   ✅    |   ✅    |  ✅  |
| `fetchFheEncryptionKeyBytes`        |  ✅  |   ✅    |   ✅    |  ✅  |

The `canDecrypt*` permission checks are not client methods. `canDecryptValue`,
`canDecryptValues`, and `canDecryptValuesFromPairs` come from
[`@fhevm/sdk/actions/decrypt`](actions.md); `canDecryptPublicValue` and
`canDecryptPublicValues` come from [`@fhevm/sdk/actions/base`](actions.md). Import
them and pass the client as the first argument.

## Encryption methods

See [Encryption](encryption.md).

```ts
encryptValues(parameters: {
  readonly values: ReadonlyArray<{ readonly type: string; readonly value: boolean | bigint | number | string }>;
  readonly contractAddress: string;
  readonly userAddress: string;
  readonly options?: RelayerInputProofOptions | undefined;
}): Promise<{ readonly encryptedValues: readonly EncryptedValue[]; readonly inputProof: BytesHex }>;

encryptValue(parameters: {
  readonly value: { readonly type: string; readonly value: boolean | bigint | number | string };
  readonly contractAddress: string;
  readonly userAddress: string;
  readonly options?: RelayerInputProofOptions | undefined;
}): Promise<{ readonly encryptedValue: EncryptedValue; readonly inputProof: BytesHex }>;
```

Valid `type` strings: `'bool' | 'uint8' | 'uint16' | 'uint32' | 'uint64' | 'uint128' | 'uint256' | 'address'`.

## Private decryption methods

See [Decryption](decryption.md).

```ts
generateTransportKeyPair(): Promise<TransportKeyPair>;

decryptValue(parameters: {
  readonly encryptedValue: EncryptedValueLike;
  readonly contractAddress: string;
  readonly transportKeyPair: TransportKeyPair;
  readonly signedPermit: SignedDecryptionPermit;
  readonly options?: RelayerUserDecryptOptions | RelayerDelegatedUserDecryptOptions | undefined;
}): Promise<TypedValue>;

decryptValues(parameters: {
  readonly encryptedValues: readonly EncryptedValueLike[];
  readonly contractAddress: string;
  readonly transportKeyPair: TransportKeyPair;
  readonly signedPermit: SignedDecryptionPermit;
  readonly options?: RelayerUserDecryptOptions | RelayerDelegatedUserDecryptOptions | undefined;
}): Promise<readonly TypedValue[]>;

decryptValuesFromPairs(parameters: {
  readonly pairs: ReadonlyArray<{ readonly encryptedValue: EncryptedValueLike; readonly contractAddress: string }>;
  readonly transportKeyPair: TransportKeyPair;
  readonly signedPermit: SignedDecryptionPermit;
  readonly options?: RelayerUserDecryptOptions | RelayerDelegatedUserDecryptOptions | undefined;
}): Promise<readonly TypedValue[]>;
```

### Permission checks

Standalone actions from [`@fhevm/sdk/actions/decrypt`](actions.md), not client
methods — pass the client as the first argument.

```ts
canDecryptValue(fhevm, parameters:
  | { readonly encryptedValue: EncryptedValueLike; readonly contractAddress: string; readonly userAddress: string }
  | { readonly encryptedValue: EncryptedValueLike; readonly contractAddress: string; readonly signedPermit: SignedDecryptionPermit; readonly transportKeyPair?: TransportKeyPair }
): Promise<{ readonly allowed: boolean; readonly details: { readonly contractAllowed: boolean; readonly userAllowed: boolean } }>;
```

`canDecryptValues` and `canDecryptValuesFromPairs` mirror the plural / pairs
forms and return an `allowed` flag with a `details` array.

## Public decryption methods

See [Decryption → Public decryption](decryption.md#public-decryption).

```ts
decryptPublicValue(parameters: {
  readonly encryptedValue: EncryptedValueLike;
  readonly options?: RelayerPublicDecryptOptions | undefined;
}): Promise<TypedValue>;

decryptPublicValues(parameters: {
  readonly encryptedValues: readonly EncryptedValueLike[];
  readonly options?: RelayerPublicDecryptOptions | undefined;
}): Promise<TypedValue[]>;

decryptPublicValuesWithSignatures(parameters: {
  readonly encryptedValues: readonly EncryptedValueLike[];
  readonly options?: RelayerPublicDecryptOptions | undefined;
}): Promise<{
  readonly clearValues: readonly TypedValue[];
  readonly checkSignaturesArgs: {
    readonly handlesList: readonly string[];
    readonly abiEncodedCleartexts: BytesHex;
    readonly decryptionProof: BytesHex;
  };
}>;
```

`canDecryptPublicValue` and `canDecryptPublicValues` are standalone actions from
[`@fhevm/sdk/actions/base`](actions.md), not client methods — pass the client as
the first argument.

```ts
canDecryptPublicValue(fhevm, parameters: { readonly encryptedValue: EncryptedValueLike }): Promise<boolean>;
canDecryptPublicValues(fhevm, parameters: { readonly encryptedValues: readonly EncryptedValueLike[] }): Promise<readonly boolean[]>;
```

## Permit and key methods

See [Decryption](decryption.md).

```ts
signDecryptionPermit(parameters: {
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationSeconds: number;
  readonly signerAddress: string;
  readonly signer: NativeSigner; // ethers Signer / viem Account | WalletClient
  readonly delegatorAddress?: string | undefined;
  readonly transportKeyPair: TransportKeyPair;
}): Promise<SignedDecryptionPermit>;

// synchronous:
serializeTransportKeyPair(parameters: { readonly transportKeyPair: TransportKeyPair }): {
  publicKey: BytesHex;
  privateKey: BytesHex;
};
serializeSignedDecryptionPermit(parameters: { readonly signedPermit: SignedDecryptionPermit }): {
  readonly version: number;
  readonly eip712: Eip712Like;
  readonly signature: string;
  readonly signerAddress: string;
};

parseTransportKeyPair(parameters: { readonly publicKey: string; readonly privateKey: string }): Promise<TransportKeyPair>;
parseSignedDecryptionPermit(parameters: {
  readonly serializedPermit: { readonly version: number; readonly eip712: Eip712Like; readonly signature: string; readonly signerAddress: string };
  readonly transportKeyPair: TransportKeyPair;
}): Promise<SignedDecryptionPermit>;

fetchFheEncryptionKeyBytes(parameters?: {
  readonly options?: RelayerKeyUrlOptions | undefined;
  readonly ignoreCache?: boolean | undefined;
}): Promise<FheEncryptionKeyBytes>;
```

## Actions

Each action takes the client as its first argument. Most mirror a client method;
the `canDecrypt*` checks are available only as actions.
See [Actions](actions.md) for import paths and examples.

```ts
// @fhevm/sdk/actions/encrypt
encryptValue(fhevm, parameters);
encryptValues(fhevm, parameters);
generateZkProof(fhevm, parameters);

// @fhevm/sdk/actions/decrypt
generateTransportKeyPair(fhevm);
decryptValue(fhevm, parameters);
decryptValues(fhevm, parameters);
decryptValuesFromPairs(fhevm, parameters);
canDecryptValue(fhevm, parameters);
canDecryptValues(fhevm, parameters);
canDecryptValuesFromPairs(fhevm, parameters);

// @fhevm/sdk/actions/base
decryptPublicValue(fhevm, parameters);
decryptPublicValues(fhevm, parameters);
decryptPublicValuesWithSignatures(fhevm, parameters);
canDecryptPublicValue(fhevm, parameters);
canDecryptPublicValues(fhevm, parameters);
fetchEncryptedValues(fhevm, parameters);

// @fhevm/sdk/actions/chain
signDecryptionPermit(fhevm, parameters);
serializeSignedDecryptionPermit(fhevm, parameters);
parseSignedDecryptionPermit(fhevm, parameters);
serializeTransportKeyPair(fhevm, parameters);
parseTransportKeyPair(fhevm, parameters);
fetchFheEncryptionKeyBytes(fhevm, parameters?);

// @fhevm/sdk/actions/host
resolveFhevmConfig(fhevm, parameters);
isAllowedForDecryption(fhevm, parameters);
persistAllowed(fhevm, parameters);
```

## Chains

Exported from `@fhevm/sdk/chains`. See [Chains](chains.md).

```ts
const mainnet: FhevmChain; // id 1
const sepolia: FhevmChain; // id 11155111

function defineFhevmChain<const chain extends FhevmChain>(chain: chain): chain;

type FhevmChain = {
  readonly id: number;
  readonly fhevm: {
    readonly contracts: {
      readonly acl: ChainContract;
      readonly inputVerifier: ChainContract;
      readonly kmsVerifier: ChainContract;
      readonly protocolConfig: ChainContract | undefined;
    };
    readonly relayerUrl: string;
    readonly gateway: {
      readonly id: number;
      readonly contracts: { readonly decryption: ChainContract; readonly inputVerification: ChainContract };
    };
  };
};

type ChainContract = { readonly address: `0x${string}`; readonly blockCreated?: number | undefined };
```

## Types

Exported from `@fhevm/sdk/types`. See [Types](types.md).

```ts
// value exports:
function asEncryptedValue(value: unknown): EncryptedValue;
function isEncryptedValue(value: unknown): value is EncryptedValue;

// type exports:
type TypedValue = /* discriminated union { type, value } */;
type EncryptedValue = /* branded bytes32 */;
type EncryptedValueLike = Uint8Array | string | { readonly bytes32Hex: string };
type Eip712Like = {
  readonly domain: Record<string, unknown>;
  readonly primaryType?: string | undefined;
  readonly types: Record<string, ReadonlyArray<{ readonly name: string; readonly type: string }>>;
  readonly message: Record<string, unknown>;
};
type FhevmEncryptClient; // client type alias
type FhevmDecryptClient; // client type alias
```

Core types referenced throughout (imported from the core package):
`FheType`, `TransportKeyPair`, `SignedDecryptionPermit`, `FheEncryptionKeyBytes`,
and the branded handle aliases `Ebool`/`Euint8`/…/`Eaddress` plus their
`External*` counterparts.

## Relayer options

Accepted as the `options` field on every network method. All fields optional.

```ts
type RelayerCommonOptions = {
  readonly auth?: Auth | undefined;
  readonly headers?: Record<string, string> | undefined;
  readonly debug?: boolean | undefined;
  readonly fetchRetries?: number | undefined;
  readonly fetchRetryDelayInMilliseconds?: number | undefined;
  readonly signal?: AbortSignal | undefined;
  readonly timeout?: number | undefined;
};
```

Per-operation option types add an `onProgress?` callback:
`RelayerInputProofOptions` (encrypt), `RelayerUserDecryptOptions` /
`RelayerDelegatedUserDecryptOptions` (private decrypt),
`RelayerPublicDecryptOptions` (public decrypt), `RelayerKeyUrlOptions` (key
fetch). `onProgress` receives a `{ state, operation }` argument where `state` is
one of `'queued' | 'throttled' | 'succeeded' | 'failed' | 'timeout' | 'abort'`.

## Errors

Identified by their `name` string. See [Error handling](error-handling.md) for
the full catalog and handling patterns.

## Related

- [Getting started](getting-started.md)
- [Clients](clients.md) · [Encryption](encryption.md) · [Decryption](decryption.md)
- [Chains](chains.md) · [Runtime configuration](runtime-configuration.md) · [Types](types.md)
```

