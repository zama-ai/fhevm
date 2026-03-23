# Clients

## Client Types

The SDK provides several client types, each loading only the modules needed for its use case. Lighter clients avoid loading unnecessary WASM modules, reducing startup time and memory usage.

| Client | Encrypt | Decrypt | Relayer | Use Case |
|--------|---------|---------|---------|----------|
| `FhevmClient` | Yes | Yes | Yes | Full capabilities |
| `FhevmEncryptClient` | Yes | No | Yes | Encrypt-only (no TKMS WASM) |
| `FhevmDecryptClient` | No | Yes | Yes | Decrypt-only (no TFHE WASM) |
| `FhevmHostClient` | No | No | No | Minimal — host contract reads only |

## Creating Clients

All client factories accept `{ provider, chain }` where `provider` is an ethers.js `ContractRunner` and `chain` is a chain definition.

### FhevmClient (Full)

```ts
import { createFhevmClient } from "@fhevm/sdk/ethers";
import { mainnet } from "@fhevm/sdk/chains";

const client = createFhevmClient({
  chain: mainnet,
  provider: ethersProvider,
});
```

**Available actions:**
- `client.encrypt(params)` — Encrypt values
- `client.fetchGlobalFhePkeParams()` — Fetch/cache global FHE public key
- `client.fetchGlobalFhePkeParamsBytes()` — Fetch as raw bytes
- `client.publicDecrypt(params)` — Public decryption
- `client.userDecrypt(params)` — User decryption with KMS key
- `client.createUserDecryptEIP712(params)` — Create user decrypt permit
- `client.createDelegatedUserDecryptEIP712(params)` — Create delegated permit
- `client.deserializeGlobalFhePkeParamsFromHex(params)` — Deserialize PKE params
- `client.serializeGlobalFhePkeParamsToHex(params)` — Serialize PKE params
- `client.resolveGlobalFhePkeParams(params)` — Resolve PKE params from source

### FhevmEncryptClient

```ts
import { createFhevmEncryptClient } from "@fhevm/sdk/ethers";

const client = createFhevmEncryptClient({
  chain: mainnet,
  provider: ethersProvider,
});
```

**Available actions:** `encrypt`, `fetchGlobalFhePkeParams`, `fetchGlobalFhePkeParamsBytes`, and PKE params serialization.

### FhevmDecryptClient

```ts
import { createFhevmDecryptClient } from "@fhevm/sdk/ethers";

const client = createFhevmDecryptClient({
  chain: mainnet,
  provider: ethersProvider,
});
```

**Available actions:** `publicDecrypt`, `userDecrypt`, `createUserDecryptEIP712`, `createDelegatedUserDecryptEIP712`.

### FhevmHostClient (Minimal)

```ts
import { createFhevmHostClient } from "@fhevm/sdk/ethers";

const client = createFhevmHostClient({
  chain: mainnet,
  provider: ethersProvider,
});
```

The host client has no action methods — it provides the base `chain`, `runtime`, and `trustedClient` properties. It is useful for reading host contract data via standalone functions:

```ts
import { readFhevmExecutorContractData, readInputVerifierContractData } from "@fhevm/sdk/ethers";

const executorData = await readFhevmExecutorContractData(client, {
  address: mainnet.fhevm.contracts.inputVerifier.address,
});
```

## Runtime Configuration

The runtime must be configured **before** creating any clients. Configuration is global and shared across all clients.

```ts
import { setFhevmRuntimeConfig } from "@fhevm/sdk/ethers";

setFhevmRuntimeConfig({
  numberOfThreads: 4,      // Worker threads for WASM (default: auto)
  singleThread: false,      // Force single-threaded (useful for debugging)
  locateFile: (file) => {   // Custom WASM file location
    return new URL(`/wasm/${file}`, import.meta.url);
  },
  logger: console,          // Logger instance
});
```

## Client Properties

All clients expose these readonly properties:

| Property | Type | Description |
|----------|------|-------------|
| `chain` | `FhevmChain` | The chain definition the client is bound to |
| `runtime` | `FhevmRuntime` | The runtime with loaded modules |
| `uid` | `string` | Unique identifier for this client instance |

## Standalone vs. Client Method Calls

Every action is available both as a standalone function and as a client method:

```ts
import { encrypt, publicDecrypt } from "@fhevm/sdk";

// Standalone — client as first argument
const proof = await encrypt(client, { ... });
const result = await publicDecrypt(client, { ... });

// Client method — first argument is curried
const proof = await client.encrypt({ ... });
const result = await client.publicDecrypt({ ... });
```

Standalone functions are useful when you need to call an action with a client that has specific capabilities:

```ts
import { userDecrypt } from "@fhevm/sdk";

// TypeScript enforces that 'client' has a relayer module
const results = await userDecrypt(client, { ... });
```

## Type Safety

The TypeScript type system tracks which modules are present on a runtime. Calling an action that requires a missing module results in a compile-time error:

```ts
const encryptClient = createFhevmEncryptClient({ ... });

// Compile error — FhevmEncryptClient does not have decrypt actions
encryptClient.userDecrypt({ ... });

// Standalone function also errors — runtime lacks WithDecryptModule
userDecrypt(encryptClient, { ... });
```
