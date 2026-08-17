# @fhevm/sdk

The JavaScript/TypeScript SDK for building applications on **FHEVM** chains. Encrypt values client-side, send encrypted inputs to your smart contracts, and decrypt results — all without exposing plaintext to the blockchain.

## Features

- **Encrypt** plaintext values client-side using TFHE (Fully Homomorphic Encryption)
- **Decrypt** private values with end-to-end encrypted transport — plaintext never leaves your application
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
import { setFhevmRuntimeConfig, createFhevmClient } from '@fhevm/sdk/ethers';
// or: from "@fhevm/sdk/viem"
import { sepolia } from '@fhevm/sdk/chains';
import { ethers } from 'ethers';

setFhevmRuntimeConfig({});

const provider = new ethers.JsonRpcProvider('https://ethereum-sepolia-rpc.publicnode.com');
const client = createFhevmClient({ chain: sepolia, provider });

// Resolve protocol versions and load WASM once, before encrypting or decrypting.
await client.ready;
```

### 2. Encrypt values

```ts
const encrypted = await client.encryptValues({
  contractAddress: '0xYourContract...',
  userAddress: '0xYourWallet...',
  values: [
    { type: 'uint32', value: 42 },
    { type: 'bool', value: true },
  ],
});

// Pass to your contract
await contract.myFunction(
  encrypted.encryptedValues[0], // externalEuint32
  encrypted.encryptedValues[1], // externalEbool
  encrypted.inputProof, // shared proof for all values
);
```

The `type` field uses Solidity value-type names (`'uint32'`, `'bool'`, `'address'`, `'uint8'`…`'uint256'`). Use `encryptValue` for a single value.

### 3. Decrypt private values

```ts
// Generate a transport key pair (private key never leaves your application)
const transportKeyPair = await client.generateTransportKeyPair();

// Create and sign a decrypt permit in one step
const signedPermit = await client.signDecryptionPermit({
  transportKeyPair,
  contractAddresses: ['0xYourContract...'],
  startTimestamp: Math.floor(Date.now() / 1000),
  durationSeconds: 7 * 24 * 60 * 60, // valid for 7 days
  signerAddress: await signer.getAddress(),
  signer,
});

// Decrypt
const decrypted = await client.decryptValue({
  transportKeyPair,
  encryptedValue: encryptedBalance, // a bytes32 handle read from the contract
  contractAddress: '0xYourContract...',
  signedPermit,
});

decrypted.value; // 42 (number), 1000n (bigint), true (boolean), or "0xAbCd..." (address)
decrypted.type; // "uint32", "uint64", "bool", "address", … (Solidity value-type name)
```

To decrypt several values at once, use `decryptValues` (same contract) or `decryptValuesFromPairs` (mixed contracts).

### 4. Read public values

```ts
const values = await client.decryptPublicValues({
  encryptedValues: [encryptedTotalSupply],
});

values[0].value; // the decrypted value
values[0].type; // its Solidity value-type name
```

## Client types

Use the lightest client for your page to minimize WASM download size:

| Client                       | Use case            | WASM loaded                   |
| ---------------------------- | ------------------- | ----------------------------- |
| `createFhevmClient()`        | Encrypt and decrypt | TFHE (~4.9MB) + TKMS (~600KB) |
| `createFhevmEncryptClient()` | Encrypt only        | TFHE (~4.9MB)                 |
| `createFhevmDecryptClient()` | Decrypt only        | TKMS (~600KB)                 |
| `createFhevmBaseClient()`    | Extend manually     | None                          |

Reading public values works on every client, including the base client. Constructing a client is synchronous and does no I/O; call `await client.ready` (an alias for `await client.init()`) once to load WASM and resolve protocol versions before you encrypt or decrypt.

## Import paths

| Path                         | What it provides                                   |
| ---------------------------- | -------------------------------------------------- |
| `@fhevm/sdk/ethers`          | Client factories and runtime config (ethers.js v6) |
| `@fhevm/sdk/viem`            | Client factories and runtime config (viem)         |
| `@fhevm/sdk/chains`          | Chain definitions (`mainnet`, `sepolia`)           |
| `@fhevm/sdk/types`           | Public TypeScript types and helpers                |
| `@fhevm/sdk/actions/base`    | Base actions (standalone functions)                |
| `@fhevm/sdk/actions/encrypt` | Encrypt actions                                    |
| `@fhevm/sdk/actions/decrypt` | Decrypt actions                                    |
| `@fhevm/sdk/actions/chain`   | Permit, key, and serialization actions             |
| `@fhevm/sdk/actions/host`    | Host contract read actions                         |

## Browser requirements

Multi-threaded encryption requires these HTTP headers:

```
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

Without them, the SDK falls back to single-threaded mode automatically.

## Supported chains

| Chain            | ID       | Status     |
| ---------------- | -------- | ---------- |
| Ethereum mainnet | 1        | Production |
| Ethereum Sepolia | 11155111 | Testnet    |

## Documentation

Full documentation is available in the [`docs/`](docs/) directory:

- [Overview](docs/README.md) — what the SDK does and where to start
- [Getting started](docs/getting-started.md) — install, configure, and run your first encryption
- [Clients](docs/clients.md) — client types and when to use each
- [Encryption](docs/encryption.md) — supported types, batch encryption, using the proof
- [Decryption](docs/decryption.md) — private decryption, public values, delegation
- [Chains](docs/chains.md) — built-in chains and custom chain definitions
- [Runtime configuration](docs/runtime-configuration.md) — threads, WASM loading, browser headers
- [Runtime compatibility](docs/runtime-compatibility.md) — supported environments, SSR/CSR, Edge, bundlers
- [Error handling](docs/error-handling.md) — error classes and handling patterns
- [Types](docs/types.md) — the TypeScript type system
- [Actions](docs/actions.md) — the tree-shakable functional API
- [API reference](docs/api-reference.md) — complete function and type reference
- [Version compatibility](docs/compatibility.md) — protocol, TFHE, KMS, and contract version matrices
- [Security model](docs/security.md) — encryption, ACL, permits, and what the SDK protects
- [Migration](docs/migration.md) — migrating from `@zama-fhe/relayer-sdk`
- [Architecture](docs/architecture.md) — internal design for contributors
- [Glossary](docs/GLOSSARY.md) — canonical naming across the SDK, docs, and Zama Protocol
- [Release notes](docs/release-notes.md) — changelog and breaking changes

## Contributing

External pull requests are not accepted. To report a bug, request a feature, or start a discussion, please [open an issue](https://github.com/zama-ai/fhevm/issues).

For security vulnerabilities, use [private vulnerability reporting](https://github.com/zama-ai/fhevm/security/advisories/new) or follow the process in [SECURITY.md](../../SECURITY.md).

## License

This project is licensed under the [BSD 3-Clause Clear License](LICENSE).

Copyright © 2025 ZAMA. All rights reserved.
