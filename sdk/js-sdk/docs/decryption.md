# Decryption

## Overview

The SDK supports two decryption modes:

| Mode | Permission Required | Use Case |
|------|-------------------|----------|
| **Public Decryption** | Handle must be marked publicly decryptable on-chain (ACL) | Revealing results to everyone |
| **User Decryption** | Signed EIP-712 permit + KMS private key | Private access to encrypted values |

Both modes are subject to:
- **2048-bit limit**: Total encrypted bits across all handles per request
- **ACL checks**: On-chain permission verification via the ACL contract
- **KMS signature verification**: All responses are verified against registered KMS signers

## Public Decryption

Public decryption reveals encrypted values that have been flagged as publicly decryptable on the ACL contract. No user-specific keys are needed.

```ts
const result = await client.publicDecrypt({
  handles: [handle1, handle2],
  extraData: "0x",
});
```

### Validation Steps

1. At least one handle is required
2. Total encrypted bits must not exceed 2048
3. All handles must belong to the same chain ID
4. ACL `allowedForDecryption` check passes for each handle
5. Relayer returns decrypted values with KMS signatures
6. KMS signatures are verified against on-chain KMS signers

### Result: PublicDecryptionProof

```ts
result.orderedDecryptedHandles  // DecryptedFhevmHandle[]
result.orderedHandles           // Original FhevmHandle[]
result.orderedAbiEncodedClearValues  // ABI-encoded values
result.kmsPublicDecryptEIP712Signatures  // KMS signatures
result.extraData                // Extra data passed through

// Access individual values
const decrypted = result.orderedDecryptedHandles[0];
decrypted.fheType;  // "euint32"
decrypted.handle;   // The original FhevmHandle
decrypted.value;    // The plaintext value (number, bigint, boolean, or address)
```

### Type-Safe Value Access

`DecryptedFhevmHandle` is a discriminated union on `fheType`:

```ts
const d: DecryptedFhevmHandle = result.orderedDecryptedHandles[0];

if (d.fheType === "ebool") {
  d.value; // boolean
} else if (d.fheType === "euint32") {
  d.value; // Uint32Number (number)
} else if (d.fheType === "euint256") {
  d.value; // Uint256BigInt (bigint)
} else if (d.fheType === "eaddress") {
  d.value; // ChecksummedAddress
}
```

**Decrypted value type mapping:**

| FHE Type | Decrypted Type | JS Type |
|----------|---------------|---------|
| `ebool` | `boolean` | `boolean` |
| `euint8` | `Uint8Number` | `number` |
| `euint16` | `Uint16Number` | `number` |
| `euint32` | `Uint32Number` | `number` |
| `euint64` | `Uint64BigInt` | `bigint` |
| `euint128` | `Uint128BigInt` | `bigint` |
| `euint256` | `Uint256BigInt` | `bigint` |
| `eaddress` | `ChecksummedAddress` | `string` |

## User Decryption

User decryption allows a specific user to decrypt their own encrypted values. It requires three components:

1. **FhevmDecryptionKey** — wraps a KMS private key (TKMS)
2. **EIP-712 permit** — signed by the user's wallet, scoped to specific contracts and time window
3. **Handle-contract pairs** — which handles to decrypt from which contracts

### Step 1: Create a Decryption Key

```ts
import { createFhevmDecryptionKey } from "@fhevm/sdk";

// Generate a new TKMS private key
const privateKey = runtime.decrypt.generateTkmsPrivateKey();

// Wrap it in a FhevmDecryptionKey
const decryptionKey = createFhevmDecryptionKey(runtime, {
  tkmsPrivateKey: privateKey,
});

// Get the corresponding public key (needed for the permit)
const publicKeyHex = decryptionKey.getTkmsPublicKeyHex();
```

The `FhevmDecryptionKey` interface exposes:
- `getTkmsPublicKeyHex()` — derives the public key from the private key
- `decryptAndReconstruct(shares)` — reconstructs cleartext from KMS shares

The private key itself is **never exposed** through the public interface.

### Step 2: Create an EIP-712 Permit

```ts
const eip712 = client.createUserDecryptEIP712({
  publicKey: publicKeyHex,
  contractAddresses: ["0xContractA...", "0xContractB..."],
  startTimestamp: Math.floor(Date.now() / 1000),
  durationDays: 7,
  extraData: "0x",
});
```

**Permit constraints:**
- Maximum 10 contract addresses per permit
- Maximum 365 days duration
- `startTimestamp` as Unix timestamp (seconds)
- The permit binds to the KMS public key (derived from the private key)

### Step 3: Sign the Permit

The user must sign the EIP-712 typed data with their Ethereum wallet. This is done outside the SDK:

```ts
// With ethers.js
const signature = await signer.signTypedData(
  eip712.domain,
  { UserDecryptRequestVerification: eip712.types.UserDecryptRequestVerification },
  eip712.message,
);
```

The EIP-712 domain is:
```ts
{
  name: "Decryption",
  version: "1",
  chainId: <gateway chain ID>,
  verifyingContract: <KMS verifier address>,
}
```

### Step 4: Decrypt

```ts
const results = await client.userDecrypt({
  decryptionKey,
  handleContractPairs: [
    { handle: handle1, contractAddress: "0xContractA..." },
    { handle: handle2, contractAddress: "0xContractA..." },
  ],
  userDecryptEIP712Signer: userAddress,
  userDecryptEIP712Message: eip712.message,
  userDecryptEIP712Signature: signature,
});

// results is readonly DecryptedFhevmHandle[]
results[0].value; // plaintext
```

### User Decryption Validation

1. ACL checks: user is allowed to decrypt each handle, contract is allowed for each handle
2. User address must differ from contract address
3. Permit is sent to relayer along with handles
4. Relayer returns KMS-encrypted shares (signcrypted)
5. SDK decrypts shares locally using TKMS WASM
6. Cleartext values are reconstructed and returned

## Delegated User Decryption

Delegated decryption allows a user to authorize another account to decrypt on their behalf.

```ts
const eip712 = client.createDelegatedUserDecryptEIP712({
  publicKey: delegatePublicKeyHex,
  contractAddresses: ["0xContract..."],
  startTimestamp: Math.floor(Date.now() / 1000),
  durationDays: 1,
  extraData: "0x",
  delegatedAccount: "0xDelegateAddress...",  // The authorized delegate
});
```

The delegated permit has an additional `delegatedAccount` field and uses the `DelegatedUserDecryptRequestVerification` EIP-712 primary type.

## Reusing Permits

Permits are designed to be reusable within their validity window:

```ts
// Sign once
const permit = { eip712, signature, signerAddress };

// Use multiple times
await client.userDecrypt({ ...permit, decryptionKey, handleContractPairs: batch1 });
await client.userDecrypt({ ...permit, decryptionKey, handleContractPairs: batch2 });
```

Different permits can cover different contract sets and validity windows, while the decryption key and wallet client remain the same.

## FhevmUserDecryptionPermit (Convenience API)

The SDK provides a higher-level permit abstraction:

```ts
type FhevmUserDecryptionPermit = {
  readonly eip712: KmsUserDecryptEIP712;
  readonly signature: Bytes65Hex;
  readonly signerAddress: ChecksummedAddress;
};
```

See [API.md](../src/API.md) for the full convenience API including `FhevmAccount`, `FhevmWalletClient`, and `signFhevmUserDecryptionPermit`.
