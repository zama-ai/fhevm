# Migrating from `@zama-fhe/relayer-sdk`

`@fhevm/sdk` is the successor to `@zama-fhe/relayer-sdk`. The concepts are the
same — encrypt inputs, decrypt results — but the API is restructured around a
client bound to a chain and an Ethereum library adapter, with a declarative
encryption call instead of a builder.

This guide maps the old API to the new one.

## What changed, at a glance

| `@zama-fhe/relayer-sdk`                                   | `@fhevm/sdk`                                                                 |
| -------------------------------------------------------- | --------------------------------------------------------------------------- |
| `initSDK()` + `createInstance(config)`                   | `setFhevmRuntimeConfig()` + `createFhevmClient({ chain, provider })`         |
| `@zama-fhe/relayer-sdk/web` vs `/node` entry points      | `@fhevm/sdk/ethers` or `@fhevm/sdk/viem` (both run in browser **and** Node)  |
| `SepoliaConfig` / `MainnetConfig` flat config objects    | `sepolia` / `mainnet` chain definitions; `defineFhevmChain()` for custom     |
| Builder: `createEncryptedInput().add32(42).encrypt()`    | Declarative: `encryptValues({ values: [{ type, value }], … })`              |
| `generateKeypair()` → raw `{ publicKey, privateKey }`    | `generateTransportKeyPair()` → opaque `TransportKeyPair`                    |
| `createEIP712()` + manual sign + `createSignedPermit()`  | `signDecryptionPermit({ signer, transportKeyPair, … })` — one step          |
| `instance.userDecrypt(…)`                                | `client.decryptValue(…)` / `decryptValues(…)`                              |
| `instance.publicDecrypt(…)`                              | `client.decryptPublicValues(…)`                                            |

Three shifts are worth internalizing:

- **A client is bound to a chain and a library.** You pick `@fhevm/sdk/ethers` or
  `@fhevm/sdk/viem` and pass a `chain` definition. The same code runs in the
  browser and Node — there are no `/web` and `/node` builds.
- **Encryption is declarative.** No builder, no `.add32()`. You pass an array of
  `{ type, value }` objects and the Fully Homomorphic Encryption (FHE) public key is fetched and cached for you.
- **Keys and permits are opaque and combined.** The transport key pair hides its
  private key, and `signDecryptionPermit` bundles EIP-712 construction, signing,
  and packaging into a single call.

## Setup

{% tabs %}
{% tab title="Before" %}

```ts
import { initSDK, createInstance, SepoliaConfig } from '@zama-fhe/relayer-sdk/web';

await initSDK();
const instance = await createInstance({
  ...SepoliaConfig,
  network: 'https://sepolia-rpc.com',
});
```

{% endtab %}
{% tab title="After" %}

```ts
import { setFhevmRuntimeConfig, createFhevmClient } from '@fhevm/sdk/ethers';
import { sepolia } from '@fhevm/sdk/chains';
import { ethers } from 'ethers';

setFhevmRuntimeConfig({});
const provider = new ethers.JsonRpcProvider('https://sepolia-rpc.com');
const client = createFhevmClient({ chain: sepolia, provider });
```

{% endtab %}
{% endtabs %}

WASM now loads lazily on first use. Call `await client.init()` if you want to
preload it eagerly (the old `initSDK()`'s role).

## Encryption

The builder is replaced by a single declarative call. Old `.add32(42)` becomes
`{ type: 'uint32', value: 42 }`.

{% tabs %}
{% tab title="Before" %}

```ts
const input = instance.createEncryptedInput(contractAddr, userAddr);
input.add32(42);
input.add8(100);
const { handles, inputProof } = await input.encrypt();
```

{% endtab %}
{% tab title="After" %}

```ts
const encrypted = await client.encryptValues({
  contractAddress: contractAddr,
  userAddress: userAddr,
  values: [
    { type: 'uint32', value: 42 },
    { type: 'uint8', value: 100 },
  ],
});

encrypted.encryptedValues; // was: handles
encrypted.inputProof; // same idea
```

{% endtab %}
{% endtabs %}

The `.addXX()` methods map directly to `type` strings: `add8` → `'uint8'`,
`add32` → `'uint32'`, `addBool` → `'bool'`, `addAddress` → `'address'`, and so on
up to `'uint256'`. See [Encryption](encryption.md) for the full table.

## Private decryption

The old three-step "generate keypair, build+sign EIP-712, userDecrypt" collapses
into "generate transport key pair, sign permit, decryptValue".

{% tabs %}
{% tab title="Before" %}

```ts
const keypair = instance.generateKeypair();
const eip712 = instance.createEIP712(keypair.publicKey, [contractAddr], start, days);
const signature = await signer.signTypedData(eip712.domain, eip712.types, eip712.message);
// …assemble the request and call userDecrypt
const result = await instance.userDecrypt(/* handles, keypair, signature, … */);
```

{% endtab %}
{% tab title="After" %}

```ts
const transportKeyPair = await client.generateTransportKeyPair();

const signedPermit = await client.signDecryptionPermit({
  transportKeyPair,
  contractAddresses: [contractAddr],
  startTimestamp: Math.floor(Date.now() / 1000),
  durationSeconds: 7 * 24 * 60 * 60,
  signerAddress: await signer.getAddress(),
  signer,
});

const decrypted = await client.decryptValue({
  transportKeyPair,
  encryptedValue,
  contractAddress: contractAddr,
  signedPermit,
});

decrypted.value; // the plaintext
decrypted.type; // "uint32", "bool", …
```

{% endtab %}
{% endtabs %}

Two differences to watch for:

- **Duration is in seconds.** The permit takes `durationSeconds`, not a day
  count. Multiply: `7 * 24 * 60 * 60` for a week.
- **The result is a `TypedValue`.** You get `{ type, value }`, with `type` a
  Solidity value-type name (`'uint32'`), not the FHE name (`'euint32'`).

## Public decryption

```ts
// Before:
const values = await instance.publicDecrypt(handles);

// After:
const values = await client.decryptPublicValues({ encryptedValues: handles });
```

## Renamed terms

If you have code or docs referencing the old vocabulary, here are the canonical
replacements:

| Old term                                      | New term                         |
| --------------------------------------------- | -------------------------------- |
| `userDecrypt` / reencrypt / reencryption      | `decryptValue` / `decryptValues` |
| `publicDecrypt`                               | `decryptPublicValue(s)`          |
| `generateKeypair` / E2E transport keypair     | `generateTransportKeyPair`       |
| `FhevmHandle` / `fheHandle` (the value)       | `EncryptedValue`                 |
| `inputHandle` / `ExternalFhevmHandle`         | `ExternalEncryptedValue`         |

See the [Glossary](../GLOSSARY.md) for the complete list of deprecated terms and
their replacements.

## Related

- [Getting started](getting-started.md) — the new end-to-end flow.
- [Clients](clients.md) — choosing an adapter and a client type.
- [Encryption](encryption.md) · [Decryption](decryption.md) — the new APIs in depth.
```

