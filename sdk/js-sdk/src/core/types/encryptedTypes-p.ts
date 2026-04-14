import type { EncryptionBits, FheType, FheTypeId, FheTypeToIdMap, SolidityPrimitiveTypeName } from './fheType.js';
import type { Bytes21Hex, Bytes32, Bytes32Hex, Bytes32HexNo0x, Uint64BigInt, Uint8Number } from './primitives.js';

////////////////////////////////////////////////////////////////////////////////

// Brand
declare const encryptedValueBrand: unique symbol;
declare const externalBrand: unique symbol;
declare const computedBrand: unique symbol;

export type EncryptedValueBrand = { readonly [encryptedValueBrand]: never };
export type ExternalBrand = { readonly [externalBrand]: never };
export type ComputedBrand = { readonly [computedBrand]: never };

////////////////////////////////////////////////////////////////////////////////

export type InputHandleBytes32 = Bytes32 & EncryptedValueBrand & ExternalBrand;
export type InputHandleBytes32Hex = Bytes32Hex & EncryptedValueBrand & ExternalBrand;
export type InputHandleBytes32HexNo0x = Bytes32HexNo0x & EncryptedValueBrand & ExternalBrand;

////////////////////////////////////////////////////////////////////////////////

export type ComputedHandleBytes32 = Bytes32 & EncryptedValueBrand & ComputedBrand;
export type ComputedHandleBytes32Hex = Bytes32Hex & EncryptedValueBrand & ExternalBrand;
export type ComputedHandleBytes32HexNo0x = Bytes32HexNo0x & EncryptedValueBrand & ExternalBrand;

////////////////////////////////////////////////////////////////////////////////

// Branded bytes
export type HandleBytes32 = ComputedHandleBytes32 | InputHandleBytes32;
export type HandleBytes32Hex = ComputedHandleBytes32Hex | InputHandleBytes32Hex;
export type HandleBytes32HexNo0x = ComputedHandleBytes32HexNo0x | InputHandleBytes32HexNo0x;

////////////////////////////////////////////////////////////////////////////////

// Base interface — class implements this
export interface EncryptedValueBase {
  readonly bytes32Hex: HandleBytes32Hex;
  readonly bytes32: HandleBytes32;
  readonly bytes32HexNo0x: HandleBytes32HexNo0x;
  readonly hash21: Bytes21Hex;
  readonly chainId: Uint64BigInt;
  readonly fheTypeId: FheTypeId;
  readonly fheType: FheType;
  readonly version: Uint8Number;
  readonly index: Uint8Number;
  readonly encryptionBits: EncryptionBits;
  readonly solidityPrimitiveTypeName: SolidityPrimitiveTypeName;
  readonly isComputed: boolean;
  readonly isExternal: boolean;
}

////////////////////////////////////////////////////////////////////////////////

// Typed base
export interface EncryptedValueOfTypeBase<etype extends FheType> extends EncryptedValueBase {
  readonly fheTypeId: FheTypeToIdMap[etype];
  readonly fheType: etype;
}

// Computed typed base
export interface ComputedEncryptedValueOfTypeBase<etype extends FheType> extends EncryptedValueOfTypeBase<etype> {
  readonly bytes32Hex: ComputedHandleBytes32Hex;
  readonly bytes32: ComputedHandleBytes32;
  readonly bytes32HexNo0x: ComputedHandleBytes32HexNo0x;
  readonly isComputed: true;
  readonly isExternal: false;
}

// External typed base
export interface ExternalEncryptedValueOfTypeBase<etype extends FheType> extends EncryptedValueOfTypeBase<etype> {
  readonly bytes32Hex: InputHandleBytes32Hex;
  readonly bytes32: InputHandleBytes32;
  readonly bytes32HexNo0x: InputHandleBytes32HexNo0x;
  readonly index: Uint8Number;
  readonly isComputed: false;
  readonly isExternal: true;
}
