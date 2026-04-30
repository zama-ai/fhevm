# The Problems

## Problem 1: FhevmHandle is not intuitive for Solidity developers.

In FHE Solidity, developers work with euint8, euint16, eaddress, etc. These are encrypted values stored on-chain as bytes32. The SDK parses these bytes32 values into rich objects with metadata (type, chain ID, version, etc.) and calls them FhevmHandle.

The issue: "handle" is a systems programming concept (file handles, resource handles) that means "opaque reference to something." A Solidity developer doesn't think in terms of handles — they think "I have an encrypted uint8." When they see FhevmHandle in the SDK API, the connection to their euint8 is not obvious.

The individual typed aliases (Ebool, Euint8, Eaddress) already match the Solidity naming and are clear. The problem is specifically the generic union type — the name that means "any encrypted value, regardless of type."

We want a public-facing name that a Solidity developer immediately understands as "the SDK representation of my encrypted value."

## Problem 2: Two kinds of encrypted values with identical bytes32 representation.

On-chain, a `euint8` and an `externalEuint8` share the same `bytes32` encoding — the bits are identical. The difference is a lifecycle stage:

1. External (externalEuint8) — freshly encrypted by the SDK, not yet verified on-chain. The encrypt function returns these.
2. Verified (euint8) — the same bytes32 after it has been verified on-chain via InputVerifier.sol and a valid inputProof. The decrypt function expects these.

The transition from external to verified happens on-chain (Solidity cast), not in the SDK. The SDK never sees this transition — it only knows which kind it created or received.

This distinction matters for type safety: the SDK should prevent passing an unverified external value to decrypt, and should make it clear that encrypt returns values that still need on-chain verification before they can be used in FHE computations or decrypted.

# FHE.sol Lexic

Argument names per type:

- inputHandle = externalEuintXXX, externalEbool, externalEaddress
- inputProof
- value = euintXXX, ebool, eaddress
- handle = to qualify any possible exxxx value

# The Solution

Use explicit union for documentation purpose

```ts
// any Handle
export type Handle<T extends FheType = FheType> = EncryptedValue<T>;
// any InputHandle
export type InputHandle<T extends FheType = FheType> = ExternalEncryptedValue<T>;
```

```ts
export type Ebool = EncryptedValue<'ebool'>;
export type Euint8 = EncryptedValue<'euint8'>;
export type Euint16 = EncryptedValue<'euint16'>;
export type Euint32 = EncryptedValue<'euint32'>;
export type Euint64 = EncryptedValue<'euint64'>;
export type Euint128 = EncryptedValue<'euint128'>;
export type Euint256 = EncryptedValue<'euint256'>;
export type Eaddress = EncryptedValue<'eaddress'>;

export type ExternalEbool = ExternalEncryptedValue<'ebool'>;
export type ExternalEuint8 = ExternalEncryptedValue<'euint8'>;
export type ExternalEuint16 = ExternalEncryptedValue<'euint16'>;
export type ExternalEuint32 = ExternalEncryptedValue<'euint32'>;
export type ExternalEuint64 = ExternalEncryptedValue<'euint64'>;
export type ExternalEuint128 = ExternalEncryptedValue<'euint128'>;
export type ExternalEuint256 = ExternalEncryptedValue<'euint256'>;
export type ExternalEaddress = ExternalEncryptedValue<'eaddress'>;

export type ClearBool = ClearValue<'ebool'>;
export type ClearUint8 = ClearValue<'euint8'>;
export type ClearUint16 = ClearValue<'euint16'>;
export type ClearUint32 = ClearValue<'euint32'>;
export type ClearUint64 = ClearValue<'euint64'>;
export type ClearUint128 = ClearValue<'euint128'>;
export type ClearUint256 = ClearValue<'euint256'>;
export type ClearAddress = ClearValue<'eaddress'>;
```

## Decrypt

```ts
type EncryptedValueArray = ReadonlyArray<{
  readonly value: EncryptedValue;
  readonly contractAddress: ChecksummedAddress;
}>;

export type DecryptParameters =
  | {
      readonly encryptedValues: EncryptedValueArray;
      readonly signedPermit: SignedSelfDecryptionPermit;
      readonly e2eTransportKeyPair: E2eTransportKeyPair;
      readonly options?: RelayerUserDecryptOptions | undefined;
    }
  | {
      readonly encryptedValues: EncryptedValueArray;
      readonly signedPermit: SignedDelegatedDecryptionPermit;
      readonly e2eTransportKeyPair: E2eTransportKeyPair;
      readonly options?: RelayerDelegatedUserDecryptOptions | undefined;
    };

export type DecryptReturnType = readonly ClearValue[];
```

## Existing

```ts
// eslint-disable-next-line @typescript-eslint/naming-convention
declare const __fhevmHandle: unique symbol;

export type FhevmHandleBrand = { readonly [__fhevmHandle]: never };
export type FhevmHandleBytes32 = Bytes32 & FhevmHandleBrand;
export type FhevmHandleBytes32Hex = Bytes32Hex & FhevmHandleBrand;
export type FhevmHandleBytes32HexNo0x = Bytes32HexNo0x & FhevmHandleBrand;

export interface FhevmHandleBytes32HexAble {
  // Core canonical representation
  readonly bytes32Hex: FhevmHandleBytes32Hex;
}

export interface FhevmHandleBytes32Able {
  // Core canonical representation
  readonly bytes32: FhevmHandleBytes32;
}

export interface FhevmHandleBase extends FhevmHandleBytes32HexAble, FhevmHandleBytes32Able {
  // Alternate representations
  readonly bytes32HexNo0x: FhevmHandleBytes32HexNo0x;
  readonly bytes32: FhevmHandleBytes32;

  // Parsed components
  readonly hash21: Bytes21Hex;
  readonly chainId: Uint64BigInt;
  readonly fheTypeId: FheTypeId;
  readonly fheType: FheType;
  readonly version: Uint8Number;
  readonly index: Uint8Number | undefined;
  readonly encryptionBits: EncryptionBits;
  readonly solidityPrimitiveTypeName: SolidityPrimitiveTypeName;
  readonly isComputed: boolean;
  readonly isExternal: boolean;
}

////////////////////////////////////////////////////////////////////////////////

export interface FhevmHandleOfTypeBase<T extends FheType> extends FhevmHandleBase {
  readonly fheTypeId: FheTypeToIdMap[T];
  readonly fheType: T;
}

export type FhevmHandleOfType<T extends FheType = FheType> = {
  [K in T]: FhevmHandleOfTypeBase<K>;
}[T];

////////////////////////////////////////////////////////////////////////////////

export interface FhevmExternalHandleOfTypeBase<T extends FheType> extends FhevmHandleOfTypeBase<T> {
  readonly index: Uint8Number;
  readonly isComputed: false;
  readonly isExternal: true;
}

export type FhevmExternalHandleOfType<T extends FheType = FheType> = {
  [K in T]: FhevmExternalHandleOfTypeBase<K>;
}[T];

////////////////////////////////////////////////////////////////////////////////

export type FhevmHandleLike = Bytes32 | Bytes32Hex | Bytes32HexAble | FhevmHandle;

////////////////////////////////////////////////////////////////////////////////

export type Ebool = FhevmHandleOfType<'ebool'>;
export type Euint8 = FhevmHandleOfType<'euint8'>;
export type Euint16 = FhevmHandleOfType<'euint16'>;
export type Euint32 = FhevmHandleOfType<'euint32'>;
export type Euint64 = FhevmHandleOfType<'euint64'>;
export type Euint128 = FhevmHandleOfType<'euint128'>;
export type Euint256 = FhevmHandleOfType<'euint256'>;
export type Eaddress = FhevmHandleOfType<'eaddress'>;

export type FhevmHandle = FhevmHandleOfType;

////////////////////////////////////////////////////////////////////////////////

export type ExternalEbool = FhevmExternalHandleOfType<'ebool'>;
export type ExternalEuint8 = FhevmExternalHandleOfType<'euint8'>;
export type ExternalEuint16 = FhevmExternalHandleOfType<'euint16'>;
export type ExternalEuint32 = FhevmExternalHandleOfType<'euint32'>;
export type ExternalEuint64 = FhevmExternalHandleOfType<'euint64'>;
export type ExternalEuint128 = FhevmExternalHandleOfType<'euint128'>;
export type ExternalEuint256 = FhevmExternalHandleOfType<'euint256'>;
export type ExternalEaddress = FhevmExternalHandleOfType<'eaddress'>;

export type ExternalFhevmHandle = FhevmExternalHandleOfType;
```

An EncryptedValue in the FHEVM ecosystem is a bytes32 whose access is governed by permission rules.
An EncryptedValue cannot exist alone, it has to be attached to at least one smart contract.
Multiple smart contracts can have access to the same given encrypted value.
Multiple users off-chain can have access to the same given encrypted value.

- a user off-chain can decrypt a pair [EncryptedValue, Contract] if and only if:
  1. the Contract has permission
  2. the user has permission

Consequently the `decrypt` function takes an array of pairs [EncryptedValue, Contract]
