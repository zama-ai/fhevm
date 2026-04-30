# Security model

This page explains the security guarantees of FHEVM and how the SDK fits into the picture. Understanding this model helps you make informed decisions about what data to encrypt and how to handle decryption.

## The big picture

In FHEVM, **encrypted data is processed without ever being decrypted**. Your smart contract adds, compares, and transforms encrypted values — and the plaintext is never visible to the blockchain, validators, or anyone observing the chain.

There are three entities involved:

| Entity                                         | What it sees                                            | What it can do                                        |
| ---------------------------------------------- | ------------------------------------------------------- | ----------------------------------------------------- |
| **Blockchain** (validators, explorers)         | Encrypted values only                                   | Execute FHE operations on ciphertexts                 |
| **Zama Protocol** (distributed infrastructure) | Nothing until decryption is requested                   | Produce decryption shares on authorized request       |
| **User** (your app, browser)                   | Plaintext before encryption, plaintext after decryption | Encrypt values, request decryption with signed permit |

## Who can decrypt what

Decryption is controlled by two mechanisms:

### 1. The ACL contract (on-chain)

The **Access Control List (ACL)** is a smart contract that tracks permissions. Every encrypted value has a list of addresses that are allowed to interact with it.

Your smart contract grants permissions using `FHE.allow()`:

```solidity
// In your Solidity contract:
FHE.allow(encryptedBalance, msg.sender);         // user can decrypt their own balance
FHE.allowForDecryption(encryptedResult);          // anyone can read this value (public)
```

The SDK checks ACL permissions before every decryption request. If the user or contract isn't in the ACL, the request fails before leaving your browser.

### 2. The EIP-712 permit (off-chain)

For private decryption, the user must also sign an EIP-712 permit that specifies:

- Which contracts they're authorizing decryption for
- A time window (start timestamp + duration in days)
- The public key the Zama Protocol should use to encrypt the response

This means even if the ACL grants permission, the user must explicitly opt in to each decryption by signing with their wallet. No one can decrypt a user's data without their wallet signature.

## How decryption works

The Zama Protocol uses a **distributed key management** system — no single server holds the full decryption key.

When you request decryption:

1. The SDK sends the request (with the signed permit) to the Zama Protocol
2. Multiple independent nodes each produce a **decryption share** — a partial decryption encrypted with your public key
3. The shares are sent back to your browser
4. The SDK reconstructs the plaintext locally using your private key (TKMS WASM)

**No server ever sees the plaintext.** Each node only sees its own partial share, and the shares are encrypted for your specific public key. The reconstruction happens entirely in your browser.

## What the SDK protects

The SDK is designed to prevent accidental exposure of sensitive material:

### Transport keys are opaque

When you call `generateE2eTransportKeyPair()`, the returned object wraps the private key in a way that prevents accidental logging or serialization:

```ts
const keypair = await client.generateE2eTransportKeyPair();
console.log(keypair); // Does NOT print the private key
```

The raw private key is stored in a private field (`#field`) and can only be accessed through the SDK's internal symbol-based access control. You can get the **public** key via `keypair.publicKey`, but the private key is never exposed to your application code.

### The provider is sealed

The ethers/viem provider you pass to `createFhevmClient()` is sealed inside an opaque `TrustedClient` wrapper. The core SDK logic can use it for RPC calls (reading contracts, checking ACL permissions) but cannot extract the underlying provider object. This prevents the core layer from accidentally depending on ethers-specific or viem-specific APIs.

### Chain definitions are frozen

Chain objects (`mainnet`, `sepolia`, or your custom chains) are deep-frozen with `Object.freeze()` after creation. This prevents accidental or malicious mutation of contract addresses at runtime.

### Error messages exclude sensitive data

SDK error messages never include private keys, signatures, or raw encrypted data. They include enough context to debug (contract addresses, error codes) without leaking sensitive information.

## What you should protect

The SDK handles internal security, but your application has its own responsibilities:

### Never log decryption results

After calling `decrypt()`, the plaintext values are in your application's memory. Don't send them to analytics, error tracking, or console logs unless you intend to make them public.

### Store transport keys securely

If you persist key pairs across sessions (via `parseE2eTransportKeyPair`), store the serialized bytes securely:

```ts
// Acceptable: encrypted localStorage, secure enclave, browser credential store
// Risky: plain localStorage (accessible to any JS on the domain)
// Never: URL parameters, cookies, unencrypted server-side storage
```

### Validate contract addresses

When passing `contractAddress` to `encrypt()` or `encryptedValues` to `decrypt()`, make sure these addresses are correct. Encrypting for the wrong contract means the data is bound to that contract's ACL — there's no way to "re-target" it after encryption.

### Permit scope should be minimal

When creating EIP-712 permits, scope them as tightly as possible:

- Include only the contracts that actually need decryption
- Use short durations when possible
- Generate a fresh key pair per session rather than reusing across sessions

## For more examples

For complete working applications that demonstrate these security patterns in practice, see the [FHEVM dApps repository](https://github.com/zama-ai/dapps).
