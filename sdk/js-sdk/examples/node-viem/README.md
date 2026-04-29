# Node.js + viem

A single example demonstrating the full `@fhevm/sdk` lifecycle using viem:
encryption, reading public values, and private decryption.

## Prerequisites

- Node.js >= 22
- SDK dependencies installed (`npm install` in the repo root)

## Setup

Optionally provide a wallet private key:

```bash
cp .env.local.example .env.local
```

Without `.env.local`, a random wallet is used. The full flow still runs but decryption will fail (encrypted values are not on-chain).

## Run

```bash
npx tsx ./test-run.ts
```

## Notes

- This example uses the **viem adapter** (`@fhevm/sdk/viem`). The API is identical to the ethers adapter — only the import path and provider type differ.
- The viem adapter uses `PublicClient<Transport, Chain>` instead of ethers' `ContractRunner`.
- EIP-712 signing uses `walletClient.signTypedData()` instead of ethers' `wallet.signTypedData()`.
- The **E2E transport key pair** is generated locally and never leaves the client. Only the corresponding public key is shared (embedded in the EIP-712 permit).
- The **Zama Protocol** processes decryption requests and returns encrypted shares that only the holder of the transport key pair can decrypt.
- **ACL permissions** are checked on-chain before decryption. If `FHE.allow()` was not called for the user + encrypted value pair, decryption will fail.
