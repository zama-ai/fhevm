# Clients

A **client** is your main interface to the SDK. You create one, and it gives you methods like `encrypt()`, `publicDecrypt()`, and `decrypt()`. Each client is bound to a specific chain and provider.

The SDK offers three client types so you only load the WASM modules you actually need.

## Which client should I use?

| If you need to...                           | Use this                     | WASM loaded                 |
| ------------------------------------------- | ---------------------------- | --------------------------- |
| Encrypt **and** decrypt                     | `createFhevmClient()`        | TFHE (~5MB) + TKMS (~600KB) |
| Only encrypt (e.g., a form submission page) | `createFhevmEncryptClient()` | TFHE (~5MB) only            |
| Only decrypt (e.g., a results page)         | `createFhevmDecryptClient()` | TKMS (~600KB) only          |

**Why does this matter?** WASM modules are large. If your page only decrypts results, there's no reason to download the 5MB encryption module. Using the right client type makes your app load faster.

Methods that don't belong to your client type are **compile-time errors** — TypeScript catches them before your code runs.

## Creating a client

All client factories take the same parameters: a `chain` definition and a `provider` (your Ethereum connection).

### Full client

Use when your page needs both encryption and decryption.

**With viem:**

```ts
import { setFhevmRuntimeConfig, createFhevmClient } from '@fhevm/sdk/viem';
import { sepolia } from '@fhevm/sdk/chains';
import { createPublicClient, http } from 'viem';
import { sepolia as viemSepolia } from 'viem/chains';

// Configure once at app startup
setFhevmRuntimeConfig({ numberOfThreads: 4 });

// Create your provider
const provider = createPublicClient({
  chain: viemSepolia,
  transport: http('https://ethereum-sepolia-rpc.publicnode.com'),
});

// Create the FHEVM client
const client = createFhevmClient({ chain: sepolia, provider });
```

**With ethers.js:**

```ts
import { setFhevmRuntimeConfig, createFhevmClient } from '@fhevm/sdk/ethers';
import { sepolia } from '@fhevm/sdk/chains';
import { ethers } from 'ethers';

setFhevmRuntimeConfig({ numberOfThreads: 4 });

const provider = new ethers.JsonRpcProvider('https://ethereum-sepolia-rpc.publicnode.com');
const client = createFhevmClient({ chain: sepolia, provider });
```

### Encrypt-only client

Use when you only need to submit encrypted data — no decryption needed on this page.

```ts
import { createFhevmEncryptClient } from "@fhevm/sdk/viem"; // or "@fhevm/sdk/ethers"

const client = createFhevmEncryptClient({ chain: sepolia, provider });

await client.encrypt({ ... });           // works
await client.decrypt({ ... });           // compile-time error
```

### Decrypt-only client

Use when you only need to read encrypted results — no new encryption needed.

```ts
import { createFhevmDecryptClient } from "@fhevm/sdk/viem"; // or "@fhevm/sdk/ethers"

const client = createFhevmDecryptClient({ chain: sepolia, provider });

await client.publicDecrypt({ ... });     // works
await client.decrypt({ ... });           // works
await client.encrypt({ ... });           // compile-time error
```

## When does WASM load?

WASM modules load **lazily** — not when you create the client, but the first time you call an action that needs them:

- First `encrypt()` call → loads TFHE WASM (~5MB) + fetches the network's public key (~50MB)
- First `decrypt()` call → loads TKMS WASM (~600KB)

If you want to load WASM eagerly (for example, behind a loading spinner at app startup), call:

```ts
await client.init();
// or equivalently:
await client.ready;
```

## Available methods

### `FhevmClient` (full client)

Has all base, encrypt, and decrypt methods.

### Base methods (all client types)

| Method                                 | Sync/Async | What it does                                 |
| -------------------------------------- | ---------- | -------------------------------------------- |
| `publicDecrypt(params)`                | async      | Decrypt publicly readable encrypted values   |
| `signDecryptionPermit(params)`         | async      | Create and sign a decrypt permit in one step |
| `parseE2eTransportKeyPair(params)`     | async      | Restore a key pair from serialized bytes     |
| `serializeE2eTransportKeyPair(params)` | sync       | Serialize a key pair for storage             |
| `fetchFheEncryptionKeyBytes(params?)`  | async      | Fetch the network's public encryption key    |
| `init()`                               | async      | Eagerly load WASM modules                    |
| `ready`                                | Promise    | Resolves when WASM modules are loaded        |

### Encrypt methods (`FhevmClient`, `FhevmEncryptClient`)

| Method            | Sync/Async | What it does                                           |
| ----------------- | ---------- | ------------------------------------------------------ |
| `encrypt(params)` | async      | Encrypt values and get encrypted handles + input proof |

### Decrypt methods (`FhevmClient`, `FhevmDecryptClient`)

| Method                                     | Sync/Async | What it does                                                          |
| ------------------------------------------ | ---------- | --------------------------------------------------------------------- |
| `decrypt(params)`                          | async      | Decrypt private encrypted values with a signed permit                 |
| `createUserDecryptEIP712(params)`          | async      | Build EIP-712 typed data for a decrypt permit (lower-level)           |
| `createDelegatedUserDecryptEIP712(params)` | async      | Build EIP-712 typed data for a delegated decrypt permit (lower-level) |
| `publicDecrypt(params)`                    | async      | Decrypt publicly readable encrypted values                            |
| `generateE2eTransportKeyPair()`            | async      | Generate a new E2E transport key pair for decryption                  |

## Client properties

Every client exposes these read-only properties:

| Property  | Type           | What it is                                   |
| --------- | -------------- | -------------------------------------------- |
| `chain`   | `FhevmChain`   | The chain definition this client is bound to |
| `runtime` | `FhevmRuntime` | The runtime with its loaded modules          |
| `uid`     | `string`       | A unique ID for this client instance         |

## Standalone functions vs. client methods

Every action is available in two forms:

```ts
import { encrypt } from "@fhevm/sdk/actions/encrypt";
import { publicDecrypt } from "@fhevm/sdk/actions/base";

// As a standalone function — pass the client as the first argument
const result = await encrypt(client, { ... });
const proof = await publicDecrypt(client, { encryptedValues: [...] });

// As a client method — the client is implicit
const result = await client.encrypt({ ... });
const proof = await client.publicDecrypt({ encryptedValues: [...] });
```

Both are equivalent. Standalone functions are **tree-shakable** — bundlers can eliminate unused functions from your bundle. Client methods are more convenient for everyday use.
