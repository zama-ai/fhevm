import type {
  Bytes32,
  Bytes8,
  BytesHex,
  BytesHexNo0x,
  Hex0x,
  Uint,
  Uint128,
  Uint16,
  Uint256,
  Uint256BigInt,
  Uint32,
  Uint32BigInt,
  Uint32Number,
  Uint64,
  Uint64BigInt,
  Uint8,
  Uint8Number,
  UintBigInt,
  UintMap,
  UintNormalizedMap,
  UintNumber,
  UintTypeName,
  ValueTypeName,
} from '../types/primitives.js';
import type {
  RecordWithPropertyType,
  RecordUintPropertyType,
  RecordUint256PropertyType,
} from '../types/record-p.js';
import type { ErrorMetadataParams } from './errors/ErrorBase.js';
import { isRecordNonNullableProperty, typeofProperty } from './record.js';
import { InvalidPropertyError } from './errors/InvalidPropertyError.js';
import { InvalidTypeError } from './errors/InvalidTypeError.js';

////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////

// 2^8 - 1 = 255
export const MAX_UINT8 = 0xff;

// 2^16 - 1 = 65535
export const MAX_UINT16 = 0xffff;

// 2^32 - 1 = 4294967295
export const MAX_UINT32 = 0xffffffff;

// 2^64 - 1 = 18446744073709551615
export const MAX_UINT64 = 0xffffffffffffffffn;

// 2^128 - 1 = 340282366920938463463374607431768211455
export const MAX_UINT128 = 0xffffffffffffffffffffffffffffffffn;

// 2^160 - 1 = 1461501637330902918203684832716283019655932542975
export const MAX_UINT160 = 0xffffffffffffffffffffffffffffffffffffffffn;

// 2^256 - 1 = 115792089237316195423570985008687907853269984665640564039457584007913129639935
export const MAX_UINT256 =
  0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffn;

////////////////////////////////////////////////////////////////////////////////

export const MAX_UINT_FOR_TYPE: Readonly<
  Record<ValueTypeName | 'uint160', number | bigint>
> = {
  bool: 1,
  uint8: MAX_UINT8,
  uint16: MAX_UINT16,
  uint32: MAX_UINT32,
  uint64: MAX_UINT64,
  uint128: MAX_UINT128,
  uint160: MAX_UINT160,
  uint256: MAX_UINT256,
  address: MAX_UINT160,
};
Object.freeze(MAX_UINT_FOR_TYPE);

////////////////////////////////////////////////////////////////////////////////

const MAX_SAFE_INTEGER_BIGINT = BigInt(Number.MAX_SAFE_INTEGER);

/**
 * Converts a `bigint` to a `number`, throwing if the value exceeds
 * `Number.MAX_SAFE_INTEGER` (2^53 - 1).
 *
 * @param value - The bigint to convert
 * @param options - Optional subject name for the error message
 * @returns The value as a `number`
 * @throws If the value exceeds the safe integer range
 */
export function bigIntToNumber(
  value: number | bigint,
  options?: { readonly subject?: string },
): number {
  if (typeof value === 'number') {
    return value;
  }
  if (value > MAX_SAFE_INTEGER_BIGINT) {
    const subject =
      options?.subject !== undefined ? ` (${options.subject})` : '';
    throw new Error(`Value${subject} ${value} exceeds Number.MAX_SAFE_INTEGER`);
  }
  return Number(value);
}

////////////////////////////////////////////////////////////////////////////////

const MAX_UINT_FOR_BYTE_LENGTH: Readonly<
  Record<1 | 2 | 4 | 8 | 16 | 20 | 32, Uint>
> = {
  1: MAX_UINT8 as Uint,
  2: MAX_UINT16 as Uint,
  4: MAX_UINT32 as Uint,
  8: MAX_UINT64 as Uint,
  16: MAX_UINT128 as Uint,
  20: MAX_UINT160 as Uint,
  32: MAX_UINT256 as Uint,
};
Object.freeze(MAX_UINT_FOR_BYTE_LENGTH);

////////////////////////////////////////////////////////////////////////////////

export function isUintNumber(
  value: unknown,
  max?: number | bigint,
): value is UintNumber {
  return (
    typeof value === 'number' &&
    Number.isInteger(value) &&
    value >= 0 &&
    (max === undefined || value <= max)
  );
}

export function isUintBigInt(
  value: unknown,
  max?: bigint | number,
): value is UintBigInt {
  return (
    typeof value === 'bigint' &&
    value >= 0 &&
    (max === undefined || value <= max)
  );
}

export function isUintForType(
  value: unknown,
  typeName?: UintTypeName,
): value is Uint {
  return isUint(
    value,
    typeName !== undefined ? MAX_UINT_FOR_TYPE[typeName] : undefined,
  );
}

export function isUintForByteLength(
  value: unknown,
  byteLength?: keyof typeof MAX_UINT_FOR_BYTE_LENGTH,
): value is Uint {
  return isUint(
    value,
    byteLength !== undefined ? MAX_UINT_FOR_BYTE_LENGTH[byteLength] : undefined,
  );
}

export function isUint(value: unknown, max?: number | bigint): value is Uint {
  return isUintNumber(value, max) || isUintBigInt(value, max);
}

export function isUint8(value: unknown): value is Uint8 {
  return isUint(value, MAX_UINT8);
}

export function isUint16(value: unknown): value is Uint16 {
  return isUint(value, MAX_UINT16);
}

export function isUint32(value: unknown): value is Uint32 {
  return isUint(value, MAX_UINT32);
}

export function isUint64(value: unknown): value is Uint64 {
  return isUint(value, MAX_UINT64);
}

export function isUint128(value: unknown): value is Uint128 {
  return isUint(value, MAX_UINT128);
}

export function isUint256(value: unknown): value is Uint256 {
  return isUint(value, MAX_UINT256);
}

export function isUint64BigInt(value: unknown): value is Uint64BigInt {
  if (typeof value !== 'bigint') return false;
  return isUint(value, MAX_UINT64);
}

////////////////////////////////////////////////////////////////////////////////
// Number Conversions
////////////////////////////////////////////////////////////////////////////////

export function numberToBytesHexNo0x(num: number): BytesHexNo0x {
  const hex = num.toString(16);
  return (hex.length % 2 !== 0 ? '0' + hex : hex) as BytesHexNo0x;
}

export function numberToBytesHex(num: number): BytesHex {
  return `0x${numberToBytesHexNo0x(num)}` as BytesHex;
}

export function numberToBytes32(num: number): Bytes32 {
  if (!isUintNumber(num)) {
    throw new InvalidTypeError({ expectedType: 'uintNumber' }, {});
  }

  const buffer = new ArrayBuffer(32);
  const view = new DataView(buffer);
  view.setBigUint64(24, BigInt(num), false);
  return new Uint8Array(buffer) as Bytes32;
}

export function numberToBytes8(num: number): Bytes8 {
  if (!isUintNumber(num)) {
    throw new InvalidTypeError({ expectedType: 'uintNumber' }, {});
  }

  const buffer = new ArrayBuffer(8);
  const view = new DataView(buffer);
  view.setBigUint64(0, BigInt(num), false);
  return new Uint8Array(buffer) as Bytes8;
}

////////////////////////////////////////////////////////////////////////////////
// Uint Conversions
////////////////////////////////////////////////////////////////////////////////

export function uintToHex0x(uint: Uint): Hex0x {
  return `0x${uint.toString(16)}` as Hex0x;
}

export function uintToBytesHex(uint: Uint): BytesHex {
  const hex = uint.toString(16);
  return (hex.length % 2 !== 0 ? `0x0${hex}` : `0x${hex}`) as BytesHex;
}

export function uintToBytesHexNo0x(uint: Uint): BytesHexNo0x {
  const hex = uint.toString(16);
  return (hex.length % 2 !== 0 ? `0${hex}` : hex) as BytesHexNo0x;
}

export function uint256ToBytes32(value: unknown): Bytes32 {
  if (!isUint256(value)) {
    throw new InvalidTypeError({ expectedType: 'uint256' }, {});
  }

  const buffer = new ArrayBuffer(32);
  const view = new DataView(buffer);

  const v = BigInt(value);

  // Fill from right to left (big-endian), 8 bytes at a time
  view.setBigUint64(24, v & 0xffffffffffffffffn, false);
  view.setBigUint64(16, (v >> 64n) & 0xffffffffffffffffn, false);
  view.setBigUint64(8, (v >> 128n) & 0xffffffffffffffffn, false);
  view.setBigUint64(0, (v >> 192n) & 0xffffffffffffffffn, false);

  return new Uint8Array(buffer) as Bytes32;
}

export function uint32ToBytes32(value: unknown): Bytes32 {
  if (!isUint32(value)) {
    throw new InvalidTypeError({ expectedType: 'uint32' }, {});
  }

  const buffer = new ArrayBuffer(32);
  const view = new DataView(buffer);

  const v = Number(value);

  view.setUint32(28, v, false);

  return new Uint8Array(buffer) as Bytes32;
}

export function uint64ToBytes32(value: unknown): Bytes32 {
  if (!isUint64(value)) {
    throw new InvalidTypeError({ expectedType: 'uint64' }, {});
  }

  const buffer = new ArrayBuffer(32);
  const view = new DataView(buffer);

  const v = BigInt(value);

  view.setBigUint64(24, v, false);

  return new Uint8Array(buffer) as Bytes32;
}

////////////////////////////////////////////////////////////////////////////////
// Asserts
////////////////////////////////////////////////////////////////////////////////

export function assertIsUint(
  value: unknown,
  options: { max?: bigint | number; subject?: string } & ErrorMetadataParams,
): asserts value is Uint {
  if (!isUint(value, options.max)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'uint',
      },
      options,
    );
  }
}

export function assertIsUintForType<T extends UintTypeName>(
  value: unknown,
  typeName: T,
  options: {
    subject?: string;
  } & ErrorMetadataParams,
): asserts value is UintMap[T] {
  if (!isUintForType(value, typeName)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: typeName,
      },
      options,
    );
  }
}

export function assertIsUintNumber(
  value: unknown,
  options: { max?: bigint | number; subject?: string } & ErrorMetadataParams,
): asserts value is UintNumber {
  if (!isUintNumber(value, options.max)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'uintNumber',
      },
      options,
    );
  }
}

export function assertIsUintBigInt(
  value: unknown,
  options: { max?: bigint | number; subject?: string } & ErrorMetadataParams,
): asserts value is UintBigInt {
  if (!isUintBigInt(value, options.max)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'uintBigInt',
      },
      options,
    );
  }
}

export function assertIsUint8(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Uint8 {
  if (!isUint8(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'uint8',
      },
      options,
    );
  }
}

export function assertIsUint16(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Uint16 {
  if (!isUint16(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'uint16',
      },
      options,
    );
  }
}

export function assertIsUint32(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Uint32 {
  if (!isUint32(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'uint32',
      },
      options,
    );
  }
}

export function assertIsUint64(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Uint64 {
  if (!isUint64(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'uint64',
      },
      options,
    );
  }
}

export function assertIsUint128(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Uint128 {
  if (!isUint256(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'uint128',
      },
      options,
    );
  }
}

export function assertIsUint256(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Uint256 {
  if (!isUint256(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'uint256',
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////
// normalizeUintForType
////////////////////////////////////////////////////////////////////////////////

export function normalizeUintForType<T extends UintTypeName>(
  value: Uint,
  typeName: T,
): UintNormalizedMap[T] {
  switch (typeName) {
    case 'uint8':
    case 'uint16':
    case 'uint32':
      return Number(value) as UintNormalizedMap[T];
    case 'uint64':
    case 'uint128':
    case 'uint160':
    case 'uint256':
      return BigInt(value) as UintNormalizedMap[T];
  }
}

////////////////////////////////////////////////////////////////////////////////
// asUintXX
////////////////////////////////////////////////////////////////////////////////

export function asUintForType<T extends UintTypeName>(
  value: unknown,
  typeName: T,
  options: {
    subject?: string;
  } & ErrorMetadataParams,
): UintMap[T] {
  assertIsUintForType(value, typeName, options);
  return value;
}

export function asUint(
  value: unknown,
  options?: {
    max?: number | bigint;
    subject?: string;
  } & ErrorMetadataParams,
): Uint {
  assertIsUint(value, options ?? {});
  return value;
}

export function asUint8(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Uint8 {
  assertIsUint8(value, options ?? {});
  return value;
}

export function asUint16(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Uint16 {
  assertIsUint16(value, options ?? {});
  return value;
}

export function asUint32(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Uint32 {
  assertIsUint32(value, options ?? {});
  return value;
}

export function asUint64(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Uint64 {
  assertIsUint64(value, options ?? {});
  return value;
}

export function asUint128(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Uint128 {
  assertIsUint128(value, options ?? {});
  return value;
}

export function asUint256(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Uint256 {
  assertIsUint256(value, options ?? {});
  return value;
}

////////////////////////////////////////////////////////////////////////////////

export function asUint8Number(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Uint8Number {
  assertIsUintNumber(value, { max: MAX_UINT8, ...options });
  return value as Uint8Number;
}

export function asUint32Number(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Uint32Number {
  assertIsUintNumber(value, { max: MAX_UINT32, ...options });
  return value as Uint32Number;
}

export function asUint32BigInt(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Uint32BigInt {
  assertIsUintBigInt(value, { max: MAX_UINT32, ...options });
  return value as Uint32BigInt;
}

export function asUint64BigInt(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Uint64BigInt {
  assertIsUintBigInt(value, { max: MAX_UINT64, ...options });
  return value as Uint64BigInt;
}

export function asUint256BigInt(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Uint256BigInt {
  assertIsUintBigInt(value, { max: MAX_UINT256, ...options });
  return value as Uint256BigInt;
}

////////////////////////////////////////////////////////////////////////////////
// Record property testing
////////////////////////////////////////////////////////////////////////////////

export function isRecordUintProperty<K extends string>(
  record: unknown,
  property: K,
): record is RecordUintPropertyType<K> {
  if (!isRecordNonNullableProperty(record, property)) {
    return false;
  }
  return isUint(record[property]);
}

export function assertRecordUintProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, Uint> {
  if (!isRecordUintProperty(record, property)) {
    throw new InvalidPropertyError(
      {
        subject: recordName,
        property,
        type: typeofProperty(record, property),
        expectedType: 'uint',
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

export function isRecordUint256Property<K extends string>(
  record: unknown,
  property: K,
): record is RecordUint256PropertyType<K> {
  if (!isRecordNonNullableProperty(record, property)) {
    return false;
  }
  return isUint256(record[property]);
}

export function assertRecordUint256Property<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, Uint256> {
  if (!isRecordUint256Property(record, property)) {
    throw new InvalidPropertyError(
      {
        subject: recordName,
        property,
        type: typeofProperty(record, property),
        expectedType: 'uint256',
      },
      options,
    );
  }
}

export function assertRecordUintNumberProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, number> {
  if (
    typeofProperty(record, property) !== 'number' ||
    !isUintNumber((record as Record<string, unknown>)[property])
  ) {
    throw new InvalidPropertyError(
      {
        subject: recordName,
        property,
        type: typeofProperty(record, property),
        expectedType: 'uintNumber',
      },
      options,
    );
  }
}

export function assertRecordUintBigIntProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, number> {
  if (
    typeofProperty(record, property) !== 'bigint' ||
    !isUintBigInt((record as Record<string, unknown>)[property])
  ) {
    throw new InvalidPropertyError(
      {
        subject: recordName,
        property,
        type: typeofProperty(record, property),
        expectedType: 'uintBigInt',
      },
      options,
    );
  }
}
