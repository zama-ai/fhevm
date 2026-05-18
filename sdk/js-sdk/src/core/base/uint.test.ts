import { describe, it, expect } from 'vitest';
import { InvalidPropertyError } from './errors/InvalidPropertyError.js';
import { InvalidTypeError } from './errors/InvalidTypeError.js';
import { hexToBytes } from './bytes.js';
import {
  assertIsUint,
  assertIsUint8,
  assertIsUint32,
  assertIsUint64,
  assertIsUint256,
  assertRecordUintProperty,
  assertRecordUint256Property,
  asUint,
  isRecordUintProperty,
  isRecordUint256Property,
  isUint,
  isUint8,
  isUint16,
  isUint32,
  isUint64,
  isUint128,
  isUint256,
  isUintBigInt,
  isUintNumber,
  MAX_UINT8,
  MAX_UINT16,
  MAX_UINT32,
  MAX_UINT64,
  MAX_UINT128,
  MAX_UINT256,
  numberToBytes32,
  numberToBytes8,
  numberToBytesHex,
  numberToBytesHexNo0x,
  uint64ToBytes32,
  uint256ToBytes32,
  uintToHex0x,
  uintToBytesHex,
  uintToBytesHexNo0x,
} from './uint.js';

////////////////////////////////////////////////////////////////////////////////
//
// npx vitest run --config src/vitest.config.ts src/core/base/uint.test.ts
//
////////////////////////////////////////////////////////////////////////////////

describe('uint', () => {
  //////////////////////////////////////////////////////////////////////////////

  it('isUint', () => {
    expect(isUint('hello')).toEqual(false);
    expect(isUint(null)).toEqual(false);
    expect(isUint(undefined)).toEqual(false);
    expect(isUint('')).toEqual(false);
    expect(isUint('123')).toEqual(false);
    expect(isUint(123)).toEqual(true);
    expect(isUint(BigInt(123))).toEqual(true);
    expect(isUint(123.0)).toEqual(true);
    expect(isUint(123.1)).toEqual(false);
    expect(isUint(-123)).toEqual(false);
    expect(isUint(0)).toEqual(true);
    expect(isUint({})).toEqual(false);
    expect(isUint([])).toEqual(false);
    expect(isUint([123])).toEqual(false);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertIsUint', () => {
    // True
    expect(() => assertIsUint(123.0, {})).not.toThrow();
    expect(() => assertIsUint(0, {})).not.toThrow();
    expect(() => assertIsUint(123, {})).not.toThrow();
    expect(() => assertIsUint(BigInt(123), {})).not.toThrow();

    const e = (type: string) =>
      new InvalidTypeError(
        {
          expectedType: 'uint',
          type,
        },
        {},
      );

    // False
    expect(() => assertIsUint('hello', {})).toThrow(e('string'));
    expect(() => assertIsUint(null, {})).toThrow(e('object'));
    expect(() => assertIsUint(undefined, {})).toThrow(e('undefined'));
    expect(() => assertIsUint('', {})).toThrow(e('string'));
    expect(() => assertIsUint('123', {})).toThrow(e('string'));
    expect(() => assertIsUint(123.1, {})).toThrow(e('number'));
    expect(() => assertIsUint(-123, {})).toThrow(e('number'));
    expect(() => assertIsUint({}, {})).toThrow(e('object'));
    expect(() => assertIsUint([], {})).toThrow(e('object'));
    expect(() => assertIsUint([123], {})).toThrow(e('object'));
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertRecordUintProperty', () => {
    // True
    expect(() => assertRecordUintProperty({ foo: 123.0 }, 'foo', 'Foo', {})).not.toThrow();

    expect(() => assertRecordUintProperty({ foo: 0 }, 'foo', 'Foo', {})).not.toThrow();

    expect(() => assertRecordUintProperty({ foo: 123 }, 'foo', 'Foo', {})).not.toThrow();

    expect(() => assertRecordUintProperty({ foo: BigInt(123) }, 'foo', 'Foo', {})).not.toThrow();

    const e = (type: string) =>
      new InvalidPropertyError(
        {
          subject: 'Foo',
          property: 'foo',
          expectedType: 'uint',
          type,
        },
        {},
      );

    // False
    expect(() => assertRecordUintProperty({ foo: null }, 'foo', 'Foo', {})).toThrow(e('undefined'));

    expect(() => assertRecordUintProperty({ foo: undefined }, 'foo', 'Foo', {})).toThrow(e('undefined'));

    expect(() => assertRecordUintProperty({}, 'foo', 'Foo', {})).toThrow(e('undefined'));

    expect(() => assertRecordUintProperty({ foo: 'hello' }, 'foo', 'Foo', {})).toThrow(e('string'));

    expect(() => assertRecordUintProperty({ foo: '' }, 'foo', 'Foo', {})).toThrow(e('string'));

    expect(() => assertRecordUintProperty({ foo: '123' }, 'foo', 'Foo', {})).toThrow(e('string'));

    expect(() => assertRecordUintProperty({ foo: 123.1 }, 'foo', 'Foo', {})).toThrow(e('number'));

    expect(() => assertRecordUintProperty({ foo: -123 }, 'foo', 'Foo', {})).toThrow(e('number'));

    expect(() => assertRecordUintProperty({ foo: {} }, 'foo', 'Foo', {})).toThrow(e('object'));

    expect(() => assertRecordUintProperty({ foo: [] }, 'foo', 'Foo', {})).toThrow(e('object'));

    expect(() => assertRecordUintProperty({ foo: [123] }, 'foo', 'Foo', {})).toThrow(e('object'));
  });

  //////////////////////////////////////////////////////////////////////////////

  it('isRecordUintProperty', () => {
    // True
    expect(isRecordUintProperty({ foo: 123 }, 'foo')).toBe(true);
    expect(isRecordUintProperty({ foo: 0 }, 'foo')).toBe(true);
    expect(isRecordUintProperty({ foo: BigInt(123) }, 'foo')).toBe(true);

    // False
    expect(isRecordUintProperty({ foo: null }, 'foo')).toBe(false);
    expect(isRecordUintProperty({ foo: undefined }, 'foo')).toBe(false);
    expect(isRecordUintProperty({}, 'foo')).toBe(false);
    expect(isRecordUintProperty(null, 'foo')).toBe(false);
    expect(isRecordUintProperty({ foo: 'hello' }, 'foo')).toBe(false);
    expect(isRecordUintProperty({ foo: 123.1 }, 'foo')).toBe(false);
    expect(isRecordUintProperty({ foo: -123 }, 'foo')).toBe(false);
  });
});

describe('numberToBytes32', () => {
  it('converts 0 to 32 zero bytes', () => {
    const result = numberToBytes32(0);
    expect(result).toBeInstanceOf(Uint8Array);
    expect(result.length).toBe(32);
    expect(result.every((b) => b === 0)).toBe(true);
  });

  it('converts 1 to bytes32 with last byte = 1', () => {
    const result = numberToBytes32(1);
    expect(result.length).toBe(32);
    expect(result[31]).toBe(1);
    //Uint8Array(32) [0, 0, ..., 0, 0, 0, 0, 0, 1]
    expect(result.slice(0, 31).every((b) => b === 0)).toBe(true);
  });

  it('converts 123 (0x7b) correctly', () => {
    const result = numberToBytes32(123);
    expect(result).toBeInstanceOf(Uint8Array);
    expect(result.length).toBe(32);
    //Uint8Array(32) [0, 0, ..., 0, 0, 0, 0, 0, 123]
    expect(result[31]).toBe(123);
    expect(result.slice(0, 31).every((b) => b === 0)).toBe(true);
  });

  it('converts 255 (0xff) correctly', () => {
    const result = numberToBytes32(255);
    expect(result).toBeInstanceOf(Uint8Array);
    expect(result.length).toBe(32);
    expect(result[31]).toBe(255);
    expect(result.slice(0, 31).every((b) => b === 0)).toBe(true);
  });

  it('converts 256 (0x100) correctly', () => {
    const result = numberToBytes32(256);
    expect(result).toBeInstanceOf(Uint8Array);
    expect(result.length).toBe(32);
    expect(result[30]).toBe(1);
    expect(result[31]).toBe(0);
  });

  it('converts 65535 (0xffff) correctly', () => {
    const result = numberToBytes32(65535);
    expect(result).toBeInstanceOf(Uint8Array);
    expect(result.length).toBe(32);
    expect(result[30]).toBe(255);
    expect(result[31]).toBe(255);
  });

  it('converts 0x01020304 correctly (big-endian)', () => {
    const result = numberToBytes32(0x01020304);
    expect(result).toBeInstanceOf(Uint8Array);
    expect(result.length).toBe(32);
    expect(result[28]).toBe(1);
    expect(result[29]).toBe(2);
    expect(result[30]).toBe(3);
    expect(result[31]).toBe(4);
  });

  it('converts MAX_SAFE_INTEGER correctly', () => {
    const result = numberToBytes32(Number.MAX_SAFE_INTEGER);
    expect(result).toBeInstanceOf(Uint8Array);
    expect(result.length).toBe(32);
    // MAX_SAFE_INTEGER = 2^53 - 1 = 0x1FFFFFFFFFFFFF (7 bytes)
    // Stored in bytes 25-31 (big-endian in last 8 bytes)
    const view = new DataView(result.buffer);
    const value = view.getBigUint64(24, false);
    expect(value).toBe(BigInt(Number.MAX_SAFE_INTEGER));
  });

  it('throws for negative numbers', () => {
    expect(() => numberToBytes32(-1)).toThrow(InvalidTypeError);
    expect(() => numberToBytes32(-100)).toThrow(InvalidTypeError);
  });

  it('throws for non-integer numbers', () => {
    expect(() => numberToBytes32(1.5)).toThrow(InvalidTypeError);
    expect(() => numberToBytes32(0.1)).toThrow(InvalidTypeError);
    expect(() => numberToBytes32(123.456)).toThrow(InvalidTypeError);
  });

  it('throws for NaN', () => {
    expect(() => numberToBytes32(NaN)).toThrow(InvalidTypeError);
  });

  it('throws for Infinity', () => {
    expect(() => numberToBytes32(Infinity)).toThrow(InvalidTypeError);
    expect(() => numberToBytes32(-Infinity)).toThrow(InvalidTypeError);
  });
});

describe('numberToBytes8', () => {
  it('converts 0 to 8 zero bytes', () => {
    const result = numberToBytes8(0);
    expect(result).toBeInstanceOf(Uint8Array);
    expect(result.length).toBe(8);
    expect(result.every((b) => b === 0)).toBe(true);
  });

  it('converts 1 to bytes8 with last byte = 1', () => {
    const result = numberToBytes8(1);
    expect(result.length).toBe(8);
    expect(result[7]).toBe(1);
    //Uint8Array(8) [0, 0, 0, 0, 0, 0, 0, 1]
    expect(result.slice(0, 7).every((b) => b === 0)).toBe(true);
  });

  it('converts 123 (0x7b) correctly', () => {
    const result = numberToBytes8(123);
    expect(result).toBeInstanceOf(Uint8Array);
    expect(result.length).toBe(8);
    //Uint8Array(8) [0, 0, 0, 0, 0, 0, 0, 123]
    expect(result[7]).toBe(123);
    expect(result.slice(0, 7).every((b) => b === 0)).toBe(true);
  });

  it('converts 255 (0xff) correctly', () => {
    const result = numberToBytes8(255);
    expect(result).toBeInstanceOf(Uint8Array);
    expect(result.length).toBe(8);
    expect(result[7]).toBe(255);
    expect(result.slice(0, 7).every((b) => b === 0)).toBe(true);
  });

  it('converts 256 (0x100) correctly', () => {
    const result = numberToBytes8(256);
    expect(result).toBeInstanceOf(Uint8Array);
    expect(result.length).toBe(8);
    expect(result[6]).toBe(1);
    expect(result[7]).toBe(0);
  });

  it('converts 65535 (0xffff) correctly', () => {
    const result = numberToBytes8(65535);
    expect(result).toBeInstanceOf(Uint8Array);
    expect(result.length).toBe(8);
    expect(result[6]).toBe(255);
    expect(result[7]).toBe(255);
  });

  it('converts 0x01020304 correctly (big-endian)', () => {
    const result = numberToBytes8(0x01020304);
    expect(result).toBeInstanceOf(Uint8Array);
    expect(result.length).toBe(8);
    expect(result[4]).toBe(1);
    expect(result[5]).toBe(2);
    expect(result[6]).toBe(3);
    expect(result[7]).toBe(4);
  });

  it('converts MAX_SAFE_INTEGER correctly', () => {
    const result = numberToBytes8(Number.MAX_SAFE_INTEGER);
    expect(result).toBeInstanceOf(Uint8Array);
    expect(result.length).toBe(8);
    // MAX_SAFE_INTEGER = 2^53 - 1 = 0x1FFFFFFFFFFFFF (7 bytes)
    // Stored in bytes 25-31 (big-endian in last 8 bytes)
    const view = new DataView(result.buffer);
    const value = view.getBigUint64(0, false);
    expect(value).toBe(BigInt(Number.MAX_SAFE_INTEGER));
  });

  it('throws for negative numbers', () => {
    expect(() => numberToBytes8(-1)).toThrow(InvalidTypeError);
    expect(() => numberToBytes8(-100)).toThrow(InvalidTypeError);
  });

  it('throws for non-integer numbers', () => {
    expect(() => numberToBytes8(1.5)).toThrow(InvalidTypeError);
    expect(() => numberToBytes8(0.1)).toThrow(InvalidTypeError);
    expect(() => numberToBytes8(123.456)).toThrow(InvalidTypeError);
  });

  it('throws for NaN', () => {
    expect(() => numberToBytes8(NaN)).toThrow(InvalidTypeError);
  });

  it('throws for Infinity', () => {
    expect(() => numberToBytes8(Infinity)).toThrow(InvalidTypeError);
    expect(() => numberToBytes8(-Infinity)).toThrow(InvalidTypeError);
  });
});

describe('isUintNumber', () => {
  it('returns true for positive integers', () => {
    expect(isUintNumber(0)).toBe(true);
    expect(isUintNumber(1)).toBe(true);
    expect(isUintNumber(255)).toBe(true);
    expect(isUintNumber(Number.MAX_SAFE_INTEGER)).toBe(true);
  });

  it('returns false for negative numbers', () => {
    expect(isUintNumber(-1)).toBe(false);
    expect(isUintNumber(-100)).toBe(false);
  });

  it('returns false for non-integer numbers', () => {
    expect(isUintNumber(1.5)).toBe(false);
    expect(isUintNumber(0.1)).toBe(false);
  });

  it('returns false for non-number types', () => {
    expect(isUintNumber(BigInt(123))).toBe(false);
    expect(isUintNumber('123')).toBe(false);
    expect(isUintNumber(null)).toBe(false);
    expect(isUintNumber(undefined)).toBe(false);
    expect(isUintNumber({})).toBe(false);
  });
});

describe('isUintBigInt', () => {
  it('returns true for non-negative bigints', () => {
    expect(isUintBigInt(BigInt(0))).toBe(true);
    expect(isUintBigInt(1n)).toBe(true);
    expect(isUintBigInt(BigInt(255))).toBe(true);
    expect(isUintBigInt(MAX_UINT256)).toBe(true);
  });

  it('returns false for negative bigints', () => {
    expect(isUintBigInt(BigInt(-1))).toBe(false);
    expect(isUintBigInt(BigInt(-100))).toBe(false);
  });

  it('returns false for non-bigint types', () => {
    expect(isUintBigInt(123)).toBe(false);
    expect(isUintBigInt('123')).toBe(false);
    expect(isUintBigInt(null)).toBe(false);
    expect(isUintBigInt(undefined)).toBe(false);
    expect(isUintBigInt({})).toBe(false);
  });
});

describe('isUint8', () => {
  // Valid uint8 values
  it('returns true for 0', () => {
    expect(isUint8(0)).toBe(true);
  });

  it('returns true for 1', () => {
    expect(isUint8(1)).toBe(true);
  });

  it('returns true for MAX_UINT8 (255)', () => {
    expect(isUint8(MAX_UINT8)).toBe(true);
    expect(isUint8(255)).toBe(true);
  });

  it('returns true for BigInt values within range', () => {
    expect(isUint8(BigInt(0))).toBe(true);
    expect(isUint8(BigInt(255))).toBe(true);
    expect(isUint8(BigInt(128))).toBe(true);
  });

  // Boundary: just over MAX_UINT8
  it('returns false for 256 (MAX_UINT8 + 1)', () => {
    expect(isUint8(256)).toBe(false);
    expect(isUint8(BigInt(256))).toBe(false);
  });

  // Invalid types
  it('returns false for non-uint values', () => {
    expect(isUint8(-1)).toBe(false);
    expect(isUint8(-128)).toBe(false);
    expect(isUint8(1.5)).toBe(false);
    expect(isUint8('255')).toBe(false);
    expect(isUint8(null)).toBe(false);
    expect(isUint8(undefined)).toBe(false);
    expect(isUint8({})).toBe(false);
  });

  // Large values
  it('returns false for values much larger than MAX_UINT8', () => {
    expect(isUint8(1000)).toBe(false);
    expect(isUint8(MAX_UINT32)).toBe(false);
    expect(isUint8(MAX_UINT64)).toBe(false);
  });
});

describe('isUint16', () => {
  // Valid uint16 values
  it('returns true for 0', () => {
    expect(isUint16(0)).toBe(true);
  });

  it('returns true for 1', () => {
    expect(isUint16(1)).toBe(true);
  });

  it('returns true for MAX_UINT8 (255)', () => {
    expect(isUint16(255)).toBe(true);
  });

  it('returns true for MAX_UINT16 (65535)', () => {
    expect(isUint16(MAX_UINT16)).toBe(true);
    expect(isUint16(65535)).toBe(true);
  });

  it('returns true for BigInt values within range', () => {
    expect(isUint16(BigInt(0))).toBe(true);
    expect(isUint16(BigInt(65535))).toBe(true);
    expect(isUint16(BigInt(32768))).toBe(true);
  });

  // Boundary: just over MAX_UINT16
  it('returns false for 65536 (MAX_UINT16 + 1)', () => {
    expect(isUint16(65536)).toBe(false);
    expect(isUint16(BigInt(65536))).toBe(false);
  });

  // Invalid types
  it('returns false for non-uint values', () => {
    expect(isUint16(-1)).toBe(false);
    expect(isUint16(-32768)).toBe(false);
    expect(isUint16(1.5)).toBe(false);
    expect(isUint16('65535')).toBe(false);
    expect(isUint16(null)).toBe(false);
    expect(isUint16(undefined)).toBe(false);
    expect(isUint16({})).toBe(false);
  });

  // Large values
  it('returns false for values larger than MAX_UINT16', () => {
    expect(isUint16(100000)).toBe(false);
    expect(isUint16(MAX_UINT32)).toBe(false);
    expect(isUint16(MAX_UINT64)).toBe(false);
  });
});

describe('isUint32', () => {
  // Valid uint32 values
  it('returns true for 0', () => {
    expect(isUint32(0)).toBe(true);
  });

  it('returns true for 1', () => {
    expect(isUint32(1)).toBe(true);
  });

  it('returns true for MAX_UINT8 (255)', () => {
    expect(isUint32(255)).toBe(true);
  });

  it('returns true for MAX_UINT32 (0xffffffff)', () => {
    expect(isUint32(MAX_UINT32)).toBe(true);
    expect(isUint32(0xffffffff)).toBe(true);
  });

  it('returns true for BigInt values within range', () => {
    expect(isUint32(BigInt(0))).toBe(true);
    expect(isUint32(BigInt(MAX_UINT32))).toBe(true);
    expect(isUint32(BigInt(0x80000000))).toBe(true); // 2^31
  });

  // Boundary: just over MAX_UINT32
  it('returns false for 0x100000000 (MAX_UINT32 + 1)', () => {
    expect(isUint32(0x100000000)).toBe(false);
    expect(isUint32(BigInt(0x100000000))).toBe(false);
  });

  // Invalid types
  it('returns false for non-uint values', () => {
    expect(isUint32(-1)).toBe(false);
    expect(isUint32(-0x80000000)).toBe(false);
    expect(isUint32(1.5)).toBe(false);
    expect(isUint32('4294967295')).toBe(false);
    expect(isUint32(null)).toBe(false);
    expect(isUint32(undefined)).toBe(false);
    expect(isUint32({})).toBe(false);
  });

  // Large values
  it('returns false for values larger than MAX_UINT32', () => {
    expect(isUint32(MAX_UINT64)).toBe(false);
    expect(isUint32(Number.MAX_SAFE_INTEGER)).toBe(false);
  });
});

describe('isUint64', () => {
  // Valid uint64 values
  it('returns true for 0', () => {
    expect(isUint64(0)).toBe(true);
  });

  it('returns true for 1', () => {
    expect(isUint64(1)).toBe(true);
  });

  it('returns true for MAX_UINT8 (255)', () => {
    expect(isUint64(255)).toBe(true);
  });

  it('returns true for MAX_UINT32', () => {
    expect(isUint64(MAX_UINT32)).toBe(true);
  });

  it('returns true for MAX_UINT64 (2^64 - 1)', () => {
    expect(isUint64(MAX_UINT64)).toBe(true);
  });

  it('returns true for Number.MAX_SAFE_INTEGER', () => {
    expect(isUint64(Number.MAX_SAFE_INTEGER)).toBe(true);
  });

  it('returns true for BigInt values within range', () => {
    expect(isUint64(BigInt(0))).toBe(true);
    expect(isUint64(BigInt(MAX_UINT32))).toBe(true);
    expect(isUint64(MAX_UINT64)).toBe(true);
    expect(isUint64(9223372036854775807n)).toBe(true); // 2^63 - 1
  });

  // Boundary: just over MAX_UINT64
  it('returns false for MAX_UINT64 + 1', () => {
    expect(isUint64(MAX_UINT64 + 1n)).toBe(false);
  });

  // Invalid types
  it('returns false for non-uint values', () => {
    expect(isUint64(-1)).toBe(false);
    expect(isUint64(BigInt(-1))).toBe(false);
    expect(isUint64(1.5)).toBe(false);
    expect(isUint64('18446744073709551615')).toBe(false);
    expect(isUint64(null)).toBe(false);
    expect(isUint64(undefined)).toBe(false);
    expect(isUint64({})).toBe(false);
  });

  // Very large values (beyond uint64)
  it('returns false for values larger than MAX_UINT64', () => {
    expect(isUint64(MAX_UINT64 + 1n)).toBe(false); // 2^64
    expect(isUint64(340282366920938463463374607431768211455n)).toBe(false); // 2^128 - 1
  });
});

describe('isUint128', () => {
  // Valid uint128 values
  it('returns true for 0', () => {
    expect(isUint128(0)).toBe(true);
  });

  it('returns true for 1', () => {
    expect(isUint128(1)).toBe(true);
  });

  it('returns true for MAX_UINT64', () => {
    expect(isUint128(MAX_UINT64)).toBe(true);
  });

  it('returns true for MAX_UINT128 (2^128 - 1)', () => {
    expect(isUint128(MAX_UINT128)).toBe(true);
  });

  it('returns true for BigInt values within range', () => {
    expect(isUint128(BigInt(0))).toBe(true);
    expect(isUint128(BigInt(MAX_UINT32))).toBe(true);
    expect(isUint128(MAX_UINT64)).toBe(true);
    expect(isUint128(MAX_UINT128)).toBe(true);
  });

  // Boundary: just over MAX_UINT128
  it('returns false for MAX_UINT128 + 1', () => {
    expect(isUint128(MAX_UINT128 + 1n)).toBe(false);
  });

  // Invalid types
  it('returns false for non-uint values', () => {
    expect(isUint128(-1)).toBe(false);
    expect(isUint128(BigInt(-1))).toBe(false);
    expect(isUint128(1.5)).toBe(false);
    expect(isUint128('123')).toBe(false);
    expect(isUint128(null)).toBe(false);
    expect(isUint128(undefined)).toBe(false);
    expect(isUint128({})).toBe(false);
  });

  // Very large values (beyond uint128)
  it('returns false for values larger than MAX_UINT128', () => {
    expect(isUint128(MAX_UINT128 + 1n)).toBe(false);
    expect(isUint128(MAX_UINT256)).toBe(false);
  });
});

describe('assertIsUint8', () => {
  // Valid uint8 values - should not throw
  it('does not throw for 0', () => {
    expect(() => assertIsUint8(0, {})).not.toThrow();
  });

  it('does not throw for 1', () => {
    expect(() => assertIsUint8(1, {})).not.toThrow();
  });

  it('does not throw for MAX_UINT8 (255)', () => {
    expect(() => assertIsUint8(MAX_UINT8, {})).not.toThrow();
    expect(() => assertIsUint8(255, {})).not.toThrow();
  });

  it('does not throw for BigInt values within range', () => {
    expect(() => assertIsUint8(BigInt(0), {})).not.toThrow();
    expect(() => assertIsUint8(BigInt(255), {})).not.toThrow();
    expect(() => assertIsUint8(BigInt(128), {})).not.toThrow();
  });

  // Boundary: just over MAX_UINT8 - should throw
  it('throws for 256 (MAX_UINT8 + 1)', () => {
    expect(() => assertIsUint8(256, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint8(BigInt(256), {})).toThrow(InvalidTypeError);
  });

  // Invalid types - should throw
  it('throws for non-uint values', () => {
    expect(() => assertIsUint8(-1, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint8(-128, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint8(1.5, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint8('255', {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint8(null, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint8(undefined, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint8({}, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint8([], {})).toThrow(InvalidTypeError);
  });

  // Large values - should throw
  it('throws for values much larger than MAX_UINT8', () => {
    expect(() => assertIsUint8(1000, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint8(MAX_UINT32, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint8(MAX_UINT64, {})).toThrow(InvalidTypeError);
  });
});

describe('assertIsUint32', () => {
  // Valid uint32 values - should not throw
  it('does not throw for 0', () => {
    expect(() => assertIsUint32(0, {})).not.toThrow();
  });

  it('does not throw for 1', () => {
    expect(() => assertIsUint32(1, {})).not.toThrow();
  });

  it('does not throw for MAX_UINT8 (255)', () => {
    expect(() => assertIsUint32(255, {})).not.toThrow();
  });

  it('does not throw for MAX_UINT32 (0xffffffff)', () => {
    expect(() => assertIsUint32(MAX_UINT32, {})).not.toThrow();
    expect(() => assertIsUint32(0xffffffff, {})).not.toThrow();
  });

  it('does not throw for BigInt values within range', () => {
    expect(() => assertIsUint32(BigInt(0), {})).not.toThrow();
    expect(() => assertIsUint32(BigInt(MAX_UINT32), {})).not.toThrow();
    expect(() => assertIsUint32(BigInt(0x80000000), {})).not.toThrow(); // 2^31
  });

  // Boundary: just over MAX_UINT32 - should throw
  it('throws for 0x100000000 (MAX_UINT32 + 1)', () => {
    expect(() => assertIsUint32(0x100000000, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint32(BigInt(0x100000000), {})).toThrow(InvalidTypeError);
  });

  // Invalid types - should throw
  it('throws for non-uint values', () => {
    expect(() => assertIsUint32(-1, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint32(-0x80000000, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint32(1.5, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint32('4294967295', {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint32(null, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint32(undefined, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint32({}, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint32([], {})).toThrow(InvalidTypeError);
  });

  // Large values - should throw
  it('throws for values larger than MAX_UINT32', () => {
    expect(() => assertIsUint32(MAX_UINT64, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint32(Number.MAX_SAFE_INTEGER, {})).toThrow(InvalidTypeError);
  });
});

describe('assertIsUint64', () => {
  // Valid uint64 values - should not throw
  it('does not throw for 0', () => {
    expect(() => assertIsUint64(0, {})).not.toThrow();
  });

  it('does not throw for 1', () => {
    expect(() => assertIsUint64(1, {})).not.toThrow();
  });

  it('does not throw for MAX_UINT8 (255)', () => {
    expect(() => assertIsUint64(255, {})).not.toThrow();
  });

  it('does not throw for MAX_UINT32', () => {
    expect(() => assertIsUint64(MAX_UINT32, {})).not.toThrow();
  });

  it('does not throw for MAX_UINT64 (2^64 - 1)', () => {
    expect(() => assertIsUint64(MAX_UINT64, {})).not.toThrow();
  });

  it('does not throw for Number.MAX_SAFE_INTEGER', () => {
    expect(() => assertIsUint64(Number.MAX_SAFE_INTEGER, {})).not.toThrow();
  });

  it('does not throw for BigInt values within range', () => {
    expect(() => assertIsUint64(BigInt(0), {})).not.toThrow();
    expect(() => assertIsUint64(BigInt(MAX_UINT32), {})).not.toThrow();
    expect(() => assertIsUint64(MAX_UINT64, {})).not.toThrow();
    expect(() => assertIsUint64(9223372036854775807n, {})).not.toThrow(); // 2^63 - 1
  });

  // Boundary: just over MAX_UINT64 - should throw
  it('throws for MAX_UINT64 + 1', () => {
    expect(() => assertIsUint64(MAX_UINT64 + 1n, {})).toThrow(InvalidTypeError);
  });

  // Invalid types - should throw
  it('throws for non-uint values', () => {
    expect(() => assertIsUint64(-1, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint64(BigInt(-1), {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint64(1.5, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint64('18446744073709551615', {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint64(null, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint64(undefined, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint64({}, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint64([], {})).toThrow(InvalidTypeError);
  });

  // Very large values (beyond uint64) - should throw
  it('throws for values larger than MAX_UINT64', () => {
    expect(() => assertIsUint64(MAX_UINT64 + 1n, {})).toThrow(InvalidTypeError); // 2^64
    expect(() => assertIsUint64(340282366920938463463374607431768211455n, {})).toThrow(InvalidTypeError); // 2^128 - 1
  });
});

describe('assertIsUint256', () => {
  // Valid uint256 values - should not throw
  it('does not throw for 0', () => {
    expect(() => assertIsUint256(0, {})).not.toThrow();
  });

  it('does not throw for 1', () => {
    expect(() => assertIsUint256(1, {})).not.toThrow();
  });

  it('does not throw for MAX_UINT8 (255)', () => {
    expect(() => assertIsUint256(255, {})).not.toThrow();
  });

  it('does not throw for MAX_UINT32', () => {
    expect(() => assertIsUint256(MAX_UINT32, {})).not.toThrow();
  });

  it('does not throw for MAX_UINT64', () => {
    expect(() => assertIsUint256(MAX_UINT64, {})).not.toThrow();
  });

  it('does not throw for MAX_UINT256 (2^256 - 1)', () => {
    expect(() => assertIsUint256(MAX_UINT256, {})).not.toThrow();
  });

  it('does not throw for Number.MAX_SAFE_INTEGER', () => {
    expect(() => assertIsUint256(Number.MAX_SAFE_INTEGER, {})).not.toThrow();
  });

  it('does not throw for BigInt values within range', () => {
    expect(() => assertIsUint256(BigInt(0), {})).not.toThrow();
    expect(() => assertIsUint256(BigInt(MAX_UINT32), {})).not.toThrow();
    expect(() => assertIsUint256(MAX_UINT64, {})).not.toThrow();
    expect(() => assertIsUint256(MAX_UINT256, {})).not.toThrow();
    // 2^128 - 1
    expect(() => assertIsUint256(340282366920938463463374607431768211455n, {})).not.toThrow();
  });

  // Boundary: just over MAX_UINT256 - should throw
  it('throws for MAX_UINT256 + 1', () => {
    expect(() => assertIsUint256(MAX_UINT256 + 1n, {})).toThrow(InvalidTypeError);
  });

  // Invalid types - should throw
  it('throws for non-uint values', () => {
    expect(() => assertIsUint256(-1, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint256(BigInt(-1), {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint256(1.5, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint256('123', {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint256(null, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint256(undefined, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint256({}, {})).toThrow(InvalidTypeError);
    expect(() => assertIsUint256([], {})).toThrow(InvalidTypeError);
  });

  // Very large values (beyond uint256) - should throw
  it('throws for values larger than MAX_UINT256', () => {
    // 2^256
    expect(() => assertIsUint256(MAX_UINT256 + 1n, {})).toThrow(InvalidTypeError);
    // 2^257
    expect(() => assertIsUint256(MAX_UINT256 * BigInt(2), {})).toThrow(InvalidTypeError);
  });
});

describe('uintToBytesHex', () => {
  // Basic conversions
  it('converts 0 to "0x00"', () => {
    expect(uintToBytesHex(asUint(0))).toBe('0x00');
  });

  it('converts 1 to "0x01"', () => {
    expect(uintToBytesHex(asUint(1))).toBe('0x01');
  });

  it('converts 15 to "0x0f"', () => {
    expect(uintToBytesHex(asUint(15))).toBe('0x0f');
  });

  it('converts 16 to "0x10"', () => {
    expect(uintToBytesHex(asUint(16))).toBe('0x10');
  });

  it('converts 255 to "0xff"', () => {
    expect(uintToBytesHex(asUint(255))).toBe('0xff');
  });

  it('converts 256 to "0x0100"', () => {
    expect(uintToBytesHex(asUint(256))).toBe('0x0100');
  });

  it('converts 65535 to "0xffff"', () => {
    expect(uintToBytesHex(asUint(65535))).toBe('0xffff');
  });

  it('converts 65536 to "0x010000"', () => {
    expect(uintToBytesHex(asUint(65536))).toBe('0x010000');
  });

  // BigInt values
  it('converts BigInt(0) to "0x00"', () => {
    expect(uintToBytesHex(asUint(BigInt(0)))).toBe('0x00');
  });

  it('converts BigInt(255) to "0xff"', () => {
    expect(uintToBytesHex(asUint(BigInt(255)))).toBe('0xff');
  });

  it('converts MAX_UINT64 correctly', () => {
    expect(uintToBytesHex(asUint(MAX_UINT64))).toBe('0xffffffffffffffff');
  });

  it('converts MAX_UINT256 correctly', () => {
    expect(uintToBytesHex(asUint(MAX_UINT256))).toBe(
      '0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff',
    );
  });

  // Ensure even-length padding
  it('pads odd-length hex to even length', () => {
    // 0x1 -> 0x01
    expect(uintToBytesHex(asUint(1))).toBe('0x01');
    // 0xfff -> 0x0fff
    expect(uintToBytesHex(asUint(0xfff))).toBe('0x0fff');
    // 0xfffff -> 0x0fffff
    expect(uintToBytesHex(asUint(0xfffff))).toBe('0x0fffff');
  });
});

describe('uintToBytesHexNo0x', () => {
  // Basic conversions
  it('converts 0 to "00"', () => {
    expect(uintToBytesHexNo0x(asUint(0))).toBe('00');
  });

  it('converts 1 to "01"', () => {
    expect(uintToBytesHexNo0x(asUint(1))).toBe('01');
  });

  it('converts 15 to "0f"', () => {
    expect(uintToBytesHexNo0x(asUint(15))).toBe('0f');
  });

  it('converts 16 to "10"', () => {
    expect(uintToBytesHexNo0x(asUint(16))).toBe('10');
  });

  it('converts 255 to "ff"', () => {
    expect(uintToBytesHexNo0x(asUint(255))).toBe('ff');
  });

  it('converts 256 to "0100"', () => {
    expect(uintToBytesHexNo0x(asUint(256))).toBe('0100');
  });

  it('converts 65535 to "ffff"', () => {
    expect(uintToBytesHexNo0x(asUint(65535))).toBe('ffff');
  });

  it('converts 65536 to "010000"', () => {
    expect(uintToBytesHexNo0x(asUint(65536))).toBe('010000');
  });

  // BigInt values
  it('converts BigInt(0) to "00"', () => {
    expect(uintToBytesHexNo0x(asUint(BigInt(0)))).toBe('00');
  });

  it('converts BigInt(255) to "ff"', () => {
    expect(uintToBytesHexNo0x(asUint(BigInt(255)))).toBe('ff');
  });

  it('converts MAX_UINT64 correctly', () => {
    expect(uintToBytesHexNo0x(asUint(MAX_UINT64))).toBe('ffffffffffffffff');
  });

  it('converts MAX_UINT256 correctly', () => {
    expect(uintToBytesHexNo0x(asUint(MAX_UINT256))).toBe(
      'ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff',
    );
  });

  // Ensure even-length padding
  it('pads odd-length hex to even length', () => {
    // 0x1 -> 01
    expect(uintToBytesHexNo0x(asUint(1))).toBe('01');
    // 0xfff -> 0fff
    expect(uintToBytesHexNo0x(asUint(0xfff))).toBe('0fff');
    // 0xfffff -> 0fffff
    expect(uintToBytesHexNo0x(asUint(0xfffff))).toBe('0fffff');
  });
});

describe('uint256ToBytes32', () => {
  // Basic cases
  it('converts 0 to 32 zero bytes', () => {
    const result = uint256ToBytes32(0);
    expect(result).toBeInstanceOf(Uint8Array);
    expect(result.length).toBe(32);
    expect(result.every((b) => b === 0)).toBe(true);
  });

  it('converts 1 to bytes32 with last byte = 1', () => {
    const result = uint256ToBytes32(1);
    expect(result.length).toBe(32);
    expect(result[31]).toBe(1);
    expect(result.slice(0, 31).every((b) => b === 0)).toBe(true);
  });

  it('converts 1n to bytes32 with last byte = 1', () => {
    const result = uint256ToBytes32(1n);
    expect(result.length).toBe(32);
    expect(result[31]).toBe(1);
    expect(result.slice(0, 31).every((b) => b === 0)).toBe(true);
  });

  // Small values
  it('converts 255 (0xff) correctly', () => {
    const result = uint256ToBytes32(255);
    expect(result.length).toBe(32);
    expect(result[31]).toBe(255);
    expect(result.slice(0, 31).every((b) => b === 0)).toBe(true);
  });

  it('converts 256 (0x100) correctly', () => {
    const result = uint256ToBytes32(256);
    expect(result.length).toBe(32);
    expect(result[30]).toBe(1);
    expect(result[31]).toBe(0);
  });

  it('converts 0x01020304 correctly (big-endian)', () => {
    const result = uint256ToBytes32(0x01020304);
    expect(result.length).toBe(32);
    expect(result[28]).toBe(1);
    expect(result[29]).toBe(2);
    expect(result[30]).toBe(3);
    expect(result[31]).toBe(4);
  });

  // Boundary values for different uint sizes
  it('converts MAX_UINT8 correctly', () => {
    const result = uint256ToBytes32(MAX_UINT8);
    expect(result.length).toBe(32);
    expect(result[31]).toBe(255);
    expect(result.slice(0, 31).every((b) => b === 0)).toBe(true);
  });

  it('converts MAX_UINT32 correctly', () => {
    const result = uint256ToBytes32(MAX_UINT32);
    expect(result.length).toBe(32);
    expect(result[28]).toBe(255);
    expect(result[29]).toBe(255);
    expect(result[30]).toBe(255);
    expect(result[31]).toBe(255);
    expect(result.slice(0, 28).every((b) => b === 0)).toBe(true);
  });

  it('converts MAX_UINT64 correctly', () => {
    const result = uint256ToBytes32(MAX_UINT64);
    expect(result.length).toBe(32);
    // Last 8 bytes should all be 0xff
    for (let i = 24; i < 32; i++) {
      expect(result[i]).toBe(255);
    }
    expect(result.slice(0, 24).every((b) => b === 0)).toBe(true);
  });

  it('converts MAX_UINT128 correctly', () => {
    const result = uint256ToBytes32(MAX_UINT128);
    expect(result.length).toBe(32);
    // Last 16 bytes should all be 0xff
    for (let i = 16; i < 32; i++) {
      expect(result[i]).toBe(255);
    }
    expect(result.slice(0, 16).every((b) => b === 0)).toBe(true);
  });

  it('converts MAX_UINT256 correctly (all bytes 0xff)', () => {
    const result = uint256ToBytes32(MAX_UINT256);
    expect(result.length).toBe(32);
    expect(result.every((b) => b === 255)).toBe(true);
  });

  // Values that span multiple 64-bit chunks
  it('converts 2^64 correctly (first byte of third chunk)', () => {
    const value = BigInt(2) ** BigInt(64);
    const result = uint256ToBytes32(value);
    expect(result.length).toBe(32);
    expect(result[23]).toBe(1); // byte at position 23 (start of third 8-byte chunk from right)
    expect(result.slice(24, 32).every((b) => b === 0)).toBe(true);
    expect(result.slice(0, 23).every((b) => b === 0)).toBe(true);
  });

  it('converts 2^128 correctly (first byte of second chunk)', () => {
    const value = BigInt(2) ** BigInt(128);
    const result = uint256ToBytes32(value);
    expect(result.length).toBe(32);
    expect(result[15]).toBe(1); // byte at position 15 (start of second 8-byte chunk from right)
    expect(result.slice(16, 32).every((b) => b === 0)).toBe(true);
    expect(result.slice(0, 15).every((b) => b === 0)).toBe(true);
  });

  it('converts 2^192 correctly (first byte of first chunk)', () => {
    const value = BigInt(2) ** BigInt(192);
    const result = uint256ToBytes32(value);
    expect(result.length).toBe(32);
    expect(result[7]).toBe(1); // byte at position 7 (start of first 8-byte chunk from right)
    expect(result.slice(8, 32).every((b) => b === 0)).toBe(true);
    expect(result.slice(0, 7).every((b) => b === 0)).toBe(true);
  });

  it('converts 2^248 correctly (second byte)', () => {
    const value = BigInt(2) ** BigInt(248);
    const result = uint256ToBytes32(value);
    expect(result.length).toBe(32);
    expect(result[0]).toBe(1);
    expect(result.slice(1, 32).every((b) => b === 0)).toBe(true);
  });

  // Mixed value across all chunks
  it('converts a value spanning all four 64-bit chunks correctly', () => {
    // 0x0102030405060708_1112131415161718_2122232425262728_3132333435363738
    const value = BigInt('0x0102030405060708111213141516171821222324252627283132333435363738');
    const result = uint256ToBytes32(value);
    expect(result.length).toBe(32);
    // Verify first chunk (bytes 0-7)
    expect(result[0]).toBe(0x01);
    expect(result[7]).toBe(0x08);
    // Verify last chunk (bytes 24-31)
    expect(result[24]).toBe(0x31);
    expect(result[31]).toBe(0x38);
  });

  // Error cases
  it('throws for negative numbers', () => {
    expect(() => uint256ToBytes32(-1)).toThrow(InvalidTypeError);
    expect(() => uint256ToBytes32(-100)).toThrow(InvalidTypeError);
    expect(() => uint256ToBytes32(BigInt(-1))).toThrow(InvalidTypeError);
  });

  it('throws for non-integer numbers', () => {
    expect(() => uint256ToBytes32(1.5)).toThrow(InvalidTypeError);
    expect(() => uint256ToBytes32(0.1)).toThrow(InvalidTypeError);
  });

  it('throws for values exceeding MAX_UINT256', () => {
    expect(() => uint256ToBytes32(MAX_UINT256 + 1n)).toThrow(InvalidTypeError);
  });

  it('throws for invalid types', () => {
    expect(() => uint256ToBytes32('123')).toThrow(InvalidTypeError);
    expect(() => uint256ToBytes32(null)).toThrow(InvalidTypeError);
    expect(() => uint256ToBytes32(undefined)).toThrow(InvalidTypeError);
    expect(() => uint256ToBytes32({})).toThrow(InvalidTypeError);
    expect(() => uint256ToBytes32([])).toThrow(InvalidTypeError);
  });

  it('throws for NaN', () => {
    expect(() => uint256ToBytes32(NaN)).toThrow(InvalidTypeError);
  });

  it('throws for Infinity', () => {
    expect(() => uint256ToBytes32(Infinity)).toThrow(InvalidTypeError);
    expect(() => uint256ToBytes32(-Infinity)).toThrow(InvalidTypeError);
  });

  // Round-trip verification using DataView
  it('can be read back correctly using DataView for small values', () => {
    const value = 0x0102030405060708n;
    const result = uint256ToBytes32(value);
    const view = new DataView(result.buffer);
    expect(view.getBigUint64(24, false)).toBe(value);
  });

  it('can be read back correctly using DataView for MAX_UINT64', () => {
    const result = uint256ToBytes32(MAX_UINT64);
    const view = new DataView(result.buffer);
    expect(view.getBigUint64(24, false)).toBe(MAX_UINT64);
    expect(view.getBigUint64(16, false)).toBe(BigInt(0));
    expect(view.getBigUint64(8, false)).toBe(BigInt(0));
    expect(view.getBigUint64(0, false)).toBe(BigInt(0));
  });

  it('equivalent to hexToBytes(num.toString(16).padStart(64, "0"))', () => {
    const arr = [
      0,
      1,
      255,
      256,
      31337,
      11155111,
      MAX_UINT8,
      MAX_UINT32,
      MAX_UINT64,
      MAX_UINT128,
      MAX_UINT256,
      BigInt(2) ** BigInt(64),
      BigInt(2) ** BigInt(128),
      BigInt(2) ** BigInt(192),
    ];
    for (const elem of arr) {
      const b1 = hexToBytes(elem.toString(16).padStart(64, '0'));
      const b2 = uint256ToBytes32(elem);
      expect(b2).toEqual(b1);
    }
  });
});

describe('numberToBytesHexNo0x', () => {
  const testCases: [number, string][] = [
    [0, '00'],
    [1, '01'],
    [15, '0f'],
    [16, '10'],
    [255, 'ff'],
    [256, '0100'],
    [4095, '0fff'],
    [4096, '1000'],
    [65535, 'ffff'],
    [65536, '010000'],
    [0x1234567, '01234567'],
    [0x0102030, '102030'],
    [0x01020304, '01020304'],
    [0xdeadbeef, 'deadbeef'],
    [Number.MAX_SAFE_INTEGER, '1fffffffffffff'],
  ];

  it('converts numbers to hex strings without 0x prefix', () => {
    for (const [input, expected] of testCases) {
      expect(numberToBytesHexNo0x(input)).toBe(expected);
    }
  });
});

describe('numberToBytesHex', () => {
  const testCases: [number, string][] = [
    [0, '0x00'],
    [1, '0x01'],
    [15, '0x0f'],
    [16, '0x10'],
    [255, '0xff'],
    [256, '0x0100'],
    [4095, '0x0fff'],
    [4096, '0x1000'],
    [65535, '0xffff'],
    [65536, '0x010000'],
    [0x1234567, '0x01234567'],
    [0x0102030, '0x102030'],
    [0x01020304, '0x01020304'],
    [0xdeadbeef, '0xdeadbeef'],
    [Number.MAX_SAFE_INTEGER, '0x1fffffffffffff'],
  ];

  it('converts numbers to hex strings with 0x prefix', () => {
    for (const [input, expected] of testCases) {
      expect(numberToBytesHex(input)).toBe(expected);
    }
  });
});

describe('uintToHex', () => {
  const testCases: [number | bigint, string][] = [
    [0, '0x0'],
    [1, '0x1'],
    [15, '0xf'],
    [16, '0x10'],
    [255, '0xff'],
    [256, '0x100'],
    [4095, '0xfff'],
    [4096, '0x1000'],
    [65535, '0xffff'],
    [65536, '0x10000'],
    [0xdeadbeef, '0xdeadbeef'],
    [BigInt(0), '0x0'],
    [BigInt(255), '0xff'],
    [MAX_UINT64, '0xffffffffffffffff'],
    [MAX_UINT256, '0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff'],
  ];

  it('converts uint to hex string with 0x prefix (no padding)', () => {
    for (const [input, expected] of testCases) {
      expect(uintToHex0x(asUint(input))).toBe(expected);
    }
  });
});

describe('isUint256', () => {
  const validCases: (number | bigint)[] = [
    0,
    1,
    255,
    MAX_UINT8,
    MAX_UINT16,
    MAX_UINT32,
    MAX_UINT64,
    MAX_UINT128,
    MAX_UINT256,
    BigInt(0),
    1n,
    BigInt(MAX_UINT32),
    Number.MAX_SAFE_INTEGER,
  ];

  const invalidCases: unknown[] = [-1, -100, BigInt(-1), 1.5, 0.1, MAX_UINT256 + 1n, '123', null, undefined, {}, []];

  it('returns true for valid uint256 values', () => {
    for (const input of validCases) {
      expect(isUint256(input)).toBe(true);
    }
  });

  it('returns false for invalid uint256 values', () => {
    for (const input of invalidCases) {
      expect(isUint256(input)).toBe(false);
    }
  });
});

describe('uint64ToBytes32', () => {
  const validCases: [number | bigint, number[]][] = [
    // [input, expected last 8 bytes (indices 24-31)]
    [0, [0, 0, 0, 0, 0, 0, 0, 0]],
    [1, [0, 0, 0, 0, 0, 0, 0, 1]],
    [255, [0, 0, 0, 0, 0, 0, 0, 255]],
    [256, [0, 0, 0, 0, 0, 0, 1, 0]],
    [0x01020304, [0, 0, 0, 0, 1, 2, 3, 4]],
    [BigInt(0), [0, 0, 0, 0, 0, 0, 0, 0]],
    [1n, [0, 0, 0, 0, 0, 0, 0, 1]],
    [MAX_UINT64, [255, 255, 255, 255, 255, 255, 255, 255]],
  ];

  it('converts uint64 to 32 bytes (big-endian, right-aligned)', () => {
    for (const [input, expectedLastBytes] of validCases) {
      const result = uint64ToBytes32(input);
      expect(result).toBeInstanceOf(Uint8Array);
      expect(result.length).toBe(32);
      // First 24 bytes should be zero
      expect(result.slice(0, 24).every((b) => b === 0)).toBe(true);
      // Last 8 bytes should match expected
      expect(Array.from(result.slice(24))).toEqual(expectedLastBytes);
    }
  });

  it('throws for values exceeding MAX_UINT64', () => {
    expect(() => uint64ToBytes32(MAX_UINT64 + 1n)).toThrow(InvalidTypeError);
    expect(() => uint64ToBytes32(MAX_UINT128)).toThrow(InvalidTypeError);
    expect(() => uint64ToBytes32(MAX_UINT256)).toThrow(InvalidTypeError);
  });

  it('throws for negative numbers', () => {
    expect(() => uint64ToBytes32(-1)).toThrow(InvalidTypeError);
    expect(() => uint64ToBytes32(BigInt(-1))).toThrow(InvalidTypeError);
  });

  it('throws for non-integer numbers', () => {
    expect(() => uint64ToBytes32(1.5)).toThrow(InvalidTypeError);
  });

  it('throws for invalid types', () => {
    expect(() => uint64ToBytes32('123')).toThrow(InvalidTypeError);
    expect(() => uint64ToBytes32(null)).toThrow(InvalidTypeError);
    expect(() => uint64ToBytes32(undefined)).toThrow(InvalidTypeError);
  });
});

describe('isRecordUint256Property', () => {
  const validCases: Record<string, unknown>[] = [
    { foo: 0 },
    { foo: 1 },
    { foo: 255 },
    { foo: MAX_UINT256 },
    { foo: BigInt(0) },
    { foo: BigInt(MAX_UINT32) },
  ];

  const invalidCases: [unknown, string][] = [
    [{ foo: null }, 'null'],
    [{ foo: undefined }, 'undefined'],
    [{}, 'missing property'],
    [null, 'null object'],
    [{ foo: 'hello' }, 'string'],
    [{ foo: -1 }, 'negative'],
    [{ foo: 1.5 }, 'float'],
    [{ foo: MAX_UINT256 + 1n }, 'exceeds MAX_UINT256'],
  ];

  it('returns true for valid uint256 properties', () => {
    for (const input of validCases) {
      expect(isRecordUint256Property(input, 'foo')).toBe(true);
    }
  });

  it('returns false for invalid uint256 properties', () => {
    for (const [input] of invalidCases) {
      expect(isRecordUint256Property(input, 'foo')).toBe(false);
    }
  });
});

describe('assertRecordUint256Property', () => {
  const validCases: Record<string, unknown>[] = [{ foo: 0 }, { foo: 1 }, { foo: MAX_UINT256 }, { foo: BigInt(123) }];

  it('does not throw for valid uint256 properties', () => {
    for (const input of validCases) {
      expect(() => assertRecordUint256Property(input, 'foo', 'TestObj', {})).not.toThrow();
    }
  });

  it('throws for null property', () => {
    expect(() => assertRecordUint256Property({ foo: null }, 'foo', 'TestObj', {})).toThrow(InvalidPropertyError);
  });

  it('throws for undefined property', () => {
    expect(() => assertRecordUint256Property({ foo: undefined }, 'foo', 'TestObj', {})).toThrow(InvalidPropertyError);
  });

  it('throws for missing property', () => {
    expect(() => assertRecordUint256Property({}, 'foo', 'TestObj', {})).toThrow(InvalidPropertyError);
  });

  it('throws for string property', () => {
    expect(() => assertRecordUint256Property({ foo: '123' }, 'foo', 'TestObj', {})).toThrow(InvalidPropertyError);
  });

  it('throws for negative number', () => {
    expect(() => assertRecordUint256Property({ foo: -1 }, 'foo', 'TestObj', {})).toThrow(InvalidPropertyError);
  });

  it('throws for float', () => {
    expect(() => assertRecordUint256Property({ foo: 1.5 }, 'foo', 'TestObj', {})).toThrow(InvalidPropertyError);
  });

  it('throws for value exceeding MAX_UINT256', () => {
    expect(() => assertRecordUint256Property({ foo: MAX_UINT256 + 1n }, 'foo', 'TestObj', {})).toThrow(
      InvalidPropertyError,
    );
  });
});
