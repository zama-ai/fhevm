# Types

The SDK's type system distinguishes two things developers routinely conflate:
plaintext values you provide, and the encrypted handles that reference them
on-chain. This page is the reference for both.

The public types are exported from `@fhevm/sdk/types`. The complete set lives
under the core package and is documented in the [API reference](api-reference.md);
this page covers the ones you touch daily.

```ts
import type { TypedValue, EncryptedValue, EncryptedValueLike, Eip712Like } from '@fhevm/sdk/types';
import { asEncryptedValue, isEncryptedValue } from '@fhevm/sdk/types';
```

## Value types vs. FHE types

Two parallel vocabularies describe the same data at different stages:

- **Value-type names** — `'bool'`, `'uint8'`, …, `'uint256'`, `'address'`. These
  are Solidity's plaintext names. You use them when **encrypting** (the `type`
  field) and receive them back when **decrypting** (`TypedValue.type`).
- **Fully Homomorphic Encryption (FHE) type names** — `'ebool'`, `'euint8'`, …, `'euint256'`, `'eaddress'`.
  These name the encrypted form on-chain. They appear in the branded handle
  aliases (`Euint32`, `Ebool`, …).

| Value type  | FHE type    | Decrypted JS value |
| ----------- | ----------- | ------------------ |
| `bool`      | `ebool`     | `boolean`          |
| `uint8`     | `euint8`    | `number`           |
| `uint16`    | `euint16`   | `number`           |
| `uint32`    | `euint32`   | `number`           |
| `uint64`    | `euint64`   | `bigint`           |
| `uint128`   | `euint128`  | `bigint`           |
| `uint256`   | `euint256`  | `bigint`           |
| `address`   | `eaddress`  | checksummed string |

There is no `uint160` (an encrypted address is `eaddress`), no encrypted `bytes`
type, and `euint4` has been removed.

```ts
type FheType = 'ebool' | 'euint8' | 'euint16' | 'euint32' | 'euint64' | 'euint128' | 'euint256' | 'eaddress';
```

## `TypedValue` — the plaintext shape

`TypedValue` is a discriminated union of `{ type, value }`. It is both the input
you encrypt and the output you get back from decryption:

```ts
type TypedValue =
  | { readonly type: 'bool'; readonly value: boolean }
  | { readonly type: 'uint8'; readonly value: number }
  | { readonly type: 'uint16'; readonly value: number }
  | { readonly type: 'uint32'; readonly value: number }
  | { readonly type: 'uint64'; readonly value: bigint }
  | { readonly type: 'uint128'; readonly value: bigint }
  | { readonly type: 'uint256'; readonly value: bigint }
  | { readonly type: 'address'; readonly value: string };
```

Because it is discriminated on `type`, narrowing gives you the right value type:

```ts
const d = await client.decryptValue({ /* … */ });

if (d.type === 'bool') {
  d.value; // boolean
} else if (d.type === 'address') {
  d.value; // string
} else {
  d.value; // number | bigint
}
```

When **encrypting**, the input is slightly looser than `TypedValue` — a `uint32`
accepts a `number` _or_ a `bigint`, and `bool` accepts `boolean`, `number`, or
`bigint`. The SDK validates and normalizes it. When **decrypting**, you always
get the strict `TypedValue` form above.

## `EncryptedValue` — the handle

An `EncryptedValue` is a `bytes32` handle: an opaque, deterministic reference to
a ciphertext held by the coprocessors. It is what your contract stores and
returns, not the ciphertext itself.

```ts
type EncryptedValue = /* branded bytes32 hex */;
```

Branded per-type aliases match the Solidity names and carry the FHE type at the
type level: `Ebool`, `Euint8`, `Euint16`, `Euint32`, `Euint64`, `Euint128`,
`Euint256`, `Eaddress`.

There are two lifecycle stages of an encrypted value, which share the same
`bytes32` bits but differ in trust:

- **External** — freshly produced by `encryptValue` / `encryptValues`, not yet
  verified on-chain. Aliases: `ExternalEbool`, `ExternalEuint8`, … These are what
  you pass to a contract alongside an `inputProof`.
- **Computed / verified** — the on-chain result of FHE operations, already
  trusted. These are what you read back and decrypt.

### `EncryptedValueLike`

Methods that accept a handle take the permissive `EncryptedValueLike`, so you can
pass whatever form you have:

```ts
type EncryptedValueLike = Uint8Array | string | { readonly bytes32Hex: string };
```

Use the helpers to validate or coerce:

```ts
import { asEncryptedValue, isEncryptedValue } from '@fhevm/sdk/types';

isEncryptedValue(x); // type guard → x is EncryptedValue
asEncryptedValue(x); // coerce to EncryptedValue, or throw
```

## `TransportKeyPair`

The key pair generated for private decryption. It is opaque — the private key is
held internally and is never exposed on the object:

```ts
type TransportKeyPair = {
  readonly publicKey: BytesHex; // safe to share; bound into the permit
  readonly tkmsVersion: TkmsVersion;
};
```

`serializeTransportKeyPair` produces `{ publicKey, privateKey }` for storage
(treat it as a secret); `parseTransportKeyPair` restores it. See
[Decryption → Persisting a session](decryption.md#persisting-a-session).

## `SignedDecryptionPermit`

The EIP-712 permit produced by `signDecryptionPermit`. It is a union over the
protocol version, but every variant shares this surface:

```ts
type SignedDecryptionPermit = {
  readonly signature: BytesHex;
  readonly signerAddress: string; // checksummed
  readonly encryptedDataOwnerAddress: string; // whose values it decrypts
  readonly transportPublicKey: BytesHex;
  readonly isDelegated: boolean; // true for delegated permits
  assertNotExpired(): void; // throws if past its window
  // …plus version-specific `eip712` typed data
};
```

## Related

- [Encryption](encryption.md) — how the value types are used as input.
- [Decryption](decryption.md) — where `TypedValue`, `TransportKeyPair`, and permits come together.
- [API reference](api-reference.md) — the complete exported type list.
- [Glossary](GLOSSARY.md) — canonical naming across the SDK and protocol.
```

