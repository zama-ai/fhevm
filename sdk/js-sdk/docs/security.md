# Security model

This page explains the security guarantees of FHEVM and how the SDK fits into
the picture. Understanding this model helps you decide what to encrypt and how to
handle decryption.

## The big picture

In FHEVM, **encrypted data is processed without ever being decrypted**. Your smart
contract adds, compares, and transforms encrypted values, and the plaintext is
never visible to the blockchain, validators, or anyone observing the chain.

Three entities are involved:

| Entity                                         | What it sees                                            | What it can do                                        |
| ---------------------------------------------- | ------------------------------------------------------- | ----------------------------------------------------- |
| **Blockchain** (validators, explorers)         | Encrypted values only                                   | Execute FHE operations on ciphertexts                 |
| **Zama Protocol** (distributed infrastructure) | Nothing until decryption is requested                   | Produce decryption shares on an authorized request    |
| **User** (your app, browser)                   | Plaintext before encryption, plaintext after decryption | Encrypt values, request decryption with a signed permit |

## Who can decrypt what

Decryption is controlled by two mechanisms working together.

### 1. The ACL contract (on-chain)

The **Access Control List (ACL)** is a smart contract that tracks permissions.
Every encrypted value has a list of addresses allowed to interact with it. Your
Solidity contract grants permissions explicitly:

```solidity
// In your Solidity contract:
FHE.allow(encryptedBalance, msg.sender);      // this user can decrypt their own balance
FHE.makePubliclyDecryptable(encryptedResult); // anyone can read this value (public)
```

The SDK checks ACL permissions as part of every decryption request. If the user or
contract is not authorized, the request fails.

### 2. The EIP-712 permit (off-chain)

For private decryption, the user must also sign an EIP-712 permit that specifies:

- which contracts they are authorizing decryption for,
- a time window (a start timestamp plus a duration in **seconds**), and
- the transport public key the Zama Protocol should use to encrypt the response.

So even when the ACL grants permission, the user must explicitly opt in to each
decryption by signing with their wallet. No one can decrypt a user's data without
their wallet signature.

## How decryption works

The Zama Protocol uses a **distributed key management** system — no single server
holds the full decryption key. When you request decryption:

1. The SDK sends the request (with the signed permit) to the Zama Protocol.
2. Multiple independent nodes each produce a **decryption share** — a partial
   decryption encrypted with your transport public key.
3. The shares are sent back to your application.
4. The SDK reconstructs the plaintext locally using your transport private key
   (TKMS WASM).

**No server ever sees the plaintext.** Each node only sees its own partial share,
and the shares are encrypted for your specific public key. Reconstruction happens
entirely on your device.

## What the SDK protects

The SDK is designed to prevent accidental exposure of sensitive material.

### Transport keys are opaque

When you call `generateTransportKeyPair()`, the returned object wraps the private
key so it cannot be accidentally logged or serialized:

```ts
const transportKeyPair = await client.generateTransportKeyPair();
console.log(transportKeyPair); // does NOT print the private key
```

The raw private key lives in an ES private field and is only reachable through the
SDK's internal, symbol-guarded access. You can read the **public** key via
`transportKeyPair.publicKey`, but the private key is never exposed to application
code.

### The provider is sealed

The ethers or viem connection you pass to `createFhevmClient()` is sealed inside an
opaque wrapper. The SDK unwraps it internally (through symbol-guarded access) for
RPC calls — reading contracts and checking ACL permissions — but application code
cannot reach the underlying object off the client. This also keeps the core layer
from depending on ethers- or viem-specific APIs.

### Chain definitions are frozen

Chain objects (`mainnet`, `sepolia`, or your custom chains) are deep-frozen after
creation, preventing accidental or malicious mutation of contract addresses at
runtime.

### Error messages exclude sensitive data

SDK errors never include private keys, signatures, or raw encrypted data. They
carry enough context to debug (contract addresses, error codes) without leaking
sensitive information.

## What you should protect

The SDK handles its internal security; your application has its own
responsibilities.

{% hint style="warning" %}
After decryption, plaintext values live in your application's memory. Treat them as
sensitive — the guarantees below are yours to uphold, not the SDK's.
{% endhint %}

### Never log decryption results

After calling `decryptValue()` or `decryptPublicValues()`, the plaintext is in your
app's memory. Don't send it to analytics, error tracking, or console logs unless you
intend to make it public.

### Store transport keys securely

If you persist a key pair across sessions with `serializeTransportKeyPair()` /
`parseTransportKeyPair()`, store the serialized bytes carefully:

- **Acceptable:** encrypted storage, a secure enclave, or the browser credential store.
- **Risky:** plain `localStorage`, which is readable by any script on the domain.
- **Never:** URL parameters, cookies, or unencrypted server-side storage.

### Validate contract addresses

When passing `contractAddress` to `encryptValues()` or an encrypted value to
`decryptValue()`, make sure the address is correct. Encrypting for the wrong
contract binds the data to that contract's ACL — there is no way to re-target it
after encryption.

### Keep permit scope minimal

When signing EIP-712 permits, scope them as tightly as possible:

- include only the contracts that actually need decryption,
- use short `durationSeconds` windows, and
- generate a fresh key pair per session rather than reusing one across sessions.

## Related

- [Decryption](decryption.md) — permits, transport key pairs, and the decryption flow.
- [Architecture](architecture.md) — how opacity and sealing are implemented internally.
- [Chains](chains.md) — the ACL and verifier contract addresses per chain.

For complete working applications that demonstrate these patterns, see the
[FHEVM dApps repository](https://github.com/zama-ai/dapps).
