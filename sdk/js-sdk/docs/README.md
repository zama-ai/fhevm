# Overview

**Welcome to the FHEVM SDK!**

`@fhevm/sdk` is the JavaScript/TypeScript SDK for building apps on **FHEVM**
chains. It lets you encrypt values in the browser, send encrypted inputs to your
confidential smart contracts, and decrypt results — without plaintext ever
touching the blockchain. It powers this end-to-end flow using **Fully Homomorphic
Encryption (FHE)**, so your contracts compute directly on encrypted data.

You install it with npm and bring your own Ethereum library — the SDK ships
identical adapters for [ethers](https://docs.ethers.org/v6/) v6 and
[viem](https://viem.sh) v2:

```bash
npm install @fhevm/sdk
```

Requires **Node.js 22** or newer.

## What you can do

- **Encrypt** plaintext client-side and produce an input proof your contract can
  verify.
- **Decrypt** private results — the plaintext is reconstructed locally and never
  leaves the browser.
- **Read public values** a contract has made publicly readable.
- **Use one API for ethers and viem** — pick an adapter; the methods are
  identical.
- **Load only the cryptography you need** — encrypt-only, decrypt-only, or both.
- **Start with zero config** — built-in definitions for Ethereum mainnet and
  Sepolia.

## Where to go next

If you're new to the Zama Protocol, start with the
[Litepaper](https://docs.zama.ai/protocol/zama-protocol-litepaper) or the
[Protocol Overview](https://docs.zama.ai/protocol) to understand the foundations.

Otherwise:

🟨 Go to [**Getting started**](getting-started.md) to run your first encryption
and decryption in under five minutes.

🟨 Go to [**Clients**](clients.md) to choose the right client for your page.

🟨 Go to [**Encryption**](encryption.md) to encrypt values and build input
proofs.

🟨 Go to [**Decryption**](decryption.md) to decrypt private and public values.

🟨 Go to [**API reference**](api-reference.md) to look up an exact signature.

🟨 Go to [**Migration**](migration.md) if you're moving from the Relayer SDK
(`@zama-fhe/relayer-sdk`).

## Supported chains

| Chain            | ID         | Status     |
| ---------------- | ---------- | ---------- |
| Ethereum mainnet | 1          | Production |
| Ethereum Sepolia | 11155111   | Testnet    |

See [Chains](chains.md) to add a custom or local chain.

## Help center

Ask technical questions and discuss with the community.

- [Community forum](https://community.zama.ai/c/zama-protocol/15)
- [Discord channel](https://discord.com/invite/zama)
