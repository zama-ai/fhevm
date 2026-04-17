# Node.js + ethers.js v6

A single example demonstrating the full `@fhevm/sdk` lifecycle using ethers.js v6:
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

- The **E2E transport key pair** is generated locally and never leaves the client. Only the corresponding public key is shared (embedded in the EIP-712 permit).
- The **EIP-712 permit** is reusable within its validity window (`durationDays`). You can decrypt multiple batches of encrypted values without re-signing.
- The **Zama Protocol** processes decryption requests and returns encrypted shares that only the holder of the transport key pair can decrypt.
- **ACL permissions** are checked on-chain before decryption. If `FHE.allow()` was not called for the user + encrypted value pair, decryption will fail.
