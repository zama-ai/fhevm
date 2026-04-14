/* eslint-disable @typescript-eslint/naming-convention */

import type { Prettify } from './utils.js';

declare const __bytes1: unique symbol;
declare const __bytes2: unique symbol;
declare const __bytes4: unique symbol;
declare const __bytes8: unique symbol;
declare const __bytes16: unique symbol;
declare const __bytes20: unique symbol;
declare const __bytes21: unique symbol;
declare const __bytes32: unique symbol;
declare const __bytes65: unique symbol;

////////////////////////////////////////////////////////////////////////////////

declare const __bits8: unique symbol;
declare const __bits16: unique symbol;
declare const __bits32: unique symbol;
declare const __bits64: unique symbol;
declare const __bits128: unique symbol;
declare const __bits160: unique symbol;
declare const __bits256: unique symbol;

////////////////////////////////////////////////////////////////////////////////

declare const __uint: unique symbol;

////////////////////////////////////////////////////////////////////////////////

export type UnsignedInt = { readonly [__uint]: never };

export type Bits8 = { readonly [__bits8]: never };
export type Bits16 = { readonly [__bits16]: never };
export type Bits32 = { readonly [__bits32]: never };
export type Bits64 = { readonly [__bits64]: never };
export type Bits128 = { readonly [__bits128]: never };
export type Bits160 = { readonly [__bits160]: never };
export type Bits256 = { readonly [__bits256]: never };

export type ByteLen1 = { readonly [__bytes1]: never };
export type ByteLen2 = { readonly [__bytes2]: never };
export type ByteLen4 = { readonly [__bytes4]: never };
export type ByteLen8 = { readonly [__bytes8]: never };
export type ByteLen20 = { readonly [__bytes20]: never };
export type ByteLen21 = { readonly [__bytes21]: never };
export type ByteLen32 = { readonly [__bytes32]: never };
export type ByteLen65 = { readonly [__bytes65]: never };

////////////////////////////////////////////////////////////////////////////////

/**
 * Unsigned integer represented as a JavaScript number.
 *
 * Note: JavaScript numbers are 64-bit floats, so this is only safe for
 * integers up to Number.MAX_SAFE_INTEGER (2^53 - 1).
 */
export type UnsignedIntNumber = number & UnsignedInt;

/**
 * Unsigned integer represented as a JavaScript bigint.
 */
export type UnsignedIntBigInt = bigint & UnsignedInt;

////////////////////////////////////////////////////////////////////////////////

/**
 * 8-bits Unsigned integer represented as a JavaScript number.
 */
export type Uint8Number = UnsignedIntNumber & Bits8;

/**
 * 16-bits Unsigned integer represented as a JavaScript number.
 */
export type Uint16Number = UnsignedIntNumber & Bits16;

/**
 * 32-bits Unsigned integer represented as a JavaScript number.
 */
export type Uint32Number = UnsignedIntNumber & Bits32;

/**
 * Maps unsigned integer type names to their `number` representation.
 *
 * Only includes widths that fit safely in a JS number (up to 32 bits).
 */
export type UintNumberMap = Readonly<{
  uint8: Uint8Number;
  uint16: Uint16Number;
  uint32: Uint32Number;
}>;

/**
 * Union of all unsigned integer number types.
 */
export type UintNumber = UintNumberMap[keyof UintNumberMap];

////////////////////////////////////////////////////////////////////////////////

/**
 * 8-bits Unsigned integer represented as a JavaScript bigint.
 */
export type Uint8BigInt = UnsignedIntBigInt & Bits8;

/**
 * 16-bits Unsigned integer represented as a JavaScript bigint.
 */
export type Uint16BigInt = UnsignedIntBigInt & Bits16;

/**
 * 32-bits Unsigned integer represented as a JavaScript bigint.
 */
export type Uint32BigInt = UnsignedIntBigInt & Bits32;

/**
 * 64-bits Unsigned integer represented as a JavaScript bigint.
 */
export type Uint64BigInt = UnsignedIntBigInt & Bits64;

/**
 * 128-bits Unsigned integer represented as a JavaScript bigint.
 */
export type Uint128BigInt = UnsignedIntBigInt & Bits128;

/**
 * 160-bits Unsigned integer represented as a JavaScript bigint.
 */
export type Uint160BigInt = UnsignedIntBigInt & Bits160;

/**
 * 256-bits Unsigned integer represented as a JavaScript bigint.
 */
export type Uint256BigInt = UnsignedIntBigInt & Bits256;

/**
 * Maps unsigned integer type names to their `bigint` representation.
 *
 * Includes all widths (8–256 bits).
 */
export type UintBigIntMap = Readonly<{
  uint8: Uint8BigInt;
  uint16: Uint16BigInt;
  uint32: Uint32BigInt;
  uint64: Uint64BigInt;
  uint128: Uint128BigInt;
  uint160: Uint160BigInt;
  uint256: Uint256BigInt;
}>;

/**
 * Union of all unsigned integer bigint types.
 */
export type UintBigInt = UintBigIntMap[keyof UintBigIntMap];

////////////////////////////////////////////////////////////////////////////////

/**
 * Maps each unsigned integer type name to its normalized TypeScript representation.
 *
 * - `uint8`, `uint16`, `uint32` → `number` (fits safely in JS number)
 * - `uint64`, `uint128`, `uint160`, `uint256` → `bigint` (exceeds JS number precision)
 */
export type UintNormalizedMap = Readonly<{
  uint8: Uint8Number;
  uint16: Uint16Number;
  uint32: Uint32Number;
  uint64: Uint64BigInt;
  uint128: Uint128BigInt;
  uint160: Uint160BigInt;
  uint256: Uint256BigInt;
}>;

/**
 * Union of all normalized unsigned integer types.
 */
export type UintNormalized = UintNormalizedMap[keyof UintNormalizedMap];

////////////////////////////////////////////////////////////////////////////////

/**
 * String literal union of normalized unsigned integer type names.
 *
 * `"uint8" | "uint16" | "uint32" | "uint64" | "uint128" | "uint160" | "uint256"`
 */
export type UintTypeName = Prettify<keyof UintNormalizedMap>;

////////////////////////////////////////////////////////////////////////////////
//
// UintN
//
// Fixed-width unsigned integer types. Smaller widths (8, 16, 32) can be
// represented as JavaScript numbers or bigints. Larger widths (64+) require
// bigints as they exceed Number.MAX_SAFE_INTEGER.
//
////////////////////////////////////////////////////////////////////////////////

/**
 * 8-bits Unsigned integer.
 */
export type Uint8 = Uint8Number | Uint8BigInt;

/**
 * 16-bits Unsigned integer.
 */
export type Uint16 = Uint8 | Uint16Number | Uint16BigInt;

/**
 * 32-bits Unsigned integer.
 */
export type Uint32 = Uint16 | Uint32Number | Uint32BigInt;

/**
 * 64-bits Unsigned integer.
 */
export type Uint64 = Uint32 | Uint64BigInt;

/**
 * 128-bits Unsigned integer.
 */
export type Uint128 = Uint64 | Uint128BigInt;

/**
 * 160-bits Unsigned integer.
 */
export type Uint160 = Uint128 | Uint160BigInt;

/**
 * 256-bits Unsigned integer.
 */
export type Uint256 = Uint160 | Uint256BigInt;

/**
 * Maps unsigned integer type names to their full type (`number | bigint`).
 *
 * Each value includes all narrower widths (e.g., `Uint16` accepts `Uint8` values).
 */
export type UintMap = Readonly<{
  uint8: Uint8;
  uint16: Uint16;
  uint32: Uint32;
  uint64: Uint64;
  uint128: Uint128;
  uint160: Uint160;
  uint256: Uint256;
}>;

/**
 * Union of all unsigned integer types.
 */
export type Uint = UintMap[keyof UintMap];

export type UintBitsMap = Readonly<{
  uint8: 8;
  uint16: 16;
  uint32: 32;
  uint64: 64;
  uint128: 128;
  uint160: 160;
  uint256: 256;
}>;

////////////////////////////////////////////////////////////////////////////////
//
// Hexadecimal string
//
////////////////////////////////////////////////////////////////////////////////

declare const __hex0x: unique symbol;
declare const __hexNo0x: unique symbol;
declare const __evenLen: unique symbol;

export type Hex0xString = { readonly [__hex0x]: never };
export type HexNo0xString = { readonly [__hexNo0x]: never };
export type EvenLen = { readonly [__evenLen]: never };

/**
 * A 0x-prefixed hexadecimal string.
 *
 * The length can be odd or even (e.g., `0x1` or `0x01` are both valid).
 */
export type Hex0x = `0x${string}` & Hex0xString;

/**
 * A hexadecimal string without `0x` prefix.
 *
 * The length can be odd or even (e.g., `"1"` or `"01"` are both valid).
 */
export type HexNo0x = string & HexNo0xString;

////////////////////////////////////////////////////////////////////////////////
//
// Hexadecimal bytes string
//
////////////////////////////////////////////////////////////////////////////////

/**
 * A 0x-prefixed hexadecimal string representing byte data.
 *
 * The length must be even (excluding the `0x` prefix) since each byte is
 * represented by two hex characters. Use `Hex` if odd-length strings are acceptable.
 *
 * @example
 * const data: BytesHex = '0x48656c6c6f'; // "Hello" in hex
 */
export type BytesHex = Hex0x & EvenLen;

/**
 * A hexadecimal string representing byte data without the `0x` prefix.
 * @see {@link BytesHex}
 */
export type BytesHexNo0x = HexNo0x & EvenLen;

/**
 * A 0x-prefixed hexadecimal string representing exactly 1 byte (4 characters including the prefix).
 */
export type Bytes1Hex = BytesHex & ByteLen1;

/**
 * A hexadecimal string representing exactly 1 byte without the `0x` prefix (2 characters).
 */
export type Bytes1HexNo0x = BytesHexNo0x & ByteLen1;

/**
 * A 0x-prefixed hexadecimal string representing exactly 2 bytes (6 characters including the prefix).
 */
export type Bytes2Hex = BytesHex & ByteLen2;

/**
 * A hexadecimal string representing exactly 2 bytes without the `0x` prefix (4 characters).
 */
export type Bytes2HexNo0x = BytesHexNo0x & ByteLen2;

/**
 * A 0x-prefixed hexadecimal string representing exactly 4 bytes (10 characters including the prefix).
 */
export type Bytes4Hex = BytesHex & ByteLen4;

/**
 * A hexadecimal string representing exactly 4 bytes without the `0x` prefix (8 characters).
 */
export type Bytes4HexNo0x = BytesHexNo0x & ByteLen4;

/**
 * A 0x-prefixed hexadecimal string representing exactly 8 bytes (18 characters including the prefix).
 */
export type Bytes8Hex = BytesHex & ByteLen8;

/**
 * A hexadecimal string representing exactly 8 bytes without the `0x` prefix (16 characters).
 */
export type Bytes8HexNo0x = BytesHexNo0x & ByteLen8;

/**
 * A 0x-prefixed hexadecimal string representing exactly 20 bytes (42 characters including the prefix).
 */
export type Bytes20Hex = BytesHex & ByteLen20;

/**
 * A hexadecimal string representing exactly 20 bytes without the `0x` prefix (40 characters).
 */
export type Bytes20HexNo0x = BytesHexNo0x & ByteLen20;

/**
 * A 0x-prefixed hexadecimal string representing exactly 21 bytes (44 characters including the prefix).
 */
export type Bytes21Hex = BytesHex & ByteLen21;

/**
 * A hexadecimal string representing exactly 21 bytes without the `0x` prefix (42 characters).
 */
export type Bytes21HexNo0x = BytesHexNo0x & ByteLen21;

/**
 * A 0x-prefixed hexadecimal string representing exactly 32 bytes (66 characters including the prefix).
 */
export type Bytes32Hex = BytesHex & ByteLen32;

/**
 * A hexadecimal string representing exactly 32 bytes without the `0x` prefix (64 characters).
 */
export type Bytes32HexNo0x = BytesHexNo0x & ByteLen32;

/**
 * A 0x-prefixed hexadecimal string representing exactly 65 bytes (132 characters including the prefix).
 */
export type Bytes65Hex = BytesHex & ByteLen65;

/**
 * A hexadecimal string representing exactly 65 bytes without the `0x` prefix (130 characters).
 */
export type Bytes65HexNo0x = BytesHexNo0x & ByteLen65;

export type BytesNHexMap = Readonly<{
  bytes1: Bytes1Hex;
  bytes2: Bytes2Hex;
  bytes4: Bytes4Hex;
  bytes8: Bytes8Hex;
  bytes20: Bytes20Hex;
  bytes21: Bytes21Hex;
  bytes32: Bytes32Hex;
  bytes65: Bytes65Hex;
}>;

/**
 * Union of all fixed-length `0x`-prefixed hex byte string types.
 */
export type BytesNHex = BytesNHexMap[keyof BytesNHexMap];

export type BytesNHexNo0xMap = Readonly<{
  bytes1: Bytes1HexNo0x;
  bytes2: Bytes2HexNo0x;
  bytes4: Bytes4HexNo0x;
  bytes8: Bytes8HexNo0x;
  bytes20: Bytes20HexNo0x;
  bytes21: Bytes21HexNo0x;
  bytes32: Bytes32HexNo0x;
  bytes65: Bytes65HexNo0x;
}>;

/**
 * Union of all fixed-length hex byte string types without the `0x` prefix.
 */
export type BytesNHexNo0x = BytesNHexNo0xMap[keyof BytesNHexNo0xMap];

export interface Bytes32HexAble {
  readonly bytes32Hex: Bytes32Hex;
}

export interface Bytes32Able {
  readonly bytes32: Bytes32;
}

////////////////////////////////////////////////////////////////////////////////
//
// Bytes
//
////////////////////////////////////////////////////////////////////////////////

export type Bytes = Uint8Array;
export type Bytes1 = Bytes & ByteLen1;
export type Bytes2 = Bytes & ByteLen2;
export type Bytes4 = Bytes & ByteLen4;
export type Bytes8 = Bytes & ByteLen8;
export type Bytes20 = Bytes & ByteLen20;
export type Bytes21 = Bytes & ByteLen21;
export type Bytes32 = Bytes & ByteLen32;
export type Bytes65 = Bytes & ByteLen65;

export type BytesNMap = Readonly<{
  bytes1: Bytes1;
  bytes2: Bytes2;
  bytes4: Bytes4;
  bytes8: Bytes8;
  bytes20: Bytes20;
  bytes21: Bytes21;
  bytes32: Bytes32;
  bytes65: Bytes65;
}>;

/**
 * Union of all fixed-length byte types.
 */
export type BytesTypeName = Prettify<keyof BytesNMap>;

/**
 * **Single Source of Truth**: Canonical mapping of byte lengths to their type names.
 *
 * This is the primary definition from which all other byte length types are derived.
 * To add a new byte length:
 * 1. Add entry here (e.g., `readonly 16: 'bytes16'`)
 * 2. Add branded type above (e.g., `export type Bytes16 = Bytes & ByteLen16`)
 * 3. All other types will be automatically derived
 */
export type ByteLengthToBytesTypeNameMap = Readonly<{
  1: 'bytes1';
  2: 'bytes2';
  4: 'bytes4';
  8: 'bytes8';
  20: 'bytes20';
  21: 'bytes21';
  32: 'bytes32';
  65: 'bytes65';
}>;

/**
 * Maps byte lengths to their corresponding branded types (8 -> Bytes8, etc.).
 */
export type ByteLengthToBytesTypeMap = Readonly<{
  1: Bytes1;
  2: Bytes2;
  4: Bytes4;
  8: Bytes8;
  20: Bytes20;
  21: Bytes21;
  32: Bytes32;
  65: Bytes65;
}>;

/**
 * Union of valid byte lengths (8 | 20 | 21 | 32 | 65).
 * Automatically derived from ByteLengthToTypeNameMapInternal.
 */
export type ByteLength = Prettify<keyof ByteLengthToBytesTypeNameMap>;

/**
 * Maps BytesN type names to their corresponding byte lengths ('bytes8' -> 8, etc.).
 */
export type BytesTypeNameToByteLengthMap = {
  [K in ByteLength as ByteLengthToBytesTypeNameMap[K]]: K;
};

/**
 * Maps BytesN type names to their corresponding branded types ('bytes8' -> Bytes8, etc.).
 */
export type BytesTypeNameToTypeMap = {
  [K in keyof BytesTypeNameToByteLengthMap]: ByteLengthToBytesTypeMap[BytesTypeNameToByteLengthMap[K]];
};

////////////////////////////////////////////////////////////////////////////////
//
// Address
//
////////////////////////////////////////////////////////////////////////////////

declare const __address: unique symbol;
declare const __checksummedAddress: unique symbol;

export type AddressString = { readonly [__address]: never };
export type ChecksummedAddressString = {
  readonly [__checksummedAddress]: never;
};

export type Address = Bytes20Hex & AddressString;
export type ChecksummedAddress = Address & ChecksummedAddressString;

export type AddressTypeName = 'checksummedAddress' | 'address';

////////////////////////////////////////////////////////////////////////////////
//
// TypedValue
//
////////////////////////////////////////////////////////////////////////////////

export type UintValueTypeMap = Prettify<Omit<UintNormalizedMap, 'uint160'>>;

export type ValueTypeMap = Prettify<
  UintValueTypeMap &
    Readonly<{
      bool: boolean;
      address: Address;
    }>
>;

export type UintValueTypeBitsMap = Prettify<Omit<UintBitsMap, 'uint160'>>;

export type ValueTypeBitsMap = Prettify<
  UintValueTypeBitsMap &
    Readonly<{
      bool: 1;
      address: 160;
    }>
>;

export type ValueLikeMap = Readonly<{
  bool: boolean | number | bigint;
  uint8: number | bigint;
  uint16: number | bigint;
  uint32: number | bigint;
  uint64: number | bigint;
  uint128: number | bigint;
  uint256: number | bigint;
  address: string;
}>;

export type ValueType<T extends keyof ValueTypeMap = keyof ValueTypeMap> = ValueTypeMap[T];
export type ValueTypeName = Prettify<keyof ValueTypeMap>;

export interface TypedValueOfBase<T extends ValueTypeName> {
  readonly type: T;
  readonly value: ValueType<T>;
}

export type TypedValueOf<T extends ValueTypeName = ValueTypeName> = {
  [K in T]: TypedValueOfBase<K>;
}[T];

export type BoolValue = Prettify<TypedValueOf<'bool'>>;
export type Uint8Value = Prettify<TypedValueOf<'uint8'>>;
export type Uint16Value = Prettify<TypedValueOf<'uint16'>>;
export type Uint32Value = Prettify<TypedValueOf<'uint32'>>;
export type Uint64Value = Prettify<TypedValueOf<'uint64'>>;
export type Uint128Value = Prettify<TypedValueOf<'uint128'>>;
export type Uint256Value = Prettify<TypedValueOf<'uint256'>>;
export type AddressValue = Prettify<TypedValueOf<'address'>>;

type TypedValueMap = Readonly<{
  bool: BoolValue;
  uint8: Uint8Value;
  uint16: Uint16Value;
  uint32: Uint32Value;
  uint64: Uint64Value;
  uint128: Uint128Value;
  uint256: Uint256Value;
  address: AddressValue;
}>;

export type TypedValue = TypedValueMap[ValueTypeName];

export type TypedValueLikeOf<T extends ValueTypeName> = Readonly<{
  value: ValueLikeMap[T];
  type: T;
}>;

export type BoolValueLike = Prettify<TypedValueLikeOf<'bool'>>;
export type Uint8ValueLike = Prettify<TypedValueLikeOf<'uint8'>>;
export type Uint16ValueLike = Prettify<TypedValueLikeOf<'uint16'>>;
export type Uint32ValueLike = Prettify<TypedValueLikeOf<'uint32'>>;
export type Uint64ValueLike = Prettify<TypedValueLikeOf<'uint64'>>;
export type Uint128ValueLike = Prettify<TypedValueLikeOf<'uint128'>>;
export type Uint256ValueLike = Prettify<TypedValueLikeOf<'uint256'>>;
export type AddressValueLike = Prettify<TypedValueLikeOf<'address'>>;

type TypedValueLikeMap = Readonly<{
  bool: BoolValueLike;
  uint8: Uint8ValueLike;
  uint16: Uint16ValueLike;
  uint32: Uint32ValueLike;
  uint64: Uint64ValueLike;
  uint128: Uint128ValueLike;
  uint256: Uint256ValueLike;
  address: AddressValueLike;
}>;

export type TypedValueLike = TypedValueLikeMap[ValueTypeName];

/**
 * Resolves an input typed-value to its validated counterpart via
 * {@link TypedValueMap} lookup.
 * @internal
 */
export type TypedValueFrom<T extends TypedValueLike> = TypedValueMap[T['type']];
