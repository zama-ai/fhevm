////////////////////////////////////////////////////////////////////////////////
//
// DecryptedFheValue
//
////////////////////////////////////////////////////////////////////////////////

import type { FheType, FheTypeToValueTypeMap } from "./fheType.js";

/**
 * Maps each {@link FheTypeName} to its branded clear-value type.
 *
 * - Small unsigned integers (8, 16, 32 bits) → `UintXXNumber` (fits in JS `number`)
 * - Large unsigned integers (64, 128, 256 bits) → `UintXXBigInt` (requires `bigint`)
 * - `ebool` → `boolean`
 * - `eaddress` → `ChecksummedAddress`
 */
export type DecryptedFheValueMap = FheTypeToValueTypeMap;

/**
 * Union of all branded clear-value types from {@link DecryptedFheValueMap}.
 * @internal
 */
export type DecryptedFheValue = DecryptedFheValueMap[FheType];

export type DecryptedEboolValue = DecryptedFheValueMap["ebool"];
export type DecryptedEaddressValue = DecryptedFheValueMap["eaddress"];
export type DecryptedEuint8Value = DecryptedFheValueMap["euint8"];
export type DecryptedEuint16Value = DecryptedFheValueMap["euint16"];
export type DecryptedEuint32Value = DecryptedFheValueMap["euint32"];
export type DecryptedEuint64Value = DecryptedFheValueMap["euint64"];
export type DecryptedEuint128Value = DecryptedFheValueMap["euint128"];
export type DecryptedEuint256Value = DecryptedFheValueMap["euint256"];
