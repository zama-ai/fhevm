# Architecture

## Overview

The SDK follows a layered architecture separating protocol-agnostic core logic from library-specific adapters.

```
┌─────────────────────────────────────────────┐
│  Application Code                           │
├─────────────────────────────────────────────┤
│  Adapter Layer (ethers/ or viem/)           │
│  - Seals library clients into TrustedClient │
│  - Manages runtime lifecycle                │
│  - Exposes public factory functions         │
├─────────────────────────────────────────────┤
│  Core Layer (core/)                         │
│  - Actions: encrypt, decrypt, key, host     │
│  - Clients: FhevmClient, FhevmEncryptClient │
│  - Modules: encrypt, decrypt, relayer       │
│  - Types, chains, KMS, handle parsing       │
├─────────────────────────────────────────────┤
│  WASM Layer (wasm/)                         │
│  - TFHE: encryption operations              │
│  - TKMS: key management / decryption        │
└─────────────────────────────────────────────┘
```

## Directory Structure

```
src/
├── core/                    # Protocol-agnostic business logic
│   ├── actions/             # Standalone action functions
│   │   ├── chain/           # On-chain verification (EIP-712)
│   │   ├── decrypt/         # Decryption actions
│   │   │   ├── public/      # Public decryption
│   │   │   └── user/        # User decryption (with KMS key)
│   │   ├── encrypt/         # Encryption, ZK proofs, serialization
│   │   ├── host/            # Host contract reads (ACL, KMSVerifier, etc.)
│   │   ├── key/             # Global FHE public key fetching
│   │   └── runtime/         # Runtime management
│   ├── base/                # Primitives, address validation, errors
│   ├── chains/              # Chain definitions (mainnet, sepolia)
│   ├── clients/             # Client type definitions and factories
│   │   └── decorators/      # Action decorators for client composition
│   ├── coprocessor/         # Coprocessor signature verification
│   ├── handle/              # FhevmHandle parsing and validation
│   ├── host-contracts/      # Host contract ABIs and readers
│   ├── kms/                 # KMS EIP-712 message construction
│   ├── modules/             # Runtime module implementations
│   │   ├── decrypt/         # TKMS WASM module
│   │   ├── encrypt/         # TFHE WASM module
│   │   ├── ethereum/        # Ethereum RPC module
│   │   └── relayer/         # Relayer HTTP client module
│   ├── runtime/             # Runtime creation and extension
│   ├── types/               # All public type definitions
│   └── user/                # FhevmDecryptionKey implementation
├── ethers/                  # Ethers.js v6 adapter
│   ├── clients/             # Ethers-specific client factories
│   └── internal/            # Runtime caching, token sealing
├── viem/                    # Viem adapter (same pattern)
└── wasm/                    # WASM binaries and bindings
    ├── tfhe/                # TFHE WASM (encryption, ~5MB)
    └── tkms/                # TKMS WASM (decryption, ~600KB)
```

## Core Design Patterns

### Opaque TrustedClient

The adapter layer seals library-specific objects (e.g., ethers `ContractRunner`) into an opaque `TrustedClient` using a private symbol token. This prevents the core layer from depending on any specific Ethereum library.

```ts
// In ethers adapter (ethers-p.ts)
const PRIVATE_ETHERS_TOKEN = Symbol("ethers");

// Seals ethers provider into a TrustedClient
createFhevmClient_(PRIVATE_ETHERS_TOKEN, {
  chain,
  runtime,
  client: ethersProvider,  // Sealed — core cannot access directly
});
```

### Runtime Module Extension

The runtime starts with a base `EthereumModule` and is progressively extended with additional modules via the `extend()` method.

```ts
const runtime = getEthersRuntime()
  .extend(encryptModule)    // Adds .encrypt (TFHE WASM)
  .extend(decryptModule)    // Adds .decrypt (TKMS WASM)
  .extend(relayerModule);   // Adds .relayer (HTTP client)
```

Each module factory receives the current runtime and returns the module's methods. The TypeScript type system tracks which modules are present, so actions that require specific modules enforce this at compile time.

### Client Composition via Decorators

Clients are composed by chaining decorator functions onto a base `CoreFhevm`:

```ts
// Internal client creation (simplified)
const base = createCoreFhevm(token, { chain, runtime, client });

// Decorators add action methods to the client
const fhevmClient = extendCoreFhevm(base)
  .with(encryptActions)        // .encrypt(), .fetchGlobalFhePkeParams()
  .with(decryptActions)        // .userDecrypt(), .publicDecrypt(), .createUserDecryptEIP712()
  .with(globalFhePkeActions);  // .deserializeGlobalFhePkeParamsFromHex(), etc.
```

### Action Function Pattern

Every action exists as a standalone function with the client as first argument. Decorators curry this into a method on the client:

```ts
// Standalone function (always available)
import { encrypt } from "@fhevm/sdk";
const proof = await encrypt(fhevmClient, { ... });

// Client method (added by decorator)
const proof = await fhevmClient.encrypt({ ... });
```

The function signature convention follows:

```
functionName(fhevm, parameters: FunctionNameParameters): Promise<FunctionNameReturnType>
```

Each action exports three things:
- `FunctionNameParameters` — input type
- `FunctionNameReturnType` — output type
- `functionName` — the function itself

### Private Implementation Files (`-p.ts`)

Files suffixed with `-p.ts` contain internal implementation details:

| File | Purpose |
|------|---------|
| `CoreFhevm-p.ts` | Core client class with private fields |
| `CoreFhevmRuntime-p.ts` | Runtime factory with module composition |
| `ethers-p.ts` | Ethers adapter internals (runtime caching, token) |
| `FhevmUserDecryptionPermit-p.ts` | Permit implementation class |
| `PublicDecryptionProof-p.ts` | Proof implementation class |
| `KmsSigncryptedShares-p.ts` | KMS share implementation |

These files are consumed by their companion public files (without the `-p` suffix) which re-export only the public API.

### Symbol-Based Access Control

Sensitive data (KMS private keys, internal state) is protected using ES2015 private fields (`#field`) and symbol-keyed static accessors:

```ts
const FHEVM_ACCOUNT_TOKEN = Symbol("FhevmAccount.token");
const GET_KMS_PRIVATE_KEY = Symbol("FhevmAccount.getKmsPrivateKey");

class FhevmAccountImpl {
  readonly #kmsPrivateKey: TkmsPrivateKey;

  // Only accessible via symbol — invisible to consumers
  static [GET_KMS_PRIVATE_KEY](account: unknown, token: symbol): TkmsPrivateKey {
    if (token !== FHEVM_ACCOUNT_TOKEN) throw new Error("Unauthorized");
    return account.#kmsPrivateKey;
  }
}
```

## Dual CJS/ESM Build

The SDK ships both CommonJS and ESM builds:

| Output | Directory | Module System |
|--------|-----------|---------------|
| ESM | `src/_esm/` | `module: "esnext"` |
| CJS | `src/_cjs/` | `module: "commonjs"` |
| Types | `src/_types/` | Declaration files |

WASM file resolution differs between environments:
- **ESM**: Uses `import.meta.url` (in `wasmBaseUrl.ts`)
- **CJS**: Uses `require('node:url').pathToFileURL(__filename)` (in `wasmBaseUrl.cts`)

The `package.json` `"imports"` field maps `#wasm/baseUrl` to the correct variant at runtime.

## Data Flow

### Encryption Flow

```
Application
  │
  ├─ fetchGlobalFhePkeParams()
  │    └─ relayer.fetchGlobalFhePkeParamsBytes()  → HTTP to relayer
  │    └─ encrypt.deserializeGlobalFhePublicKey()  → TFHE WASM
  │    └─ encrypt.deserializeGlobalFheCrs()        → TFHE WASM
  │
  └─ encrypt()
       ├─ generateZkProof()
       │    └─ encrypt.buildWithProofPacked()      → TFHE WASM (CPU intensive)
       │
       └─ fetchVerifiedInputProof()
            └─ relayer.fetchCoprocessorSignatures() → HTTP to relayer
            └─ coprocessor signature verification   → on-chain via RPC
```

### User Decryption Flow

```
Application
  │
  ├─ createUserDecryptEIP712()       → Constructs EIP-712 message
  │    └─ User signs with wallet     → External (MetaMask, etc.)
  │
  └─ userDecrypt()
       ├─ checkUserAllowedForDecryption()  → ACL check via RPC
       ├─ relayer.fetchUserDecrypt()       → HTTP to relayer → KMS shares
       └─ decrypt.decryptAndReconstruct()  → TKMS WASM (reconstruct cleartext)
```

### Public Decryption Flow

```
Application
  │
  └─ publicDecrypt()
       ├─ Handle validation (non-empty, bit limit, chain ID)
       ├─ checkAllowedForDecryption()          → ACL check via RPC
       ├─ relayer.fetchPublicDecrypt()          → HTTP to relayer
       └─ createPublicDecryptionProof()         → KMS signature verification
            └─ Verify KMS signer signatures     → on-chain via RPC
```
