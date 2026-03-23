# Encryption

## Overview

Encryption converts plaintext values into FHE ciphertexts that can be processed by fhEVM smart contracts. The process involves:

1. Fetching the network's global FHE public encryption parameters
2. Generating a zero-knowledge proof of correct encryption
3. Obtaining coprocessor signatures from the relayer
4. Returning a `VerifiedInputProof` to pass to your smart contract

## Supported Types

| FHE Type | Input Type | Value Range | Encrypted Bits |
|----------|-----------|-------------|----------------|
| `ebool` | `boolean \| number \| bigint` | `true`/`false` | 2 |
| `euint8` | `number \| bigint` | 0 – 255 | 8 |
| `euint16` | `number \| bigint` | 0 – 65,535 | 16 |
| `euint32` | `number \| bigint` | 0 – 4,294,967,295 | 32 |
| `euint64` | `number \| bigint` | 0 – 2^64-1 | 64 |
| `euint128` | `number \| bigint` | 0 – 2^128-1 | 128 |
| `euint256` | `number \| bigint` | 0 – 2^256-1 | 256 |
| `eaddress` | `string` | Ethereum address | 160 |

## Basic Usage

### Fetching Global FHE Public Key Parameters

Before encrypting, you need the network's public encryption parameters. These are large (~50MB) and cached automatically.

```ts
// Returns deserialized TFHE objects (GlobalFhePkeParams)
const params = await client.fetchGlobalFhePkeParams();

// Or fetch raw bytes for manual handling
const paramsBytes = await client.fetchGlobalFhePkeParamsBytes();
```

### Encrypting Values

```ts
const proof = await client.encrypt({
  globalFhePublicEncryptionParams: params,
  contractAddress: "0xYourContract...",
  userAddress: "0xYourWallet...",
  values: [
    { type: "uint32", value: 100 },
    { type: "bool", value: true },
    { type: "address", value: "0xSomeAddress..." },
    { type: "uint256", value: 12345678901234567890n },
  ],
  extraData: "0x",
});
```

### Using the Proof

The returned `VerifiedInputProof` contains:

```ts
proof.bytesHex           // Encoded proof to pass to your contract
proof.externalHandles    // Array of ExternalFhevmHandle (encrypted handles)
proof.coprocessorSignatures  // Coprocessor EIP-712 signatures
proof.verified           // Always true for VerifiedInputProof
```

Each `ExternalFhevmHandle` in `externalHandles` corresponds to one input value (same order):

```ts
const handle0 = proof.externalHandles[0]; // Handle for { type: "uint32", value: 100 }
handle0.fheType;    // "euint32"
handle0.bytes32Hex; // The 32-byte handle as hex
handle0.index;      // 0 (position in the proof)
```

## Step-by-Step Encryption

For more control, you can use the individual steps separately.

### 1. Generate ZK Proof

```ts
import { generateZkProof } from "@fhevm/sdk";

const zkProof = await generateZkProof(client, {
  globalFhePublicEncryptionParams: params,
  contractAddress: "0xYourContract...",
  userAddress: "0xYourWallet...",
  values: [{ type: "uint32", value: 42 }],
});
```

This step is CPU-intensive — it runs TFHE WASM to produce a compact proven ciphertext.

### 2. Fetch Verified Input Proof

```ts
import { fetchVerifiedInputProof } from "@fhevm/sdk";

const proof = await fetchVerifiedInputProof(client, {
  zkProof,
  extraData: "0x",
  options: {
    // Optional fetch options (timeout, retries, etc.)
  },
});
```

This sends the ZK proof to the relayer, which returns coprocessor signatures and the final handles.

## Serialization

Global FHE public key parameters can be serialized/deserialized for caching or transfer:

```ts
// Serialize to hex
const hex = client.serializeGlobalFhePkeParamsToHex({ globalFhePkeParams: params });

// Deserialize from hex
const params = client.deserializeGlobalFhePkeParamsFromHex({
  globalFhePkeParamsBytesHex: hex,
});

// Serialize to bytes (Uint8Array)
const bytes = serializeGlobalFhePkeParams(client, { globalFhePkeParams: params });

// Deserialize from bytes
const params = deserializeGlobalFhePkeParams(client, {
  globalFhePkeParamsBytes: bytes,
});
```

## Caching

`fetchGlobalFhePkeParams` and `fetchGlobalFhePkeParamsBytes` automatically cache results by relayer URL. You can manage the cache manually:

```ts
import { clearGlobalFhePkeParamsCache, deleteGlobalFhePkeParamsCache } from "@fhevm/sdk";

// Clear entire cache
clearGlobalFhePkeParamsCache();

// Remove a specific relayer's cached params
deleteGlobalFhePkeParamsCache("https://relayer.mainnet.zama.org");
```

## Relayer Fetch Options

The `options` parameter on `encrypt` and `fetchVerifiedInputProof` accepts `RelayerFetchOptions` for controlling the HTTP request to the relayer (timeouts, retries, abort signals, etc.).
