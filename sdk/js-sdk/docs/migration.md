# Migration from `@zama-fhe/relayer-sdk`

This guide helps you migrate from the old SDK (`@zama-fhe/relayer-sdk` v0.4.x) to the new SDK (`@fhevm/sdk`). The new SDK is a complete rewrite with a different API, but the concepts are the same.

## Overview of changes

| What changed              | Old SDK (`@zama-fhe/relayer-sdk`)                          | New SDK (`@fhevm/sdk`)                                                                    |
| ------------------------- | ---------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| Package name              | `@zama-fhe/relayer-sdk`                                    | `@fhevm/sdk`                                                                              |
| Entry points              | `/web`, `/node`, `/bundle`                                 | `/ethers`, `/viem`, `/chains`                                                             |
| Initialization            | `initSDK()` + `createInstance(config)`                     | `setFhevmRuntimeConfig()` + `createFhevmClient({ chain, provider })`                      |
| Configuration             | Flat config object (`SepoliaConfig`)                       | Chain definitions (`sepolia` from `@fhevm/sdk/chains`)                                    |
| Encryption                | Builder pattern: `.add32(42).encrypt()`                    | Declarative: `encrypt({ values: [{ type: "uint32", value: 42 }] })`                       |
| Key generation            | `generateKeyPair()` → raw `{ publicKey, privateKey }`      | `generateE2eTransportKeyPair()` → opaque `E2eTransportKeyPair`                            |
| Permit creation + signing | Separate: `createEIP712()` + wallet sign + manual bundling | Combined: `signDecryptionPermit({ signer, e2eTransportKeyPair, ... })`                    |
| Decrypt                   | 9 positional args                                          | Object: `decrypt({ e2eTransportKeyPair, encryptedValues, signedPermit })`                 |
| Read public values        | `publicDecrypt(handles)` → `{ clearValues }`               | `publicDecrypt({ encryptedValues })` → `PublicDecryptionProof` with `.orderedClearValues` |
| Provider                  | Passed in config as `network`                              | Passed directly as `provider`                                                             |
| Framework support         | Framework-agnostic (single entry)                          | Explicit adapters (`/ethers`, `/viem`)                                                    |

---

## Step 1: Update imports

**Before:**

```ts
import { initSDK, createInstance, SepoliaConfig } from '@zama-fhe/relayer-sdk/web';
```

**After:**

```ts
import { setFhevmRuntimeConfig, createFhevmClient } from '@fhevm/sdk/ethers'; // or "@fhevm/sdk/viem"
import { sepolia } from '@fhevm/sdk/chains';
```

The new SDK has separate imports for ethers.js and viem. Choose the one matching your project. The APIs are identical — only the provider type differs.

---

## Step 2: Replace initialization

**Before:**

```ts
await initSDK();

const instance = await createInstance({
  ...SepoliaConfig,
  network: provider, // or a URL string
});
```

**After:**

```ts
setFhevmRuntimeConfig({ numberOfThreads: 4 }); // optional config

const client = createFhevmClient({
  chain: sepolia,
  provider, // ethers Provider or viem PublicClient
});
```

Key differences:

- `setFhevmRuntimeConfig()` replaces `initSDK()`. It's synchronous and configures WASM threading/logging.
- `createFhevmClient()` replaces `createInstance()`. It's also synchronous — no `await` needed. WASM loads lazily on first use.
- Chain config is a pre-built object (`sepolia`, `mainnet`) instead of a flat config. No more `chainId`, `gatewayChainId`, `aclContractAddress`, etc. — the chain object has all of it.

---

## Step 3: Migrate encryption

This is the biggest API change. The old builder pattern is replaced by a declarative object.

**Before:**

```ts
const input = instance.createEncryptedInput(contractAddress, userAddress);
input.add32(42);
input.add8(100);
input.addBool(true);
const { handles, inputProof } = await input.encrypt();
```

**After:**

```ts
const encrypted = await client.encrypt({
  contractAddress,
  userAddress,
  values: [
    { type: 'uint32', value: 42 },
    { type: 'uint8', value: 100 },
    { type: 'bool', value: true },
  ],
});

const handles = encrypted.externalEncryptedValues;
const inputProof = encrypted.inputProof;
```

Key differences:

- The public encryption key is fetched and cached automatically on first `encrypt()` call. If you want to control when the ~50MB download happens (for example, behind a loading spinner), call `await client.init()` at app startup.
- Values are declared as an array of `{ type, value }` objects instead of chained `.add*()` calls.
- Type names use Solidity conventions: `"uint32"`, `"bool"`, `"address"` (not `add32`, `addBool`, `addAddress`).
- The result has `externalEncryptedValues` (array of `ExternalEncryptedValue`) and `inputProof` (hex string).

---

## Step 4: Migrate key generation

**Before:**

```ts
const { publicKey, privateKey } = instance.generateKeyPair();
// publicKey and privateKey are hex strings
```

**After:**

```ts
const e2eTransportKeyPair = await client.generateE2eTransportKeyPair();
// privateKey is hidden inside the key pair — never exposed
```

The new SDK wraps the private key in an opaque `E2eTransportKeyPair` object. You can't access the raw private key directly — this prevents accidental exposure. The key pair is what you pass to `signDecryptionPermit()` and `decrypt()`.

---

## Step 5: Migrate permit creation and signing

The biggest workflow change: permit creation and signing are now a **single step**.

**Before:**

```ts
// 1. Create EIP-712 data
const eip712 = instance.createEIP712(
  publicKey, // string
  contractAddresses, // string[]
  startTimestamp, // number
  durationDays, // number
);

// 2. Sign with wallet
const signature = await signer.signTypedData(eip712.domain, eip712.types, eip712.message);

// 3. Bundle manually
const signedPermit = createSignedPermit(eip712, signature, userAddress);
```

**After:**

```ts
// All in one step — SDK creates EIP-712, signs it, and bundles the result
const signedPermit = await client.signDecryptionPermit({
  contractAddresses: ['0xContractA...'],
  startTimestamp: Math.floor(Date.now() / 1000),
  durationDays: 7,
  signerAddress: await signer.getAddress(),
  signer,
  e2eTransportKeyPair,
});
```

Key differences:

- No more separate create + sign + bundle steps. `signDecryptionPermit()` handles everything.
- The signer is passed directly — the SDK calls `signTypedData` internally.
- The key pair is passed as an object, not as separate public/private key strings.

---

## Step 6: Migrate decryption

**Before:**

```ts
const results = await instance.userDecrypt(
  handleContractPairs, // HandleContractPair[]
  privateKey, // string
  publicKey, // string
  signature, // string
  contractAddresses, // string[]
  userAddress, // string
  startTimestamp, // number
  durationDays, // number
);
// results is a Record<handle, value>
```

**After:**

```ts
const results = await client.decrypt({
  e2eTransportKeyPair,
  encryptedValues: [{ encryptedValue: '0x...', contractAddress: '0xContractA...' }],
  signedPermit,
});
// results is ClearValue[] with typed values
```

Key differences:

- Named parameters instead of 9 positional arguments.
- Pass the `e2eTransportKeyPair` object instead of separate `privateKey` + `publicKey` strings.
- Pass a `signedPermit` from `signDecryptionPermit()` instead of raw signature + permit params.
- The result is a typed array (`ClearValue[]`) instead of a plain `Record`. Each entry has `.value` (typed correctly as `number`, `bigint`, `boolean`, or `string`), `.fheType`, and `.encryptedValue`.

---

## Step 7: Migrate reading public values

**Before:**

```ts
const { clearValues, decryptionProof } = await instance.publicDecrypt(handles);
// clearValues is Record<handle, bigint | boolean | string>
```

**After:**

```ts
const result = await client.publicDecrypt({
  encryptedValues: [handle1, handle2],
});

const values = result.orderedClearValues;
// values[0].value — typed correctly
// values[0].fheType — "euint32", "ebool", etc.
```

Key differences:

- Pass encrypted values via `{ encryptedValues: [...] }` parameter object.
- Results are ordered `ClearValue[]` via `result.orderedClearValues` instead of a handle-keyed record.

---

## Step 8: Migrate decrypting on behalf of

**Before:**

```ts
const eip712 = instance.createDelegatedUserDecryptEIP712(
  publicKey,
  contractAddresses,
  delegatorAddress,
  startTimestamp,
  durationDays,
);

const results = await instance.delegatedUserDecrypt(
  handleContractPairs,
  privateKey,
  publicKey,
  signature,
  contractAddresses,
  delegatorAddress,
  delegateAddress,
  startTimestamp,
  durationDays,
);
```

**After:**

```ts
const signedPermit = await client.signDecryptionPermit({
  contractAddresses: ['0xContract...'],
  startTimestamp: Math.floor(Date.now() / 1000),
  durationDays: 1,
  signerAddress: await signer.getAddress(),
  signer,
  e2eTransportKeyPair,
  onBehalfOf: '0xDataOwnerAddress...',
});
```

Same flow as regular decryption — `onBehalfOf` replaces the separate function.

---

## Removed APIs

These old SDK APIs have no direct equivalent in the new SDK:

| Old API                                 | What to do instead                                                |
| --------------------------------------- | ----------------------------------------------------------------- |
| `instance.getPublicKey()`               | Use `client.fetchFheEncryptionKeyBytes()`                         |
| `instance.getPublicParams(bits)`        | Use `client.fetchFheEncryptionKeyBytes()`                         |
| `instance.config`                       | Access `client.chain` for chain info                              |
| `instance.requestZKProofVerification()` | Built into `client.encrypt()` automatically                       |
| `initSDK({ tfheParams, kmsParams })`    | Use `setFhevmRuntimeConfig({ locateFile })` for custom WASM paths |
| `createSignedPermit()`                  | Built into `client.signDecryptionPermit()`                        |

---

## Full before/after example

**Before (old SDK):**

```ts
import { initSDK, createInstance, SepoliaConfig } from '@zama-fhe/relayer-sdk/web';

await initSDK();
const instance = await createInstance({ ...SepoliaConfig, network: provider });

// Encrypt
const input = instance.createEncryptedInput(contractAddr, userAddr);
input.add32(42);
input.add8(100);
const { handles, inputProof } = await input.encrypt();

// User decrypt
const { publicKey, privateKey } = instance.generateKeyPair();
const eip712 = instance.createEIP712(publicKey, [contractAddr], startTs, 7);
const sig = await signer.signTypedData(eip712.domain, eip712.types, eip712.message);
const results = await instance.userDecrypt(
  [{ handle, contractAddress: contractAddr }],
  privateKey,
  publicKey,
  sig,
  [contractAddr],
  userAddr,
  startTs,
  7,
);
```

**After (new SDK):**

```ts
import { setFhevmRuntimeConfig, createFhevmClient } from '@fhevm/sdk/ethers';
import { sepolia } from '@fhevm/sdk/chains';

setFhevmRuntimeConfig({});
const client = createFhevmClient({ chain: sepolia, provider });

// Encrypt
const encrypted = await client.encrypt({
  contractAddress: contractAddr,
  userAddress: userAddr,
  values: [
    { type: 'uint32', value: 42 },
    { type: 'uint8', value: 100 },
  ],
});

// Decrypt
const e2eTransportKeyPair = await client.generateE2eTransportKeyPair();
const signedPermit = await client.signDecryptionPermit({
  contractAddresses: [contractAddr],
  startTimestamp: startTs,
  durationDays: 7,
  signerAddress: userAddr,
  signer,
  e2eTransportKeyPair,
});
const results = await client.decrypt({
  e2eTransportKeyPair,
  encryptedValues: [{ encryptedValue: handle, contractAddress: contractAddr }],
  signedPermit,
});
```
