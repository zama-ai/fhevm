# Decryption

There are two kinds of decryption, and they have very different trust models:

- **Private decryption** reveals a value only to a user who is authorized to see
  it. The plaintext is re-encrypted under a key only that user holds, so nobody
  else — not even the Relayer — learns it.
- **Public decryption** reads a value the contract has explicitly marked as
  publicly decryptable. The result is revealed to everyone.

Private decryption needs `createFhevmClient` or `createFhevmDecryptClient`.
Public decryption works on **every** client, including the base client — it needs
no cryptography WASM.

## Private decryption

Reading a private value takes three ingredients:

1. A **transport key pair** — generated in your app. The Key Management System (KMS) encrypts the result
   under its public half so only you can reconstruct the plaintext. The private
   half never leaves your application (your browser or Node process).
2. A **signed decryption permit** — an EIP-712 message signed by the value's
   owner, authorizing decryption of a specific set of contracts for a limited
   time window.
3. The **encrypted value** to decrypt, plus the contract it belongs to.

{% hint style="info" %}
The signed permit is reusable. Sign once, then decrypt many values across the listed contracts until it expires — no re-signing per value.
{% endhint %}

### Step 1 — generate a transport key pair

```ts
const transportKeyPair = await client.generateTransportKeyPair();

transportKeyPair.publicKey; // BytesHex — safe to send; embedded in the permit
// the private key is held internally and is never exposed on the object
```

### Step 2 — sign a decryption permit

`signDecryptionPermit` builds and signs the EIP-712 permit in one step. The
`signer` is passed here — it is the only place a client touches a wallet.

```ts
const now = Math.floor(Date.now() / 1000);

const signedPermit = await client.signDecryptionPermit({
  transportKeyPair,
  contractAddresses: ['0xYourContract…'],
  startTimestamp: now,
  durationSeconds: 7 * 24 * 60 * 60, // 7 days
  signerAddress: await signer.getAddress(),
  signer, // ethers Signer, or a viem Account / WalletClient
});
```

| Parameter          | Type                | Notes                                                     |
| ------------------ | ------------------- | --------------------------------------------------------- |
| `transportKeyPair` | `TransportKeyPair`  | From step 1; its public key is bound into the permit.     |
| `contractAddresses`| `readonly string[]` | Every contract this permit authorizes decryption for.     |
| `startTimestamp`   | `number`            | Unix time in seconds (since 1970-01-01). When the permit becomes valid. |
| `durationSeconds`  | `number`            | Validity window length, in **seconds**.                   |
| `signerAddress`    | `string`            | The address that signs — normally the value's owner.      |
| `signer`           | native signer       | ethers `Signer` / viem `Account` or `WalletClient`.       |
| `delegatorAddress` | `string` (optional) | Present only for delegated decryption — see below.        |

{% hint style="warning" %}
`durationSeconds` is a number of **seconds**, not days. For a one-week permit, pass `7 * 24 * 60 * 60`.
{% endhint %}

The signed permit is reusable: sign once, then decrypt many values across the
listed contracts until it expires. `signedPermit.assertNotExpired()` throws if it
has.

### Step 3 — decrypt

```ts
const decrypted = await client.decryptValue({
  transportKeyPair,
  encryptedValue, // a bytes32 handle read from your contract
  contractAddress: '0xYourContract…',
  signedPermit,
});

decrypted.value; // 42 (number), 1000n (bigint), true, or "0x…" (address)
decrypted.type; // "uint32", "bool", "address", … (Solidity value-type name)
```

The result is a `TypedValue`. The mapping from encrypted type to JavaScript type:

| Encrypted type            | `type`    | `value`             |
| ------------------------- | --------- | ------------------- |
| `euint8` / `16` / `32`    | matching  | `number`            |
| `euint64` / `128` / `256` | matching  | `bigint`            |
| `ebool`                   | `'bool'`  | `boolean`           |
| `eaddress`                | `'address'` | checksummed string |

The plaintext is reconstructed locally from the KMS shares — it is never
transmitted in the clear.

### Decrypting several values

Two batch variants avoid signing and network round-trips per value:

```ts
// All values from the same contract:
const results = await client.decryptValues({
  transportKeyPair,
  contractAddress: '0xYourContract…',
  encryptedValues: [handleA, handleB, handleC],
  signedPermit,
});

// Values spread across different contracts:
const results = await client.decryptValuesFromPairs({
  transportKeyPair,
  pairs: [
    { encryptedValue: handleA, contractAddress: '0xContractA…' },
    { encryptedValue: handleB, contractAddress: '0xContractB…' },
  ],
  signedPermit, // must list every contract referenced above
});
```

Both return `readonly TypedValue[]` in input order.

## What can be decrypted

The `encryptedValue` you pass is a `bytes32` handle. The SDK accepts it in
several shapes (`EncryptedValueLike`):

- a hex string, e.g. read from a contract getter;
- a `Uint8Array` of the 32 bytes;
- an `EncryptedValue` handle object.

You typically read the handle straight from a contract call. For example, reading
an encrypted counter:

```ts
const rawCount = await counter.getCount(); // bigint from a uint256 getter
const countHex = '0x' + rawCount.toString(16).padStart(64, '0');

const decrypted = await client.decryptValue({
  transportKeyPair,
  encryptedValue: countHex,
  contractAddress,
  signedPermit,
});
```

## Checking permission before decrypting

A decrypt call fails if the Access Control List (ACL) doesn't allow it. To check first — for example to
grey out a "reveal" button — use the `canDecrypt*` **actions**. They return a boolean
plus a breakdown, and never throw on a permission miss. These are standalone
actions rather than client methods, so import them and pass the client as the first
argument:

```ts
import { canDecryptValue } from '@fhevm/sdk/actions/decrypt';

const { allowed, details } = await canDecryptValue(client, {
  encryptedValue,
  contractAddress,
  signedPermit, // or: userAddress: '0x…'
});

allowed; // boolean
details.contractAllowed; // is the contract allowed to hold this value?
details.userAllowed; // is the user allowed to decrypt it?
```

Plural forms `canDecryptValues` and `canDecryptValuesFromPairs` (also from
[`@fhevm/sdk/actions/decrypt`](actions.md)) mirror the batch decrypt methods.

## Delegated decryption

Delegation lets one account decrypt values owned by **another** account — for
example, a service decrypting on behalf of a user who authorized it on-chain.

Sign the permit with the delegate's `signer`, and name the owner in
`delegatorAddress`:

```ts
const signedPermit = await client.signDecryptionPermit({
  transportKeyPair,
  contractAddresses: ['0xYourContract…'],
  startTimestamp: now,
  durationSeconds: 24 * 60 * 60,
  signerAddress: delegateAddress, // the delegate signs
  signer: delegateSigner,
  delegatorAddress: ownerAddress, // whose values are being decrypted
});
```

The resulting permit reports `isDelegated: true`. Decrypt with it exactly as
above. The delegation itself must already be granted on-chain in the ACL;
`signDecryptionPermit` only authorizes the request.

## Public decryption

When a contract makes a value publicly decryptable — the result of a confidential
computation that everyone should see, like a closed auction's winning bid — read
it with the public methods. No transport key pair or permit is required.

```ts
// A single value:
const value = await client.decryptPublicValue({ encryptedValue });
value.value; // decrypted plaintext
value.type; // "uint32", "bool", …

// A batch:
const values = await client.decryptPublicValues({
  encryptedValues: [handleA, handleB],
});
```

Both return `TypedValue`(s), identical in shape to private decryption results.

### Verifying a public decryption on-chain

To prove to a contract that a handle decrypts to a specific clear value, use
`decryptPublicValuesWithSignatures`. It returns the KMS signatures and the exact
arguments a verifier contract expects:

```ts
const { clearValues, checkSignaturesArgs } = await client.decryptPublicValuesWithSignatures({
  encryptedValues: [handle],
});

clearValues; // the decrypted TypedValue[]
checkSignaturesArgs.handlesList; // the handles
checkSignaturesArgs.abiEncodedCleartexts; // ABI-encoded clear values
checkSignaturesArgs.decryptionProof; // KMS quorum signatures
```

Pass `checkSignaturesArgs` to your contract's verification function so it can
confirm the KMS quorum attested to these clear values.

## Persisting a session

Both the transport key pair and the signed permit can be serialized to plain
objects — useful for caching a decryption session across page reloads so the user
doesn't re-sign on every visit.

```ts
// Serialize (synchronous):
const kp = client.serializeTransportKeyPair({ transportKeyPair });
const permit = client.serializeSignedDecryptionPermit({ signedPermit });
// persist `kp` and `permit` (e.g. in storage)

// Restore later:
const transportKeyPair = await client.parseTransportKeyPair(kp);
const signedPermit = await client.parseSignedDecryptionPermit({
  serializedPermit: permit,
  transportKeyPair,
});
```

The serialized transport key pair contains the private key — treat it as a
secret and store it accordingly.

{% hint style="danger" %}
The serialized transport key pair contains the private key. Treat it as a secret: never log it, embed it in a URL, or send it to a server. Store it only where you would store a session secret.
{% endhint %}

## Related

- [Encryption](encryption.md) — produce the encrypted values you read back.
- [Types](types.md) — `TransportKeyPair`, `SignedDecryptionPermit`, `TypedValue`.
- [Actions](actions.md) — the same operations as standalone functions.
- [Error handling](error-handling.md) — `AclUserDecryptionError`, `AclPublicDecryptionError`.
```

