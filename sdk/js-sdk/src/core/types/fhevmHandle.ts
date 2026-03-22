import type {
  EncryptionBits,
  FheType,
  FheTypeId,
  FheTypeToIdMap,
  SolidityPrimitiveTypeName,
} from "./fheType.js";
import type {
  Bytes21Hex,
  Bytes32,
  Bytes32Hex,
  Bytes32HexAble,
  Bytes32HexNo0x,
  Uint64BigInt,
  Uint8Number,
} from "./primitives.js";

////////////////////////////////////////////////////////////////////////////////
//
// FhevmHandle
//
////////////////////////////////////////////////////////////////////////////////

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

export interface FhevmHandleBase
  extends FhevmHandleBytes32HexAble,
    FhevmHandleBytes32Able {
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

export interface FhevmHandleOfTypeBase<T extends FheType>
  extends FhevmHandleBase {
  readonly fheTypeId: FheTypeToIdMap[T];
  readonly fheType: T;
}

export type FhevmHandleOfType<T extends FheType = FheType> = {
  [K in T]: FhevmHandleOfTypeBase<K>;
}[T];

////////////////////////////////////////////////////////////////////////////////

export interface FhevmExternalHandleOfTypeBase<T extends FheType>
  extends FhevmHandleOfTypeBase<T> {
  readonly index: Uint8Number;
  readonly isComputed: false;
  readonly isExternal: true;
}

export type FhevmExternalHandleOfType<T extends FheType = FheType> = {
  [K in T]: FhevmExternalHandleOfTypeBase<K>;
}[T];

////////////////////////////////////////////////////////////////////////////////

export type FhevmHandleLike =
  | Bytes32
  | Bytes32Hex
  | Bytes32HexAble
  | FhevmHandle;

////////////////////////////////////////////////////////////////////////////////

export type Ebool = FhevmHandleOfType<"ebool">;
export type Euint8 = FhevmHandleOfType<"euint8">;
export type Euint16 = FhevmHandleOfType<"euint16">;
export type Euint32 = FhevmHandleOfType<"euint32">;
export type Euint64 = FhevmHandleOfType<"euint64">;
export type Euint128 = FhevmHandleOfType<"euint128">;
export type Euint256 = FhevmHandleOfType<"euint256">;
export type Eaddress = FhevmHandleOfType<"eaddress">;

export type FhevmHandle = FhevmHandleOfType;

////////////////////////////////////////////////////////////////////////////////

export type ExternalEbool = FhevmExternalHandleOfType<"ebool">;
export type ExternalEuint8 = FhevmExternalHandleOfType<"euint8">;
export type ExternalEuint16 = FhevmExternalHandleOfType<"euint16">;
export type ExternalEuint32 = FhevmExternalHandleOfType<"euint32">;
export type ExternalEuint64 = FhevmExternalHandleOfType<"euint64">;
export type ExternalEuint128 = FhevmExternalHandleOfType<"euint128">;
export type ExternalEuint256 = FhevmExternalHandleOfType<"euint256">;
export type ExternalEaddress = FhevmExternalHandleOfType<"eaddress">;

export type ExternalFhevmHandle = FhevmExternalHandleOfType;
