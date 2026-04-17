# @fhevm/sdk

The JavaScript/TypeScript SDK for building applications on **FHEVM** chains. Encrypt values in the browser, send encrypted inputs to your smart contracts, and decrypt results — all without exposing plaintext to the blockchain.

## Features

- **Encrypt** plaintext values client-side using TFHE (Fully Homomorphic Encryption)
- **Decrypt** private values with end-to-end encrypted transport — plaintext never leaves the browser
- **Read public values** that contracts have marked as publicly decryptable
- **Dual adapter support** — identical API for both ethers.js v6 and viem
- **Tree-shakable** — only load the WASM modules you need (encrypt-only, decrypt-only, or both)
- **Zero config** — works out of the box with built-in chain definitions for Ethereum mainnet and Sepolia

## Installation

```bash
npm install @fhevm/sdk
```

Requires **Node.js >= 22.0**.

## Quick start

### 1. Configure the runtime and create a client

```ts
import { setFhevmRuntimeConfig, createFhevmClient } from "@fhevm/sdk/ethers";
// or: from "@fhevm/sdk/viem"
import { sepolia } from "@fhevm/sdk/chains";
import { ethers } from "ethers";

setFhevmRuntimeConfig({ numberOfThreads: 4 });

const provider = new ethers.JsonRpcProvider("https://ethereum-sepolia-rpc.publicnode.com");
const client = createFhevmClient({ chain: sepolia, provider });
```

### 2. Encrypt values

```ts
const encrypted = await client.encrypt({
  contractAddress: "0xYourContract...",
  userAddress: "0xYourWallet...",
  values: [
    { type: "uint32", value: 42 },
    { type: "bool", value: true },
  ],
});

// Pass to your contract
await contract.myFunction(
  encrypted.externalEncryptedValues[0], // externalEuint32
  encrypted.externalEncryptedValues[1], // externalEbool
  encrypted.inputProof,                 // shared proof for all values
);
```

### 3. Decrypt private values

```ts
// Generate a transport key pair (private key never leaves the browser)
const e2eTransportKeypair = await client.generateE2eTransportKeypair();

// Create and sign a decrypt permit in one step
const signedPermit = await client.signDecryptionPermit({
  contractAddresses: ["0xYourContract..."],
  startTimestamp: Math.floor(Date.now() / 1000),
  durationDays: 7,
  signerAddress: await signer.getAddress(),
  signer,
  e2eTransportKeypair,
});

// Decrypt
const results = await client.decrypt({
  encryptedValues: [
    { encryptedValue: encryptedBalance, contractAddress: "0xYourContract..." },
  ],
  e2eTransportKeypair,
  signedPermit,
});

results[0].value;   // 42 (number), 1000n (bigint), true (boolean), or "0xAbCd..." (address)
results[0].fheType; // "euint32", "euint64", "ebool", "eaddress", etc.
```

### 4. Read public values

```ts
const result = await client.publicDecrypt({
  encryptedValues: [encryptedTotalSupply],
});

result.orderedClearValues[0].value; // the decrypted value
```

## Client types

Use the lightest client for your page to minimize WASM download size:

| Client | Use case | WASM loaded |
| --- | --- | --- |
| `createFhevmClient()` | Encrypt and decrypt | TFHE (~5MB) + TKMS (~600KB) |
| `createFhevmEncryptClient()` | Encrypt only | TFHE (~5MB) |
| `createFhevmDecryptClient()` | Decrypt only | TKMS (~600KB) |
| `createFhevmBaseClient()` | Extend manually | None |

WASM modules load **lazily** on first use. Call `await client.init()` to preload eagerly.

## Import paths

| Path | What it provides |
| --- | --- |
| `@fhevm/sdk/ethers` | Client factories and runtime config (ethers.js v6) |
| `@fhevm/sdk/viem` | Client factories and runtime config (viem) |
| `@fhevm/sdk/chains` | Chain definitions (`mainnet`, `sepolia`) |
| `@fhevm/sdk/actions/base` | Base actions (standalone functions) |
| `@fhevm/sdk/actions/encrypt` | Encrypt actions |
| `@fhevm/sdk/actions/decrypt` | Decrypt actions |
| `@fhevm/sdk/actions/chain` | Chain utility actions |
| `@fhevm/sdk/actions/host` | Host contract read actions |

## Browser requirements

Multi-threaded encryption requires these HTTP headers:

```
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

Without them, the SDK falls back to single-threaded mode automatically.

## Supported chains

| Chain | ID | Status |
| --- | --- | --- |
| Ethereum mainnet | 1 | Production |
| Ethereum Sepolia | 11155111 | Testnet |

## Documentation

Full documentation is available in the [`docs/`](docs/) directory:

- [Getting started](docs/getting-started.md) — Install, configure, and run your first encryption
- [Clients](docs/clients.md) — Client types and when to use each
- [Encryption](docs/encryption.md) — Supported types, batch encryption, step-by-step flow
- [Decryption](docs/decryption.md) — Private decryption, public values, delegation
- [Chains](docs/chains.md) — Built-in chains and custom chain definitions
- [API reference](docs/api-reference.md) — Complete function and type reference
- [Types](docs/types.md) — TypeScript type system
- [Migration](docs/migration.md) — Migrating from `@zama-fhe/relayer-sdk`
- [Architecture](docs/architecture.md) — Internal design for contributors

## Glossary

See [GLOSSARY.md](GLOSSARY.md) for canonical naming conventions across the SDK, docs, and Zama Protocol — including encrypted values, clear values, key pairs, permits, and FHE types.

## License

This project is licensed under the [BSD 3-Clause Clear License](LICENSE).

Copyright © 2025 ZAMA. All rights reserved.
