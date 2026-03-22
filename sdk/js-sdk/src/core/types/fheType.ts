////////////////////////////////////////////////////////////////////////////////
//
// FheType
//
////////////////////////////////////////////////////////////////////////////////

import type {
  UintNormalizedMap,
  UintValueTypeMap,
  ValueTypeBitsMap,
  ValueTypeMap,
} from "./primitives.js";

/**
 * **FHE Type Mapping for Input Builders**
 * * Maps the **number of encrypted bits** used by a FHEVM primary type
 * to its corresponding **FheTypeId**. This constant is primarily used by
 * `EncryptedInput` and `RelayerEncryptedInput` builders to determine the correct
 * input type and calculate the total required bit-length.
 *
 * **Structure: \{ Encrypted Bit Length: FheTypeId \}**
 *
 * | Bits | FheTypeId | FHE Type Name | Note |
 * | :--- | :-------- | :------------ | :--- |
 * | 2    | 0         | `ebool`         | The boolean type. |
 * | (N/A)| 1         | `euint4`        | **Deprecated** and omitted from this map. |
 * | 8    | 2         | `euint8`        | |
 * | 16   | 3         | `euint16`       | |
 * | 32   | 4         | `euint32`       | |
 * | 64   | 5         | `euint64`       | |
 * | 128  | 6         | `euint128`      | |
 * | 160  | 7         | `eaddress`      | Used for encrypted Ethereum addresses. |
 * | 256  | 8         | `euint256`      | The maximum supported integer size. |
 */
export type FheType = `e${keyof ValueTypeMap}`;
export type EuintType = keyof UintToEuintMap;
export type FheTypeId = FheTypeToIdMap[keyof FheTypeToIdMap];

export type ValueTypeNameToFheTypeMap = {
  [K in keyof ValueTypeMap]: `e${K}`;
};

export type FheTypeToValueTypeMap = {
  [K in keyof ValueTypeMap as `e${K}`]: ValueTypeMap[K];
};

export type FheTypeToValueTypeNameMap = {
  [K in keyof ValueTypeMap as `e${K}`]: K;
};

export type UintToEuintMap = {
  [K in keyof UintValueTypeMap]: `e${K}`;
};

export type EuintToUintMap = {
  [K in keyof UintToEuintMap as `e${K}`]: K;
};

export type EuintToUintNormalizedMap = {
  [K in keyof UintToEuintMap as `e${K}`]: UintNormalizedMap[K];
};

export type FheTypeToIdMap = Readonly<{
  ebool: 0;
  //euint4: 1; has been deprecated
  euint8: 2;
  euint16: 3;
  euint32: 4;
  euint64: 5;
  euint128: 6;
  eaddress: 7;
  euint256: 8;
}>;

export type FheTypeIdToNameMap = {
  [K in keyof FheTypeToIdMap as FheTypeToIdMap[K]]: K;
};

// ebool is encrypted on 2 bits
export type EncryptionBitsMap = Readonly<
  { ebool: 2 } & {
    [K in keyof Omit<ValueTypeBitsMap, "bool"> as `e${K}`]: ValueTypeBitsMap[K];
  }
>;

export type EncryptionBits = EncryptionBitsMap[keyof EncryptionBitsMap];
export type EncryptionBitsOf<T extends keyof EncryptionBitsMap> =
  EncryptionBitsMap[T];

export type FheTypeIdToEncryptionBitsMap = {
  [K in keyof FheTypeToIdMap as FheTypeToIdMap[K]]: EncryptionBitsMap[K];
};

export type EncryptionBitsToFheTypeIdMap = {
  [K in keyof EncryptionBitsMap as EncryptionBitsMap[K]]: FheTypeToIdMap[K];
};

export type EncryptionBitsToFheTypeMap = {
  [K in keyof EncryptionBitsMap as EncryptionBitsMap[K]]: K;
};

export type SolidityPrimitiveTypeName = "bool" | "uint256" | "address";
