<p align="center">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/zama-ai/fhevm/main/docs/.gitbook/assets/fhevm-header-dark.png">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/zama-ai/fhevm/main/docs/.gitbook/assets/fhevm-header-light.png">
  <img width=500 alt="FHEVM" src="https://raw.githubusercontent.com/zama-ai/fhevm/main/docs/.gitbook/assets/fhevm-header-light.png">
</picture>
</p>

<hr/>

<p align="center">
  <a href="https://github.com/zama-ai/fhevm/tree/main/sdk/js-sdk/docs"> 📒 Documentation</a> | <a href="https://community.zama.org"> 💛 Community support</a> | <a href="https://github.com/zama-ai/awesome-zama"> 📚 FHE resources by Zama</a>
</p>

<p align="center">
  <a href="https://www.npmjs.com/package/@fhevm/sdk">
    <img src="https://img.shields.io/npm/v/%40fhevm%2Fsdk?label=latest%20release&style=flat-square" alt="Latest release"></a>
  <a href="https://www.npmjs.com/package/@fhevm/sdk">
    <img src="https://img.shields.io/npm/last-update/%40fhevm%2Fsdk/latest?style=flat-square" alt="Latest release last updated"></a>
  <a href="https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/LICENSE">
    <img src="https://img.shields.io/badge/License-BSD--3--Clause--Clear-%23ffb243?style=flat-square" alt="License"></a>
  <a href="https://github.com/zama-ai/bounty-program">
    <img src="https://img.shields.io/badge/Contribute-Zama%20Bounty%20Program-%23ffd208?style=flat-square" alt="Zama Bounty Program"></a>
</p>

# @fhevm/sdk

Low-level TypeScript SDK for building applications on **FHEVM** chains. Encrypt values client-side, send encrypted inputs to your smart contracts, and decrypt results — all without exposing plaintext to the blockchain.

Use this package in browser or Node.js code when you need direct access to the FHEVM primitives: encrypted inputs, private and public decryption, decryption permits, and host contract reads.

> **Building a confidential token app?** The higher-level [`@zama-fhe/sdk`](https://www.npmjs.com/package/@zama-fhe/sdk) (and [`@zama-fhe/react-sdk`](https://www.npmjs.com/package/@zama-fhe/react-sdk)) provides ERC-7984 token operations, sessions, and React hooks on top of the Zama Protocol.
>
> **Migrating from `@zama-fhe/relayer-sdk`?** See the [migration guide](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/migration.md).

## Main features

- **Encrypt** plaintext values client-side using TFHE (Fully Homomorphic Encryption), with a single proof for a batch of values.
- **Decrypt** private values with end-to-end encrypted transport — plaintext never leaves your application.
- **Read public values** that contracts have marked as publicly decryptable.
- **Dual adapter support** — identical API for both ethers.js v6 and viem.
- **Tree-shakable** — only load the WASM modules you need (encrypt-only, decrypt-only, or both), plus standalone functional actions.
- **Zero config** — built-in chain definitions for Ethereum and Polygon, mainnet and testnet.

## Installation

```bash
npm install @fhevm/sdk
# or
pnpm add @fhevm/sdk
# or
yarn add @fhevm/sdk
```

Install the EVM library you use alongside it: `ethers` (v6) for `@fhevm/sdk/ethers`, or `viem` for `@fhevm/sdk/viem`.

Requires **Node.js >= 22**.

## Quick start

### 1. Configure the runtime and create a client

```ts
import { setFhevmRuntimeConfig, createFhevmClient } from '@fhevm/sdk/ethers';
// or: from '@fhevm/sdk/viem'
import { sepolia } from '@fhevm/sdk/chains';
import { ethers } from 'ethers';

// Call once, at startup, before creating any client.
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
// Generate a transport key pair (the private key never leaves your application)
const transportKeyPair = await client.generateTransportKeyPair();

// Create and sign a decryption permit in one step
const signedPermit = await client.signDecryptionPermit({
  transportKeyPair,
  contractAddresses: ['0xYourContract...'],
  startTimestamp: Math.floor(Date.now() / 1000),
  durationSeconds: 7 * 24 * 60 * 60, // valid for 7 days
  signerAddress: await signer.getAddress(),
  signer,
});

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

## What this package includes

### Clients

Use the lightest client for your page to minimize WASM download size:

| Client                       | Use case            | WASM loaded                   |
| ---------------------------- | ------------------- | ----------------------------- |
| `createFhevmClient()`        | Encrypt and decrypt | TFHE (~4.9MB) + TKMS (~600KB) |
| `createFhevmEncryptClient()` | Encrypt only        | TFHE (~4.9MB)                 |
| `createFhevmDecryptClient()` | Decrypt only        | TKMS (~600KB)                 |
| `createFhevmBaseClient()`    | Extend manually     | None                          |

Reading public values works on every client, including the base client. Constructing a client is synchronous and does no I/O; call `await client.ready` (an alias for `await client.init()`) once to load WASM and resolve protocol versions before you encrypt or decrypt.

### Import paths

| Path                                                       | What it provides                                                |
| ---------------------------------------------------------- | --------------------------------------------------------------- |
| `@fhevm/sdk/ethers`                                        | Client factories and runtime config (ethers.js v6)              |
| `@fhevm/sdk/viem`                                          | Client factories and runtime config (viem)                      |
| `@fhevm/sdk/ethers/cleartext`, `@fhevm/sdk/viem/cleartext` | Cleartext clients for local development against cleartext hosts |
| `@fhevm/sdk/chains`                                        | Chain definitions and `defineFhevmChain` for custom chains      |
| `@fhevm/sdk/types`                                         | Public TypeScript types and helpers                             |
| `@fhevm/sdk/actions/base`                                  | Base actions (standalone functions)                             |
| `@fhevm/sdk/actions/encrypt`                               | Encrypt actions                                                 |
| `@fhevm/sdk/actions/decrypt`                               | Decrypt actions                                                 |
| `@fhevm/sdk/actions/chain`                                 | Permit, key, and serialization actions                          |
| `@fhevm/sdk/actions/host`                                  | Host contract read actions                                      |

## Browser requirements

Multi-threaded encryption requires these HTTP headers:

```text
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

Without them, the SDK falls back to single-threaded mode automatically. See [Runtime compatibility](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/runtime-compatibility.md) for SSR, Edge, and bundler notes.

## Documentation

Full documentation lives in the [`sdk/js-sdk/docs`](https://github.com/zama-ai/fhevm/tree/main/sdk/js-sdk/docs) directory of the FHEVM repository:

- [Getting started](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/getting-started.md) — install, configure, and run your first encryption
- [Clients](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/clients.md), [Encryption](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/encryption.md), [Decryption](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/decryption.md), and [Chains](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/chains.md) — focused guides
- [Runtime configuration](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/runtime-configuration.md) — threads, WASM loading, browser headers
- [Error handling](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/error-handling.md) — error classes and handling patterns
- [Actions](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/actions.md) — the tree-shakable functional API
- [API reference](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/api-reference.md) — complete function and type reference
- [Version compatibility](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/compatibility.md) — protocol, TFHE, KMS, and contract version matrices
- [Security model](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/security.md) — encryption, ACL, permits, and what the SDK protects
- [Migration](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/migration.md) — migrating from `@zama-fhe/relayer-sdk`
- [Release notes](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/docs/release-notes.md) — changelog and breaking changes

## Contributing

External pull requests are not accepted. To report a bug, request a feature, or start a discussion, please [open an issue](https://github.com/zama-ai/fhevm/issues).

For security vulnerabilities, use [private vulnerability reporting](https://github.com/zama-ai/fhevm/security/advisories/new) or follow the process in [SECURITY.md](https://github.com/zama-ai/fhevm/blob/main/SECURITY.md).

## License

This software is distributed under the **BSD-3-Clause-Clear** license. Read [this](https://github.com/zama-ai/fhevm/blob/main/sdk/js-sdk/LICENSE) for more details.

## Support

🌟 If you find this project helpful or interesting, please consider giving [FHEVM](https://github.com/zama-ai/fhevm) a star on GitHub! Your support helps to grow the community and motivates further development.

<a target="_blank" href="https://community.zama.org">
  💛 Community forum on Discourse
</a>
