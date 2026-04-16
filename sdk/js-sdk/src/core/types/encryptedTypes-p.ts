import type {
  EncryptionBits,
  FheType,
  FheTypeId,
  FheTypeToIdMap,
  FheTypeToValueTypeNameMap,
  SolidityPrimitiveTypeName,
} from './fheType.js';
import type {
  Bytes21Hex,
  Bytes32,
  Bytes32Hex,
  Bytes32HexNo0x,
  TypedValueOfBase,
  Uint64BigInt,
  Uint8Number,
  ValueTypeName,
} from './primitives.js';

////////////////////////////////////////////////////////////////////////////////

// Brand
declare const encryptedValueBrand: unique symbol;
declare const inputBrand: unique symbol;
declare const computedBrand: unique symbol;

export type EncryptedValueBrand = { readonly [encryptedValueBrand]: never };
export type InputBrand = { readonly [inputBrand]: never };
export type ComputedBrand = { readonly [computedBrand]: never };

////////////////////////////////////////////////////////////////////////////////

export type InputHandleBytes32 = Bytes32 & EncryptedValueBrand & InputBrand;
export type InputHandleBytes32Hex = Bytes32Hex & EncryptedValueBrand & InputBrand;
export type InputHandleBytes32HexNo0x = Bytes32HexNo0x & EncryptedValueBrand & InputBrand;

////////////////////////////////////////////////////////////////////////////////

export type ComputedHandleBytes32 = Bytes32 & EncryptedValueBrand & ComputedBrand;
export type ComputedHandleBytes32Hex = Bytes32Hex & EncryptedValueBrand & ComputedBrand;
export type ComputedHandleBytes32HexNo0x = Bytes32HexNo0x & EncryptedValueBrand & ComputedBrand;

////////////////////////////////////////////////////////////////////////////////

// Branded bytes
export type HandleBytes32 = ComputedHandleBytes32 | InputHandleBytes32;
export type HandleBytes32Hex = ComputedHandleBytes32Hex | InputHandleBytes32Hex;
export type HandleBytes32HexNo0x = ComputedHandleBytes32HexNo0x | InputHandleBytes32HexNo0x;

////////////////////////////////////////////////////////////////////////////////

// Base interface — class implements this
export interface HandleBaseV0 {
  readonly bytes32Hex: HandleBytes32Hex;
  readonly bytes32: HandleBytes32;
  readonly bytes32HexNo0x: HandleBytes32HexNo0x;
  readonly hash21: Bytes21Hex;
  readonly chainId: Uint64BigInt;
  readonly fheTypeId: FheTypeId;
  readonly fheType: FheType;
  readonly clearType: ValueTypeName;
  readonly version: Uint8Number;
  readonly index: Uint8Number;
  readonly encryptionBits: EncryptionBits;
  readonly solidityPrimitiveTypeName: SolidityPrimitiveTypeName;
  readonly isComputed: boolean;
  readonly isExternal: boolean;
}

////////////////////////////////////////////////////////////////////////////////

// Typed base
export interface HandleOfTypeBaseV0<etype extends FheType> extends HandleBaseV0 {
  readonly fheTypeId: FheTypeToIdMap[etype];
  readonly fheType: etype;
}

// Computed typed base
export interface ComputedHandleOfTypeBaseV0<etype extends FheType> extends HandleOfTypeBaseV0<etype> {
  readonly bytes32Hex: ComputedHandleBytes32Hex;
  readonly bytes32: ComputedHandleBytes32;
  readonly bytes32HexNo0x: ComputedHandleBytes32HexNo0x;
  readonly isComputed: true;
  readonly isExternal: false;
}

// External typed base
export interface InputHandleOfTypeBaseV0<etype extends FheType> extends HandleOfTypeBaseV0<etype> {
  readonly bytes32Hex: InputHandleBytes32Hex;
  readonly bytes32: InputHandleBytes32;
  readonly bytes32HexNo0x: InputHandleBytes32HexNo0x;
  readonly index: Uint8Number;
  readonly isComputed: false;
  readonly isExternal: true;
}

////////////////////////////////////////////////////////////////////////////////
// Public types
////////////////////////////////////////////////////////////////////////////////

/**
 * An encrypted FHE value (`handle` in `FHE.sol` / FHEVM whitepaper).
 * Either a {@link ComputedHandle} (verified, on-chain) or an
 * {@link InputHandle} (unverified input). Narrowable via `isExternal`.
 */
export type Handle<etype extends FheType = FheType> = ComputedHandle<etype> | InputHandle<etype>;

/** A computed encrypted value — verified on-chain, result of an FHE operation. */
export type ComputedHandle<etype extends FheType = FheType> = {
  [K in etype]: ComputedHandleOfTypeBaseV0<K>;
}[etype];

/** An unverified encrypted value (`inputHandle` in `FHE.sol`). */
export type InputHandle<etype extends FheType = FheType> = {
  [K in etype]: InputHandleOfTypeBaseV0<K>;
}[etype];

// /**
//  * Alias for {@link EncryptedValuePipo} using `FHE.sol` terminology.
//  * In `FHE.sol`, a `handle` is the `bytes32` reference to any encrypted value.
//  */
// export type HandlePipo<etype extends FheType = FheType> = EncryptedValuePipo<etype>;

// /**
//  * Alias for {@link ExternalEncryptedValuePipo} using `FHE.sol` terminology.
//  * In `FHE.sol`, an `inputHandle` is an encrypted value that has not yet been
//  * verified on-chain via `InputVerifier.sol`.
//  */
// export type InputHandlePipo<etype extends FheType = FheType> = ExternalEncryptedValuePipo<etype>;

// /** Alias for {@link ComputedEncryptedValuePipo} using `FHE.sol` terminology. */
// export type ComputedHandlePipo<etype extends FheType = FheType> = ComputedEncryptedValuePipo<etype>;

// ////////////////////////////////////////////////////////////////////////////////
// // Typed shortcuts
// ////////////////////////////////////////////////////////////////////////////////

// /** Encrypted boolean (`ebool` in Solidity). */
// export type EboolPipo = EncryptedValuePipo<'ebool'>;
// /** Encrypted unsigned 8-bit integer (`euint8` in Solidity). */
// export type Euint8Pipo = EncryptedValuePipo<'euint8'>;
// /** Encrypted unsigned 16-bit integer (`euint16` in Solidity). */
// export type Euint16Pipo = EncryptedValuePipo<'euint16'>;
// /** Encrypted unsigned 32-bit integer (`euint32` in Solidity). */
// export type Euint32Pipo = EncryptedValuePipo<'euint32'>;
// /** Encrypted unsigned 64-bit integer (`euint64` in Solidity). */
// export type Euint64Pipo = EncryptedValuePipo<'euint64'>;
// /** Encrypted unsigned 128-bit integer (`euint128` in Solidity). */
// export type Euint128Pipo = EncryptedValuePipo<'euint128'>;
// /** Encrypted unsigned 256-bit integer (`euint256` in Solidity). */
// export type Euint256Pipo = EncryptedValuePipo<'euint256'>;
// /** Encrypted address (`eaddress` in Solidity). */
// export type EaddressPipo = EncryptedValuePipo<'eaddress'>;

// /** Unverified encrypted boolean (`externalEbool` in Solidity). Requires on-chain verification before use. */
// export type ExternalEboolPipo = ExternalEncryptedValuePipo<'ebool'>;
// /** Unverified encrypted unsigned 8-bit integer (`externalEuint8` in Solidity). */
// export type ExternalEuint8Pipo = ExternalEncryptedValuePipo<'euint8'>;
// /** Unverified encrypted unsigned 16-bit integer (`externalEuint16` in Solidity). */
// export type ExternalEuint16Pipo = ExternalEncryptedValuePipo<'euint16'>;
// /** Unverified encrypted unsigned 32-bit integer (`externalEuint32` in Solidity). */
// export type ExternalEuint32Pipo = ExternalEncryptedValuePipo<'euint32'>;
// /** Unverified encrypted unsigned 64-bit integer (`externalEuint64` in Solidity). */
// export type ExternalEuint64Pipo = ExternalEncryptedValuePipo<'euint64'>;
// /** Unverified encrypted unsigned 128-bit integer (`externalEuint128` in Solidity). */
// export type ExternalEuint128Pipo = ExternalEncryptedValuePipo<'euint128'>;
// /** Unverified encrypted unsigned 256-bit integer (`externalEuint256` in Solidity). */
// export type ExternalEuint256Pipo = ExternalEncryptedValuePipo<'euint256'>;
// /** Unverified encrypted address (`externalEaddress` in Solidity). */
// export type ExternalEaddressPipo = ExternalEncryptedValuePipo<'eaddress'>;

// ////////////////////////////////////////////////////////////////////////////////

/**
 * Any value that can be interpreted as an encrypted value (bytes32 handle).
 *
 * - `Uint8Array` — raw 32-byte handle (`Bytes32`)
 * - `string` — 0x-prefixed hex-encoded 32-byte handle (`Bytes32Hex`, e.g. `"0xabcd..."`)
 * - `{ bytes32Hex: string }` — object with a hex-encoded handle property
 * - `EncryptedValue` — an already-parsed encrypted value
 */
export type EncryptedValueLike = Uint8Array | string | { readonly bytes32Hex: string };

// export type HandleLike = EncryptedValueLike;

// /**
//  * Any value that can be interpreted as an external encrypted value (bytes32 input handle).
//  * An input handle is a user-encrypted value that has not yet been verified on-chain via `InputVerifier.sol`.
//  *
//  * - `Uint8Array` — raw 32-byte handle (`Bytes32`)
//  * - `string` — 0x-prefixed hex-encoded 32-byte handle (`Bytes32Hex`, e.g. `"0xabcd..."`)
//  * - `{ bytes32Hex: string }` — object with a hex-encoded handle property
//  * - `ExternalEncryptedValue` — an already-parsed external encrypted value
//  */
// export type ExternalEncryptedValueLike =
//   | Uint8Array
//   | string
//   | { readonly bytes32Hex: string }
//   | ExternalEncryptedValuePipo;

// export type InputHandleLike = ExternalEncryptedValueLike;

// ////////////////////////////////////////////////////////////////////////////////

export type ClearValueOfFheType<etype extends FheType> = TypedValueOfBase<ClearValueTypeName<etype>> & {
  readonly handle: Handle<etype>;
};

/**
 * The decrypted clear value of an FHE encrypted value.
 * @typeParam T - The FHE type (e.g. `"euint8"`, `"ebool"`). Defaults to all types.
 */
export type ClearValue<etype extends FheType = FheType> = {
  [K in etype]: ClearValueOfFheType<K>;
}[etype];

export type ClearValueTypeName<etype extends FheType = FheType> = FheTypeToValueTypeNameMap[etype];

export type ClearBool = ClearValue<'ebool'>;
export type ClearUint8 = ClearValue<'euint8'>;
export type ClearUint16 = ClearValue<'euint16'>;
export type ClearUint32 = ClearValue<'euint32'>;
export type ClearUint64 = ClearValue<'euint64'>;
export type ClearUint128 = ClearValue<'euint128'>;
export type ClearUint256 = ClearValue<'euint256'>;
export type ClearAddress = ClearValue<'eaddress'>;
