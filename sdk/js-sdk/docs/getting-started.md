# Getting Started

## Prerequisites

- Node.js >= 22.0
- An Ethereum provider (ethers.js v6 or viem)
- Access to an fhEVM-enabled chain (mainnet or Sepolia testnet)

## Installation

```bash
npm install @fhevm/sdk
```

## Quick Start

### 1. Configure the Runtime

The runtime manages WASM modules (TFHE for encryption, TKMS for decryption). Configure it once before creating any clients.

```ts
import { setFhevmRuntimeConfig } from "@fhevm/sdk/ethers";

setFhevmRuntimeConfig({
  numberOfThreads: 4,        // WASM worker threads (optional)
  singleThread: false,       // Force single-threaded mode (optional)
  // locateFile: (f) => ..., // Custom WASM file locator (optional)
  // logger: myLogger,       // Custom logger (optional)
});
```

### 2. Create a Client

The SDK provides different client types depending on your needs.

**Full client** (encrypt + decrypt + relayer):

```ts
import { createFhevmClient } from "@fhevm/sdk/ethers";
import { mainnet } from "@fhevm/sdk/chains";
import { ethers } from "ethers";

const provider = new ethers.JsonRpcProvider("https://rpc.mainnet.zama.org");

const client = createFhevmClient({
  chain: mainnet,
  provider,
});
```

**Encrypt-only client** (lighter, no decrypt module loaded):

```ts
import { createFhevmEncryptClient } from "@fhevm/sdk/ethers";

const encryptClient = createFhevmEncryptClient({
  chain: mainnet,
  provider,
});
```

**Decrypt-only client**:

```ts
import { createFhevmDecryptClient } from "@fhevm/sdk/ethers";

const decryptClient = createFhevmDecryptClient({
  chain: mainnet,
  provider,
});
```

### 3. Encrypt Values

```ts
// Fetch the global FHE public encryption parameters (cached after first call)
const params = await client.fetchGlobalFhePkeParams();

// Encrypt values for a specific contract
const proof = await client.encrypt({
  globalFhePublicEncryptionParams: params,
  contractAddress: "0xYourContractAddress...",
  userAddress: "0xYourWalletAddress...",
  values: [
    { type: "uint32", value: 42 },
    { type: "bool", value: true },
  ],
  extraData: "0x",
});

// proof.externalHandles contains the encrypted handles
// proof.bytesHex contains the encoded proof to pass to your contract
```

### 4. Public Decryption

Public decryption does not require any user-specific keys — it decrypts handles that have been marked as publicly decryptable on-chain.

```ts
const result = await client.publicDecrypt({
  handles: [encryptedHandle],
  extraData: "0x",
});

// result.orderedDecryptedHandles[0].value → the plaintext value
```

### 5. User Decryption

User decryption requires a KMS private key and a signed EIP-712 permit.

```ts
// Generate a KMS private key (or load from storage)
const decryptionKey = await createFhevmDecryptionKey(runtime, {
  tkmsPrivateKey: privateKeyBytes,
});

// Create an EIP-712 permit message
const eip712 = client.createUserDecryptEIP712({
  publicKey: decryptionKey.getTkmsPublicKeyHex(),
  contractAddresses: ["0xYourContractAddress..."],
  durationDays: 7,
  startTimestamp: Math.floor(Date.now() / 1000),
  extraData: "0x",
});

// Sign the permit with the user's wallet (external step)
// const signature = await signer.signTypedData(eip712.domain, eip712.types, eip712.message);

// Decrypt
const results = await client.userDecrypt({
  decryptionKey,
  handleContractPairs: [
    { handle: encryptedHandle, contractAddress: "0xYourContractAddress..." },
  ],
  userDecryptEIP712Signer: userAddress,
  userDecryptEIP712Message: eip712.message,
  userDecryptEIP712Signature: signature,
});

// results[0].value → the plaintext value
// results[0].fheType → "euint32", "ebool", etc.
```

## Import Paths

| Path | Contents |
|------|----------|
| `@fhevm/sdk/ethers` | Ethers.js v6 adapter — client factories, runtime config |
| `@fhevm/sdk/chains` | Chain definitions (mainnet, sepolia) |
| `@fhevm/sdk` | Core types and standalone action functions |

## Next Steps

- [Clients](./clients.md) — Understand the different client types and when to use each
- [Encryption](./encryption.md) — Deep dive into the encryption flow
- [Decryption](./decryption.md) — Public vs. user decryption, permits, and delegated decryption
- [Types](./types.md) — Understand FHE types, handles, and the branded type system
