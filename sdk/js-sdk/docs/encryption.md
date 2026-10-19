# Encryption

Encryption turns plaintext values into opaque encrypted values plus the input
data your contract consumes. Everything happens client-side — the plaintext never
leaves your application.

The SDK offers two encryption paths, each on its own client:

| Path | Client | Methods | Contract input | WASM |
| ---- | ------ | ------- | -------------- | ---- |
| **Input proof** (ZKPoK, default) | `createFhevmClient`, `createFhevmEncryptClient` | `encryptValue`, `encryptValues` | `inputProof` | TFHE (~4.9 MB) |
| **Pad-based** (RFC-030 transciphering, experimental) | `createFhevmEncryptTransciClient` | `encryptValueTransci`, `encryptValuesTransci` | `inputPackage` | TKMS (~600 KB) |

Use the input-proof path unless you are targeting pad-based inputs: it is the
flow every deployed FHEVM chain verifies, and it is unchanged from previous SDK
versions. The rest of this page up to
[Pad-based encryption](#pad-based-encryption-transciphering) describes it.

## The two methods

- **`encryptValues`** — encrypt a batch of values under one shared proof. Use
  this whenever a contract call takes more than one encrypted argument; a single
  proof covers the whole batch.
- **`encryptValue`** — encrypt exactly one value. A convenience wrapper for the
  single-argument case.

```ts
const encrypted = await client.encryptValues({
  contractAddress: '0xYourContract…',
  userAddress: '0xYourWallet…',
  values: [
    { type: 'uint32', value: 42 },
    { type: 'bool', value: true },
  ],
});

encrypted.encryptedValues; // readonly EncryptedValue[] — one per input, same order
encrypted.inputProof; // BytesHex — one proof for the whole batch
```

```ts
const encrypted = await client.encryptValue({
  contractAddress: '0xYourContract…',
  userAddress: '0xYourWallet…',
  value: { type: 'uint64', value: 1000n },
});

encrypted.encryptedValue; // a single EncryptedValue
encrypted.inputProof; // BytesHex
```

## Binding: contract and user

Both parameters are mandatory and both are cryptographically bound into the
proof:

- **`contractAddress`** — the contract that will consume the encrypted values.
  The proof is only valid for this address.
- **`userAddress`** — the address that will submit the transaction. The proof is
  only valid when this user sends it.

If either differs at submission time, on-chain verification fails. Encrypt with
the same values, contract, and sender that you will actually transact with.

{% hint style="warning" %}
Both `contractAddress` and `userAddress` are cryptographically bound into the proof. Re-encrypt if either changes — a proof generated for one sender or contract is worthless for another.
{% endhint %}

## Supported input types

The `type` field uses **Solidity value-type names**, not Fully Homomorphic
Encryption (FHE) names. Each maps to
an on-chain `externalEuintXX` / `externalEbool` / `externalEaddress`.

| `type`      | Accepted JS value      | Maps to on-chain    |
| ----------- | ---------------------- | ------------------- |
| `'bool'`    | `boolean` / `number` / `bigint` | `externalEbool`     |
| `'uint8'`   | `number` / `bigint`    | `externalEuint8`    |
| `'uint16'`  | `number` / `bigint`    | `externalEuint16`   |
| `'uint32'`  | `number` / `bigint`    | `externalEuint32`   |
| `'uint64'`  | `number` / `bigint`    | `externalEuint64`   |
| `'uint128'` | `number` / `bigint`    | `externalEuint128`  |
| `'uint256'` | `number` / `bigint`    | `externalEuint256`  |
| `'address'` | `string` (hex address) | `externalEaddress`  |

There is no `uint160` type — an encrypted Ethereum address is `'address'`. There
is no encrypted `bytes` type. `euint4` has been removed.

For large integers (`uint64` and above) prefer `bigint` to avoid JavaScript's
`Number.MAX_SAFE_INTEGER` limit:

```ts
values: [
  { type: 'uint256', value: 123456789012345678901234567890n },
  { type: 'address', value: '0xAbC0000000000000000000000000000000000001' },
];
```

## Using the result in a contract call

Each entry in `encryptedValues` is passed to its matching `externalEuintXX`
argument, in order. The shared `inputProof` is the trailing `bytes` argument your
FHEVM contract expects.

{% tabs %}
{% tab title="ethers.js" %}

```ts
const encrypted = await client.encryptValues({
  contractAddress,
  userAddress,
  values: [{ type: 'uint32', value: 42 }],
});

await contract.increment(
  encrypted.encryptedValues[0], // externalEuint32
  encrypted.inputProof, // bytes
);
```

{% endtab %}
{% tab title="viem" %}

```ts
const encrypted = await client.encryptValues({
  contractAddress,
  userAddress,
  values: [{ type: 'uint32', value: 42 }],
});

await walletClient.writeContract({
  address: contractAddress,
  abi,
  functionName: 'increment',
  args: [encrypted.encryptedValues[0], encrypted.inputProof],
});
```

{% endtab %}
{% endtabs %}

On-chain, the contract calls `FHE.fromExternal(externalValue, inputProof)` to
verify each input and convert it to a usable `euintXX` before computing on it.

## Batching

There are two reasons to encrypt as a batch rather than one value at a time:

1. **One proof.** A batch produces a single `inputProof`, which is cheaper to
   verify than several independent proofs.
2. **Atomicity.** All values in a batch share the same binding to contract and
   user.

A single input ciphertext can pack up to 256 encrypted variables. Exceeding that
throws a `TooManyHandlesError`.

{% hint style="info" %}
A single input ciphertext packs at most 256 encrypted variables. Split larger batches across multiple `encryptValues` calls.
{% endhint %}

## Request options and progress

Every encrypt call accepts an optional `options` object to control the Relayer
request that fetches the verified proof:

```ts
const encrypted = await client.encryptValues({
  contractAddress,
  userAddress,
  values,
  options: {
    timeout: 60_000,
    signal: abortController.signal,
    onProgress: (args) => console.log(args.type), // 'queued' | 'throttled' | 'succeeded' | 'timeout' | 'abort' | 'failed'
  },
});
```

Common fields: `timeout`, `signal` (an `AbortSignal`), `headers`, `query`,
`fetchRetries`, `fetchRetryDelayInMilliseconds`, and `onProgress`. See
[API reference](api-reference.md#relayer-options) for the full set.

## What happens under the hood

`encryptValues` runs a two-step pipeline you normally never see:

1. **Generate a ZK proof** locally in WASM (TFHE) — a zero-knowledge proof that
   you encrypted your plaintext correctly under the FHE public key, without
   revealing it. The FHE public key is fetched from the Relayer and cached on
   first use.
2. **Exchange it for a verified input proof** — the Relayer's coprocessors verify
   the proof and sign it, producing the `inputProof` your contract trusts.

If you need to run these steps separately (for example, to generate a proof
offline and submit it later), use the standalone actions `generateZkProof` (from
[`@fhevm/sdk/actions/encrypt`](actions.md)) and `fetchEncryptedValues` (from
[`@fhevm/sdk/actions/base`](actions.md)).

## Pad-based encryption (transciphering)

RFC-030 pad-based inputs mask the plaintext with a one-time pad that only the
user can obtain from the KMS. No TFHE ciphertext, zero-knowledge proof, or
coprocessor signature is involved, so the client loads the TKMS WASM instead of
the TFHE one.

{% hint style="warning" %}
Pad-based encryption is experimental. The SDK only decodes the cleartext pad-share encoding for now, so it works against the v0.16 cleartext host contracts (`createFhevmCleartextEncryptTransciClient`), not against deployed chains.
{% endhint %}

### The client and its methods

```ts
import { createFhevmEncryptTransciClient } from '@fhevm/sdk/ethers'; // or '@fhevm/sdk/viem'

const client = createFhevmEncryptTransciClient({ chain, provider });
await client.ready;

const encrypted = await client.encryptValuesTransci({
  contractAddress: '0xYourContract…',
  userAddress: '0xYourWallet…',
  values: [
    { type: 'uint32', value: 42 },
    { type: 'bool', value: true },
  ],
  signedPermit, // v2 (unified) permit signed by userAddress
});

encrypted.encryptedValues; // readonly EncryptedValue[] — one per input, same order
encrypted.inputPackage; // BytesHex — one package for the whole batch
```

`encryptValueTransci` is the single-value form; it takes `value` and returns
`encryptedValue` and `inputPackage`. Both accept the same
[supported input types](#supported-input-types) as the input-proof path.

### The signed permit

Each encryption fetches a fresh one-time pad from the KMS through a pad
decryption request. That request is authenticated with the same v2 (unified)
EIP-712 permit used for user decryption, so a permit you already hold for
decryption can be reused:

```ts
const transportKeyPair = await client.generateTransportKeyPair();
const signedPermit = await client.signUnifiedDecryptionPermit({
  transportKeyPair,
  contractAddresses: [contractAddress],
  startTimestamp: Math.floor(Date.now() / 1000),
  durationSeconds: 24 * 3600,
  signerAddress: userAddress,
  signer,
});
```

The permit must be version 2, signed by `userAddress`, and not expired; a legacy
(v1) permit is rejected. The encrypt-transciphering client generates the
transport key pair itself (`generateTransportKeyPair`), so no decrypt client is
needed to encrypt. A key pair it generates is interchangeable with one from a
decrypt client: you can reuse it to decrypt with a decrypt client from the same
adapter.

### Binding, limits, and the contract call

- **`contractAddress`** is the witness *consumer* (the contract that will call
  `FHE.fromExternal`) and **`userAddress`** is the witness *submitter* (the
  account that sends the transaction). If either differs at submission time,
  `fromExternal` reverts.
- A package holds between 1 and 255 values. Each value is padded to whole
  bytes, and the total may not exceed 2048 bits (for example, eight `uint256`
  values). Split larger batches across several calls.
- Pass `inputPackage` where you would pass `inputProof`: it is the trailing
  `bytes` argument, and the contract calls
  `FHE.fromExternal(externalValue, inputPackage)`.

The optional `options` object takes the plain Relayer request options
(`timeout`, `signal`, `auth`, `headers`, `debug`, `fetchRetries`,
`fetchRetryDelayInMilliseconds`); there is no `onProgress`.

### What happens under the hood

1. **Build a witness** — submitter (`userAddress`), consumer
   (`contractAddress`), chain id, an expiration timestamp (one week), and a
   fresh random nonce. Its hash is the pad *label*.
2. **Fetch the pad** — a pad decryption request, authenticated by your signed
   permit, asks the KMS for exactly as many pad bits as the values need. Each KMS
   node returns a signed share; the SDK checks the signatures against the
   context's KMS signers and reconstructs the pad.
3. **Mask and package** — the packed values are XOR-ed with the pad, and the
   witness, type information, and ciphertext form the input package. Each
   external handle is derived from the package digest, the value's index, and
   its type.

Because every package uses a fresh nonce, a pad is never reused.

## Related

- [Decryption](decryption.md) — read encrypted values back to plaintext, and sign the permit pad-based encryption needs.
- [Types](types.md) — the encrypted-value and typed-value type system.
- [Actions](actions.md) — the standalone `generateZkProof` (encrypt) / `fetchEncryptedValues` (base) functions, and the `*Transci` encrypt actions.
- [Error handling](error-handling.md) — `EncryptionError`, `ZkProofError`, `TooManyHandlesError`.

