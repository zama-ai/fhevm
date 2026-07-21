# Getting started

This guide takes you from an empty project to a working end-to-end flow: encrypt
a value, send it to a confidential contract, and decrypt a result — all without
plaintext ever touching the chain.

## What you'll learn

By the end you will have run, in order:

1. Configured the FHEVM runtime.
2. Created a client bound to a chain and an RPC provider.
3. Encrypted values and produced an input proof for a contract.
4. Decrypted a private value back to plaintext in your app.

## Prerequisites

- **Node.js >= 22**.
- Either [ethers](https://docs.ethers.org/v6/) v6 **or** [viem](https://viem.sh)
  v2 installed. `@fhevm/sdk` ships an adapter for each and treats both as
  optional peer dependencies — install the one you already use.
- A deployed FHEVM contract to talk to. This guide uses the public `FHECounter`
  example already deployed on Sepolia.

## Install

```bash
npm install @fhevm/sdk
# plus your Ethereum library of choice:
npm install ethers   # or: npm install viem
```

Every import in this guide comes from one of two adapter entry points —
`@fhevm/sdk/ethers` or `@fhevm/sdk/viem`. Pick one and use it consistently. The
method names and shapes are identical across both.

## 1. Configure the runtime

Call `setFhevmRuntimeConfig` once, at startup, before you create any client. An
empty object accepts every default, which is the right starting point.

```ts
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';

setFhevmRuntimeConfig({});
```

The runtime controls how the WebAssembly cryptography modules are loaded — thread
count, asset location, and version pinning. See
[Runtime configuration](runtime-configuration.md) for every option. Calling it
twice with the same config is safe; calling it with a _different_ config throws,
so keep it in one place.

## 2. Create a client

A client bundles a chain definition with a read connection to the network. The
default `createFhevmClient` factory can both encrypt and decrypt.

{% tabs %}
{% tab title="ethers.js" %}

```ts
import { createFhevmClient } from '@fhevm/sdk/ethers';
import { sepolia } from '@fhevm/sdk/chains';
import { ethers } from 'ethers';

const provider = new ethers.JsonRpcProvider('https://ethereum-sepolia-rpc.publicnode.com');

const client = createFhevmClient({ chain: sepolia, provider });
```

{% endtab %}
{% tab title="viem" %}

```ts
import { createFhevmClient } from '@fhevm/sdk/viem';
import { sepolia } from '@fhevm/sdk/chains';
import { createPublicClient, http } from 'viem';
import { sepolia as viemSepolia } from 'viem/chains';

const publicClient = createPublicClient({
  chain: viemSepolia,
  transport: http('https://ethereum-sepolia-rpc.publicnode.com'),
});

const client = createFhevmClient({ chain: sepolia, publicClient });
```

{% endtab %}
{% endtabs %}

Constructing a client is synchronous and does no network or WASM work. Before
you encrypt or decrypt, resolve the protocol versions and load the WASM once:

```ts
await client.ready; // alias for `await client.init()`
```

`ready` is idempotent — awaiting it again returns the same cached promise. If you
only need to encrypt or only need to decrypt, use a lighter factory to download
less WASM — see [Clients](clients.md).

## 3. Encrypt values for a contract

`encryptValues` takes a batch of plaintext values, encrypts them client-side, and
returns opaque encrypted values plus a single `inputProof` that proves to the
contract they were encrypted correctly.

```ts
const userAddress = '0xYourWalletAddress';
const contractAddress = '0xef6c6230bF565015f8B37f2966d200C8804b409a';

const encrypted = await client.encryptValues({
  contractAddress,
  userAddress,
  values: [
    { type: 'uint32', value: 42 },
    { type: 'bool', value: true },
  ],
});

encrypted.encryptedValues; // [externalEuint32, externalEbool] — one per input
encrypted.inputProof; // one shared proof for the whole batch
```

Two rules make encryption work:

- **The `type` field uses Solidity names** (`'uint32'`, `'bool'`, `'address'`),
  not Fully Homomorphic Encryption (FHE) names. Values can be `number`,
  `bigint`, `boolean`, or an address string depending on the type.
- **Encryption is bound to `contractAddress` and `userAddress`.** The resulting
  proof is only valid when that exact user submits it to that exact contract.

{% hint style="warning" %}
The input proof is bound to both `contractAddress` and `userAddress`. If either differs when you submit the transaction, on-chain verification fails. Encrypt with the exact contract and sender you will transact with.
{% endhint %}

Pass the results straight to your contract call — each encrypted value maps to an
`externalEuintXX` / `externalEbool` argument, and the proof is the trailing
`bytes` argument:

```ts
await contract.increment(
  encrypted.encryptedValues[0], // externalEuint32
  encrypted.inputProof, // bytes
);
```

See [Encryption](encryption.md) for the full type table and single-value
`encryptValue`.

## 4. Decrypt a private value

Reading an encrypted value back requires proving you are allowed to see it. That
takes two pieces:

- A **transport key pair** — a throwaway key pair generated in your app. The
  Key Management System (KMS) encrypts the result under its public half so only
  you can reconstruct the
  plaintext. The private half never leaves your application.
- A **signed decryption permit** — an EIP-712 message, signed by the value's
  owner, authorizing decryption of a specific set of contracts for a limited
  time.

```ts
// A signer that can produce EIP-712 signatures (ethers Wallet / Signer,
// or a viem Account / WalletClient).
const signerAddress = '0xYourWalletAddress';

// 1. Generate the transport key pair.
const transportKeyPair = await client.generateTransportKeyPair();

// 2. Build and sign the permit in one step.
const signedPermit = await client.signDecryptionPermit({
  transportKeyPair,
  contractAddresses: [contractAddress],
  startTimestamp: Math.floor(Date.now() / 1000),
  durationSeconds: 7 * 24 * 60 * 60, // valid for 7 days
  signerAddress,
  signer, // ethers signer or viem account/wallet client
});

// 3. Decrypt an encrypted value the contract returned to you.
const decrypted = await client.decryptValue({
  transportKeyPair,
  encryptedValue, // a bytes32 handle read from the contract
  contractAddress,
  signedPermit,
});

decrypted.value; // 42 (number), 1000n (bigint), true, or "0x…" (address)
decrypted.type; // "uint32", "bool", "address", … (Solidity value-type names)
```

The result is a `TypedValue` — `{ type, value }` in the same value-domain
vocabulary you encrypt with. `uint8`/`uint16`/`uint32` come back as `number`,
`uint64`/`uint128`/`uint256` as `bigint`, `bool` as `boolean`, and `address` as
a checksummed string.

The plaintext is reconstructed locally from KMS shares — it is never sent in the
clear over the network. To decrypt several values at once, or to read values a
contract has made _public_, see [Decryption](decryption.md).

{% hint style="success" %}
That's the full loop: you encrypted a value, sent it to a contract, and decrypted a private result. Everything else in these docs builds on these three calls.
{% endhint %}

## Full example

The repository ships runnable end-to-end scripts for both adapters. They encrypt,
read public values, and privately decrypt a live Sepolia contract:

- [`examples/node-ethers/test-run.ts`](../examples/node-ethers/test-run.ts)
- [`examples/node-viem/test-run.ts`](../examples/node-viem/test-run.ts)

```bash
npx tsx ./examples/node-ethers/test-run.ts
```

## Next steps

- [Clients](clients.md) — choose the lightest client for your page.
- [Encryption](encryption.md) — supported types and batching.
- [Decryption](decryption.md) — private decryption, public values, delegation.
- [Runtime configuration](runtime-configuration.md) — threads, WASM loading, and
  browser headers.
- [Migration](migration.md) — moving from `@zama-fhe/relayer-sdk`.
```

