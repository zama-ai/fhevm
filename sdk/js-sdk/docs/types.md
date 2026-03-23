# Types

## FHE Types

The SDK uses a strongly-typed system to represent encrypted values across the fhEVM.

### FheType

The `FheType` union represents all supported FHE encrypted types:

```ts
type FheType = "ebool" | "euint8" | "euint16" | "euint32" | "euint64" | "euint128" | "euint256" | "eaddress";
```

### FheTypeId

Numeric identifiers used on-chain:

| FheType | FheTypeId | Encrypted Bits | Solidity Primitive |
|---------|-----------|----------------|-------------------|
| `ebool` | 0 | 2 | `bool` |
| `euint8` | 2 | 8 | `uint256` |
| `euint16` | 3 | 16 | `uint256` |
| `euint32` | 4 | 32 | `uint256` |
| `euint64` | 5 | 64 | `uint256` |
| `euint128` | 6 | 128 | `uint256` |
| `eaddress` | 7 | 160 | `address` |
| `euint256` | 8 | 256 | `uint256` |

> Note: `euint4` (id: 1) has been deprecated and is omitted.

## Handles

### FhevmHandle

A 32-byte opaque reference to an encrypted ciphertext on-chain. The handle encodes metadata in its bytes:

```ts
interface FhevmHandleBase {
  readonly bytes32Hex: FhevmHandleBytes32Hex;    // "0x..." (66 chars)
  readonly bytes32: FhevmHandleBytes32;           // Uint8Array (32 bytes)
  readonly bytes32HexNo0x: FhevmHandleBytes32HexNo0x;

  // Parsed components
  readonly hash21: Bytes21Hex;         // First 21 bytes (content hash)
  readonly chainId: Uint64BigInt;      // Chain the handle belongs to
  readonly fheTypeId: FheTypeId;       // Numeric type ID
  readonly fheType: FheType;           // Type name ("euint32", etc.)
  readonly version: Uint8Number;       // Handle version
  readonly index: Uint8Number | undefined;  // Index within proof (external handles)
  readonly encryptionBits: EncryptionBits;  // Number of encrypted bits
  readonly solidityPrimitiveTypeName: SolidityPrimitiveTypeName;  // "bool" | "uint256" | "address"
  readonly isComputed: boolean;        // On-chain computed vs. external
  readonly isExternal: boolean;        // From input proof
}
```

### Typed Handles

Type-specific handle aliases for compile-time safety:

```ts
type Ebool     = FhevmHandleOfType<"ebool">;
type Euint8    = FhevmHandleOfType<"euint8">;
type Euint16   = FhevmHandleOfType<"euint16">;
type Euint32   = FhevmHandleOfType<"euint32">;
type Euint64   = FhevmHandleOfType<"euint64">;
type Euint128  = FhevmHandleOfType<"euint128">;
type Euint256  = FhevmHandleOfType<"euint256">;
type Eaddress  = FhevmHandleOfType<"eaddress">;
```

### ExternalFhevmHandle

A handle returned from encryption — always has an `index` and `isExternal: true`:

```ts
type ExternalFhevmHandle = FhevmExternalHandleOfType;  // index is Uint8Number (not undefined)
```

### FhevmHandleLike

Accepted input formats for handle parameters:

```ts
type FhevmHandleLike = Bytes32 | Bytes32Hex | Bytes32HexAble | FhevmHandle;
```

## Decrypted Values

### DecryptedFhevmHandle

Pairs an encrypted handle with its plaintext value. Uses a discriminated union on `fheType`:

```ts
interface DecryptedFhevmHandleOfTypeBase<T extends FheType> {
  readonly fheType: T;
  readonly handle: FhevmHandleOfType<T>;
  readonly value: DecryptedFheValueMap[T];
}
```

**Value type mapping:**

| FheType | Decrypted Value Type | JS Representation |
|---------|---------------------|-------------------|
| `ebool` | `boolean` | `true` / `false` |
| `euint8` | `Uint8Number` | `number` (0–255) |
| `euint16` | `Uint16Number` | `number` (0–65535) |
| `euint32` | `Uint32Number` | `number` (0–4294967295) |
| `euint64` | `Uint64BigInt` | `bigint` |
| `euint128` | `Uint128BigInt` | `bigint` |
| `euint256` | `Uint256BigInt` | `bigint` |
| `eaddress` | `ChecksummedAddress` | `string` |

Type-specific aliases:

```ts
type DecryptedEbool    = DecryptedFhevmHandleOfType<"ebool">;
type DecryptedEuint8   = DecryptedFhevmHandleOfType<"euint8">;
type DecryptedEuint32  = DecryptedFhevmHandleOfType<"euint32">;
// ... etc.
```

## Typed Values (Encryption Input)

### TypedValueLike

Input format for encryption:

```ts
type TypedValueLike = {
  readonly value: ValueLikeMap[T];  // Flexible input (number | bigint for uints)
  readonly type: T;                  // "bool" | "uint8" | ... | "address"
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

### TypedValue

Validated/normalized version of `TypedValueLike`:

```ts
type TypedValue = {
  readonly value: ValueTypeMap[T];  // Exact type (Uint32Number, Uint64BigInt, etc.)
  readonly type: T;
};
```

## Primitive Types

### Branded Number Types

The SDK uses branded types (via `unique symbol` intersections) for type-safe numeric values:

```ts
// Number-based (safe for JS number precision)
type Uint8Number   = number & UnsignedInt & Bits8;
type Uint16Number  = number & UnsignedInt & Bits16;
type Uint32Number  = number & UnsignedInt & Bits32;

// BigInt-based (required for larger values)
type Uint64BigInt  = bigint & UnsignedInt & Bits64;
type Uint128BigInt = bigint & UnsignedInt & Bits128;
type Uint160BigInt = bigint & UnsignedInt & Bits160;
type Uint256BigInt = bigint & UnsignedInt & Bits256;
```

### Address Types

```ts
type Address           = Bytes20Hex & AddressString;             // Any valid address
type ChecksummedAddress = Address & ChecksummedAddressString;    // EIP-55 checksummed
```

Validate addresses:

```ts
import { assertIsChecksummedAddress } from "@fhevm/sdk/ethers";

assertIsChecksummedAddress("0xAbCdEf...", {}); // Throws if not valid checksummed
```

### Hex String Types

```ts
type Hex0x      = `0x${string}` & Hex0xString;  // Any 0x-prefixed hex
type BytesHex   = Hex0x & EvenLen;               // Even-length 0x hex (bytes)

// Fixed-length variants
type Bytes1Hex  = BytesHex & ByteLen1;    // 0x + 2 chars
type Bytes8Hex  = BytesHex & ByteLen8;    // 0x + 16 chars
type Bytes20Hex = BytesHex & ByteLen20;   // 0x + 40 chars (address size)
type Bytes32Hex = BytesHex & ByteLen32;   // 0x + 64 chars (handle size)
type Bytes65Hex = BytesHex & ByteLen65;   // 0x + 130 chars (signature size)
```

### Byte Array Types

```ts
type Bytes   = Uint8Array;
type Bytes1  = Bytes & ByteLen1;
type Bytes32 = Bytes & ByteLen32;
type Bytes65 = Bytes & ByteLen65;
// ... etc.
```

## Proof Types

### ZkProof

Zero-knowledge proof of correct encryption, produced by TFHE WASM:

```ts
type ZkProof; // Opaque — passed to fetchVerifiedInputProof
```

### InputProof / VerifiedInputProof

```ts
type VerifiedInputProof = {
  readonly bytesHex: BytesHex;
  readonly coprocessorSignatures: readonly Bytes65Hex[];
  readonly externalHandles: readonly ExternalFhevmHandle[];
  readonly extraData: BytesHex;
  readonly verified: true;
  readonly coprocessorSignedParams?: {
    contractAddress: ChecksummedAddress;
    userAddress: ChecksummedAddress;
  };
};
```

### PublicDecryptionProof

```ts
type PublicDecryptionProof = {
  readonly orderedDecryptedHandles: readonly DecryptedFhevmHandle[];
  readonly orderedHandles: readonly FhevmHandle[];
  readonly orderedAbiEncodedClearValues: BytesHex;
  readonly kmsPublicDecryptEIP712Signatures: readonly Bytes65Hex[];
  readonly extraData: BytesHex;
};
```

## KMS Types

### KmsUserDecryptEIP712

The EIP-712 typed data structure for user decryption permits:

```ts
type KmsUserDecryptEIP712 = {
  readonly domain: KmsEIP712Domain;
  readonly types: KmsUserDecryptEIP712Types;
  readonly message: KmsUserDecryptEIP712Message;
  readonly primaryType: "UserDecryptRequestVerification";
};

type KmsUserDecryptEIP712Message = {
  readonly publicKey: BytesHex;
  readonly contractAddresses: readonly ChecksummedAddress[];
  readonly startTimestamp: string;
  readonly durationDays: string;
  readonly extraData: BytesHex;
};

type KmsEIP712Domain = {
  readonly name: "Decryption";
  readonly version: "1";
  readonly chainId: Uint64BigInt;
  readonly verifyingContract: ChecksummedAddress;
};
```

### KmsDelegatedUserDecryptEIP712

Extends the user decrypt EIP-712 with a `delegatedAccount` field:

```ts
type KmsDelegatedUserDecryptEIP712Message = KmsUserDecryptEIP712Message & {
  readonly delegatedAccount: ChecksummedAddress;
};
```

## Global FHE Public Key Parameters

```ts
type GlobalFhePkeParams = {
  readonly publicKey: GlobalFhePublicKey;  // TFHE public key object
  readonly crs: GlobalFheCrs;              // Common Reference String object
};

type GlobalFhePkeParamsBytes = {
  readonly publicKey: Bytes;    // Raw bytes
  readonly crs: Bytes;
};

type GlobalFhePkeParamsBytesHex = {
  readonly publicKey: BytesHex; // Hex-encoded bytes
  readonly crs: BytesHex;
};
```
