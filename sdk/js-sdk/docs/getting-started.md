# Getting started

This guide walks you through installing the SDK, connecting to an FHEVM chain, and performing your first encryption and decryption. By the end, you'll understand the basic flow of working with encrypted data.

## Prerequisites

- **Node.js >= 22.0**
- **An Ethereum provider** — either ethers.js v6 or viem. If you don't have a preference, viem is a good default.
- **Access to an FHEVM chain** — currently Ethereum mainnet or Sepolia testnet. For testing, use Sepolia.

## Installation

```bash
npm install @fhevm/sdk
```

The SDK ships with everything you need — no additional WASM downloads or native dependencies.

## Step 1: Configure the runtime

Before you can do anything, you need to tell the SDK how to run its internal WASM modules. This is a one-time setup that you do at app startup, before creating any clients. The configuration is different depending on whether you're running in Node.js or the browser.

### Node.js

In Node.js, the SDK resolves WASM file paths automatically. You only need to set the thread count:

```ts
import { setFhevmRuntimeConfig } from '@fhevm/sdk/viem'; // or "@fhevm/sdk/ethers"

setFhevmRuntimeConfig({
  numberOfThreads: 4,
});
```

That's it — the SDK finds the WASM files using `__filename` (CJS) or `import.meta.url` (ESM) and uses `node:worker_threads` for multi-threading.

### Browser

In the browser, there are two extra things to configure: **WASM file locations** and **HTTP headers for multi-threading**.

**WASM files:** The SDK embeds WASM as base64 strings by default, so it works without any file hosting. However, if you want to serve the WASM files from your own server or CDN (recommended for production — avoids inlining ~5MB of base64 in your JavaScript bundle), use `locateFile`:

```ts
import { setFhevmRuntimeConfig } from '@fhevm/sdk/viem';

setFhevmRuntimeConfig({
  numberOfThreads: navigator.hardwareConcurrency || 4,
  locateFile: (file) => new URL(`/wasm/${file}`, window.location.origin),
});
```

When using `locateFile`, you need to copy these files to your `/wasm/` directory (or wherever you point to):

| File                     | Size | Purpose                                   |
| ------------------------ | ---- | ----------------------------------------- |
| `tfhe_bg.v1.5.3.wasm`    | ~5MB | TFHE encryption WASM binary               |
| `tfhe-worker.v1.5.3.mjs` | ~2KB | Web Worker script for multi-threaded TFHE |

**HTTP headers for multi-threading:** The TFHE WASM module uses `SharedArrayBuffer` for multi-threading, which browsers only enable when your server sends these headers:

```
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

For **Next.js**, add to `next.config.js`:

```js
module.exports = {
  async headers() {
    return [
      {
        source: '/(.*)',
        headers: [
          { key: 'Cross-Origin-Opener-Policy', value: 'same-origin' },
          { key: 'Cross-Origin-Embedder-Policy', value: 'require-corp' },
        ],
      },
    ];
  },
};
```

For **Vite**, add to `vite.config.ts`:

```ts
export default defineConfig({
  server: {
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
  },
});
```

Without these headers, the SDK falls back to **single-threaded mode** automatically. Encryption still works — it's just slower. For more details, see [Runtime configuration](runtime-configuration.md).

## Step 2: Create a client

A **client** is your main interface to the SDK. It's bound to a specific chain and provider, and gives you methods like `encrypt()`, `decrypt()`, and `publicDecrypt()`.

**With viem:**

```ts
import { createFhevmClient } from '@fhevm/sdk/viem';
import { sepolia } from '@fhevm/sdk/chains';
import { createPublicClient, http } from 'viem';
import { sepolia as viemSepolia } from 'viem/chains';

const provider = createPublicClient({
  chain: viemSepolia,
  transport: http('https://ethereum-sepolia-rpc.publicnode.com'),
});

const client = createFhevmClient({ chain: sepolia, provider });
```

**With ethers.js:**

```ts
import { createFhevmClient } from '@fhevm/sdk/ethers';
import { sepolia } from '@fhevm/sdk/chains';
import { ethers } from 'ethers';

const provider = new ethers.JsonRpcProvider('https://ethereum-sepolia-rpc.publicnode.com');
const client = createFhevmClient({ chain: sepolia, provider });
```

Creating a client is instant — no network calls, no WASM loading. Everything happens lazily on first use. If you want to preload WASM and the public encryption key (for example, behind a loading spinner at app startup), you can:

```ts
await client.init(); // or: await client.ready
```

This is optional — if you skip it, initialization happens automatically on the first `encrypt()` call.

## Step 3: Encrypt and send to your contract

Encryption turns plaintext values into encrypted inputs that your smart contract can work with. The flow is: **encrypt on the frontend**, then **pass the result to your contract**.

### 3a. Encrypt values on the frontend

Call `encrypt()` with the values you want to encrypt, the target contract address, and the user's wallet address:

```ts
const encrypted = await client.encrypt({
  contractAddress: '0xYourCounter...',
  userAddress: '0xYourWallet...',
  values: [{ type: 'uint32', value: 42 }],
});
```

The result contains:

- `encrypted.externalEncryptedValues` — one encrypted value per input, in the same order you provided them
- `encrypted.inputProof` — a ZK proof that the encryption was done correctly

On the first call, the SDK automatically downloads the network's public encryption key (~50MB, cached for subsequent calls) and initializes the TFHE WASM module.

### 3b. Pass the encrypted values to your contract

Now send the encrypted value and proof to your contract. The two values map directly to the Solidity function parameters:

```ts
await contract.increment(
  encrypted.externalEncryptedValues[0], // → externalEuint32 in Solidity
  encrypted.inputProof, // → bytes calldata inputProof in Solidity
);
```

On the Solidity side, the contract accepts these as `externalEuint32` + `bytes calldata inputProof`, and converts them into an encrypted type it can compute on:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {FHE, euint32, externalEuint32} from "fhevm/lib/FHE.sol";

contract Counter {
    euint32 private count;

    function increment(externalEuint32 encryptedAmount, bytes calldata inputProof) public {
        // Convert the external encrypted input into a usable encrypted value
        euint32 amount = FHE.fromExternal(encryptedAmount, inputProof);
        // Perform encrypted addition
        count = FHE.add(count, amount);
        // Allow this contract to use the new count value in future operations
        FHE.allowThis(count);
    }
}
```

### Encrypting multiple values

You can encrypt multiple values in a single `encrypt()` call (up to 2048 encrypted bits total). Each value gets its own entry in `externalEncryptedValues`, in the same order:

```ts
const encrypted = await client.encrypt({
  contractAddress: '0xYourContract...',
  userAddress: '0xYourWallet...',
  values: [
    { type: 'uint32', value: 42 },
    { type: 'bool', value: true },
    { type: 'address', value: '0xAbCdEf0123456789AbCdEf0123456789AbCdEf01' },
  ],
});

// Pass to a contract that accepts (externalEuint32, externalEbool, externalEaddress, bytes)
await contract.myFunction(
  encrypted.externalEncryptedValues[0], // externalEuint32
  encrypted.externalEncryptedValues[1], // externalEbool
  encrypted.externalEncryptedValues[2], // externalEaddress
  encrypted.inputProof, // one shared proof for all values
);
```

The `inputProof` is shared — one proof covers all encrypted values in the batch. For the full list of supported types and their bit costs, see [Encryption](encryption.md).

## Step 4: Decrypt private values

`decrypt()` is for private data — only the user who owns the encrypted value can see the plaintext. On the Solidity side, the contract must grant the user permission when storing or updating the value:

```solidity
function deposit(externalEuint64 encryptedAmount, bytes calldata inputProof) public {
    euint64 amount = FHE.fromExternal(encryptedAmount, inputProof);
    balances[msg.sender] = FHE.add(balances[msg.sender], amount);

    // Grant the user permission to decrypt their own balance
    FHE.allow(balances[msg.sender], msg.sender);
    FHE.allowThis(balances[msg.sender]);
}
```

On the SDK side, decryption requires two things:

1. **A transport key pair** — generated by the SDK, encrypts the communication between your app and the Zama Protocol so that only you can read the decrypted result
2. **A signed decrypt permit** — created and signed in a single step via `signDecryptionPermit()`, authorizing decryption for specific contracts and a time window

Here's the full flow:

```ts
// 1. Generate a transport key pair
//    This encrypts the channel between your app and the Zama Protocol —
//    only your app can read the decrypted values.
const e2eTransportKeyPair = await client.generateE2eTransportKeyPair();

// 2. Create and sign the decrypt permit in one step
//    The SDK constructs the EIP-712 message and signs it with your signer.
const signedPermit = await client.signDecryptionPermit({
  contractAddresses: ['0xYourContract...'],
  startTimestamp: Math.floor(Date.now() / 1000),
  durationDays: 7,
  signerAddress: await signer.getAddress(), // or walletClient.account.address for viem
  signer, // ethers Signer or viem WalletClient
  e2eTransportKeyPair,
});

// 3. Decrypt — pass the encrypted value you read from your contract
const encryptedBalance = await contract.balances(userAddress);

const results = await client.decrypt({
  encryptedValues: [{ encryptedValue: encryptedBalance, contractAddress: '0xYourContract...' }],
  e2eTransportKeyPair,
  signedPermit,
});

// Access the plaintext
results[0].value; // e.g. 42 (number) or 1000n (bigint)
results[0].fheType; // "euint32", "euint64", "ebool", etc.
```

Decrypted values are typed based on their FHE type:

| FHE type                          | JavaScript type | Example       |
| --------------------------------- | --------------- | ------------- |
| `ebool`                           | `boolean`       | `true`        |
| `euint8`, `euint16`, `euint32`    | `number`        | `42`          |
| `euint64`, `euint128`, `euint256` | `bigint`        | `1000n`       |
| `eaddress`                        | `string`        | `"0xAbCd..."` |

**Why so many steps?** Security. The permit system ensures that decryption only happens when the user explicitly authorizes it, for specific contracts, within a specific time window. The transport key pair never leaves the browser — the Zama Protocol sends encrypted shares that only this key can reconstruct.

The signed permit is **reusable** — you can save it (e.g., in your database or local storage) and use it for multiple `decrypt()` calls without asking the user to sign again, as long as the permit hasn't expired.

### Decrypt private values for somebody else

If a user grants you access to decrypt (via the ACL on-chain), you can create a permit that decrypts their values. On the Solidity side, the data owner grants access in a prior transaction:

```solidity
// Called by the data owner — grants delegate access to their encrypted balance
FHE.allow(balances[owner], delegate);
```

On the SDK side — same flow as above, but with `onBehalfOf` to specify whose data you're decrypting:

```ts
const signedPermit = await client.signDecryptionPermit({
  contractAddresses: ['0xYourContract...'],
  startTimestamp: Math.floor(Date.now() / 1000),
  durationDays: 7,
  signerAddress: await signer.getAddress(),
  signer,
  e2eTransportKeyPair,
  onBehalfOf: '0xDataOwnerAddress...',
});
```

Everything else (calling `decrypt()`) is the same. The Zama Protocol verifies that the on-chain ACL grants the signer access to the owner's values.

## Step 5: Read public values

`publicDecrypt()` reveals encrypted values that your smart contract has explicitly marked as publicly readable. Anyone can call this — no keys or signatures needed.

On the Solidity side, the contract marks a value as publicly decryptable:

```solidity
// Make the total supply readable by anyone
FHE.makePubliclyDecryptable(totalSupply);
```

> **Warning:** Once a value is made publicly readable, it **cannot be reverted**. The plaintext becomes visible to everyone permanently. See the [ACL guide](https://docs.zama.org/protocol/solidity-guides/smart-contract/acl) for details.

On the SDK side, read the encrypted value from your contract, then pass it to `publicDecrypt()`:

```ts
// Read the encrypted value from the contract
const encryptedTotalSupply = await contract.totalSupply();

const result = await client.publicDecrypt({
  encryptedValues: [encryptedTotalSupply],
});

// Access the plaintext
const value = result.orderedClearValues[0].value; // the decrypted value
const type = result.orderedClearValues[0].fheType; // "euint32", "ebool", etc.
```

**When would you use this?** For values that should be visible to everyone — like the result of a completed auction, a public vote tally, or a game outcome.

## Import paths

| Path                         | What it gives you                                                |
| ---------------------------- | ---------------------------------------------------------------- |
| `@fhevm/sdk/ethers`          | Client factories + runtime config (ethers.js v6 adapter)         |
| `@fhevm/sdk/viem`            | Client factories + runtime config (viem adapter) — identical API |
| `@fhevm/sdk/chains`          | Chain definitions: `mainnet`, `sepolia`                          |
| `@fhevm/sdk/actions/base`    | Base actions (standalone functions)                              |
| `@fhevm/sdk/actions/encrypt` | Encrypt actions (standalone functions)                           |
| `@fhevm/sdk/actions/decrypt` | Decrypt actions (standalone functions)                           |
| `@fhevm/sdk/actions/chain`   | Chain-only utility actions (standalone functions)                |
| `@fhevm/sdk/actions/host`    | Host contract read actions (standalone functions)                |

The ethers and viem adapters have **identical APIs**. You can switch between them by changing only the import path — no other code changes.

## Where to go next

- Clone the [**FHEVM React template**](https://github.com/zama-ai/fhevm-react-template) for a ready-made Next.js project with encryption and decryption already wired up.

- Go to [**Clients**](clients.md) to understand when to use `createFhevmClient` vs. `createFhevmEncryptClient` vs. `createFhevmDecryptClient`.

- Go to [**Encryption**](encryption.md) for the full encryption API, supported types, and serialization.

- Go to [**Decryption**](decryption.md) for reading public values, decrypting private data, decrypting on behalf of other users, and permits.
