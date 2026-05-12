# Types

This page documents the TypeScript types used throughout the SDK. You'll encounter these types in function parameters, return values, and when working with encrypted values.

## FHE types

Every encrypted value in FHEVM has a **type** that determines what operations you can perform on it and how many bits it uses. The SDK uses a strongly-typed system to represent these.

### `FheType`

The `FheType` union represents all supported FHE encrypted types:

```ts
type FheType = 'ebool' | 'euint8' | 'euint16' | 'euint32' | 'euint64' | 'euint128' | 'euint256' | 'eaddress';
```

### `FheTypeId`

Numeric identifiers used on-chain. You rarely need these directly — the SDK handles the conversion — but they appear in handle bytes and on-chain contract calls:

| FheType    | FheTypeId | Encrypted Bits | Solidity Primitive |
| ---------- | --------- | -------------- | ------------------ |
| `ebool`    | 0         | 2              | `bool`             |
| `euint8`   | 2         | 8              | `uint256`          |
| `euint16`  | 3         | 16             | `uint256`          |
| `euint32`  | 4         | 32             | `uint256`          |
| `euint64`  | 5         | 64             | `uint256`          |
| `euint128` | 6         | 128            | `uint256`          |
| `eaddress` | 7         | 160            | `address`          |
| `euint256` | 8         | 256            | `uint256`          |

> Note: `euint4` (id: 1) has been deprecated and is omitted.

## Encrypted value references

### `EncryptedValue`

A 32-byte opaque reference to an encrypted value on-chain (called a "handle" in FHE.sol / FHEVM whitepaper terminology). When you read an encrypted value from a contract, you get an `EncryptedValue`. It encodes metadata in its bytes — you can inspect the type, chain ID, and other properties without making an RPC call.

There are two subtypes:

- `ComputedEncryptedValue` — verified on-chain, the result of an FHE operation
- `ExternalEncryptedValue` — unverified encrypted input, returned from `encrypt()`

Both share a common set of properties:

```ts
// Common properties on all EncryptedValue variants
readonly bytes32Hex: Bytes32Hex;        // "0x..." (66 chars)
readonly bytes32: Bytes32;              // Uint8Array (32 bytes)
readonly chainId: Uint64BigInt;         // Chain the value belongs to
readonly fheTypeId: FheTypeId;          // Numeric type ID
readonly fheType: FheType;              // Type name ("euint32", etc.)
readonly version: Uint8Number;          // Handle version
readonly index: Uint8Number | undefined; // Index within proof (external values)
readonly encryptionBits: EncryptionBits; // Number of encrypted bits
readonly isComputed: boolean;           // On-chain computed vs. external
readonly isExternal: boolean;           // From input proof
```

A `Handle<T>` alias is available for developers familiar with FHE.sol terminology.

### Typed encrypted values

Type-specific aliases for compile-time safety. Use these when you know the FHE type at compile time:

```ts
type Ebool = EncryptedValue<'ebool'>;
type Euint8 = EncryptedValue<'euint8'>;
type Euint16 = EncryptedValue<'euint16'>;
type Euint32 = EncryptedValue<'euint32'>;
type Euint64 = EncryptedValue<'euint64'>;
type Euint128 = EncryptedValue<'euint128'>;
type Euint256 = EncryptedValue<'euint256'>;
type Eaddress = EncryptedValue<'eaddress'>;
```

### `ExternalEncryptedValue`

An encrypted value returned from `encrypt()` — always has an `index` and `isExternal: true`:

```ts
type ExternalEncryptedValue<T extends FheType = FheType>;
```

Typed variants: `ExternalEbool`, `ExternalEuint8`, `ExternalEuint16`, `ExternalEuint32`, `ExternalEuint64`, `ExternalEuint128`, `ExternalEuint256`, `ExternalEaddress`.

### `EncryptedValueLike`

Accepted input formats for encrypted value parameters:

```ts
type EncryptedValueLike = Uint8Array | string | { readonly bytes32Hex: string } | EncryptedValue;
```

## Clear values (decrypted)

### `ClearValue`

Pairs an encrypted value with its decrypted plaintext. Uses a discriminated union on `fheType`:

```ts
type ClearValueOfType<T extends FheType> = {
  readonly value: ClearValueType<T>;
  readonly encryptedValue: EncryptedValue<T>;
  readonly fheType: T;
  readonly valueType: ClearValueTypeName<T>;
};
```

**Value type mapping:**

| FheType    | Decrypted Value Type | JS Representation       |
| ---------- | -------------------- | ----------------------- |
| `ebool`    | `boolean`            | `true` / `false`        |
| `euint8`   | `Uint8Number`        | `number` (0-255)        |
| `euint16`  | `Uint16Number`       | `number` (0-65535)      |
| `euint32`  | `Uint32Number`       | `number` (0-4294967295) |
| `euint64`  | `Uint64BigInt`       | `bigint`                |
| `euint128` | `Uint128BigInt`      | `bigint`                |
| `euint256` | `Uint256BigInt`      | `bigint`                |
| `eaddress` | `ChecksummedAddress` | `string`                |

Type-specific aliases:

```ts
type ClearBool = ClearValue<'ebool'>;
type ClearUint8 = ClearValue<'euint8'>;
type ClearUint16 = ClearValue<'euint16'>;
type ClearUint32 = ClearValue<'euint32'>;
type ClearUint64 = ClearValue<'euint64'>;
type ClearUint128 = ClearValue<'euint128'>;
type ClearUint256 = ClearValue<'euint256'>;
type ClearAddress = ClearValue<'eaddress'>;
```

## Typed values (encryption input)

### `TypedValueLike`

Input format for encryption:

```ts
type TypedValueLike = {
  readonly value: ValueLikeMap[T]; // Flexible input (number | bigint for uints)
  readonly type: T; // "bool" | "uint8" | ... | "address"
};
```

Note: the type names use Solidity conventions (`"uint32"`, `"bool"`, `"address"`) not FHE conventions (`"euint32"`, `"ebool"`).

```ts
// Examples
{ type: "bool", value: true }
{ type: "uint32", value: 42 }
{ type: "uint64", value: 123n }
{ type: "uint256", value: 999999999999999999n }
{ type: "address", value: "0xAbCdEf..." }
```

### `TypedValue`

Validated/normalized version of `TypedValueLike`:

```ts
type TypedValue = {
  readonly value: ValueTypeMap[T]; // Exact type (Uint32Number, Uint64BigInt, etc.)
  readonly type: T;
};
```

## Primitive types

### Branded number types

The SDK uses branded types (via `unique symbol` intersections) for type-safe numeric values:

```ts
// Number-based (safe for JS number precision)
type Uint8Number = number & UnsignedInt & Bits8;
type Uint16Number = number & UnsignedInt & Bits16;
type Uint32Number = number & UnsignedInt & Bits32;

// BigInt-based (required for larger values)
type Uint64BigInt = bigint & UnsignedInt & Bits64;
type Uint128BigInt = bigint & UnsignedInt & Bits128;
type Uint160BigInt = bigint & UnsignedInt & Bits160;
type Uint256BigInt = bigint & UnsignedInt & Bits256;
```

### Address types

```ts
type Address = Bytes20Hex & AddressString; // Any valid address
type ChecksummedAddress = Address & ChecksummedAddressString; // EIP-55 checksummed
```

Validate addresses:

```ts
import { assertIsChecksummedAddress } from '@fhevm/sdk/ethers';

assertIsChecksummedAddress('0xAbCdEf...', {}); // Throws if not valid checksummed
```

### Hex string types

```ts
type Hex0x = `0x${string}` & Hex0xString; // Any 0x-prefixed hex
type BytesHex = Hex0x & EvenLen; // Even-length 0x hex (bytes)

// Fixed-length variants
type Bytes1Hex = BytesHex & ByteLen1; // 0x + 2 chars
type Bytes8Hex = BytesHex & ByteLen8; // 0x + 16 chars
type Bytes20Hex = BytesHex & ByteLen20; // 0x + 40 chars (address size)
type Bytes32Hex = BytesHex & ByteLen32; // 0x + 64 chars (handle size)
type Bytes65Hex = BytesHex & ByteLen65; // 0x + 130 chars (signature size)
```

### Byte array types

```ts
type Bytes = Uint8Array;
type Bytes1 = Bytes & ByteLen1;
type Bytes32 = Bytes & ByteLen32;
type Bytes65 = Bytes & ByteLen65;
// ... etc.
```

## Proof types

### `ZkProof`

Zero-knowledge proof of correct encryption, produced by TFHE WASM:

```ts
type ZkProof; // Opaque — passed to fetchVerifiedInputProof
```

### `InputProof` / `VerifiedInputProof`

```ts
type VerifiedInputProof = {
  readonly bytesHex: BytesHex;
  readonly coprocessorSignatures: readonly Bytes65Hex[];
  readonly inputHandles: readonly InputHandle[];
  readonly extraData: BytesHex;
  readonly verified: true;
  readonly signedHandleAccess: {
    readonly contractAddress: ChecksummedAddress;
    readonly userAddress: ChecksummedAddress;
  };
};
```

### `PublicDecryptionProof`

```ts
type PublicDecryptionProof = {
  readonly orderedClearValues: readonly ClearValue[];
  readonly decryptionProof: BytesHex;
  readonly orderedAbiEncodedClearValues: BytesHex;
  readonly extraData: BytesHex;
};
```

## Permit types

### `SignedSelfDecryptionPermit` / `SignedDelegatedDecryptionPermit`

Signed permits returned by `signDecryptionPermit()`. These bundle the EIP-712 typed data, the signature, and the signer address into a reusable object.

- `SignedSelfDecryptionPermit` — for decrypting your own values
- `SignedDelegatedDecryptionPermit` — for decrypting on behalf of another user (has `onBehalfOf` field)
- `SignedDecryptionPermit` — union of both

### `KmsUserDecryptEIP712`

The EIP-712 typed data structure for decrypt permits (lower-level, used by `createKmsUserDecryptEIP712`):

```ts
type KmsUserDecryptEIP712 = {
  readonly domain: KmsEIP712Domain;
  readonly types: KmsUserDecryptEIP712Types;
  readonly message: KmsUserDecryptEIP712Message;
  readonly primaryType: 'UserDecryptRequestVerification';
};

type KmsEIP712Domain = {
  readonly name: 'Decryption';
  readonly version: '1';
  readonly chainId: Uint64BigInt;
  readonly verifyingContract: ChecksummedAddress;
};
```

### `KmsDelegatedUserDecryptEIP712`

Extends the decrypt permit EIP-712 with a `delegatedAccount` field for decrypting on behalf of another user.

## E2E transport key pair

### `TransportKeyPair`

An opaque key pair object for end-to-end encrypted communication with the Zama Protocol. The private key is never directly accessible — this prevents accidental exposure.

```ts
type TransportKeyPair = {
  readonly publicKey: BytesHex;
};
```

Create with `generateTransportKeyPair()`, serialize with `serializeTransportKeyPair()`, restore with `parseTransportKeyPair()`.
