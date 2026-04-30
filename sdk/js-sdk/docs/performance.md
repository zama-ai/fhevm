# Performance

Encryption with FHE is heavier than a typical Ethereum transaction. The SDK is designed to minimize the impact, but understanding the costs helps you build a better user experience.

## What costs what

| Operation                                   | Time    | Network            | WASM loaded    |
| ------------------------------------------- | ------- | ------------------ | -------------- |
| `setFhevmRuntimeConfig()`                   | Instant | None               | None           |
| `createFhevmClient()`                       | Instant | None               | None           |
| `fetchFheEncryptionKeyBytes()` (first call) | 2-10s   | ~50MB download     | TFHE (~5MB)    |
| `fetchFheEncryptionKeyBytes()` (cached)     | Instant | None               | None           |
| `encrypt()` (first call)                    | 3-15s   | Relayer call       | TFHE init      |
| `encrypt()` (subsequent)                    | 1-5s    | Relayer call       | Already loaded |
| `publicDecrypt()`                           | 1-3s    | Relayer call + RPC | None           |
| `decrypt()` (first call)                    | 2-5s    | Relayer call + RPC | TKMS (~600KB)  |
| `decrypt()` (subsequent)                    | 1-3s    | Relayer call + RPC | Already loaded |
| `signDecryptionPermit()`                    | <100ms  | None               | None           |
| `generateE2eTransportKeyPair()`             | <100ms  | None               | TKMS           |

Times are approximate and depend on network speed, device, and thread count.

## Managing the public key download

Encryption requires the network's **public encryption key** (~50MB). You have three options for when and how this download happens:

### Option 1: Auto-fetch (default)

Just call `encrypt()`. The SDK fetches the key from the Relayer on first use and caches it in memory. This is the simplest path — no extra code needed.

```ts
// The public key is fetched automatically on the first call
const result = await client.encrypt({
  contractAddress: '0x...',
  userAddress: '0x...',
  values: [{ type: 'uint32', value: 42 }],
});
```

The downside is that the first `encrypt()` call is slow (~2-10s for the key download, plus WASM initialization). The user sees a delay on their first transaction.

### Option 2: Eager preload

Call `await client.init()` at app startup — for example, behind a loading spinner. This downloads the public key and initializes WASM upfront, so the first `encrypt()` is fast.

```ts
const client = createFhevmClient({ chain: sepolia, provider });

// Show a loading spinner, then:
await client.init();
// WASM + public key are ready — encrypt() will be fast
```

`client.init()` and `client.ready` are equivalent — both return a promise that resolves when all modules are initialized. Calling either multiple times is safe (idempotent).

### Option 3: Manual preload

If you want explicit control over when the public key is fetched (for example, to show a progress indicator), use `fetchFheEncryptionKeyBytes()`:

```ts
// Preload the public encryption key
await client.fetchFheEncryptionKeyBytes();

// Now encrypt() won't need to download it
const result = await client.encrypt({ ... });
```

## Choosing the right client type

Loading unnecessary WASM slows down your app. Use the lightest client for your page:

| Page type                   | Client                       | What loads           |
| --------------------------- | ---------------------------- | -------------------- |
| Submit form (encrypt only)  | `createFhevmEncryptClient()` | TFHE only (~5MB)     |
| Results page (decrypt only) | `createFhevmDecryptClient()` | TKMS only (~600KB)   |
| Full app (both)             | `createFhevmClient()`        | TFHE + TKMS (~5.6MB) |

A page that only shows decrypted results should never load the 5MB TFHE module.

## Multi-threading

The TFHE WASM module supports multi-threading. More threads = faster `encrypt()`:

```ts
setFhevmRuntimeConfig({
  numberOfThreads: navigator.hardwareConcurrency || 4,
});
```

In browsers, this requires COOP/COEP headers (see [Runtime configuration](runtime-configuration.md)). Without them, the SDK falls back to single-threaded mode.

TKMS (decryption) does **not** use multi-threading — it's a much lighter operation.

## Batch operations

Both `encrypt()` and `publicDecrypt()` accept multiple values in a single call. Batching is more efficient than multiple individual calls because:

- One ZK proof covers all values (for encryption)
- One Relayer round-trip handles all values (for decryption)
- ACL checks are batched

```ts
// Good: batch values in a single encrypt() call
const result = await client.encrypt({
  values: [
    { type: "uint32", value: 42 },
    { type: "uint8", value: 100 },
    { type: "bool", value: true },
  ],
  // ...
});

// Less efficient: separate calls
const result1 = await client.encrypt({ values: [{ type: "uint32", value: 42 }], ... });
const result2 = await client.encrypt({ values: [{ type: "uint8", value: 100 }], ... });
```

Remember the **2048-bit limit** per call. If you need more, split into multiple calls.

## Client reuse

Clients are lightweight and stateless — create one per chain and reuse it. There's no benefit to creating multiple clients for the same chain, and no penalty for keeping one alive.

```ts
// Good: create once, use everywhere
const client = createFhevmClient({ chain: sepolia, provider });
export { client };

// Unnecessary: creating multiple clients for the same chain
const client1 = createFhevmClient({ chain: sepolia, provider });
const client2 = createFhevmClient({ chain: sepolia, provider }); // wastes memory
```
