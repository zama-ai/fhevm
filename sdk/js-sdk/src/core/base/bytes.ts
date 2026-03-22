import type {
  Bytes,
  Bytes20,
  Bytes20Hex,
  Bytes20HexNo0x,
  Bytes21,
  Bytes21Hex,
  Bytes21HexNo0x,
  Bytes32,
  Bytes32Hex,
  Bytes32HexNo0x,
  Bytes65,
  Bytes65Hex,
  Bytes65HexNo0x,
  Bytes8,
  Bytes8Hex,
  Bytes8HexNo0x,
  BytesHex,
  BytesHexNo0x,
  BytesTypeNameToByteLengthMap,
  BytesTypeNameToTypeMap,
  ByteLength,
  Uint8Number,
  Uint64BigInt,
  UintNumber,
  Bytes1Hex,
  Bytes1,
  Bytes1HexNo0x,
  UintBigInt,
  BytesTypeName,
  BytesNMap,
} from "../types/primitives.js";
import type {
  RecordUint8ArrayPropertyType,
  RecordWithPropertyType,
} from "../types/record-p.js";
import type { ErrorMetadataParams } from "./errors/ErrorBase.js";
import {
  assertRecordArrayProperty,
  isRecordNonNullableProperty,
  typeofProperty,
} from "./record.js";
import { is0x, isNo0x, remove0x } from "./string.js";
import { InvalidPropertyError } from "./errors/InvalidPropertyError.js";
import { InvalidTypeError } from "./errors/InvalidTypeError.js";
import { isUintForByteLength } from "./uint.js";

////////////////////////////////////////////////////////////////////////////////

/**
 * Maps BytesXX type names to their corresponding byte lengths.
 * Type is derived from BytesTypeNameToByteLengthMap to ensure consistency.
 */
export const ByteLengthForType: BytesTypeNameToByteLengthMap = {
  bytes1: 1,
  bytes2: 2,
  bytes4: 4,
  bytes8: 8,
  bytes20: 20,
  bytes21: 21,
  bytes32: 32,
  bytes65: 65,
};
Object.freeze(ByteLengthForType);

////////////////////////////////////////////////////////////////////////////////

const bytesHexRegex = /^0x[a-fA-F0-9]*$/;
const bytesHexNo0xRegex = /^[a-fA-F0-9]*$/;

////////////////////////////////////////////////////////////////////////////////
// isBytesXX
////////////////////////////////////////////////////////////////////////////////

export function isBytes(
  value: unknown,
  byteLength?: ByteLength,
): value is Bytes {
  if (value === undefined || value === null) {
    return false;
  }

  if (!(value instanceof Uint8Array)) {
    return false;
  }

  return byteLength !== undefined ? value.length === byteLength : true;
}

////////////////////////////////////////////////////////////////////////////////

export function isBytesForType<T extends BytesTypeName>(
  value: unknown,
  typeName?: T,
): value is BytesTypeNameToTypeMap[T] {
  return isBytes(
    value,
    typeName !== undefined ? ByteLengthForType[typeName] : undefined,
  );
}

////////////////////////////////////////////////////////////////////////////////

export function isBytes1(value: unknown): value is Bytes1 {
  return isBytes(value, 1 satisfies ByteLength);
}

export function isBytes8(value: unknown): value is Bytes8 {
  return isBytes(value, 8 satisfies ByteLength);
}

export function isBytes20(value: unknown): value is Bytes20 {
  return isBytes(value, 20 satisfies ByteLength);
}

export function isBytes21(value: unknown): value is Bytes21 {
  return isBytes(value, 21 satisfies ByteLength);
}

export function isBytes32(value: unknown): value is Bytes32 {
  return isBytes(value, 32 satisfies ByteLength);
}

export function isBytes65(value: unknown): value is Bytes65 {
  return isBytes(value, 65 satisfies ByteLength);
}

////////////////////////////////////////////////////////////////////////////////
// isBytesXXHex
////////////////////////////////////////////////////////////////////////////////

export function isBytesHex(
  value: unknown,
  byteLength?: ByteLength,
): value is BytesHex {
  if (!is0x(value)) {
    return false;
  }

  if (byteLength !== undefined && value.length !== 2 * byteLength + 2) {
    return false;
  }

  if ((value.length - 2) % 2 !== 0) {
    return false;
  }

  return bytesHexRegex.test(value);
}

////////////////////////////////////////////////////////////////////////////////

export function isBytes1Hex(value: unknown): value is Bytes1Hex {
  return isBytesHex(value, 1 satisfies ByteLength);
}

export function isBytes8Hex(value: unknown): value is Bytes8Hex {
  return isBytesHex(value, 8 satisfies ByteLength);
}

export function isBytes20Hex(value: unknown): value is Bytes20Hex {
  return isBytesHex(value, 20 satisfies ByteLength);
}

export function isBytes21Hex(value: unknown): value is Bytes21Hex {
  return isBytesHex(value, 21 satisfies ByteLength);
}

export function isBytes32Hex(value: unknown): value is Bytes32Hex {
  return isBytesHex(value, 32 satisfies ByteLength);
}

export function isBytes65Hex(value: unknown): value is Bytes65Hex {
  return isBytesHex(value, 65 satisfies ByteLength);
}

////////////////////////////////////////////////////////////////////////////////
// isBytesXXHexNo0x
////////////////////////////////////////////////////////////////////////////////

export function isBytesHexNo0x(
  value: unknown,
  byteLength?: ByteLength,
): value is BytesHexNo0x {
  if (!isNo0x(value)) {
    return false;
  }

  if (byteLength !== undefined && value.length !== 2 * byteLength) {
    return false;
  }

  if (value.length % 2 !== 0) {
    return false;
  }

  if (!bytesHexNo0xRegex.test(value)) {
    return false;
  }

  return true;
}

////////////////////////////////////////////////////////////////////////////////

export function isBytes1HexNo0x(value: unknown): value is Bytes1HexNo0x {
  return isBytesHexNo0x(value, 1 satisfies ByteLength);
}

export function isBytes8HexNo0x(value: unknown): value is Bytes8HexNo0x {
  return isBytesHexNo0x(value, 8 satisfies ByteLength);
}

export function isBytes20HexNo0x(value: unknown): value is Bytes20HexNo0x {
  return isBytesHexNo0x(value, 20 satisfies ByteLength);
}

export function isBytes21HexNo0x(value: unknown): value is Bytes21HexNo0x {
  return isBytesHexNo0x(value, 21 satisfies ByteLength);
}

export function isBytes32HexNo0x(value: unknown): value is Bytes32HexNo0x {
  return isBytesHexNo0x(value, 32 satisfies ByteLength);
}

export function isBytes65HexNo0x(value: unknown): value is Bytes65HexNo0x {
  return isBytesHexNo0x(value, 65 satisfies ByteLength);
}

////////////////////////////////////////////////////////////////////////////////
// assertIsBytesXX
////////////////////////////////////////////////////////////////////////////////

export function assertIsBytes(
  value: unknown,
  options?: { byteLength?: ByteLength; subject?: string } & ErrorMetadataParams,
): asserts value is Bytes {
  if (!isBytes(value, options?.byteLength)) {
    throw new InvalidTypeError(
      {
        subject: options?.subject,
        expectedType: `bytes${options?.byteLength ?? ""}`,
      },
      options ?? {},
    );
  }
}

export function assertIsBytesForType<T extends BytesTypeName>(
  value: unknown,
  options?: {
    typeName: T;
    subject?: string;
  } & ErrorMetadataParams,
): asserts value is BytesNMap[T] {
  if (!isBytesForType(value, options?.typeName)) {
    throw new InvalidTypeError(
      {
        subject: options?.subject,
        expectedType: options?.typeName ?? "bytes",
      },
      options ?? {},
    );
  }
}

export function assertIsBytes1(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes1 {
  assertIsBytes(value, { byteLength: 1, ...options });
}

export function assertIsBytes8(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes8 {
  assertIsBytes(value, { byteLength: 8, ...options });
}

export function assertIsBytes20(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes20 {
  assertIsBytes(value, { byteLength: 20, ...options });
}

export function assertIsBytes21(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes21 {
  assertIsBytes(value, { byteLength: 21, ...options });
}

export function assertIsBytes32(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes32 {
  assertIsBytes(value, { byteLength: 32, ...options });
}

export function assertIsBytes65(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes65 {
  assertIsBytes(value, { byteLength: 65, ...options });
}

////////////////////////////////////////////////////////////////////////////////

export function asBytesForType(
  value: unknown,
  options?: {
    typeName: BytesTypeName;
    subject?: string;
  } & ErrorMetadataParams,
): Bytes {
  assertIsBytesForType(value, options);
  return value;
}

export function asBytes(
  value: unknown,
  options?: {
    byteLength?: ByteLength;
    subject?: string;
  } & ErrorMetadataParams,
): Bytes {
  assertIsBytes(value, options);
  return value;
}

export function asBytes1(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Bytes1 {
  assertIsBytes(value, { byteLength: 1, ...options });
  return value as Bytes1;
}

export function asBytes8(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Bytes8 {
  assertIsBytes(value, { byteLength: 8, ...options });
  return value as Bytes8;
}

export function asBytes20(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Bytes20 {
  assertIsBytes(value, { byteLength: 20, ...options });
  return value as Bytes20;
}

export function asBytes21(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Bytes21 {
  assertIsBytes(value, { byteLength: 21, ...options });
  return value as Bytes21;
}

export function asBytes32(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Bytes32 {
  assertIsBytes(value, { byteLength: 32, ...options });
  return value as Bytes32;
}

export function asBytes65(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Bytes65 {
  assertIsBytes(value, { byteLength: 65, ...options });
  return value as Bytes65;
}

////////////////////////////////////////////////////////////////////////////////
// assertIsBytesXXHex
////////////////////////////////////////////////////////////////////////////////

/**
 * Assertion function that validates a value is a valid hex bytes string.
 * A valid BytesHex string must start with "0x" followed by an even number of hexadecimal characters.
 * Throws an `InvalidTypeError` if validation fails.
 *
 * @param value - The value to validate (can be any type)
 * @param options - Validation options
 * @param options.byteLength - Optional expected byte length (e.g., 8, 20, 32)
 * @param options.subject - Optional name of the value being validated (used in error messages)
 * @throws {InvalidTypeError} When the value is not a string or not valid BytesHex format
 * @throws {never} No other errors are thrown
 *
 * @example
 * ```typescript
 * function processHash(hash: unknown) {
 *   assertIsBytesHex(hash, { byteLength: 32, subject: 'hash' });
 *   console.log(hash); // TypeScript now knows hash is BytesHex
 * }
 * ```
 */
export function assertIsBytesHex(
  value: unknown,
  options: { byteLength?: ByteLength; subject?: string } & ErrorMetadataParams,
): asserts value is BytesHex {
  if (!isBytesHex(value, options.byteLength)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: `bytes${options.byteLength ?? ""}Hex`,
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsBytes1Hex(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes1Hex {
  assertIsBytesHex(value, { byteLength: 1, ...options });
}

export function assertIsBytes8Hex(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes8Hex {
  assertIsBytesHex(value, { byteLength: 8, ...options });
}

export function assertIsBytes20Hex(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes20Hex {
  assertIsBytesHex(value, { byteLength: 20, ...options });
}

export function assertIsBytes21Hex(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes21Hex {
  assertIsBytesHex(value, { byteLength: 21, ...options });
}

export function assertIsBytes32Hex(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes32Hex {
  assertIsBytesHex(value, { byteLength: 32, ...options });
}

export function assertIsBytes65Hex(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes65Hex {
  assertIsBytesHex(value, { byteLength: 65, ...options });
}

////////////////////////////////////////////////////////////////////////////////

export function asBytesHex(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): BytesHex {
  assertIsBytesHex(value, options ?? {});
  return value;
}

export function asBytes1Hex(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Bytes1Hex {
  assertIsBytesHex(value, { byteLength: 1, ...options });
  return value as Bytes1Hex;
}

export function asBytes8Hex(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Bytes8Hex {
  assertIsBytesHex(value, { byteLength: 8, ...options });
  return value as Bytes8Hex;
}

export function asBytes20Hex(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Bytes20Hex {
  assertIsBytesHex(value, { byteLength: 20, ...options });
  return value as Bytes20Hex;
}

export function asBytes21Hex(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Bytes21Hex {
  assertIsBytesHex(value, { byteLength: 21, ...options });
  return value as Bytes21Hex;
}

export function asBytes32Hex(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Bytes32Hex {
  assertIsBytesHex(value, { byteLength: 32, ...options });
  return value as Bytes32Hex;
}

export function asBytes65Hex(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Bytes65Hex {
  assertIsBytesHex(value, { byteLength: 65, ...options });
  return value as Bytes65Hex;
}

////////////////////////////////////////////////////////////////////////////////
// assertIsBytesXXHexNo0x
////////////////////////////////////////////////////////////////////////////////

/**
 * Assertion function that validates a value is a valid hex bytes string without "0x" prefix.
 * A valid BytesHexNo0x string must contain only hexadecimal characters (no "0x" prefix).
 * Throws an `InvalidTypeError` if validation fails.
 *
 * @param value - The value to validate (can be any type)
 * @param options - Validation options
 * @param options.byteLength - Optional expected byte length (e.g., 8, 20, 32)
 * @param options.subject - Optional name of the value being validated (used in error messages)
 * @throws {InvalidTypeError} When the value is not a string or not valid BytesHexNo0x format
 * @throws {never} No other errors are thrown
 *
 * @example
 * ```typescript
 * function processSignature(sig: unknown) {
 *   assertIsBytesHexNo0x(sig, { byteLength: 65, subject: 'signature' });
 *   console.log(sig); // TypeScript now knows sig is BytesHexNo0x
 * }
 * ```
 */
export function assertIsBytesHexNo0x(
  value: unknown,
  options: { byteLength?: ByteLength; subject?: string } & ErrorMetadataParams,
): asserts value is BytesHexNo0x {
  if (!isBytesHexNo0x(value, options.byteLength)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: `bytes${options.byteLength ?? ""}HexNo0x`,
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsBytes1HexNo0x(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes1HexNo0x {
  assertIsBytesHexNo0x(value, { byteLength: 1, ...options });
}

export function assertIsBytes8HexNo0x(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes8HexNo0x {
  assertIsBytesHexNo0x(value, { byteLength: 8, ...options });
}

export function assertIsBytes20HexNo0x(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes20HexNo0x {
  assertIsBytesHexNo0x(value, { byteLength: 20, ...options });
}

export function assertIsBytes21HexNo0x(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes21HexNo0x {
  assertIsBytesHexNo0x(value, { byteLength: 21, ...options });
}

export function assertIsBytes32HexNo0x(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes32HexNo0x {
  assertIsBytesHexNo0x(value, { byteLength: 32, ...options });
}

export function assertIsBytes65HexNo0x(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes65HexNo0x {
  assertIsBytesHexNo0x(value, { byteLength: 65, ...options });
}

////////////////////////////////////////////////////////////////////////////////
// asBytesXXHexNo0x
////////////////////////////////////////////////////////////////////////////////

export function asBytesHexNo0x(
  value: unknown,
  options?: { byteLength?: ByteLength; subject?: string } & ErrorMetadataParams,
): BytesHexNo0x {
  assertIsBytesHexNo0x(value, options ?? {});
  return value;
}

export function asBytes1HexNo0x(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Bytes1HexNo0x {
  assertIsBytesHexNo0x(value, { byteLength: 1, ...options });
  return value as Bytes1HexNo0x;
}

export function asBytes8HexNo0x(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Bytes8HexNo0x {
  assertIsBytesHexNo0x(value, { byteLength: 8, ...options });
  return value as Bytes8HexNo0x;
}

export function asBytes20HexNo0x(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Bytes20HexNo0x {
  assertIsBytesHexNo0x(value, { byteLength: 20, ...options });
  return value as Bytes20HexNo0x;
}

export function asBytes21HexNo0x(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Bytes21HexNo0x {
  assertIsBytesHexNo0x(value, { byteLength: 21, ...options });
  return value as Bytes21HexNo0x;
}

export function asBytes32HexNo0x(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Bytes32HexNo0x {
  assertIsBytesHexNo0x(value, { byteLength: 32, ...options });
  return value as Bytes32HexNo0x;
}

export function asBytes65HexNo0x(
  value: unknown,
  options?: { subject?: string } & ErrorMetadataParams,
): Bytes65HexNo0x {
  assertIsBytesHexNo0x(value, { byteLength: 65, ...options });
  return value as Bytes65HexNo0x;
}

////////////////////////////////////////////////////////////////////////////////
// assertIsBytesXXHexArray
////////////////////////////////////////////////////////////////////////////////

/**
 * Assertion function that validates a value is an array of valid hex bytes strings.
 * Each element must start with "0x" followed by hexadecimal characters.
 * Throws an `InvalidTypeError` if validation fails.
 *
 * @param value - The value to validate (can be any type)
 * @param options - Validation options
 * @param options.byteLength - Optional expected byte length for each element (e.g., 8, 20, 32)
 * @param options.subject - Optional name of the value being validated (used in error messages)
 * @throws {InvalidTypeError} When the value is not an array or any element is not valid BytesHex format
 * @throws {never} No other errors are thrown
 *
 * @example
 * ```typescript
 * function processHashes(hashes: unknown) {
 *   assertIsBytesHexArray(hashes, { byteLength: 32, subject: 'hashes' });
 *   hashes.forEach(hash => console.log(hash)); // TypeScript knows each is BytesHex
 * }
 * ```
 */
export function assertIsBytesHexArray(
  value: unknown,
  options: { byteLength?: ByteLength; subject?: string } & ErrorMetadataParams,
): asserts value is BytesHex[] {
  if (!Array.isArray(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: `bytes${options.byteLength ?? ""}Hex[]`,
      },
      options,
    );
  }
  for (let i = 0; i < value.length; ++i) {
    if (!isBytesHex(value[i], options.byteLength)) {
      throw new InvalidTypeError(
        {
          subject: options.subject,
          index: i,
          type: typeof value[i],
          expectedType: `bytes${options.byteLength ?? ""}Hex`,
        },
        options,
      );
    }
  }
}

export function assertIsBytesArray(
  value: unknown,
  options: { byteLength?: ByteLength; subject?: string } & ErrorMetadataParams,
): asserts value is Bytes[] {
  if (!Array.isArray(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: `bytes${options.byteLength ?? ""}[]`,
      },
      options,
    );
  }
  for (let i = 0; i < value.length; ++i) {
    if (!isBytes(value[i], options.byteLength)) {
      throw new InvalidTypeError(
        {
          subject: options.subject,
          index: i,
          type: typeof value[i],
          expectedType: `bytes${options.byteLength ?? ""}`,
        },
        options,
      );
    }
  }
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsBytes1HexArray(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes1Hex[] {
  assertIsBytesHexArray(value, { byteLength: 1, ...options });
}

export function assertIsBytes8HexArray(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes8Hex[] {
  assertIsBytesHexArray(value, { byteLength: 8, ...options });
}

export function assertIsBytes20HexArray(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes20Hex[] {
  assertIsBytesHexArray(value, { byteLength: 20, ...options });
}

export function assertIsBytes21HexArray(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes21Hex[] {
  assertIsBytesHexArray(value, { byteLength: 21, ...options });
}

export function assertIsBytes32HexArray(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes32Hex[] {
  assertIsBytesHexArray(value, { byteLength: 32, ...options });
}

export function assertIsBytes65HexArray(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes65Hex[] {
  assertIsBytesHexArray(value, { byteLength: 65, ...options });
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsBytes32Array(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is Bytes32[] {
  assertIsBytesArray(value, { byteLength: 32, ...options });
}

////////////////////////////////////////////////////////////////////////////////
// isRecordBytesXXHexProperty
////////////////////////////////////////////////////////////////////////////////

/**
 * Type guard that checks if a property exists on an object and is a valid hex bytes string.
 * A valid BytesHex string starts with "0x" followed by an even number of hexadecimal characters.
 *
 * @template K - The property key type (string literal)
 * @param record - The value to check (can be any type)
 * @param property - The property name to check for
 * @param byteLength - Optional byte length (e.g., 8, 20, 32)
 * @returns True if `o` is an object with the specified property that is a valid BytesHex string
 *
 * @example
 * ```typescript
 * const data: unknown = { hash: "0x1234abcd", value: 42 };
 * if (isRecordBytesHexProperty(data, 'hash')) {
 *   console.log(data.hash); // "0x1234abcd"
 * }
 * ```
 */
export function isRecordBytesHexProperty<K extends string>(
  record: unknown,
  property: K,
  byteLength?: ByteLength,
): record is RecordWithPropertyType<K, BytesHex> {
  if (!isRecordNonNullableProperty(record, property)) {
    return false;
  }
  return isBytesHex(record[property], byteLength);
}

////////////////////////////////////////////////////////////////////////////////

export function isRecordBytes1HexProperty<K extends string>(
  record: unknown,
  property: K,
): record is RecordWithPropertyType<K, Bytes1Hex> {
  return isRecordBytesHexProperty(record, property, 1);
}

export function isRecordBytes8HexProperty<K extends string>(
  record: unknown,
  property: K,
): record is RecordWithPropertyType<K, Bytes8Hex> {
  return isRecordBytesHexProperty(record, property, 8);
}

export function isRecordBytes20HexProperty<K extends string>(
  record: unknown,
  property: K,
): record is RecordWithPropertyType<K, Bytes20Hex> {
  return isRecordBytesHexProperty(record, property, 20);
}

export function isRecordBytes21HexProperty<K extends string>(
  record: unknown,
  property: K,
): record is RecordWithPropertyType<K, Bytes21Hex> {
  return isRecordBytesHexProperty(record, property, 21);
}

export function isRecordBytes32HexProperty<K extends string>(
  record: unknown,
  property: K,
): record is RecordWithPropertyType<K, Bytes32Hex> {
  return isRecordBytesHexProperty(record, property, 32);
}

export function isRecordBytes65HexProperty<K extends string>(
  record: unknown,
  property: K,
): record is RecordWithPropertyType<K, Bytes65Hex> {
  return isRecordBytesHexProperty(record, property, 65);
}

////////////////////////////////////////////////////////////////////////////////
// assertRecordBytesXXHexProperty
////////////////////////////////////////////////////////////////////////////////

/**
 * Assertion function that validates a property exists on an object and is a valid hex bytes string.
 * A valid BytesHex string must start with "0x" followed by an even number of hexadecimal characters.
 * Throws an `InvalidPropertyError` if validation fails.
 *
 * @template K - The property key type (string literal)
 * @param record - The value to validate (can be any type)
 * @param property - The property name to check for
 * @param recordName - The name of the object being validated (used in error messages)
 * @throws {InvalidPropertyError} When the property is missing, not a string, or not valid BytesHex format
 * @throws {never} No other errors are thrown
 *
 * @example
 * ```typescript
 * function processTransaction(data: unknown) {
 *   assertRecordBytesHexProperty(data, 'txHash', 'transaction');
 *   console.log(data.txHash); // e.g., "0x1234..."
 * }
 * ```
 */
export function assertRecordBytesHexProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: { byteLength?: ByteLength } & ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, BytesHex> {
  if (!isRecordBytesHexProperty(record, property, options.byteLength)) {
    throw new InvalidPropertyError(
      {
        subject: recordName,
        property,
        expectedType: `bytes${options.byteLength ?? ""}Hex`,
        type: typeofProperty(record, property),
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

export function assertRecordBytes1HexProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, Bytes1Hex> {
  assertRecordBytesHexProperty(record, property, recordName, {
    byteLength: 1,
    ...options,
  });
}

export function assertRecordBytes8HexProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, Bytes8Hex> {
  assertRecordBytesHexProperty(record, property, recordName, {
    byteLength: 8,
    ...options,
  });
}

export function assertRecordBytes20HexProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, Bytes20Hex> {
  assertRecordBytesHexProperty(record, property, recordName, {
    byteLength: 20,
    ...options,
  });
}

export function assertRecordBytes21HexProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, Bytes21Hex> {
  assertRecordBytesHexProperty(record, property, recordName, {
    byteLength: 21,
    ...options,
  });
}

export function assertRecordBytes32HexProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, Bytes32Hex> {
  assertRecordBytesHexProperty(record, property, recordName, {
    byteLength: 32,
    ...options,
  });
}

export function assertRecordBytes65HexProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, Bytes65Hex> {
  assertRecordBytesHexProperty(record, property, recordName, {
    byteLength: 65,
    ...options,
  });
}

////////////////////////////////////////////////////////////////////////////////
// isRecordBytesHexXXNo0xProperty
////////////////////////////////////////////////////////////////////////////////

/**
 * Type guard that checks if a property exists on an object and is a valid hex bytes string without "0x" prefix.
 * A valid BytesHexNo0x string contains only hexadecimal characters (no "0x" prefix).
 *
 * @template K - The property key type (string literal)
 * @param record - The value to check (can be any type)
 * @param property - The property name to check for
 * @param byteLength - Optional byte length (e.g., 8, 20, 32)
 * @returns True if `o` is an object with the specified property that is a valid BytesHexNo0x string
 *
 * @example
 * ```typescript
 * const data: unknown = { signature: "1234abcd", value: 42 };
 * if (isRecordBytesHexNo0xProperty(data, 'signature')) {
 *   console.log(data.signature); // "1234abcd"
 * }
 * ```
 */
export function isRecordBytesHexNo0xProperty<K extends string>(
  record: unknown,
  property: K,
  byteLength?: ByteLength,
): record is RecordWithPropertyType<K, BytesHexNo0x> {
  if (!isRecordNonNullableProperty(record, property)) {
    return false;
  }
  return isBytesHexNo0x(record[property], byteLength);
}

////////////////////////////////////////////////////////////////////////////////
// assertRecordBytesXXHexNo0xProperty
////////////////////////////////////////////////////////////////////////////////

/**
 * Assertion function that validates a property exists on an object and is a valid hex bytes string without "0x" prefix.
 * A valid BytesHexNo0x string contains only hexadecimal characters (no "0x" prefix).
 * Throws an `InvalidPropertyError` if validation fails.
 *
 * @template K - The property key type (string literal)
 * @param record - The value to validate (can be any type)
 * @param property - The property name to check for
 * @param recordName - The name of the object being validated (used in error messages)
 * @param options - Validation options
 * @param options.byteLength - Optional expected byte length (e.g., 8, 20, 32)
 * @throws {InvalidPropertyError} When the property is missing, not a string, or not valid BytesHexNo0x format
 * @throws {never} No other errors are thrown
 *
 * @example
 * ```typescript
 * function processData(data: unknown) {
 *   assertRecordBytesHexNo0xProperty(data, 'signature', 'response', {});
 *   console.log(data.signature); // e.g., "1234abcd..."
 * }
 * ```
 */
export function assertRecordBytesHexNo0xProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: { byteLength?: ByteLength } & ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, BytesHexNo0x> {
  if (!isRecordBytesHexNo0xProperty(record, property, options.byteLength)) {
    throw new InvalidPropertyError(
      {
        subject: recordName,
        property,
        expectedType: `bytes${options.byteLength ?? ""}HexNo0x`,
        type: typeofProperty(record, property),
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////
// assertRecordBytesXXHexArrayProperty
////////////////////////////////////////////////////////////////////////////////

/**
 * Assertion function that validates a property exists on an object, is an array,
 * and every element is a valid hex bytes string (with "0x" prefix).
 * Throws an `InvalidPropertyError` if validation fails.
 *
 * @template K - The property key type (string literal)
 * @param record - The record to validate (can be any type)
 * @param property - The property name to check for
 * @param recordName - The name of the record being validated (used in error messages)
 * @throws {InvalidPropertyError} When the property is missing, not an array, or any element is not valid BytesHex
 * @throws {never} No other errors are thrown
 *
 * @example
 * ```typescript
 * function processHashes(data: unknown) {
 *   assertRecordBytesHexArrayProperty(data, 'txHashes', 'transaction');
 *   data.txHashes.forEach(hash => {
 *     console.log(hash); // e.g., "0x1234abcd..."
 *   });
 * }
 * ```
 */
export function assertRecordBytesHexArrayProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: { byteLength?: ByteLength } & ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, BytesHex[]> {
  assertRecordArrayProperty(record, property, recordName, options);
  const arr = record[property];
  for (let i = 0; i < arr.length; ++i) {
    if (!isBytesHex(arr[i], options.byteLength)) {
      throw new InvalidPropertyError(
        {
          subject: recordName,
          index: i,
          property,
          expectedType: `bytes${options.byteLength ?? ""}Hex`,
          type: typeof arr[i],
        },
        options,
      );
    }
  }
}

////////////////////////////////////////////////////////////////////////////////

export function assertRecordBytes1HexArrayProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, Bytes1Hex[]> {
  assertRecordBytesHexArrayProperty(record, property, recordName, {
    byteLength: 1,
    ...options,
  });
}

export function assertRecordBytes8HexArrayProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, Bytes8Hex[]> {
  assertRecordBytesHexArrayProperty(record, property, recordName, {
    byteLength: 8,
    ...options,
  });
}

export function assertRecordBytes20HexArrayProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, Bytes20Hex[]> {
  assertRecordBytesHexArrayProperty(record, property, recordName, {
    byteLength: 20,
    ...options,
  });
}

export function assertRecordBytes21HexArrayProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, Bytes21Hex[]> {
  assertRecordBytesHexArrayProperty(record, property, recordName, {
    byteLength: 21,
    ...options,
  });
}

export function assertRecordBytes32HexArrayProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, Bytes32Hex[]> {
  assertRecordBytesHexArrayProperty(record, property, recordName, {
    byteLength: 32,
    ...options,
  });
}

export function assertRecordBytes65HexArrayProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, Bytes65Hex[]> {
  assertRecordBytesHexArrayProperty(record, property, recordName, {
    byteLength: 65,
    ...options,
  });
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Assertion function that validates a property exists on an object, is an array,
 * and every element is a valid hex bytes string (without "0x" prefix).
 * Throws an `InvalidPropertyError` if validation fails.
 *
 * @template K - The property key type (string literal)
 * @param record - The value to validate (can be any type)
 * @param property - The property name to check for
 * @param recordName - The name of the object being validated (used in error messages)
 * @throws {InvalidPropertyError} When the property is missing, not an array, or any element is not valid BytesHexNo0x
 * @throws {never} No other errors are thrown
 *
 * @example
 * ```typescript
 * function processSignatures(data: unknown) {
 *   assertRecordBytesHexNo0xArrayProperty(data, 'signatures', 'response');
 *   data.signatures.forEach(sig => {
 *     console.log(sig); // e.g., "1234abcd..." (no 0x prefix)
 *   });
 * }
 * ```
 */
export function assertRecordBytesHexNo0xArrayProperty<K extends string>(
  record: unknown,
  property: K,
  recordName: string,
  options: { byteLength?: ByteLength } & ErrorMetadataParams,
): asserts record is RecordWithPropertyType<K, BytesHexNo0x[]> {
  assertRecordArrayProperty(record, property, recordName, options);
  const arr = record[property];
  for (let i = 0; i < arr.length; ++i) {
    if (!isBytesHexNo0x(arr[i])) {
      throw new InvalidPropertyError(
        {
          subject: recordName,
          index: i,
          property,
          expectedType: `bytes${options.byteLength ?? ""}HexNo0x`,
          type: typeof arr[i],
        },
        options,
      );
    }
  }
}

////////////////////////////////////////////////////////////////////////////////
// - isRecordUint8ArrayProperty
// - assertRecordUint8ArrayProperty
////////////////////////////////////////////////////////////////////////////////

export function isRecordUint8ArrayProperty<K extends string>(
  o: unknown,
  property: K,
): o is RecordUint8ArrayPropertyType<K> {
  if (!isRecordNonNullableProperty(o, property)) {
    return false;
  }
  return o[property] instanceof Uint8Array;
}

export function assertRecordUint8ArrayProperty<K extends string>(
  o: unknown,
  property: K,
  objName: string,
  options: ErrorMetadataParams,
): asserts o is RecordWithPropertyType<K, Uint8Array> {
  if (!isRecordUint8ArrayProperty(o, property)) {
    throw new InvalidPropertyError(
      {
        subject: objName,
        property,
        expectedType: "Uint8Array",
        type: typeofProperty(o, property),
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////
// Hex
////////////////////////////////////////////////////////////////////////////////

const HEX_CHARS: Record<string, number> = {
  "0": 0,
  "1": 1,
  "2": 2,
  "3": 3,
  "4": 4,
  "5": 5,
  "6": 6,
  "7": 7,
  "8": 8,
  "9": 9,
  a: 10,
  b: 11,
  c: 12,
  d: 13,
  e: 14,
  f: 15,
  A: 10,
  B: 11,
  C: 12,
  D: 13,
  E: 14,
  F: 15,
} as const;
Object.freeze(HEX_CHARS);

const HEX_BYTES: string[] = Array.from({ length: 256 }, (_, i) =>
  i.toString(16).padStart(2, "0"),
);
Object.freeze(HEX_BYTES);

const HEX_CHARS_CODES = new Uint8Array([
  48,
  49,
  50,
  51,
  52,
  53,
  54,
  55,
  56,
  57, // '0'-'9'
  97,
  98,
  99,
  100,
  101,
  102, // 'a'-'f'
]);

/**
 * Convert a Uint8Array to a hex string (without 0x prefix).
 *
 * @param bytes - The byte array to convert. If `undefined` or empty, returns `""`.
 * @returns A lowercase hex string with even length (2 chars per byte), without `0x` prefix.
 *
 * @example
 * bytesToHexNo0x(new Uint8Array([0x48, 0x65])) // "4865"
 * bytesToHexNo0x(new Uint8Array([0x0a]))       // "0a"
 * bytesToHexNo0x(undefined)                    // ""
 * bytesToHexNo0x(new Uint8Array([]))           // ""
 */
export function bytesToHexNo0x(bytes: Uint8Array | undefined): BytesHexNo0x {
  if (!bytes || bytes.length === 0) {
    return "" as BytesHexNo0x;
  }
  let hex = "";
  for (const byte of bytes) {
    hex += byte.toString(16).padStart(2, "0");
  }
  return hex as BytesHexNo0x;
}

/**
 * Convert a Uint8Array to a `0x`-prefixed hex string.
 *
 * @param bytes - The byte array to convert. If `undefined` or empty, returns `"0x"`.
 * @returns A lowercase `0x`-prefixed hex string with even length (2 chars per byte after prefix).
 *
 * @example
 * bytesToHex(new Uint8Array([0x48, 0x65])) // "0x4865"
 * bytesToHex(new Uint8Array([0x0a]))       // "0x0a"
 * bytesToHex(undefined)                    // "0x"
 * bytesToHex(new Uint8Array([]))           // "0x"
 */
export function bytesToHex(bytes: Uint8Array | undefined): BytesHex {
  return `0x${bytesToHexNo0x(bytes)}` as BytesHex;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Convert a 1-byte long Uint8Array to a 0x prefixed hex string (length=4)
 */
export function bytes1ToHex(bytes: unknown): Bytes1Hex {
  if (!isBytes1(bytes)) {
    throw new Error("Invalid bytes1 argument");
  }
  return `0x${bytesToHexNo0x(bytes)}` as Bytes1Hex;
}

/**
 * Convert a 8-bytes long Uint8Array to a 0x prefixed hex string (length=18)
 */
export function bytes8ToHex(bytes: unknown): Bytes8Hex {
  if (!isBytes8(bytes)) {
    throw new Error("Invalid bytes8 argument");
  }
  return `0x${bytesToHexNo0x(bytes)}` as Bytes8Hex;
}

/**
 * Convert a 20-bytes long Uint8Array to a 0x prefixed hex string (length=42)
 */
export function bytes20ToHex(bytes: unknown): Bytes20Hex {
  if (!isBytes20(bytes)) {
    throw new Error("Invalid bytes20 argument");
  }
  return `0x${bytesToHexNo0x(bytes)}` as Bytes20Hex;
}

/**
 * Convert a 21-bytes long Uint8Array to a 0x prefixed hex string (length=44)
 */
export function bytes21ToHex(bytes: unknown): Bytes21Hex {
  if (!isBytes21(bytes)) {
    throw new Error("Invalid bytes21 argument");
  }
  return `0x${bytesToHexNo0x(bytes)}` as Bytes21Hex;
}

/**
 * Convert a 32-bytes long Uint8Array to a 0x prefixed hex string (length=66).
 *
 * @param bytes - Input to convert (asserted to be Bytes32)
 * @returns Hex string with 0x prefix
 * @throws Error if bytes is not a valid 32-byte Uint8Array
 */
export function bytes32ToHex(bytes: unknown): Bytes32Hex {
  if (!isBytes32(bytes)) {
    throw new Error("Invalid bytes32 argument");
  }
  return `0x${bytesToHexNo0x(bytes)}` as Bytes32Hex;
}

/**
 * Convert a 65-bytes long Uint8Array to a 0x prefixed hex string (length=132)
 */
export function bytes65ToHex(bytes: Uint8Array | undefined): Bytes65Hex {
  if (!isBytes65(bytes)) {
    throw new Error("Invalid bytes65 argument");
  }
  return `0x${bytesToHexNo0x(bytes)}` as Bytes65Hex;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Converts a Uint8Array to a hex string, optimized for large byte arrays.
 *
 * Unlike {@link bytesToHex}, this function avoids `Array.from` and string
 * concatenation, making it more efficient for large inputs.
 *
 * @param bytes - The byte array to convert
 * @param no0x - If `true`, returns the hex string without the `0x` prefix
 * @returns The hex string representation of the bytes
 * @example bytesToHexLarge(new Uint8Array([255, 0]), false) // '0xff00'
 * @example bytesToHexLarge(new Uint8Array([255, 0]), true) // 'ff00'
 */
export function bytesToHexLarge(bytes: Uint8Array, no0x: true): BytesHexNo0x;
export function bytesToHexLarge(bytes: Uint8Array, no0x?: false): BytesHex;
export function bytesToHexLarge(
  bytes: Uint8Array,
  no0x?: boolean,
): BytesHex | BytesHexNo0x {
  const len = no0x === true ? bytes.length * 2 : bytes.length * 2 + 2;
  const out = new Uint8Array(len);

  let i0 = 0;
  if (no0x !== true) {
    out[0] = 48; // '0'
    out[1] = 120; // 'x'
    i0 = 2;
  }

  let j = i0;
  for (const byte of bytes) {
    // byte >> 4 is always 0–15, byte & 0xf is always 0–15
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    out[j] = HEX_CHARS_CODES[byte >> 4]!;
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    out[j + 1] = HEX_CHARS_CODES[byte & 0xf]!;
    j += 2;
  }

  const txt = new TextDecoder().decode(out);
  if (no0x === true) {
    return txt as BytesHexNo0x;
  } else {
    return txt as BytesHex;
  }
}

/**
 * Convert a hex string prefixed by 0x or not to a Uint8Array
 * Any invalid byte string is converted to 0
 * "0xzzff" = [0, 255]
 * "0xzfff" = [0, 255]
 */
export function hexToBytes(hexString: string): Uint8Array {
  if (hexString.length % 2 !== 0) {
    throw new Error("Invalid hex string: odd length");
  }
  const arr = hexString.replace(/^(0x)/, "").match(/.{1,2}/g);
  if (!arr) return new Uint8Array();
  return Uint8Array.from(arr.map((byte) => parseInt(byte, 16)));
}

/**
 * Convert a hex string (with or without 0x prefix) to a 1-byte Uint8Array.
 * Left-pads with zeros if shorter than 1 byte.
 * Empty string or "0x" returns 1 zero byte.
 *
 * @param hexString - Hex string with or without 0x prefix
 * @returns 1-byte Uint8Array
 * @throws Error if hex string has odd length or exceeds 2 hex characters
 *
 * @example
 * // Full 1-byte input
 * hexToBytes1("0xff") // Uint8Array [255]
 *
 * @example
 * // Empty input - returns 1 zero byte
 * hexToBytes1("")   // Uint8Array [0]
 * hexToBytes1("0x") // Uint8Array [0]
 */
export function hexToBytes1(hexString: string): Bytes1 {
  const hex = remove0x(hexString);
  if (hex.length % 2 !== 0 || hex.length > 2) {
    throw new Error("Invalid bytes1 hex string");
  }

  return hexToBytes("0x" + hex.padStart(2, "0")) as Bytes1;
}

/**
 * Convert a hex string (with or without 0x prefix) to an 8-byte Uint8Array.
 * Left-pads with zeros if shorter than 8 bytes.
 * Empty string or "0x" returns 8 zero bytes.
 *
 * @param hexString - Hex string with or without 0x prefix
 * @returns 8-byte Uint8Array
 * @throws Error if hex string has odd length or exceeds 16 hex characters
 *
 * @example
 * // Full 8-byte input
 * hexToBytes8("0x0102030405060708") // Uint8Array [1,2,3,4,5,6,7,8]
 *
 * @example
 * // Short input - left-padded with zeros
 * hexToBytes8("0x1234") // Uint8Array [0,0,0,0,0,0,0x12,0x34]
 * hexToBytes8("1234")   // Same result, 0x prefix optional
 * hexToBytes8("0xff")   // Uint8Array [0,0,0,0,0,0,0,255]
 *
 * @example
 * // Empty input - returns 8 zero bytes
 * hexToBytes8("")   // Uint8Array [0,0,0,0,0,0,0,0]
 * hexToBytes8("0x") // Uint8Array [0,0,0,0,0,0,0,0]
 */
export function hexToBytes8(hexString: string): Bytes8 {
  const hex = remove0x(hexString);
  if (hex.length % 2 !== 0 || hex.length > 16) {
    throw new Error("Invalid bytes8 hex string");
  }

  return hexToBytes("0x" + hex.padStart(16, "0")) as Bytes8;
}

/**
 * Convert a hex string (with or without 0x prefix) to a 20-byte Uint8Array.
 * Left-pads with zeros if shorter than 20 bytes.
 * Empty string or "0x" returns 20 zero bytes.
 *
 * @param hexString - Hex string with or without 0x prefix
 * @returns 20-byte Uint8Array (suitable for Ethereum addresses)
 * @throws Error if hex string has odd length or exceeds 40 hex characters
 *
 * @example
 * // Full 20-byte address
 * hexToBytes20("0x742d35Cc6634C0532925a3b844Bc9e7595f8fE00")
 * // Uint8Array [0x74,0x2d,0x35,...,0xf8,0xfe,0x00]
 *
 * @example
 * // Short input - left-padded with zeros
 * hexToBytes20("0x1234") // Uint8Array [0,0,...,0,0x12,0x34] (18 zeros + 2 bytes)
 * hexToBytes20("0xff")   // Uint8Array [0,0,...,0,255] (19 zeros + 1 byte)
 *
 * @example
 * // Empty input - returns 20 zero bytes
 * hexToBytes20("")   // Uint8Array [0,0,0,...,0] (20 zeros)
 * hexToBytes20("0x") // Uint8Array [0,0,0,...,0] (20 zeros)
 */
export function hexToBytes20(hexString: string): Bytes20 {
  const hex = remove0x(hexString);
  if (hex.length % 2 !== 0 || hex.length > 40) {
    throw new Error("Invalid bytes20 hex string");
  }

  return hexToBytes("0x" + hex.padStart(40, "0")) as Bytes20;
}

/**
 * Convert a hex string (with or without 0x prefix) to a 21-byte Uint8Array.
 * Left-pads with zeros if shorter than 21 bytes.
 * Empty string or "0x" returns 21 zero bytes.
 *
 * @param hexString - Hex string with or without 0x prefix
 * @returns 21-byte Uint8Array
 * @throws Error if hex string has odd length or exceeds 42 hex characters
 *
 * @example
 * // Full 21-byte input
 * hexToBytes21("0x01" + "00".repeat(20)) // Uint8Array [1,0,0,...,0] (21 bytes)
 *
 * @example
 * // Short input - left-padded with zeros
 * hexToBytes21("0x1234") // Uint8Array [0,0,...,0,0x12,0x34] (19 zeros + 2 bytes)
 * hexToBytes21("0xff")   // Uint8Array [0,0,...,0,255] (20 zeros + 1 byte)
 *
 * @example
 * // Empty input - returns 21 zero bytes
 * hexToBytes21("")   // Uint8Array [0,0,0,...,0] (21 zeros)
 * hexToBytes21("0x") // Uint8Array [0,0,0,...,0] (21 zeros)
 */
export function hexToBytes21(hexString: string): Bytes21 {
  const hex = remove0x(hexString);
  if (hex.length % 2 !== 0 || hex.length > 42) {
    throw new Error("Invalid bytes21 hex string");
  }

  return hexToBytes("0x" + hex.padStart(42, "0")) as Bytes21;
}

/**
 * Convert a hex string (with or without 0x prefix) to a 32-byte Uint8Array.
 * Left-pads with zeros if shorter than 32 bytes.
 * Empty string or "0x" returns 32 zero bytes.
 *
 * @param hexString - Hex string with or without 0x prefix
 * @returns 32-byte Uint8Array (suitable for hashes, private keys, etc.)
 * @throws Error if hex string has odd length or exceeds 64 hex characters
 *
 * @example
 * // Full 32-byte hash
 * hexToBytes32("0xabcdef..." /* 64 hex chars *\/) // Uint8Array [0xab,0xcd,0xef,...]
 *
 * @example
 * // Short input - left-padded with zeros (useful for encoding small numbers)
 * hexToBytes32("0x1234") // Uint8Array [0,0,...,0,0x12,0x34] (30 zeros + 2 bytes)
 * hexToBytes32("0x01")   // Uint8Array [0,0,...,0,1] (31 zeros + 1 byte)
 * hexToBytes32("0xff")   // Uint8Array [0,0,...,0,255] (31 zeros + 1 byte)
 *
 * @example
 * // Empty input - returns 32 zero bytes
 * hexToBytes32("")   // Uint8Array [0,0,0,...,0] (32 zeros)
 * hexToBytes32("0x") // Uint8Array [0,0,0,...,0] (32 zeros)
 */
export function hexToBytes32(hexString: string): Bytes32 {
  const hex = remove0x(hexString);
  if (hex.length % 2 !== 0 || hex.length > 64) {
    throw new Error("Invalid bytes32 hex string");
  }

  return hexToBytes("0x" + hex.padStart(64, "0")) as Bytes32;
}

/**
 * Convert a hex string (with or without 0x prefix) to a 65-byte Uint8Array.
 * Left-pads with zeros if shorter than 65 bytes.
 * Empty string or "0x" returns 65 zero bytes.
 *
 * @param hexString - Hex string with or without 0x prefix
 * @returns 65-byte Uint8Array (suitable for ECDSA signatures with recovery byte)
 * @throws Error if hex string has odd length or exceeds 130 hex characters
 *
 * @example
 * // Full 65-byte signature (r + s + v)
 * hexToBytes65("0x" + "ab".repeat(65)) // Uint8Array [0xab,0xab,...] (65 bytes)
 *
 * @example
 * // Short input - left-padded with zeros
 * hexToBytes65("0x1234") // Uint8Array [0,0,...,0,0x12,0x34] (63 zeros + 2 bytes)
 * hexToBytes65("0xff")   // Uint8Array [0,0,...,0,255] (64 zeros + 1 byte)
 *
 * @example
 * // Empty input - returns 65 zero bytes
 * hexToBytes65("")   // Uint8Array [0,0,0,...,0] (65 zeros)
 * hexToBytes65("0x") // Uint8Array [0,0,0,...,0] (65 zeros)
 */
export function hexToBytes65(hexString: string): Bytes65 {
  const hex = remove0x(hexString);
  if (hex.length % 2 !== 0 || hex.length > 130) {
    throw new Error("Invalid bytes65 hex string");
  }

  return hexToBytes("0x" + hex.padStart(130, "0")) as Bytes65;
}

/**
 * Convert a hex string prefixed by 0x or not to a Uint8Array
 */
export function hexToBytesFaster(
  hexString: string,
  options?: {
    strict?: boolean;
    byteLength?: ByteLength;
    subject?: string;
  } & ErrorMetadataParams,
): Uint8Array {
  const strict = options?.strict === true;
  // eslint-disable-next-line @typescript-eslint/prefer-string-starts-ends-with
  const offset = hexString[0] === "0" && hexString[1] === "x" ? 2 : 0;
  const len = hexString.length - offset;

  if (len % 2 !== 0) {
    throw new InvalidTypeError(
      {
        subject: options?.subject,
        expectedType: "bytesHex",
        metaMessages: ["hex string length must be even"],
      },
      options ?? {},
    );
  }

  if (options?.byteLength !== undefined) {
    if (len !== options.byteLength) {
      throw new InvalidTypeError(
        {
          subject: options.subject,
          expectedType: "bytesHex",
          metaMessages: [
            `expected ${options.byteLength} bytes, got ${len / 2}`,
          ],
        },
        options,
      );
    }
  }

  const bytes = new Uint8Array(len / 2);
  for (let i = 0; i < bytes.length; i++) {
    // If index is out of bounds, hexString[] returns undefined which
    // propagates through HEX_CHARS as undefined, caught by the check below.
    /* eslint-disable @typescript-eslint/non-nullable-type-assertion-style */
    const hi = HEX_CHARS[hexString[offset + i * 2] as string];
    const lo = HEX_CHARS[hexString[offset + i * 2 + 1] as string];
    /* eslint-enable @typescript-eslint/non-nullable-type-assertion-style */
    if (hi === undefined || lo === undefined) {
      if (strict) {
        throw new InvalidTypeError(
          {
            subject: options.subject,
            expectedType: "bytesHex",
            metaMessages: [
              `invalid hex character at position ${offset + i * 2}`,
            ],
          },
          options,
        );
      }
      bytes[i] = ((hi ?? 0) << 4) | (lo ?? 0);
      continue;
    }
    bytes[i] = (hi << 4) | lo;
  }

  return bytes;
}

/**
 * Convert a Uint8Array to a bigint (big-endian interpretation).
 * The most significant byte is at index 0.
 *
 * @param byteArray - The byte array to convert (big-endian)
 * @returns The bigint representation of the bytes
 *
 * @example
 * // Standard conversion (big-endian)
 * bytesToBigInt(new Uint8Array([0x01, 0x00])) // 256n (0x0100)
 * bytesToBigInt(new Uint8Array([0xff]))       // 255n
 * bytesToBigInt(new Uint8Array([0x01, 0x02, 0x03])) // 66051n (0x010203)
 *
 * @example
 * // Edge cases
 * bytesToBigInt(new Uint8Array([]))  // 0n
 * bytesToBigInt(new Uint8Array([0])) // 0n
 * bytesToBigInt(undefined)           // 0n
 *
 * @example
 * // 32-byte value (e.g., uint256)
 * const bytes32 = new Uint8Array(32);
 * bytes32[31] = 0x2a; // 42 in last byte
 * bytesToBigInt(bytes32) // 42n
 */
export function bytesToBigInt(byteArray: Uint8Array | undefined): UintBigInt {
  if (!byteArray || byteArray.length === 0) {
    return BigInt(0) as UintBigInt;
  }
  let result = BigInt(0);
  for (const byte of byteArray) {
    result = (result << BigInt(8)) | BigInt(byte);
  }
  return result as UintBigInt;
}

/**
 * Converts a non-negative bigint to a `0x`-prefixed hex string.
 *
 * When `byteLength` is provided, the output is zero-padded to exactly
 * that many bytes and an overflow check is performed.
 * When omitted, the output is the minimal even-length hex representation.
 *
 * @param value - Non-negative bigint to convert
 * @param options.byteLength - Fixed byte width (enables zero-padding and overflow check)
 * @param options.subject - Name used in error messages
 * @returns `0x`-prefixed hex string
 * @throws {InvalidTypeError} If value is negative or exceeds the byte length
 *
 * @example
 * ```typescript
 * bigIntToBytesHex(0n)                          // '0x00'
 * bigIntToBytesHex(255n)                        // '0xff'
 * bigIntToBytesHex(256n)                        // '0x0100'
 * bigIntToBytesHex(1n, { byteLength: 4 })      // '0x00000001'
 * bigIntToBytesHex(255n, { byteLength: 1 })    // '0xff'
 * bigIntToBytesHex(256n, { byteLength: 1 })    // throws (overflow)
 * bigIntToBytesHex(-1n)                         // throws (negative)
 * ```
 */
export function bigIntToBytesHex(
  value: bigint,
  options: { byteLength: 1; subject?: string } & ErrorMetadataParams,
): Bytes1Hex;
export function bigIntToBytesHex(
  value: bigint,
  options: { byteLength: 8; subject?: string } & ErrorMetadataParams,
): Bytes8Hex;
export function bigIntToBytesHex(
  value: bigint,
  options: { byteLength: 20; subject?: string } & ErrorMetadataParams,
): Bytes20Hex;
export function bigIntToBytesHex(
  value: bigint,
  options: { byteLength: 32; subject?: string } & ErrorMetadataParams,
): Bytes32Hex;
export function bigIntToBytesHex(
  value: bigint,
  options?: {
    byteLength?: 1 | 2 | 4 | 8 | 16 | 20 | 32;
    subject?: string;
  } & ErrorMetadataParams,
): BytesHex;
export function bigIntToBytesHex(
  value: bigint,
  options?: {
    byteLength?: 1 | 2 | 4 | 8 | 16 | 20 | 32;
    subject?: string;
  } & ErrorMetadataParams,
): BytesHex {
  const byteLength = options?.byteLength;

  // checks overflow and negative values
  if (!isUintForByteLength(value, byteLength)) {
    throw new InvalidTypeError(
      {
        subject: options?.subject,
        type: typeof value,
        expectedType: "uintBigInt",
      },
      options ?? {},
    );
  }

  let v = value.toString(16);

  if (byteLength !== undefined) {
    v = v.padStart(byteLength * 2, "0");
  }

  if (v.length % 2 !== 0) {
    return ("0x0" + v) as BytesHex;
  }

  return ("0x" + v) as BytesHex;
}

/**
 * Converts an array of Bytes32 or Bytes32Hex values to a uniform Bytes32Hex array.
 * Accepts mixed input: both 32-byte Uint8Arrays and hex strings are normalized to Bytes32Hex.
 *
 * @param arr - Array of Bytes32 (Uint8Array) or Bytes32Hex (string) or Bytes32HexAble values.
 * @returns Array of Bytes32Hex strings.
 */
export function toBytes32HexArray(arr: readonly unknown[]): Bytes32Hex[] {
  return arr.map((b) => {
    if (typeof b === "string") {
      return asBytes32Hex(b);
    } else if (isBytes(b)) {
      return bytesToHexLarge(asBytes32(b)) as Bytes32Hex;
    } else if (typeof b === "object" && b !== null && "bytes32Hex" in b) {
      // Bytes32HexAble
      return asBytes32Hex((b as Record<string, unknown>).bytes32Hex);
    } else {
      throw new InvalidTypeError(
        {
          expectedType: "bytes32Hex[]",
        },
        {},
      );
    }
  });
}

/**
 * Converts a Bytes32 or Bytes32Hex values to a Bytes32.
 * Accepts mixed input: both 32-byte Uint8Arrays and hex strings are normalized to Bytes32.
 */
export function toBytes32(value: unknown): Bytes32 {
  if (isBytes(value)) {
    return asBytes32(value);
  } else if (typeof value === "string") {
    return hexToBytes32(asBytes32Hex(value));
  } else if (
    // Bytes32Able
    typeof value === "object" &&
    value !== null &&
    "bytes32" in value
  ) {
    return asBytes32((value as Record<string, unknown>).bytes32);
  } else {
    throw new InvalidTypeError(
      {
        expectedType: "bytes32",
      },
      {},
    );
  }
}

/**
 * Converts a Bytes or BytesHex values to a Bytes.
 * Accepts mixed input: both n-byte Uint8Arrays and hex strings are normalized to Bytes.
 */
export function toBytes(
  value: unknown,
  options?: {
    byteLength?: ByteLength;
    subject?: string;
    copy?: boolean;
  } & ErrorMetadataParams,
): Bytes {
  if (isBytes(value)) {
    if (options?.copy === true) {
      return new Uint8Array(value);
    }
    return value;
  } else if (typeof value === "string") {
    return hexToBytesFaster(value, { strict: true });
  } else {
    throw new InvalidTypeError(
      {
        subject: options?.subject,
        expectedType: ["bytes", "bytesHex", "bytesHexNo0x"],
      },
      options ?? {},
    );
  }
}

export function concatBytes(...arrays: Uint8Array[]): Uint8Array {
  let totalLength = 0;
  for (const arr of arrays) {
    totalLength += arr.length;
  }

  const result = new Uint8Array(totalLength);
  let offset = 0;

  for (const arr of arrays) {
    result.set(arr, offset);
    offset += arr.length;
  }

  return result;
}

export function concatBytesHex(values: BytesHex[]): BytesHex {
  return ("0x" + values.map((v) => v.substring(2)).join("")) as BytesHex;
}

/**
 * Compares two byte arrays for equality.
 *
 * **⚠️ WARNING: NOT TIMING-SAFE**
 *
 * This function uses early-exit comparison which leaks timing information
 * about where the first difference occurs. Do NOT use for comparing:
 * - Secret keys or passwords
 * - HMACs or authentication tags
 * - Any security-sensitive data
 *
 * For security-sensitive comparisons, use a timing-safe equals function instead.
 *
 * @param a - First byte array
 * @param b - Second byte array
 * @returns true if arrays are equal, false otherwise
 */
export function unsafeBytesEquals(a: unknown, b: unknown): boolean {
  if (!isBytes(a) || !isBytes(b)) {
    return false;
  }
  if (a.length !== b.length) {
    return false;
  }
  for (let i = 0; i < a.length; ++i) {
    if (a[i] !== b[i]) {
      return false;
    }
  }
  return true;
}

/**
 * Normalizes byte-like values to a Uint8Array.
 *
 * Handles both same-realm and cross-realm inputs (e.g., from iframes or
 * different JavaScript contexts).
 *
 * Supported input types:
 * - `Uint8Array` - returned as-is (same-realm) or re-wrapped (cross-realm)
 * - `ArrayBuffer` - wrapped in a new Uint8Array
 *
 * Other TypedArray types (Uint16Array, Int32Array, etc.) are explicitly
 * rejected to prevent silent endianness-dependent byte reinterpretation.
 *
 * @param value - The value to normalize
 * @returns A `Uint8Array` representing the bytes
 * @throws `TypeError` If the value is not a Uint8Array or ArrayBuffer
 *
 * @example
 * ```ts
 * normalizeBytes(new Uint8Array([1, 2, 3]));        // ✅ returns as-is
 * normalizeBytes(new ArrayBuffer(8));               // ✅ wraps in Uint8Array
 * normalizeBytes(new Uint16Array([1, 2]));          // ❌ throws TypeError
 * normalizeBytes(iframeUint8Array);                 // ✅ cross-realm supported
 * ```
 */
export function normalizeBytes(value: unknown): Bytes {
  // same realm Uint8Array
  if (value instanceof Uint8Array) return value;
  // same realm ArrayBuffer
  if (value instanceof ArrayBuffer) return new Uint8Array(value);

  const tag = Object.prototype.toString.call(value);

  // cross realm Uint8Array
  if (ArrayBuffer.isView(value) && tag === "[object Uint8Array]") {
    return new Uint8Array(value.buffer, value.byteOffset, value.byteLength);
  }
  // cross realm ArrayBuffer
  if (tag === "[object ArrayBuffer]") {
    return new Uint8Array(value as ArrayBuffer);
  }

  throw new TypeError(`Unsupported bytes type: ${tag}`);
}

/**
 * Extracts a single byte from a bytes array at the given byte position.
 * @param bytes - The bytes array (Uint8Array)
 * @param position - Byte position (0-indexed)
 * @returns The byte value as Uint8Number (0-255)
 * @throws RangeError if position is out of bounds
 */
export function bytesUint8At(bytes: Bytes, position: Uint8Number): Uint8Number {
  if (position >= bytes.length) {
    throw new RangeError(
      `Position ${position} out of bounds for bytes of length ${bytes.length}`,
    );
  }
  return bytes[position] as Uint8Number;
}

/**
 * Extracts a single byte from a 0x-hex string at the given byte position.
 * @param bytesHex - The hex string (0x-prefixed)
 * @param position - Byte position (0-indexed)
 * @throws `RangeError` if position is out of bounds
 */
export function bytesHexUint8At(
  bytesHex: BytesHex,
  position: Uint8Number,
): Uint8Number {
  const bytes1Hex: Bytes1Hex = bytesHexSlice(bytesHex, position, 1);
  return parseInt(bytes1Hex, 16) as Uint8Number;
}

/**
 * Extracts a Uint64 (8 bytes) from a 0x-hex string at the given byte position.
 * @param bytesHex - The hex string (0x-prefixed)
 * @param position - Byte position (0-indexed)
 * @throws `RangeError` if position is out of bounds
 */
export function bytesHexUint64At(
  bytesHex: BytesHex,
  position: Uint8Number,
): Uint64BigInt {
  const bytes8Hex: Bytes8Hex = bytesHexSlice(bytesHex, position, 8);
  return BigInt(bytes8Hex) as Uint64BigInt;
}

/**
 * Extracts a slice of bytes from a hex string starting at the given byte position.
 * @param bytesHex - The hex string (0x-prefixed)
 * @param position - Starting byte position (0-indexed)
 * @param length - Number of bytes to extract (1, 8, 20, 21, 32, 65 for typed result, or any number for BytesHex)
 * @throws `RangeError` if position + length is out of bounds
 */
export function bytesHexSlice(
  bytesHex: BytesHex,
  position: Uint8Number,
  length: 1,
): Bytes1Hex;
export function bytesHexSlice(
  bytesHex: BytesHex,
  position: Uint8Number,
  length: 8,
): Bytes8Hex;
export function bytesHexSlice(
  bytesHex: BytesHex,
  position: Uint8Number,
  length: 20,
): Bytes20Hex;
export function bytesHexSlice(
  bytesHex: BytesHex,
  position: Uint8Number,
  length: 21,
): Bytes21Hex;
export function bytesHexSlice(
  bytesHex: BytesHex,
  position: Uint8Number,
  length: 32,
): Bytes32Hex;
export function bytesHexSlice(
  bytesHex: BytesHex,
  position: Uint8Number,
  length: 65,
): Bytes65Hex;
export function bytesHexSlice(
  bytesHex: BytesHex,
  position: Uint8Number,
  length: UintNumber,
): BytesHex;
export function bytesHexSlice(
  bytesHex: BytesHex,
  position: Uint8Number,
  length: number,
): BytesHex {
  const index = 2 + position * 2;
  if (index + 2 * length > bytesHex.length) {
    throw new RangeError(
      `Position ${position} with length ${length} out of bounds`,
    );
  }
  return `0x${bytesHex.slice(index, index + 2 * length)}` as BytesHex;
}
