# Node.js: Encryption & User Decryption

A minimal Node.js example demonstrating the full `@fhevm/sdk` encrypt → user-decrypt lifecycle.

## What This Does

1. **Configures** the FHEVM runtime (WASM threads, logging)
2. **Creates** a full FHEVM client bound to Sepolia testnet
3. **Fetches** the global FHE public encryption parameters from the relayer
4. **Encrypts** a `euint32` and an `ebool` value for a target contract
5. **Generates** a KMS private key and wraps it in a `FhevmDecryptionKey`
6. **Creates** an EIP-712 user decryption permit scoped to the target contract
7. **Signs** the permit with the user's Ethereum wallet
8. **Decrypts** the encrypted handles via the relayer + KMS

## Prerequisites

- Node.js >= 22
- The SDK dependencies installed (`npm install` in the repo root)
- A wallet private key with access to a deployed FHE contract on Sepolia
- The target contract must have called `TFHE.allow(handle, userAddress)` for the encrypted values

## Setup

Edit `encrypt-and-user-decrypt.ts` and replace:

```ts
const CONTRACT_ADDRESS = "0x1E7eA8fE4877E6ea5dc8856f0dA92da8d5066241"; // your contract
const WALLET_PRIVATE_KEY = "0xYOUR_PRIVATE_KEY_HERE";                   // your key
```

## Run

```bash
npx tsx ./examples/node-encrypt-decrypt/encrypt-and-user-decrypt.ts
```

## Flow Diagram

```
┌─────────┐     ┌─────────┐     ┌─────────┐     ┌─────┐
│  User    │     │   SDK   │     │ Relayer │     │ KMS │
└────┬────┘     └────┬────┘     └────┬────┘     └──┬──┘
     │               │               │              │
     │  encrypt()    │               │              │
     │──────────────>│  ZK proof     │              │
     │               │──────────────>│              │
     │               │  signatures   │              │
     │               │<──────────────│              │
     │  proof        │               │              │
     │<──────────────│               │              │
     │               │               │              │
     │  sign permit  │               │              │
     │──────────────>│               │              │
     │               │               │              │
     │  userDecrypt() │              │              │
     │──────────────>│  ACL check   │              │
     │               │──────────────>│              │
     │               │  KMS shares   │              │
     │               │<─────────────────────────────│
     │               │  reconstruct  │              │
     │  plaintext    │  (TKMS WASM)  │              │
     │<──────────────│               │              │
```

## Notes

- The **KMS private key** is generated locally and never leaves the client. Only the corresponding public key is shared (embedded in the EIP-712 permit).
- The **EIP-712 permit** is reusable within its validity window (`durationDays`). You can decrypt multiple batches of handles without re-signing.
- The **relayer** coordinates between the SDK and the KMS network. It returns encrypted shares that only the holder of the KMS private key can decrypt.
- **ACL permissions** are checked on-chain before decryption. If `TFHE.allow()` was not called for the user + handle pair, decryption will fail with an `ACLError`.
