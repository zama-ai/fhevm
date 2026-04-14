import type {
  ClearValueType,
  EncryptionBits,
  EncryptionBitsToFheTypeIdMap,
  EuintToUintNormalizedMap,
  FheType,
  FheTypeId,
  FheTypeIdToEncryptionBitsMap,
  FheTypeIdToNameMap,
  FheTypeToIdMap,
  SolidityPrimitiveTypeName,
  ValueTypeNameToFheTypeMap,
} from '../types/fheType.js';
import type {
  Address,
  Bytes,
  Uint128BigInt,
  Uint16Number,
  Uint256BigInt,
  Uint32Number,
  Uint64BigInt,
  Uint8Number,
} from '../types/primitives.js';
import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type { ValueTypeName } from '../types/primitives.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';
import { assert } from '../base/errors/InternalError.js';
import {
  assertIsUint,
  assertIsUintBigInt,
  assertIsUintNumber,
  MAX_UINT128,
  MAX_UINT16,
  MAX_UINT256,
  MAX_UINT32,
  MAX_UINT64,
  MAX_UINT8,
  MAX_UINT_FOR_TYPE,
} from '../base/uint.js';
import { bigIntToBytesHex, bytesToBigInt } from '../base/bytes.js';
import { addressToChecksummedAddress, asAddress, assertIsAddress } from '../base/address.js';
import { assertNever } from '../base/errors/utils.js';
import { asBoolean } from '../base/boolean.js';

////////////////////////////////////////////////////////////////////////////////

// TFHE encryption requires a minimum of 2 bits per value.
// Booleans use 2 bits despite only needing 1 bit for the value itself.
const MINIMUM_ENCRYPTION_BIT_WIDTH = 2;

////////////////////////////////////////////////////////////////////////////////
// Lookup Maps
////////////////////////////////////////////////////////////////////////////////

const FheTypeNameToId: FheTypeToIdMap = {
  ebool: 0,
  //euint4: 1, has been deprecated
  euint8: 2,
  euint16: 3,
  euint32: 4,
  euint64: 5,
  euint128: 6,
  eaddress: 7,
  euint256: 8,
} as const;

const FheTypeIdToName: FheTypeIdToNameMap = {
  0: 'ebool',
  //1: 'euint4', has been deprecated
  2: 'euint8',
  3: 'euint16',
  4: 'euint32',
  5: 'euint64',
  6: 'euint128',
  7: 'eaddress',
  8: 'euint256',
} as const;

const ValueTypeNameToFheTypeName: ValueTypeNameToFheTypeMap = {
  bool: 'ebool',
  uint8: 'euint8',
  uint16: 'euint16',
  uint32: 'euint32',
  uint64: 'euint64',
  uint128: 'euint128',
  uint256: 'euint256',
  address: 'eaddress',
} as const;

// TFHE encryption requires a minimum of 2 bits per value.
// Booleans use 2 bits despite only needing 1 bit for the value itself.
const FheTypeIdToEncryptionBits: FheTypeIdToEncryptionBitsMap = {
  0: 2,
  //1:?, euint4 has been deprecated
  2: 8,
  3: 16,
  4: 32,
  5: 64,
  6: 128,
  7: 160,
  8: 256,
} as const;

const EncryptionBitwidthToFheTypeId: EncryptionBitsToFheTypeIdMap = {
  2: 0,
  //?:1, euint4 has been deprecated
  8: 2,
  16: 3,
  32: 4,
  64: 5,
  128: 6,
  160: 7,
  256: 8,
} as const;

const FheTypeIdToSolidityPrimitiveTypeName: Readonly<Record<FheTypeId, SolidityPrimitiveTypeName>> = {
  0: 'bool',
  //1:'uint256', euint4 has been deprecated
  2: 'uint256',
  3: 'uint256',
  4: 'uint256',
  5: 'uint256',
  6: 'uint256',
  7: 'address',
  8: 'uint256',
} as const;

const FheTypeToMaxValue: Readonly<EuintToUintNormalizedMap> = {
  euint8: MAX_UINT8 as Uint8Number,
  euint16: MAX_UINT16 as Uint16Number,
  euint32: MAX_UINT32 as Uint32Number,
  euint64: MAX_UINT64 as Uint64BigInt,
  euint128: MAX_UINT128 as Uint128BigInt,
  euint256: MAX_UINT256 as Uint256BigInt,
} as const;

Object.freeze(FheTypeNameToId);
Object.freeze(FheTypeIdToEncryptionBits);
Object.freeze(EncryptionBitwidthToFheTypeId);
Object.freeze(FheTypeIdToSolidityPrimitiveTypeName);
Object.freeze(FheTypeToMaxValue);

////////////////////////////////////////////////////////////////////////////////
// Type Guards
////////////////////////////////////////////////////////////////////////////////

/**
 * Checks if a value is a valid FheTypeId.
 * @example isFheTypeId(2) // true (euint8)
 * @example isFheTypeId(1) // false (euint4 is deprecated)
 */
export function isFheTypeId(value: unknown): value is FheTypeId {
  if (typeof value !== 'number') {
    return false;
  }
  return value in FheTypeIdToName;
}

/**
 * Asserts that a value is a valid FheTypeId.
 * @throws A {@link InvalidTypeError} If value is not a valid FheTypeId.
 * @example assertIsFheTypeId(2) // passes
 * @example assertIsFheTypeId(1) // throws (deprecated)
 */
export function assertIsFheTypeId(
  value: unknown,
  options: ErrorMetadataParams & { subject?: string },
): asserts value is FheTypeId {
  if (!isFheTypeId(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        expectedType: Object.keys(FheTypeIdToName).join('|'),
      },
      options,
    );
  }
}

/**
 * Try to cast a value to FheTypeId, throwing if invalid.
 * @throws A {@link InvalidTypeError} If value is not a valid FheTypeId.
 * @example const name = asFheTypeId(2) // 2 as FheTypeId
 * @example const name = asFheTypeId(1) // throws (deprecated)
 */
export function asFheTypeId(value: unknown): FheTypeId {
  assertIsFheTypeId(value, {});
  return value;
}

/**
 * Checks if a value is a valid FheType.
 * @example isFheType('euint8') // true
 * @example isFheType('euint4') // false (deprecated)
 */
export function isFheType(value: unknown): value is FheType {
  if (typeof value !== 'string') {
    return false;
  }
  return value in FheTypeNameToId;
}

/**
 * Asserts that a value is a valid FheType.
 * @throws A {@link InvalidTypeError} If value is not a valid FheType.
 * @example assertIsFheType('euint8') // passes
 * @example assertIsFheType('euint4') // throws (deprecated)
 */
export function assertIsFheType(
  value: unknown,
  options: ErrorMetadataParams & { subject?: string },
): asserts value is FheType {
  if (!isFheType(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        expectedType: Object.keys(FheTypeNameToId).join('|'),
      },
      options,
    );
  }
}

/**
 * Try to cast a value to FheType, throwing if invalid.
 * @throws A {@link InvalidTypeError} If value is not a valid FheType.
 * @example const name = asFheType('euint8') // 'euint8' as FheType
 * @example const name = asFheType('euint4') // throws (deprecated)
 */
export function asFheType(value: unknown): FheType {
  assertIsFheType(value, {});
  return value;
}

/**
 * Checks if a value is a valid encryption bit width.
 * @example isEncryptionBits(8) // true
 * @example isEncryptionBits(4) // false (euint4 is deprecated)
 */
export function isEncryptionBits(value: unknown): value is EncryptionBits {
  if (typeof value !== 'number') {
    return false;
  }
  return value in EncryptionBitwidthToFheTypeId;
}

/**
 * Try to cast a value to EncryptionBits, throwing if invalid.
 * @throws A {@link InvalidTypeError} If value is not a valid encryption bit width.
 * @example const b8 = asEncryptionBits(8) // 8 as EncryptionBits
 * @example const b4 = asEncryptionBits(4) // throws (euint4 is deprecated)
 */
export function asEncryptionBits(value: unknown): EncryptionBits {
  assertIsEncryptionBits(value, {});
  return value;
}

/**
 * Asserts that a value is a valid encryption bit width.
 * @throws A {@link InvalidTypeError} If value is not a valid encryption bit width.
 * @example assertIsEncryptionBits(8) // passes
 * @example assertIsEncryptionBits(4) // throws (euint4 is deprecated)
 */
export function assertIsEncryptionBits(
  value: unknown,
  options: ErrorMetadataParams & { subject?: string },
): asserts value is EncryptionBits {
  if (!isEncryptionBits(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        expectedType: Object.keys(EncryptionBitwidthToFheTypeId).join('|'),
      },
      options,
    );
  }
}

/**
 * Asserts that a value is a valid encryption bit width.
 * @throws A {@link InvalidTypeError} If value is not a valid encryption bit width.
 * @example assertIsEncryptionBits(8) // passes
 * @example assertIsEncryptionBits(4) // throws (euint4 is deprecated)
 */
export function assertIsEncryptionBitsArray(
  value: unknown,
  options: ErrorMetadataParams & { subject?: string },
): asserts value is EncryptionBits[] {
  if (!Array.isArray(value)) {
    throw new InvalidTypeError(
      {
        type: typeof value,
        expectedType: 'EncryptionBits[]',
      },
      options,
    );
  }
  for (let i = 0; i < value.length; ++i) {
    if (!isEncryptionBits(value[i])) {
      throw new InvalidTypeError(
        {
          subject: options.subject,
          index: i,
          type: typeof value[i],
          expectedType: 'EncryptionBits',
        },
        options,
      );
    }
  }
}

////////////////////////////////////////////////////////////////////////////////
// FheTypeId extractors
////////////////////////////////////////////////////////////////////////////////

/**
 * Converts an encryption bit width to its corresponding FheTypeId.
 * Accepts loose `number` input; validates internally via `isEncryptionBits`.
 * @throws A {@link FheTypeError} If bitwidth is not a valid encryption bit width.
 * @example fheTypeIdFromEncryptionBits(8) // 2 (euint8)
 */
export function fheTypeIdFromEncryptionBits(bitwidth: EncryptionBits): FheTypeId {
  return EncryptionBitwidthToFheTypeId[bitwidth];
}

/**
 * Converts an FheType to its corresponding FheTypeId.
 * Accepts loose `string` input; validates internally via `isFheType`.
 * @throws A {@link FheTypeError} If name is not a valid FheType.
 * @example fheTypeIdFromName('euint8') // 2
 */
export function fheTypeIdFromName(name: FheType): FheTypeId {
  return FheTypeNameToId[name];
}

/**
 * Converts an FheTypeId to its corresponding FheType.
 * @throws A {@link FheTypeError} If id is not a valid FheTypeId.
 * @example fheTypeNameFromId(2) // 'euint8'
 */
export function fheTypeNameFromId(typeId: FheTypeId): FheType {
  return FheTypeIdToName[typeId];
}

/**
 * Converts a TypeName to its corresponding FheType.
 * @throws A {@link FheTypeError} If id is not a valid FheTypeId.
 * @example fheTypeNameFromTypeName('uint8') // 'euint8'
 */
export function fheTypeNameFromTypeName(typeName: ValueTypeName): FheType {
  return ValueTypeNameToFheTypeName[typeName];
}

////////////////////////////////////////////////////////////////////////////////
// Solidity primitive type names
////////////////////////////////////////////////////////////////////////////////

/**
 * Returns the Solidity primitive type name for an FheTypeId.
 * Accepts loose `number` input; validates internally via `isFheTypeId`.
 * @example solidityPrimitiveTypeNameFromFheTypeId(0) // 'bool'
 * @example solidityPrimitiveTypeNameFromFheTypeId(7) // 'address'
 * @example solidityPrimitiveTypeNameFromFheTypeId(2) // 'uint256'
 */
export function solidityPrimitiveTypeNameFromFheTypeId(typeId: FheTypeId): SolidityPrimitiveTypeName {
  return FheTypeIdToSolidityPrimitiveTypeName[typeId];
}

////////////////////////////////////////////////////////////////////////////////
// Encryption Bits
////////////////////////////////////////////////////////////////////////////////

/**
 * Returns the encryption bit width for an FheTypeId.
 * @param typeId - The FHE type Id
 * @returns The encryption bit width (always \>= 2)
 * @example encryptionBitsFromFheTypeId(2) // 8 (euint8)
 * @example encryptionBitsFromFheTypeId(7) // 160 (eaddress)
 */
export function encryptionBitsFromFheTypeId(typeId: FheTypeId): EncryptionBits {
  const bw = FheTypeIdToEncryptionBits[typeId];

  // Invariant: bit width must be >= 2 (TFHE minimum encryption granularity)
  _assertMinimumEncryptionBitWidth(bw);

  return bw;
}

/**
 * Returns the encryption bit width for an FheType name.
 * @param name - The FHE type name (e.g., 'ebool', 'euint32', 'eaddress')
 * @returns The encryption bit width (always \>= 2)
 * @example encryptionBitsFromFheTypeName('ebool') // 2
 * @example encryptionBitsFromFheTypeName('euint32') // 32
 * @example encryptionBitsFromFheTypeName('eaddress') // 160
 */
export function encryptionBitsFromFheType(name: FheType): EncryptionBits {
  const bw = FheTypeIdToEncryptionBits[FheTypeNameToId[name]];

  // Invariant: bit width must be >= 2 (TFHE minimum encryption granularity)
  _assertMinimumEncryptionBitWidth(bw);

  return bw;
}

function _assertMinimumEncryptionBitWidth(bw: number): void {
  assert(
    bw >= MINIMUM_ENCRYPTION_BIT_WIDTH,
    `Invalid FheType encryption bit width: ${bw}. Minimum encryption bit width is ${MINIMUM_ENCRYPTION_BIT_WIDTH} bits.`,
  );
}

export function bytesToClearValueType<etype extends FheType>(fheType: etype, bytes: Bytes): ClearValueType<etype> {
  const bn = bytesToBigInt(bytes);
  // needed to type narrowing
  const ft: FheType = fheType;

  switch (ft) {
    case 'ebool':
      return (bn !== 0n) as ClearValueType<etype>;
    case 'eaddress': {
      // Return a checksummed (EIP-55) address for consistency with ethers/viem
      const addr = asAddress(bigIntToBytesHex(bn, { byteLength: 20 }));
      const checksummed = addressToChecksummedAddress(addr);
      return checksummed as Address as ClearValueType<etype>;
    }
    case 'euint8':
    case 'euint16':
    case 'euint32': {
      assertIsUintBigInt(bn, {
        max: BigInt(FheTypeToMaxValue[ft]),
        subject: 'value',
      });
      return Number(bn) as ClearValueType<etype>;
    }
    case 'euint64':
    case 'euint128':
    case 'euint256': {
      assertIsUintBigInt(bn, {
        max: BigInt(FheTypeToMaxValue[ft]),
        subject: 'value',
      });
      return bn as ClearValueType<etype>;
    }
    default:
      return assertNever(ft, `Unknown fheTypeName: ${ft}`);
  }
}

/**
 * Asserts that `value` is already the correct JS type for the given `fheTypeName`
 * and returns it narrowed. No conversion is performed.
 *
 * - `ebool` → `boolean`
 * - `eaddress` → `string` (checksummed address)
 * - `euint8/16/32` → `number`
 * - `euint64/128/256` → `bigint`
 *
 * @throws If `value` is not the expected JS type or exceeds the type's range.
 */
export function asClearValueType<etype extends FheType>(
  fheTypeName: etype,
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): ClearValueType<etype> {
  switch (fheTypeName) {
    case 'ebool':
      return asBoolean(value, options) as ClearValueType<etype>;
    case 'eaddress':
      return asAddress(value, options) as ClearValueType<etype>;
    case 'euint8':
    case 'euint16':
    case 'euint32': {
      assertIsUintNumber(value, {
        ...options,
        max: MAX_UINT_FOR_TYPE[fheTypeName],
      });
      return value as ClearValueType<etype>;
    }
    case 'euint64':
    case 'euint128':
    case 'euint256': {
      assertIsUintBigInt(value, {
        ...options,
        max: MAX_UINT_FOR_TYPE[fheTypeName],
      });
      return value as ClearValueType<etype>;
    }
    default:
      return assertNever(fheTypeName, `Unknown fheTypeName: ${fheTypeName}`);
  }
}

/**
 * Converts `value` to the correct JS type for the given `fheTypeName`.
 * Accepts any uint-like input (number, bigint, string) and coerces it.
 *
 * - `ebool` → `boolean`
 * - `eaddress` → `string` (validated address)
 * - `euint8/16/32` → coerced to `number` via `Number()`
 * - `euint64/128/256` → coerced to `bigint` via `BigInt()`
 *
 * @throws If `value` cannot be converted or exceeds the type's range.
 */
export function toClearValueType<etype extends FheType>(
  fheTypeName: etype,
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): ClearValueType<etype> {
  switch (fheTypeName) {
    case 'ebool':
      return asBoolean(value, options) as ClearValueType<etype>;
    case 'eaddress':
      assertIsAddress(value, options ?? {});
      return value as ClearValueType<etype>;
    case 'euint8':
    case 'euint16':
    case 'euint32': {
      assertIsUint(value, {
        ...options,
        max: MAX_UINT_FOR_TYPE[fheTypeName],
      });
      return Number(value) as ClearValueType<etype>;
    }
    case 'euint64':
    case 'euint128':
    case 'euint256': {
      assertIsUint(value, {
        ...options,
        max: MAX_UINT_FOR_TYPE[fheTypeName],
      });
      return BigInt(value) as ClearValueType<etype>;
    }
    default:
      return assertNever(fheTypeName, `Unknown fheTypeName: ${fheTypeName}`);
  }
}
