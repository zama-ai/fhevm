import type { UintNormalizedMap, UintValueTypeMap, ValueTypeBitsMap, ValueTypeMap } from './primitives.js';

////////////////////////////////////////////////////////////////////////////////
//
// FheType
//
////////////////////////////////////////////////////////////////////////////////

export type FheTypeToValueTypeMap = {
  readonly ebool: 'bool';
  //euint4 has been deprecated
  readonly euint8: 'uint8';
  readonly euint16: 'uint16';
  readonly euint32: 'uint32';
  readonly euint64: 'uint64';
  readonly euint128: 'uint128';
  readonly euint256: 'uint256';
  readonly eaddress: 'address';
};

// Compile-time proof that keys match FheType exactly.
// Missing or extra keys will cause a TS error.
type AssertFheTypeKeys<T extends Readonly<Record<FheType, unknown>>> = T;

export type FheTypeToIdMap = AssertFheTypeKeys<
  Readonly<{
    ebool: 0;
    //euint4: 1; has been deprecated
    euint8: 2;
    euint16: 3;
    euint32: 4;
    euint64: 5;
    euint128: 6;
    eaddress: 7;
    euint256: 8;
  }>
>;

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
// eslint-disable-next-line @typescript-eslint/no-redundant-type-constituents
export type FheType = keyof FheTypeToValueTypeMap & string;

export type EuintType = keyof UintToEuintMap;
export type FheTypeId = FheTypeToIdMap[keyof FheTypeToIdMap];

// Reverse of FheTypeToValueTypeMap: "bool" → "ebool", "uint8" → "euint8", etc.
export type ValueTypeNameToFheTypeMap = {
  [etype in FheType as FheTypeToValueTypeMap[etype]]: etype;
};

/**
 * The JavaScript type of the decrypted (clear) value for a given FHE type.
 *
 * | FheType      | ClearValueType   |
 * | ------------ | ---------------- |
 * | `"ebool"`    | `boolean`        |
 * | `"euint8"`   | `Uint8Number`    |
 * | `"euint16"`  | `Uint16Number`   |
 * | `"euint32"`  | `Uint32Number`   |
 * | `"euint64"`  | `Uint64BigInt`   |
 * | `"euint128"` | `Uint128BigInt`  |
 * | `"euint256"` | `Uint256BigInt`  |
 * | `"eaddress"` | `Address`        |
 *
 * @typeParam T - The FHE type. Defaults to the union of all clear value types.
 */
export type ClearValueType<etype extends FheType = FheType> = ValueTypeMap[FheTypeToValueTypeMap[etype]];

// Same as ClearValueType but as a full map: "ebool" → boolean, "euint8" → Uint8Number, etc.
export type ClearValueTypeMap = {
  [etype in FheType]: ValueTypeMap[FheTypeToValueTypeMap[etype]];
};

// Same as FheTypeToValueTypeMap (kept for backward compat)
export type FheTypeToValueTypeNameMap = FheTypeToValueTypeMap;

// Uint-only subset: "uint8" → "euint8", etc.
export type UintToEuintMap = {
  [etype in FheType as FheTypeToValueTypeMap[etype] extends keyof UintValueTypeMap
    ? FheTypeToValueTypeMap[etype]
    : never]: etype;
};

// Reverse: "euint8" → "uint8", etc.
export type EuintToUintMap = {
  [K in keyof UintToEuintMap as UintToEuintMap[K]]: K;
};

export type EuintToUintNormalizedMap = {
  [K in keyof UintToEuintMap as UintToEuintMap[K]]: UintNormalizedMap[K];
};

export type FheTypeIdToNameMap = {
  [etype in keyof FheTypeToIdMap as FheTypeToIdMap[etype]]: etype;
};

// ebool is encrypted on 2 bits (not 1)
export type EncryptionBitsMap = Readonly<{
  [etype in FheType]: etype extends 'ebool' ? 2 : ValueTypeBitsMap[FheTypeToValueTypeMap[etype]];
}>;

export type EncryptionBits = EncryptionBitsMap[keyof EncryptionBitsMap];
export type EncryptionBitsOf<etype extends keyof EncryptionBitsMap> = EncryptionBitsMap[etype];

export type FheTypeIdToEncryptionBitsMap = {
  [etype in keyof FheTypeToIdMap as FheTypeToIdMap[etype]]: EncryptionBitsMap[etype];
};

export type EncryptionBitsToFheTypeIdMap = {
  [etype in keyof EncryptionBitsMap as EncryptionBitsMap[etype]]: FheTypeToIdMap[etype];
};

export type EncryptionBitsToFheTypeMap = {
  [etype in keyof EncryptionBitsMap as EncryptionBitsMap[etype]]: etype;
};

export type SolidityPrimitiveTypeName = 'bool' | 'uint256' | 'address';
