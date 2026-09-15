# Encryption

Encryption turns plaintext values into opaque encrypted values plus a proof that
your contract can verify. Everything happens client-side — the plaintext never
leaves your application.

Encryption is available on `createFhevmClient` and `createFhevmEncryptClient`.

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

Common fields: `timeout`, `signal` (an `AbortSignal`), `headers`, `fetchRetries`,
`fetchRetryDelayInMilliseconds`, and `onProgress`. See
[API reference](api-reference.md#relayer-options) for the full set.

## Deterministic encryption

By default, encrypting the same value twice produces a different
`encryptedValue` and `inputProof` each time — the underlying proof draws its
nonce from a CSPRNG. Pass an optional `seed` (at least 16 bytes) to make
encryption deterministic instead: the same `seed` plus the same `value`
always reproduce the same underlying ciphertext bytes.

```ts
const seed = new Uint8Array(16); // at least 16 bytes; fill with your own entropy

const encrypted = await client.encryptValue({
  contractAddress,
  userAddress,
  value: { type: 'uint64', value: 100n },
  seed,
});

// Re-encrypting the same value with the same seed yields the same result
const encrypted2 = await client.encryptValue({
  contractAddress,
  userAddress,
  value: { type: 'uint64', value: 100n },
  seed,
});

encrypted.encryptedValue === encrypted2.encryptedValue; // true
```

{% hint style="danger" %}
**`seed` reuse creates a plaintext-equality oracle that spans contracts.**
The randomness for the packed ciphertext is derived from `seed` alone —
`contractAddress`, `userAddress`, and the rest of the proof metadata are only
folded in as associated data that binds the *proof*, not the ciphertext
bytes. Concretely, encrypting the same value with the same `seed` under two
different contracts (or two different users) produces byte-identical
ciphertext output. Anyone observing the two on-chain input ciphertexts can
tell the underlying plaintexts are equal, even without decrypting either
one — across contract and user boundaries, not just within one call site.

Only reuse a `seed` when that reproducibility is exactly what you want (e.g.
deterministic test fixtures, idempotent retries of the same input). Never
reuse a `seed` for values you intend to keep confidential from each other —
generate a fresh, unpredictable seed per distinct plaintext otherwise.
{% endhint %}

`seed` requires a TFHE module version that supports seeded proof building
(`'1.6.2'` or later). Select it explicitly via `moduleVersions.tfhe` if your
chain would otherwise resolve an older version — see
[Runtime configuration](runtime-configuration.md) and
[API reference](api-reference.md#runtime-config).

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

## Related

- [Decryption](decryption.md) — read encrypted values back to plaintext.
- [Types](types.md) — the encrypted-value and typed-value type system.
- [Actions](actions.md) — the standalone `generateZkProof` (encrypt) / `fetchEncryptedValues` (base) functions.
- [Error handling](error-handling.md) — `EncryptionError`, `ZkProofError`, `TooManyHandlesError`.
- [API reference](api-reference.md#encryption-methods) — full `encryptValue(s)` signatures, including `seed`.
- [Runtime configuration](runtime-configuration.md) — select the TFHE module version (`moduleVersions.tfhe`) that seeded encryption requires.
```

