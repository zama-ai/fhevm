# Architecture

This page explains how the SDK is built internally. **You don't need to read this to use the SDK** — it's here for contributors, advanced users, and anyone curious about the design decisions.

## Layered design

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
│  - Actions: encrypt, decrypt, base, chain   │
│  - Clients: decorators + type composition   │
│  - Modules: encrypt, decrypt, relayer       │
│  - Types, chains, KMS, handle parsing       │
├─────────────────────────────────────────────┤
│  WASM Layer (wasm/)                         │
│  - TFHE: encryption (~5MB)                  │
│  - TKMS: decryption (~600KB)                │
└─────────────────────────────────────────────┘
```

**Dependency direction is strictly top-down.** Core never imports from adapters. Actions never import from decorators. Modules never import from actions.

---

## Source layout

```
src/
├── core/                    # Protocol-agnostic business logic
│   ├── actions/             # Standalone action functions
│   │   ├── base/            # publicDecrypt, fetchVerifiedInputProof, ACL checks, signers
│   │   ├── chain/           # EIP-712 creation, verification, signDecryptionPermit, keypair ops
│   │   ├── decrypt/         # decrypt, generateTransportKeyPair, decryptKmsSignedcryptedShares
│   │   ├── encrypt/         # encrypt, generateZkProof
│   │   └── host/            # Contract reads (ACL, KMSVerifier, InputVerifier, FhevmExecutor)
│   ├── base/                # Primitives (address, bytes, errors, trustedValue)
│   ├── chains/              # Chain definitions (mainnet, sepolia)
│   ├── clients/
│   │   └── decorators/      # baseActions, encryptActions, decryptActions
│   ├── modules/
│   │   ├── encrypt/         # EncryptModule (TFHE WASM)
│   │   ├── decrypt/         # DecryptModule (TKMS WASM)
│   │   ├── ethereum/        # EthereumModule interface + TrustedClient
│   │   └── relayer/         # RelayerModule (HTTP client)
│   ├── runtime/             # CoreFhevm-p.ts (client), CoreFhevmRuntime-p.ts (runtime)
│   ├── types/               # All shared type definitions
│   └── kms/                 # TransportKeyPair, SignedDecryptionPermit
├── ethers/                  # Ethers.js v6 adapter
│   ├── clients/             # createFhevmClient, createFhevmEncryptClient, createFhevmDecryptClient
│   └── internal/            # Runtime config, TrustedClient sealing, EthereumModule impl
├── viem/                    # Viem adapter (same pattern)
│   ├── clients/
│   └── internal/
└── wasm/                    # WASM bindings
    ├── tfhe/                # TFHE (~5MB, encryption)
    └── tkms/                # TKMS (~600KB, decryption)
```

---

## Client composition

Clients are built by composing a base `CoreFhevm` with decorator actions via `.extend()`:

```
createCoreFhevm() → base client (chain, runtime, trustedClient)
  ↓ .extend() chains decorators:
  ├─ baseActions       → publicDecrypt, signDecryptionPermit, parseTransportKeyPair, ...
  ├─ decryptActions    → decrypt, generateTransportKeyPair, createUserDecryptEIP712, ...
  └─ encryptActions    → encrypt
```

Four factory functions pre-compose the right set:

| Factory                    | Decorators               | WASM        |
| -------------------------- | ------------------------ | ----------- |
| `createFhevmClient`        | base + decrypt + encrypt | TFHE + TKMS |
| `createFhevmEncryptClient` | base + encrypt           | TFHE only   |
| `createFhevmDecryptClient` | base + decrypt           | TKMS only   |
| `createFhevmBaseClient`    | base                     | None        |

---

## Runtime module extension

The runtime starts with `EthereumModule` + `RelayerModule` and is extended with WASM modules:

```ts
const runtime = getAdapterRuntime() // EthereumModule + RelayerModule
  .extend(encryptModule) // + EncryptModule (TFHE WASM)
  .extend(decryptModule); // + DecryptModule (TKMS WASM)
```

The TypeScript type system tracks which modules are present — actions that require specific modules enforce this at compile time:

```ts
// encrypt() requires WithEncrypt runtime — compile error if module missing
async function encrypt(fhevm: Fhevm<FhevmChain, WithEncrypt>, ...): Promise<...>
```

---

## Action function pattern

Every action is a standalone function with the client as first argument. Decorators curry this into a client method:

```ts
// Standalone (tree-shakable)
import { encrypt } from "@fhevm/sdk/actions/encrypt";
const result = await encrypt(fhevmClient, { ... });

// Client method (via decorator)
const result = await fhevmClient.encrypt({ ... });
```

Each action file exports three things:

- `FunctionNameParameters` — input type
- `FunctionNameReturnType` — output type
- `functionName` — the function itself

---

## Security patterns

### Opaque `TrustedClient`

Adapter-specific clients (ethers `ContractRunner`, viem `PublicClient`) are sealed into an opaque `TrustedClient` using a private symbol token. The core layer never sees the native client.

```ts
// In viem adapter
const PRIVATE_VIEM_TOKEN = Symbol('viem.token');
const trusted = createTrustedClient(viemPublicClient, PRIVATE_VIEM_TOKEN);
// Only the viem adapter can unseal it
const original = verifyTrustedValue(trusted, PRIVATE_VIEM_TOKEN);
```

### Symbol-based access control

Sensitive data (KMS private keys, internal state) is protected using `#privateFields` and symbol-keyed static accessors:

```ts
const GET_KEY = Symbol('FhevmAccount.getKmsPrivateKey');

class FhevmAccountImpl {
  readonly #kmsPrivateKey: TkmsPrivateKey;
  static [GET_KEY](account: unknown, token: symbol): TkmsPrivateKey {
    if (token !== FHEVM_ACCOUNT_TOKEN) throw new Error('Unauthorized');
    return account.#kmsPrivateKey;
  }
}
```

### Frozen objects

All chain definitions, runtime instances, and EIP-712 messages are deep-frozen with `Object.freeze()`.

---

## Private implementation files (`-p.ts`)

Files suffixed with `-p.ts` contain internal implementation. The public file (without `-p`) re-exports only the public API:

| File                          | Purpose                                  |
| ----------------------------- | ---------------------------------------- |
| `CoreFhevm-p.ts`              | Core client class with private fields    |
| `CoreFhevmRuntime-p.ts`       | Runtime factory with module composition  |
| `ethers-p.ts` / `viem-p.ts`   | Adapter internals (runtime cache, token) |
| `TransportKeyPair-p.ts`       | E2E transport key pair implementation    |
| `SignedDecryptionPermit-p.ts` | Signed permit implementation             |

---

## Dual CJS/ESM build

```
src/_esm/    ← ESM (module: esnext, sideEffects: false)
src/_cjs/    ← CJS (module: commonjs)
src/_types/  ← Declaration files (.d.ts)
```

WASM base URL resolved via `package.json` `"imports"` field:

- ESM: `import.meta.url` (in `wasmBaseUrl.ts`)
- CJS: `require('node:url').pathToFileURL(__filename)` (in `wasmBaseUrl.cts`)

---

## Data flow

### Encryption

```
fetchFheEncryptionKeyBytes()
  └─ relayer HTTP call                         → fetch ~50MB public key
  └─ encrypt.deserializeGlobalFhePublicKey()   → TFHE WASM
  └─ encrypt.deserializeGlobalFheCrs()         → TFHE WASM

encrypt()
  ├─ generateZkProof()
  │    └─ encrypt.buildWithProofPacked()       → TFHE WASM (CPU intensive)
  └─ fetchVerifiedInputProof()
       └─ relayer.fetchCoprocessorSignatures() → HTTP to relayer
       └─ coprocessor signature verification   → on-chain via RPC
```

### Private decryption

```
signDecryptionPermit()       → Constructs EIP-712 + signs with wallet → SignedDecryptionPermit

decrypt()
  ├─ fetchKmsSignedcryptedShares()
  │    ├─ checkUserAllowedForDecryption() → ACL check via RPC
  │    └─ relayer.fetchUserDecrypt()      → HTTP to Zama Protocol → encrypted shares
  └─ decryptKmsSignedcryptedShares()
       └─ decrypt.decryptAndReconstruct() → TKMS WASM (reconstruct cleartext)
```

### Reading public values

```
publicDecrypt()
  ├─ Validation (non-empty, bit limit, chain ID)
  ├─ checkAllowedForDecryption()     → ACL check via RPC
  ├─ relayer.fetchPublicDecrypt()    → HTTP to Zama Protocol
  └─ createPublicDecryptionProof()   → signature verification via RPC
```
