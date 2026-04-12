# Decryption

Decryption is how you get data **out** of an FHEVM smart contract. There are two modes:

| Mode | Who can see the result | What's needed | When to use |
| --- | --- | --- | --- |
| **Reading public values** | Everyone | Nothing — just the encrypted values | Auction results, vote tallies, game outcomes |
| **Decrypting private values** | Only the requesting user | E2E transport key pair + signed permit | Account balances, private bids, personal data |

Both modes enforce the same limits: **2048-bit total** per request and **ACL permission checks** on every encrypted value.

---

## Reading public values

`publicDecrypt()` reveals encrypted values that your smart contract has marked as publicly readable via the Access Control List (ACL) contract. Anyone can call this — no keys, no signatures, no WASM.

```ts
const result = await client.publicDecrypt({
  encryptedValues: [encryptedValue1, encryptedValue2],
});
```

### What you get back

The result is a `PublicDecryptionProof` containing the decrypted values and cryptographic proof that the decryption was performed correctly by the Zama Protocol:

```ts
result.orderedClearValues              // The decrypted values (same order as input)
result.orderedAbiEncodedClearValues    // ABI-encoded values (for on-chain use)
result.decryptionProof                 // Cryptographic proof of correct decryption
```

### Reading decrypted values

Each decrypted value in `result.orderedClearValues` has a `fheType` field that tells you the type, and a `value` field with the plaintext:

```ts
const d = result.orderedClearValues[0];

if (d.fheType === "ebool") {
  d.value; // boolean
} else if (d.fheType === "euint32") {
  d.value; // number (Uint32Number)
} else if (d.fheType === "euint64") {
  d.value; // bigint (Uint64BigInt)
} else if (d.fheType === "eaddress") {
  d.value; // string (ChecksummedAddress)
}
```

**Value type mapping:**

| FHE type | JavaScript type | Example |
| --- | --- | --- |
| `ebool` | `boolean` | `true` |
| `euint8` | `number` | `255` |
| `euint16` | `number` | `65535` |
| `euint32` | `number` | `4294967295` |
| `euint64` | `bigint` | `123n` |
| `euint128` | `bigint` | `999n` |
| `euint256` | `bigint` | `999999999999n` |
| `eaddress` | `string` | `"0xAbCd..."` |

### What gets validated

The SDK validates your request before sending it to the Zama Protocol:

1. At least one encrypted value is required
2. Total encrypted bits must not exceed 2048
3. All encrypted values must belong to the same chain
4. Each encrypted value must be marked as `allowedForDecryption` on the ACL contract
5. The response is verified against on-chain signers

If any check fails, the SDK throws a descriptive error (see [Errors](errors.md)).

---

## Decrypting private values

`decrypt()` is for **private data** — the plaintext is never exposed on-chain or to anyone else. The Zama Protocol sends encrypted shares to your browser, and the SDK reconstructs the plaintext locally using an end-to-end transport key pair that never leaves the browser.

The flow has two steps:

### Step 1: Generate an E2E transport key pair

The E2E transport key pair encrypts the communication channel between your app and the Zama Protocol. The private key stays in your browser; the public key is included in the permit so the protocol knows how to encrypt its response for you.

```ts
const e2eTransportKeypair = await client.generateE2eTransportKeypair();
```

The returned `E2eTransportKeypair` is an opaque object — you can't access the raw private key. This is intentional: the SDK protects it to prevent accidental exposure.

**Saving and restoring keys:** If you want to persist a key across sessions (e.g., in localStorage), you can serialize and restore it:

```ts
// Save
const serialized = client.serializeE2eTransportKeypair({ e2eTransportKeypair });
localStorage.setItem("fhevm-key", JSON.stringify(serialized));

// Restore
const restored = await client.parseE2eTransportKeypair({
  serialized: localStorage.getItem("fhevm-key"),
});
```

### Step 2: Create a signed permit and decrypt

The `signDecryptionPermit()` method constructs an EIP-712 permit and signs it with your wallet in a single step. The permit authorizes decryption of data from specific contracts, for a specific time window.

```ts
const signedPermit = await client.signDecryptionPermit({
  contractAddresses: ["0xContractA...", "0xContractB..."],
  startTimestamp: Math.floor(Date.now() / 1000),
  durationDays: 7,
  signerAddress: await signer.getAddress(), // or walletClient.account.address for viem
  signer,                                    // ethers Signer or viem WalletClient
  e2eTransportKeypair,
});
```

**Permit constraints:**
- Up to **10 contract addresses** per permit
- Up to **365 days** duration
- `startTimestamp` is a Unix timestamp in **seconds**

Now you have everything needed to request decryption:

```ts
const results = await client.decrypt({
  e2eTransportKeypair,
  encryptedValues: [
    { encryptedValue: encryptedValue1, contractAddress: "0xContractA..." },
    { encryptedValue: encryptedValue2, contractAddress: "0xContractA..." },
  ],
  signedPermit,
});

results[0].value;   // the plaintext
results[0].fheType; // "euint32", "ebool", etc.
```

**What happens behind the scenes:**
1. The SDK checks ACL permissions for each encrypted value
2. The permit and encrypted values are sent to the Zama Protocol
3. The protocol returns encrypted shares
4. The SDK decrypts the shares locally using your `e2eTransportKeypair` (TKMS WASM)
5. The plaintext is reconstructed and returned

The plaintext never touches the blockchain or any server — it's reconstructed entirely in your browser.

---

## Decrypting on behalf of another user

Sometimes you want another account (like a backend service) to decrypt on a user's behalf. The flow is the same, but the permit includes an `onBehalfOf` field:

```ts
const signedPermit = await client.signDecryptionPermit({
  contractAddresses: ["0xContract..."],
  startTimestamp: Math.floor(Date.now() / 1000),
  durationDays: 1,
  signerAddress: await signer.getAddress(),
  signer,
  e2eTransportKeypair,
  onBehalfOf: "0xDataOwnerAddress...",
});
```

The delegate signs this permit with their own wallet, and can then decrypt on behalf of the data owner. The Zama Protocol verifies that the on-chain ACL grants the delegate access to the owner's encrypted values.

---

## Reusing permits

You don't need to create a new permit for every decryption. A single signed permit is valid for any number of decryptions within its time window and contract scope:

```ts
// Sign once
const signedPermit = await client.signDecryptionPermit({
  contractAddresses: ["0xContract..."],
  startTimestamp: Math.floor(Date.now() / 1000),
  durationDays: 7,
  signerAddress: await signer.getAddress(),
  signer,
  e2eTransportKeypair,
});

// Decrypt multiple batches with the same permit
await client.decrypt({ e2eTransportKeypair, encryptedValues: batch1, signedPermit });
await client.decrypt({ e2eTransportKeypair, encryptedValues: batch2, signedPermit });
```

Permits are valid from `startTimestamp` until `startTimestamp + durationDays`. This design minimizes the number of wallet signature requests your users see — you can ask once and use the permit for all subsequent decryptions during that session.
