import type { BytesHex } from '../types/primitives.js';
import { describe, it, expect } from 'vitest';
import {
  assertRecordBytesHexProperty,
  assertRecordBytesHexArrayProperty,
  assertRecordBytes32HexArrayProperty,
  assertRecordBytes32HexProperty,
  assertIsBytes32Hex,
  assertIsBytesHex,
  assertIsBytesOrBytesHex,
  isBytes32Hex,
  isBytesHex,
  isBytesHexNo0x,
  assertIsBytesHexNo0x,
  hexToBytes,
  hexToBytesFaster,
  bytesToBigInt,
  bytesToHex,
  bytesToHexLarge,
  isBytes,
  isBytes32,
  isBytes65,
  isBytes32HexNo0x,
  isBytes65Hex,
  isBytes65HexNo0x,
  assertIsBytes32,
  assertIsBytes65,
  assertIsBytes65Hex,
  assertIsBytes32HexArray,
  assertIsBytesHexArray,
  assertIsBytes65HexArray,
  isRecordBytesHexNo0xProperty,
  assertRecordBytesHexNo0xProperty,
  assertRecordBytes65HexArrayProperty,
  assertRecordBytesHexNo0xArrayProperty,
  isRecordUint8ArrayProperty,
  assertRecordUint8ArrayProperty,
  bytesToHexNo0x,
  hexToBytes32,
  concatBytes,
  concatBytesHex,
  unsafeBytesEquals,
  isRecordBytesHexProperty,
  isRecordBytes32HexProperty,
  toBytes32HexArray,
  toBytes32,
  toBytes,
  bigIntToBytesHex,
  normalizeBytes,
  bytesHexSlice,
  bytesUint8At,
  bytesHexUint8At,
  bytesHexUint64At,
  createDeadbeefBytes,
} from './bytes.js';
import { InvalidTypeError } from './errors/InvalidTypeError.js';
import { InvalidPropertyError } from './errors/InvalidPropertyError.js';
import { MAX_UINT256 } from './uint.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/base/bytes.test.ts
////////////////////////////////////////////////////////////////////////////////

describe('bytes', () => {
  //////////////////////////////////////////////////////////////////////////////

  it('hexToBytes', () => {
    let arr = hexToBytes('0x');
    expect(arr instanceof Uint8Array).toBe(true);
    expect(arr.length).toBe(0);

    arr = hexToBytes('');
    expect(arr instanceof Uint8Array).toBe(true);
    expect(arr.length).toBe(0);

    arr = hexToBytes('0xff');
    expect(arr instanceof Uint8Array).toBe(true);
    expect(arr.length).toBe(1);
    expect(arr[0]).toBe(255);

    arr = hexToBytes('0x00');
    expect(arr instanceof Uint8Array).toBe(true);
    expect(arr.length).toBe(1);
    expect(arr[0]).toBe(0);

    expect(() => hexToBytes('0xf')).toThrow('Invalid hex string: odd length');

    arr = hexToBytes('za');
    expect(arr instanceof Uint8Array).toBe(true);
    expect(arr.length).toBe(1);
    expect(arr[0]).toBe(0);
    arr = hexToBytes('za');

    arr = hexToBytes('0xzazazazazaza');
    expect(arr instanceof Uint8Array).toBe(true);
    expect(arr.length).toBe(6);
    expect(arr[0]).toBe(0);
    expect(arr[1]).toBe(0);
    expect(arr[2]).toBe(0);
    expect(arr[3]).toBe(0);
    expect(arr[4]).toBe(0);
    expect(arr[5]).toBe(0);

    arr = hexToBytes('0xzzff');
    expect(arr instanceof Uint8Array).toBe(true);
    expect(arr.length).toBe(2);
    expect(arr[0]).toBe(0);
    expect(arr[1]).toBe(255);

    arr = hexToBytes('0xzfff');
    expect(arr instanceof Uint8Array).toBe(true);
    expect(arr.length).toBe(2);
    expect(arr[0]).toBe(0);
    expect(arr[1]).toBe(255);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('isBytesHex', () => {
    // True
    expect(isBytesHex('0x')).toEqual(true);
    expect(isBytesHex('0x00')).toEqual(true);
    expect(isBytesHex('0xdeadbeef')).toEqual(true);
    expect(isBytesHex('0x00', 32)).toEqual(false);
    expect(isBytesHex('0xdeadbeef')).toEqual(true);
    expect(isBytesHex('0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef', 32)).toEqual(true);
    expect(isBytesHex('0xdeadbeef', 32)).toEqual(false);

    // False
    expect(isBytesHex('deadbee')).toEqual(false);
    expect(isBytesHex('0x0')).toEqual(false);
    expect(isBytesHex('0xhello')).toEqual(false);
    expect(isBytesHex('0xdeadbee')).toEqual(false);
    expect(isBytesHex('0xdeadbeefzz')).toEqual(false);
    expect(isBytesHex('hello')).toEqual(false);
    expect(isBytesHex(null)).toEqual(false);
    expect(isBytesHex(undefined)).toEqual(false);
    expect(isBytesHex('')).toEqual(false);
    expect(isBytesHex('123')).toEqual(false);
    expect(isBytesHex(123)).toEqual(false);
    expect(isBytesHex(BigInt(123))).toEqual(false);
    expect(isBytesHex(123.0)).toEqual(false);
    expect(isBytesHex(123.1)).toEqual(false);
    expect(isBytesHex(-123)).toEqual(false);
    expect(isBytesHex(0)).toEqual(false);
    expect(isBytesHex({})).toEqual(false);
    expect(isBytesHex([])).toEqual(false);
    expect(isBytesHex([123])).toEqual(false);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertIsBytesHex', () => {
    // True
    expect(() => assertIsBytesHex('0x', {})).not.toThrow();
    expect(() => assertIsBytesHex('0x00', {})).not.toThrow();
    expect(() => assertIsBytesHex('0xdeadbeef', {})).not.toThrow();

    const e = (type: string) =>
      new InvalidTypeError(
        {
          expectedType: 'bytesHex',
          type,
        },
        {},
      );

    // False
    expect(() => assertIsBytesHex('deadbeef', {})).toThrow(e('string'));
    expect(() => assertIsBytesHex('0x0', {})).toThrow(e('string'));
    expect(() => assertIsBytesHex('0xhello', {})).toThrow(e('string'));
    expect(() => assertIsBytesHex('0xdeadbeefzz', {})).toThrow(e('string'));
    expect(() => assertIsBytesHex('0xdeadbee', {})).toThrow(e('string'));
    expect(() => assertIsBytesHex('hello', {})).toThrow(e('string'));
    expect(() => assertIsBytesHex(null, {})).toThrow(e('object'));
    expect(() => assertIsBytesHex(undefined, {})).toThrow(e('undefined'));
    expect(() => assertIsBytesHex('', {})).toThrow(e('string'));
    expect(() => assertIsBytesHex('123', {})).toThrow(e('string'));
    expect(() => assertIsBytesHex(123, {})).toThrow(e('number'));
    expect(() => assertIsBytesHex(BigInt(123), {})).toThrow(e('bigint'));
    expect(() => assertIsBytesHex(123.0, {})).toThrow(e('number'));
    expect(() => assertIsBytesHex(123.1, {})).toThrow(e('number'));
    expect(() => assertIsBytesHex(-123, {})).toThrow(e('number'));
    expect(() => assertIsBytesHex(0, {})).toThrow(e('number'));
    expect(() => assertIsBytesHex({}, {})).toThrow(e('object'));
    expect(() => assertIsBytesHex([], {})).toThrow(e('object'));
    expect(() => assertIsBytesHex([123], {})).toThrow(e('object'));
  });

  it('isBytes32Hex', () => {
    // True
    expect(isBytes32Hex('0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef')).toEqual(true);

    // False
    expect(isBytes32Hex('0x')).toEqual(false);
    expect(isBytes32Hex('0x00')).toEqual(false);
    expect(isBytes32Hex('0xdeadbeef')).toEqual(false);
    expect(isBytes32Hex('deadbee')).toEqual(false);
    expect(isBytes32Hex('0xdeadbeefdeadbeefdeadbeefdeadbeef')).toEqual(false);
    expect(isBytes32Hex('0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef')).toEqual(false);
  });

  it('assertIsBytes32Hex', () => {
    // True
    expect(() =>
      assertIsBytes32Hex('0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef', {}),
    ).not.toThrow();

    const e = (type: string) =>
      new InvalidTypeError(
        {
          expectedType: 'bytes32Hex',
          type,
        },
        {},
      );

    // False
    expect(() => assertIsBytes32Hex('0x', {})).toThrow(e('string'));
    expect(() => assertIsBytes32Hex('0x00', {})).toThrow(e('string'));
    expect(() => assertIsBytes32Hex('0xdeadbeef', {})).toThrow(e('string'));
    expect(() => assertIsBytes32Hex('deadbeef', {})).toThrow(e('string'));
    expect(() => assertIsBytes32Hex('0xdeadbeefdeadbeefdeadbeefdeadbeef', {})).toThrow(e('string'));
    expect(() => assertIsBytes32Hex('0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef', {})).toThrow(e('string'));
    expect(() => assertIsBytes32Hex('deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef', {})).toThrow(
      e('string'),
    );
    expect(() => assertIsBytes32Hex('deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefff', {})).toThrow(
      e('string'),
    );
  });

  it('assertBytes32HexProperty', () => {
    // True
    expect(() =>
      assertRecordBytes32HexProperty(
        {
          foo: '0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef',
        },
        'foo',
        'Foo',
        {},
      ),
    ).not.toThrow();

    // False
    expect(() => assertRecordBytes32HexProperty({ foo: null }, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'bytes32Hex',
          property: 'foo',
          type: 'undefined',
        },
        {},
      ),
    );

    expect(() => assertRecordBytes32HexProperty({ foo: undefined }, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'bytes32Hex',
          property: 'foo',
          type: 'undefined',
        },
        {},
      ),
    );

    expect(() => assertRecordBytes32HexProperty({}, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'bytes32Hex',
          property: 'foo',
          type: 'undefined',
        },
        {},
      ),
    );

    expect(() =>
      assertRecordBytes32HexProperty({ foo: 'DeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF' }, 'foo', 'Foo', {}),
    ).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'bytes32Hex',
          property: 'foo',
          type: 'string',
        },
        {},
      ),
    );
  });

  it('assertBytes32HexArrayProperty', () => {
    // True
    expect(() =>
      assertRecordBytes32HexArrayProperty(
        {
          foo: ['0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef'],
        },
        'foo',
        'Foo',
        {},
      ),
    ).not.toThrow();

    // False
    expect(() => assertRecordBytes32HexArrayProperty({ foo: null }, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'Array',
          type: 'undefined',
          property: 'foo',
        },
        {},
      ),
    );

    expect(() => assertRecordBytes32HexArrayProperty({ foo: undefined }, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'Array',
          type: 'undefined',
          property: 'foo',
        },
        {},
      ),
    );

    expect(() =>
      assertRecordBytes32HexArrayProperty({ foo: '0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF' }, 'foo', 'Foo', {}),
    ).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'Array',
          type: 'string',
          property: 'foo',
        },
        {},
      ),
    );

    expect(() => assertRecordBytes32HexArrayProperty({ foo: ['0xDeaDbeefdEAdbeef'] }, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'bytes32Hex',
          type: 'string',
          property: 'foo',
          index: 0,
        },
        {},
      ),
    );
  });

  it('assertBytesHexProperty', () => {
    // True
    expect(() =>
      assertRecordBytesHexProperty(
        {
          foo: '0xdead',
        },
        'foo',
        'Foo',
        {},
      ),
    ).not.toThrow();

    // False
    expect(() => assertRecordBytesHexProperty({ foo: null }, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'bytesHex',
          property: 'foo',
          type: 'undefined',
        },
        {},
      ),
    );

    expect(() => assertRecordBytesHexProperty({ foo: undefined }, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'bytesHex',
          property: 'foo',
          type: 'undefined',
        },
        {},
      ),
    );

    expect(() =>
      assertRecordBytesHexProperty({ foo: 'DeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF' }, 'foo', 'Foo', {}),
    ).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'bytesHex',
          property: 'foo',
          type: 'string',
        },
        {},
      ),
    );

    expect(() => assertRecordBytesHexProperty({ foo: '0xdeadbee' }, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'bytesHex',
          type: 'string',
          property: 'foo',
        },
        {},
      ),
    );

    expect(() => assertRecordBytesHexProperty({}, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'bytesHex',
          property: 'foo',
          type: 'undefined',
        },
        {},
      ),
    );
  });

  it('assertRecordBytesHexArrayProperty', () => {
    // True
    expect(() =>
      assertRecordBytesHexArrayProperty(
        {
          foo: ['0xdeadbeef'],
        },
        'foo',
        'Foo',
        {},
      ),
    ).not.toThrow();

    // False
    expect(() => assertRecordBytesHexArrayProperty({ foo: null }, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'Array',
          property: 'foo',
          type: 'undefined',
        },
        {},
      ),
    );

    expect(() => assertRecordBytesHexArrayProperty({ foo: undefined }, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'Array',
          property: 'foo',
          type: 'undefined',
        },
        {},
      ),
    );

    expect(() =>
      assertRecordBytesHexArrayProperty({ foo: '0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF' }, 'foo', 'Foo', {}),
    ).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'Array',
          property: 'foo',
          type: 'string',
        },
        {},
      ),
    );

    expect(() => assertRecordBytesHexArrayProperty({ foo: ['0xdeadbee'] }, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'bytesHex',
          property: 'foo',
          type: 'string',
          index: 0,
        },
        {},
      ),
    );
  });

  it('isBytesHexNo0x', () => {
    // True
    expect(isBytesHexNo0x('')).toEqual(true);
    expect(isBytesHexNo0x('00')).toEqual(true);
    expect(isBytesHexNo0x('deadbeef')).toEqual(true);

    // False
    expect(isBytesHexNo0x('0xdeadbeef')).toEqual(false);
    expect(isBytesHexNo0x('0')).toEqual(false);
    expect(isBytesHexNo0x('hello')).toEqual(false);
    expect(isBytesHexNo0x('deadbee')).toEqual(false);
    expect(isBytesHexNo0x('deadbeefzz')).toEqual(false);
    expect(isBytesHexNo0x(null)).toEqual(false);
    expect(isBytesHexNo0x(undefined)).toEqual(false);
    expect(isBytesHexNo0x('123')).toEqual(false);
    expect(isBytesHexNo0x(123)).toEqual(false);
    expect(isBytesHexNo0x(BigInt(123))).toEqual(false);
    expect(isBytesHexNo0x(123.0)).toEqual(false);
    expect(isBytesHexNo0x(123.1)).toEqual(false);
    expect(isBytesHexNo0x(-123)).toEqual(false);
    expect(isBytesHexNo0x(0)).toEqual(false);
    expect(isBytesHexNo0x({})).toEqual(false);
    expect(isBytesHexNo0x([])).toEqual(false);
    expect(isBytesHexNo0x([123])).toEqual(false);
  });

  it('assertIsBytesHexNo0x', () => {
    // True
    expect(() => assertIsBytesHexNo0x('', {})).not.toThrow();
    expect(() => assertIsBytesHexNo0x('00', {})).not.toThrow();
    expect(() => assertIsBytesHexNo0x('deadbeef', {})).not.toThrow();

    const e = (type: string) =>
      new InvalidTypeError(
        {
          expectedType: 'bytesHexNo0x',
          type,
        },
        {},
      );

    // False
    expect(() => assertIsBytesHexNo0x('0xdeadbeef', {})).toThrow(e('string'));
    expect(() => assertIsBytesHexNo0x('0', {})).toThrow(e('string'));
    expect(() => assertIsBytesHexNo0x('hello', {})).toThrow(e('string'));
    expect(() => assertIsBytesHexNo0x('deadbeefzz', {})).toThrow(e('string'));
    expect(() => assertIsBytesHexNo0x('deadbee', {})).toThrow(e('string'));
    expect(() => assertIsBytesHexNo0x('hello', {})).toThrow(e('string'));
    expect(() => assertIsBytesHexNo0x(null, {})).toThrow(e('object'));
    expect(() => assertIsBytesHexNo0x(undefined, {})).toThrow(e('undefined'));
    expect(() => assertIsBytesHexNo0x('123', {})).toThrow(e('string'));
    expect(() => assertIsBytesHexNo0x(123, {})).toThrow(e('number'));
    expect(() => assertIsBytesHexNo0x(BigInt(123), {})).toThrow(e('bigint'));
    expect(() => assertIsBytesHexNo0x(123.0, {})).toThrow(e('number'));
    expect(() => assertIsBytesHexNo0x(123.1, {})).toThrow(e('number'));
    expect(() => assertIsBytesHexNo0x(-123, {})).toThrow(e('number'));
    expect(() => assertIsBytesHexNo0x(0, {})).toThrow(e('number'));
    expect(() => assertIsBytesHexNo0x({}, {})).toThrow(e('object'));
    expect(() => assertIsBytesHexNo0x([], {})).toThrow(e('object'));
    expect(() => assertIsBytesHexNo0x([123], {})).toThrow(e('object'));
  });
});

describe('bytesToBigInt', () => {
  it('should return 0n for undefined input', () => {
    expect(bytesToBigInt(undefined)).toBe(BigInt(0));
  });

  it('should return 0n for empty array', () => {
    expect(bytesToBigInt(new Uint8Array([]))).toBe(BigInt(0));
  });

  it('should convert single byte correctly', () => {
    expect(bytesToBigInt(new Uint8Array([0]))).toBe(BigInt(0));
    expect(bytesToBigInt(new Uint8Array([1]))).toBe(BigInt(1));
    expect(bytesToBigInt(new Uint8Array([255]))).toBe(BigInt(255));
  });

  it('should convert two bytes correctly (big-endian)', () => {
    expect(bytesToBigInt(new Uint8Array([0x01, 0x00]))).toBe(BigInt(256));
    expect(bytesToBigInt(new Uint8Array([0x01, 0x01]))).toBe(BigInt(257));
    expect(bytesToBigInt(new Uint8Array([0xff, 0xff]))).toBe(BigInt(65535));
  });

  it('should convert multiple bytes correctly', () => {
    // 0x010203 = 66051
    expect(bytesToBigInt(new Uint8Array([0x01, 0x02, 0x03]))).toBe(BigInt(66051));
  });

  it('should handle large values', () => {
    // 32 bytes (256-bit value)
    const bytes = new Uint8Array(32);
    bytes[0] = 0x01;
    expect(bytesToBigInt(bytes)).toBe(BigInt('0x0100000000000000000000000000000000000000000000000000000000000000'));
  });

  it('should handle max uint256', () => {
    const maxUint256 = new Uint8Array(32).fill(0xff);
    expect(bytesToBigInt(maxUint256)).toBe(MAX_UINT256);
  });
});

describe('bytesToHex - hexToBytes', () => {
  it('converts a hex zero address to bytes', async () => {
    const value = '0x0000000000000000000000000000000000000000';
    const bytes20 = hexToBytes(value);
    expect(bytes20).toEqual(new Uint8Array([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
  });

  it('converts a hex to bytes', async () => {
    const value = '0xff';
    const bytes1 = hexToBytes(value);
    expect(bytes1).toEqual(new Uint8Array([255]));

    const bytes2 = hexToBytes('0x');
    expect(bytes2).toEqual(new Uint8Array([]));
  });

  it('converts a bytes to hex', async () => {
    const bytes1 = bytesToHex(new Uint8Array([255]));
    expect(bytes1).toEqual('0xff');
    expect(bytesToHexLarge(new Uint8Array([255]))).toEqual(bytes1);

    const bytes2 = bytesToHex(new Uint8Array());
    expect(bytesToHexLarge(new Uint8Array())).toEqual(bytes2);
    expect(bytes2).toEqual('0x');
  });

  it('converts bytes to number', async () => {
    const value = new Uint8Array([23, 200, 15]);
    const bigint1 = bytesToBigInt(value);
    expect(bigint1.toString()).toBe('1558543');

    const value2 = new Uint8Array([37, 6, 210, 166, 239]);
    const bigint2 = bytesToBigInt(value2);
    expect(bigint2.toString()).toBe('159028258543');

    const value0 = new Uint8Array();
    const bigint0 = bytesToBigInt(value0);
    expect(bigint0.toString()).toBe('0');
  });

  describe('hexToBytes edge cases', () => {
    // Empty inputs
    it('handles empty string', () => {
      expect(hexToBytes('')).toEqual(new Uint8Array([]));
    });

    it('handles 0x only', () => {
      expect(hexToBytes('0x')).toEqual(new Uint8Array([]));
    });

    // Single character (odd length) - throws error
    it('throws for single hex character (odd length)', () => {
      expect(() => hexToBytes('f')).toThrow('Invalid hex string: odd length');
      expect(() => hexToBytes('0')).toThrow('Invalid hex string: odd length');
      expect(() => hexToBytes('a')).toThrow('Invalid hex string: odd length');
    });

    it('throws for 0x + single character (odd length)', () => {
      expect(() => hexToBytes('0xf')).toThrow('Invalid hex string: odd length');
      expect(() => hexToBytes('0x0')).toThrow('Invalid hex string: odd length');
      expect(() => hexToBytes('0xa')).toThrow('Invalid hex string: odd length');
    });

    // Valid hex strings
    it('handles valid lowercase hex', () => {
      expect(hexToBytes('0xdeadbeef')).toEqual(new Uint8Array([0xde, 0xad, 0xbe, 0xef]));
    });

    it('handles valid uppercase hex', () => {
      expect(hexToBytes('0xDEADBEEF')).toEqual(new Uint8Array([0xde, 0xad, 0xbe, 0xef]));
    });

    it('handles mixed case hex', () => {
      expect(hexToBytes('0xDeAdBeEf')).toEqual(new Uint8Array([0xde, 0xad, 0xbe, 0xef]));
    });

    it('handles hex without 0x prefix', () => {
      expect(hexToBytes('deadbeef')).toEqual(new Uint8Array([0xde, 0xad, 0xbe, 0xef]));
    });

    // Invalid characters (parseInt returns NaN -> 0)
    it('converts invalid hex chars to 0 (via parseInt NaN)', () => {
      // 'zz' -> parseInt('zz', 16) = NaN -> becomes 0 in Uint8Array
      expect(hexToBytes('zz')).toEqual(new Uint8Array([0]));
      expect(hexToBytes('0xzz')).toEqual(new Uint8Array([0]));
    });

    it('handles mixed valid and invalid chars', () => {
      // 'zf' -> parseInt('zf', 16) = NaN -> 0
      // 'ff' -> 255
      expect(hexToBytes('0xzfff')).toEqual(new Uint8Array([0, 255]));
      expect(hexToBytes('0xffzf')).toEqual(new Uint8Array([255, 0]));
    });

    it('handles all zeros', () => {
      expect(hexToBytes('0x0000')).toEqual(new Uint8Array([0, 0]));
      expect(hexToBytes('0x00000000')).toEqual(new Uint8Array([0, 0, 0, 0]));
    });

    it('handles all ones (0xff)', () => {
      expect(hexToBytes('0xffff')).toEqual(new Uint8Array([255, 255]));
      expect(hexToBytes('0xffffffff')).toEqual(new Uint8Array([255, 255, 255, 255]));
    });

    // Boundary values
    it('handles boundary byte values', () => {
      expect(hexToBytes('0x00')).toEqual(new Uint8Array([0]));
      expect(hexToBytes('0x01')).toEqual(new Uint8Array([1]));
      expect(hexToBytes('0x7f')).toEqual(new Uint8Array([127])); // max signed byte
      expect(hexToBytes('0x80')).toEqual(new Uint8Array([128])); // min negative in signed
      expect(hexToBytes('0xfe')).toEqual(new Uint8Array([254]));
      expect(hexToBytes('0xff')).toEqual(new Uint8Array([255]));
    });

    // Whitespace - odd length after removing 0x prefix throws
    it('whitespace in string throws for odd length', () => {
      // '0x ff' after removing '0x' is ' ff' (3 chars, odd length)
      expect(() => hexToBytes('0x ff')).toThrow('Invalid hex string: odd length');
    });

    // Long strings
    it('handles 32-byte (256-bit) hex string', () => {
      const hex = '0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef';
      const bytes = hexToBytes(hex);
      expect(bytes.length).toBe(32);
      expect(bytes[0]).toBe(0x01);
      expect(bytes[1]).toBe(0x23);
      expect(bytes[31]).toBe(0xef);
    });

    // Special prefix handling
    it('handles 0X (uppercase X) prefix', () => {
      // The regex only removes lowercase 0x
      const result = hexToBytes('0Xff');
      // '0Xff' without 0x removal -> ['0X', 'ff'] -> [NaN, 255] -> [0, 255]
      expect(result).toEqual(new Uint8Array([0, 255]));
    });

    // Leading zeros preservation
    it('preserves leading zeros', () => {
      expect(hexToBytes('0x0001')).toEqual(new Uint8Array([0, 1]));
      expect(hexToBytes('0x000000ff')).toEqual(new Uint8Array([0, 0, 0, 255]));
    });

    // Three-character odd length strings - throws error
    it('throws for three-character hex (odd length)', () => {
      expect(() => hexToBytes('fff')).toThrow('Invalid hex string: odd length');
      expect(() => hexToBytes('0xfff')).toThrow('Invalid hex string: odd length');
      expect(() => hexToBytes('abc')).toThrow('Invalid hex string: odd length');
      expect(() => hexToBytes('0xabc')).toThrow('Invalid hex string: odd length');
      expect(() => hexToBytes('123')).toThrow('Invalid hex string: odd length');
      expect(() => hexToBytes('0x123')).toThrow('Invalid hex string: odd length');
    });

    // Five-character odd length strings - throws error
    it('throws for five-character hex (odd length)', () => {
      expect(() => hexToBytes('0x12345')).toThrow('Invalid hex string: odd length');
      expect(() => hexToBytes('abcde')).toThrow('Invalid hex string: odd length');
    });

    // Ethereum address format (20 bytes)
    it('handles Ethereum address format (20 bytes)', () => {
      const address = '0x1234567890abcdef1234567890abcdef12345678';
      const bytes = hexToBytes(address);
      expect(bytes.length).toBe(20);
      expect(bytes[0]).toBe(0x12);
      expect(bytes[19]).toBe(0x78);
    });

    // Ethereum address without 0x prefix
    it('handles Ethereum address without 0x prefix', () => {
      const address = '1234567890abcdef1234567890abcdef12345678';
      const bytes = hexToBytes(address);
      expect(bytes.length).toBe(20);
      expect(bytes[0]).toBe(0x12);
      expect(bytes[19]).toBe(0x78);
    });

    // Round-trip: hexToBytes -> bytesToHex
    it('round-trips correctly with bytesToHex', () => {
      const testCases = ['0x', '0xff', '0x00', '0xdeadbeef', '0x0123456789abcdef', '0x000000ff'];
      for (const hex of testCases) {
        expect(bytesToHex(hexToBytes(hex))).toBe(hex);
        expect(bytesToHexLarge(hexToBytes(hex))).toBe(hex);
      }
    });
  });

  describe('hexToBytes', () => {
    it('converts a hex to bytes', async () => {
      const value = '0xff';
      const bytes1 = hexToBytes(value);
      expect(bytes1).toEqual(new Uint8Array([255]));

      const bytes2 = hexToBytes('0x');
      expect(bytes2).toEqual(new Uint8Array([]));
    });
  });
});

describe('hexToBytesFaster', () => {
  // Empty inputs
  it('handles empty string', () => {
    expect(hexToBytesFaster('')).toEqual(new Uint8Array([]));
  });

  it('handles 0x only', () => {
    expect(hexToBytesFaster('0x')).toEqual(new Uint8Array([]));
  });

  it('converts a hex zero address to bytes', async () => {
    const value = '0x0000000000000000000000000000000000000000';
    const bytes20 = hexToBytesFaster(value);
    expect(bytes20).toEqual(new Uint8Array([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
  });

  // Basic conversions
  it('converts single byte hex', () => {
    expect(hexToBytesFaster('0xff')).toEqual(new Uint8Array([255]));
    expect(hexToBytesFaster('0x00')).toEqual(new Uint8Array([0]));
    expect(hexToBytesFaster('0x01')).toEqual(new Uint8Array([1]));
    expect(hexToBytesFaster('0x7f')).toEqual(new Uint8Array([127]));
    expect(hexToBytesFaster('0x80')).toEqual(new Uint8Array([128]));
    expect(hexToBytesFaster('0xfe')).toEqual(new Uint8Array([254]));
  });

  it('converts multi-byte hex with 0x prefix', () => {
    expect(hexToBytesFaster('0xdeadbeef')).toEqual(new Uint8Array([0xde, 0xad, 0xbe, 0xef]));
    expect(hexToBytesFaster('0x0123456789abcdef')).toEqual(
      new Uint8Array([0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]),
    );
  });

  it('converts hex without 0x prefix', () => {
    expect(hexToBytesFaster('ff')).toEqual(new Uint8Array([255]));
    expect(hexToBytesFaster('deadbeef')).toEqual(new Uint8Array([0xde, 0xad, 0xbe, 0xef]));
  });

  // Case handling
  it('handles lowercase hex', () => {
    expect(hexToBytesFaster('0xabcdef')).toEqual(new Uint8Array([0xab, 0xcd, 0xef]));
  });

  it('handles uppercase hex', () => {
    expect(hexToBytesFaster('0xABCDEF')).toEqual(new Uint8Array([0xab, 0xcd, 0xef]));
  });

  it('handles mixed case hex', () => {
    expect(hexToBytesFaster('0xAbCdEf')).toEqual(new Uint8Array([0xab, 0xcd, 0xef]));
  });

  // Odd length - throws error
  it('throws for odd length hex string', () => {
    expect(() => hexToBytesFaster('f')).toThrow('hex string length must be even');
    expect(() => hexToBytesFaster('0xf')).toThrow('hex string length must be even');
    expect(() => hexToBytesFaster('fff')).toThrow('hex string length must be even');
    expect(() => hexToBytesFaster('0xfff')).toThrow('hex string length must be even');
    expect(() => hexToBytesFaster('0x12345')).toThrow('hex string length must be even');
  });

  // Invalid characters - non-strict mode (default)
  it('converts invalid hex chars to 0 in non-strict mode', () => {
    // Invalid chars result in undefined from lookup, which becomes NaN -> 0
    expect(hexToBytesFaster('zz')).toEqual(new Uint8Array([0]));
    expect(hexToBytesFaster('0xzz')).toEqual(new Uint8Array([0]));
    expect(hexToBytesFaster('0xzzff')).toEqual(new Uint8Array([0, 255]));
    expect(hexToBytesFaster('0xffzz')).toEqual(new Uint8Array([255, 0]));
  });

  // Invalid characters - strict mode
  it('throws for invalid hex chars in strict mode', () => {
    expect(() => hexToBytesFaster('zz', { strict: true })).toThrow('invalid hex character at position 0');
    expect(() => hexToBytesFaster('0xzz', { strict: true })).toThrow('invalid hex character at position 2');
    expect(() => hexToBytesFaster('0xffzz', { strict: true })).toThrow('invalid hex character at position 4');
    expect(() => hexToBytesFaster('0xghij', { strict: true })).toThrow('invalid hex character at position 2');
  });

  it('does not throw for valid hex in strict mode', () => {
    expect(hexToBytesFaster('0xdeadbeef', { strict: true })).toEqual(new Uint8Array([0xde, 0xad, 0xbe, 0xef]));
    expect(hexToBytesFaster('0xABCDEF', { strict: true })).toEqual(new Uint8Array([0xab, 0xcd, 0xef]));
    expect(hexToBytesFaster('0x0123456789abcdef', { strict: true })).toEqual(
      new Uint8Array([0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]),
    );
  });

  // Leading zeros preservation
  it('preserves leading zeros', () => {
    expect(hexToBytesFaster('0x0001')).toEqual(new Uint8Array([0, 1]));
    expect(hexToBytesFaster('0x000000ff')).toEqual(new Uint8Array([0, 0, 0, 255]));
  });

  // Long strings
  it('handles 32-byte (256-bit) hex string', () => {
    const hex = '0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef';
    const bytes = hexToBytesFaster(hex);
    expect(bytes.length).toBe(32);
    expect(bytes[0]).toBe(0x01);
    expect(bytes[1]).toBe(0x23);
    expect(bytes[31]).toBe(0xef);
  });

  // Ethereum address format (20 bytes)
  it('handles Ethereum address format (20 bytes)', () => {
    const address = '0x1234567890abcdef1234567890abcdef12345678';
    const bytes = hexToBytesFaster(address);
    expect(bytes.length).toBe(20);
    expect(bytes[0]).toBe(0x12);
    expect(bytes[19]).toBe(0x78);
  });

  // 0X uppercase prefix - treated as part of the hex string (no special handling)
  it('handles 0X (uppercase X) prefix differently than 0x', () => {
    // '0Xff' - the '0X' is not recognized as prefix, so it's parsed as hex
    // '0X' -> invalid chars -> 0
    // 'ff' -> 255
    const result = hexToBytesFaster('0Xff');
    expect(result).toEqual(new Uint8Array([0, 255]));
  });

  // Round-trip with bytesToHex
  it('round-trips correctly with bytesToHex', () => {
    const testCases = ['0x', '0xff', '0x00', '0xdeadbeef', '0x0123456789abcdef', '0x000000ff'];
    for (const hex of testCases) {
      expect(bytesToHex(hexToBytesFaster(hex))).toBe(hex);
    }
  });

  // Comparison with hexToBytes for valid inputs
  it('produces same results as hexToBytes for valid even-length hex', () => {
    const testCases = [
      '',
      '0x',
      '0xff',
      '0x00',
      '0xdeadbeef',
      '0xDEADBEEF',
      '0xDeAdBeEf',
      'deadbeef',
      '0x0123456789abcdef',
      '0x000000ff',
      '0x1234567890abcdef1234567890abcdef12345678',
    ];
    for (const hex of testCases) {
      expect(hexToBytesFaster(hex)).toEqual(hexToBytes(hex));
    }
  });
});

describe('isBytes', () => {
  it('returns true for Uint8Array', () => {
    expect(isBytes(new Uint8Array([]))).toBe(true);
    expect(isBytes(new Uint8Array([1, 2, 3]))).toBe(true);
    expect(isBytes(new Uint8Array(32))).toBe(true);
  });

  it('returns false for non-Uint8Array', () => {
    expect(isBytes(null)).toBe(false);
    expect(isBytes(undefined)).toBe(false);
    expect(isBytes([])).toBe(false);
    expect(isBytes([1, 2, 3])).toBe(false);
    expect(isBytes('0x1234')).toBe(false);
    expect(isBytes(123)).toBe(false);
    expect(isBytes({})).toBe(false);
  });

  it('validates bytewidth when provided', () => {
    const bytes32 = new Uint8Array(32);
    const bytes65 = new Uint8Array(65);
    const bytes20 = new Uint8Array(20);

    expect(isBytes(bytes32, 32)).toBe(true);
    expect(isBytes(bytes65, 65)).toBe(true);
    expect(isBytes(bytes32, 65)).toBe(false);
    expect(isBytes(bytes65, 32)).toBe(false);
    expect(isBytes(bytes20, 32)).toBe(false);
  });
});

describe('isBytes32', () => {
  it('returns true for 32-byte Uint8Array', () => {
    expect(isBytes32(new Uint8Array(32))).toBe(true);
    expect(isBytes32(new Uint8Array(32).fill(0xff))).toBe(true);
  });

  it('returns false for non-32-byte values', () => {
    expect(isBytes32(new Uint8Array(31))).toBe(false);
    expect(isBytes32(new Uint8Array(33))).toBe(false);
    expect(isBytes32(new Uint8Array(0))).toBe(false);
    expect(isBytes32(new Uint8Array(65))).toBe(false);
    expect(isBytes32(null)).toBe(false);
    expect(isBytes32('0x' + 'de'.repeat(32))).toBe(false);
  });
});

describe('isBytes65', () => {
  it('returns true for 65-byte Uint8Array', () => {
    expect(isBytes65(new Uint8Array(65))).toBe(true);
    expect(isBytes65(new Uint8Array(65).fill(0xff))).toBe(true);
  });

  it('returns false for non-65-byte values', () => {
    expect(isBytes65(new Uint8Array(64))).toBe(false);
    expect(isBytes65(new Uint8Array(66))).toBe(false);
    expect(isBytes65(new Uint8Array(32))).toBe(false);
    expect(isBytes65(new Uint8Array(0))).toBe(false);
    expect(isBytes65(null)).toBe(false);
    expect(isBytes65('0x' + 'de'.repeat(65))).toBe(false);
  });
});

describe('isBytes32HexNo0x', () => {
  const valid32HexNo0x = 'de'.repeat(32);

  it('returns true for valid 32-byte hex without 0x', () => {
    expect(isBytes32HexNo0x(valid32HexNo0x)).toBe(true);
    expect(isBytes32HexNo0x('00'.repeat(32))).toBe(true);
    expect(isBytes32HexNo0x('ff'.repeat(32))).toBe(true);
  });

  it('returns false for invalid values', () => {
    expect(isBytes32HexNo0x('0x' + valid32HexNo0x)).toBe(false);
    expect(isBytes32HexNo0x('de'.repeat(31))).toBe(false);
    expect(isBytes32HexNo0x('de'.repeat(33))).toBe(false);
    expect(isBytes32HexNo0x('')).toBe(false);
    expect(isBytes32HexNo0x(null)).toBe(false);
    expect(isBytes32HexNo0x(undefined)).toBe(false);
  });
});

describe('isBytes65Hex', () => {
  const valid65Hex = '0x' + 'de'.repeat(65);

  it('returns true for valid 65-byte hex with 0x', () => {
    expect(isBytes65Hex(valid65Hex)).toBe(true);
    expect(isBytes65Hex('0x' + '00'.repeat(65))).toBe(true);
    expect(isBytes65Hex('0x' + 'ff'.repeat(65))).toBe(true);
  });

  it('returns false for invalid values', () => {
    expect(isBytes65Hex('de'.repeat(65))).toBe(false);
    expect(isBytes65Hex('0x' + 'de'.repeat(64))).toBe(false);
    expect(isBytes65Hex('0x' + 'de'.repeat(66))).toBe(false);
    expect(isBytes65Hex('0x' + 'de'.repeat(32))).toBe(false);
    expect(isBytes65Hex('0x')).toBe(false);
    expect(isBytes65Hex(null)).toBe(false);
  });
});

describe('isBytes65HexNo0x', () => {
  const valid65HexNo0x = 'de'.repeat(65);

  it('returns true for valid 65-byte hex without 0x', () => {
    expect(isBytes65HexNo0x(valid65HexNo0x)).toBe(true);
    expect(isBytes65HexNo0x('00'.repeat(65))).toBe(true);
    expect(isBytes65HexNo0x('ff'.repeat(65))).toBe(true);
  });

  it('returns false for invalid values', () => {
    expect(isBytes65HexNo0x('0x' + valid65HexNo0x)).toBe(false);
    expect(isBytes65HexNo0x('de'.repeat(64))).toBe(false);
    expect(isBytes65HexNo0x('de'.repeat(66))).toBe(false);
    expect(isBytes65HexNo0x('')).toBe(false);
    expect(isBytes65HexNo0x(null)).toBe(false);
  });
});

describe('assertIsBytes32', () => {
  it('does not throw for valid 32-byte Uint8Array', () => {
    expect(() => assertIsBytes32(new Uint8Array(32), {})).not.toThrow();
  });

  it('throws for invalid values', () => {
    expect(() => assertIsBytes32(new Uint8Array(31), {})).toThrow(
      new InvalidTypeError({ expectedType: 'bytes32' }, {}),
    );
    expect(() => assertIsBytes32(new Uint8Array(33), {})).toThrow(InvalidTypeError);
    expect(() => assertIsBytes32(null, {})).toThrow(InvalidTypeError);
    expect(() => assertIsBytes32('0x' + 'de'.repeat(32), {})).toThrow(InvalidTypeError);
  });
});

describe('assertIsBytes65', () => {
  it('does not throw for valid 65-byte Uint8Array', () => {
    expect(() => assertIsBytes65(new Uint8Array(65), {})).not.toThrow();
  });

  it('throws for invalid values', () => {
    expect(() => assertIsBytes65(new Uint8Array(64), {})).toThrow(
      new InvalidTypeError({ expectedType: 'bytes65' }, {}),
    );
    expect(() => assertIsBytes65(new Uint8Array(66), {})).toThrow(InvalidTypeError);
    expect(() => assertIsBytes65(null, {})).toThrow(InvalidTypeError);
  });
});

describe('assertIsBytes65Hex', () => {
  const valid65Hex = '0x' + 'de'.repeat(65);

  it('does not throw for valid 65-byte hex', () => {
    expect(() => assertIsBytes65Hex(valid65Hex, {})).not.toThrow();
  });

  it('throws for invalid values', () => {
    expect(() => assertIsBytes65Hex('0x' + 'de'.repeat(64), {})).toThrow(
      new InvalidTypeError({ type: 'string', expectedType: 'bytes65Hex' }, {}),
    );
    expect(() => assertIsBytes65Hex('de'.repeat(65), {})).toThrow(InvalidTypeError);
    expect(() => assertIsBytes65Hex(null, {})).toThrow(InvalidTypeError);
  });
});

describe('assertIsBytes32HexArray', () => {
  const valid32Hex = '0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef';

  it('does not throw for valid array', () => {
    expect(() => assertIsBytes32HexArray([], {})).not.toThrow();
    expect(() => assertIsBytes32HexArray([valid32Hex], {})).not.toThrow();
    expect(() => assertIsBytes32HexArray([valid32Hex, valid32Hex], {})).not.toThrow();
  });

  it('throws for non-array', () => {
    expect(() => assertIsBytes32HexArray(null, {})).toThrow(
      new InvalidTypeError({ type: 'object', expectedType: 'bytes32Hex[]' }, {}),
    );
    expect(() => assertIsBytes32HexArray('string', {})).toThrow(InvalidTypeError);
  });

  it('throws for array with invalid elements', () => {
    expect(() => assertIsBytes32HexArray(['0xdeadbeef'], {})).toThrow(
      new InvalidTypeError({ type: 'string', expectedType: 'bytes32Hex' }, {}),
    );
    expect(() => assertIsBytes32HexArray([valid32Hex, '0xinvalid'], {})).toThrow(InvalidTypeError);
  });
});

describe('assertIsBytesHexArray', () => {
  it('does not throw for valid array', () => {
    expect(() => assertIsBytesHexArray([], {})).not.toThrow();
    expect(() => assertIsBytesHexArray(['0xdeadbeef'], {})).not.toThrow();
    expect(() => assertIsBytesHexArray(['0x', '0xdeadbeef'], {})).not.toThrow();
  });

  it('throws for non-array', () => {
    expect(() => assertIsBytesHexArray(null, {})).toThrow(
      new InvalidTypeError({ type: 'object', expectedType: 'bytesHex[]' }, {}),
    );
    expect(() => assertIsBytesHexArray('string', {})).toThrow(InvalidTypeError);
  });

  it('throws for array with invalid elements', () => {
    expect(() => assertIsBytesHexArray(['deadbeef'], {})).toThrow(
      new InvalidTypeError({ type: 'string', expectedType: 'bytesHex' }, {}),
    );
    expect(() => assertIsBytesHexArray(['0xdeadbeef', '0xdeadbee'], {})).toThrow(InvalidTypeError);
  });
});

describe('assertIsBytes65HexArray', () => {
  const valid65Hex = '0x' + 'de'.repeat(65);

  it('does not throw for valid array', () => {
    expect(() => assertIsBytes65HexArray([], {})).not.toThrow();
    expect(() => assertIsBytes65HexArray([valid65Hex], {})).not.toThrow();
    expect(() => assertIsBytes65HexArray([valid65Hex, valid65Hex], {})).not.toThrow();
  });

  it('throws for non-array', () => {
    expect(() => assertIsBytes65HexArray(null, {})).toThrow(
      new InvalidTypeError({ type: 'object', expectedType: 'bytes65Hex[]' }, {}),
    );
  });

  it('throws for array with invalid elements', () => {
    expect(() => assertIsBytes65HexArray(['0xdeadbeef'], {})).toThrow(
      new InvalidTypeError({ type: 'string', expectedType: 'bytes65Hex' }, {}),
    );
  });
});

describe('isRecordBytesHexProperty', () => {
  it('returns true for valid BytesHex property', () => {
    expect(isRecordBytesHexProperty({ foo: '0xdeadbeef' }, 'foo')).toBe(true);
    expect(isRecordBytesHexProperty({ foo: '0x' }, 'foo')).toBe(true);
  });

  it('returns false for invalid values', () => {
    expect(isRecordBytesHexProperty({ foo: 'deadbeef' }, 'foo')).toBe(false);
    expect(isRecordBytesHexProperty({ foo: null }, 'foo')).toBe(false);
    expect(isRecordBytesHexProperty({}, 'foo')).toBe(false);
    expect(isRecordBytesHexProperty(null, 'foo')).toBe(false);
  });
});

describe('isRecordBytes32HexProperty', () => {
  const valid32Hex = '0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef';

  it('returns true for valid Bytes32Hex property', () => {
    expect(isRecordBytes32HexProperty({ foo: valid32Hex }, 'foo')).toBe(true);
  });

  it('returns false for invalid values', () => {
    expect(isRecordBytes32HexProperty({ foo: '0xdeadbeef' }, 'foo')).toBe(false);
    expect(isRecordBytes32HexProperty({ foo: null }, 'foo')).toBe(false);
    expect(isRecordBytes32HexProperty({}, 'foo')).toBe(false);
    expect(isRecordBytes32HexProperty(null, 'foo')).toBe(false);
  });
});

describe('isRecordBytesHexNo0xProperty', () => {
  it('returns true for valid BytesHexNo0x property', () => {
    expect(isRecordBytesHexNo0xProperty({ foo: 'deadbeef' }, 'foo')).toBe(true);
    expect(isRecordBytesHexNo0xProperty({ foo: '' }, 'foo')).toBe(true);
  });

  it('returns false for invalid values', () => {
    expect(isRecordBytesHexNo0xProperty({ foo: '0xdeadbeef' }, 'foo')).toBe(false);
    expect(isRecordBytesHexNo0xProperty({ foo: 'deadbee' }, 'foo')).toBe(false);
    expect(isRecordBytesHexNo0xProperty({ foo: null }, 'foo')).toBe(false);
    expect(isRecordBytesHexNo0xProperty({}, 'foo')).toBe(false);
    expect(isRecordBytesHexNo0xProperty(null, 'foo')).toBe(false);
  });
});

describe('assertRecordBytesHexNo0xProperty', () => {
  it('does not throw for valid BytesHexNo0x property', () => {
    expect(() => assertRecordBytesHexNo0xProperty({ foo: 'deadbeef' }, 'foo', 'Obj', {})).not.toThrow();
    expect(() => assertRecordBytesHexNo0xProperty({ foo: '' }, 'foo', 'Obj', {})).not.toThrow();
  });

  it('throws for invalid values', () => {
    expect(() => assertRecordBytesHexNo0xProperty({ foo: '0xdeadbeef' }, 'foo', 'Obj', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Obj',
          property: 'foo',
          expectedType: 'bytesHexNo0x',
          type: 'string',
        },
        {},
      ),
    );
    expect(() => assertRecordBytesHexNo0xProperty({}, 'foo', 'Obj', {})).toThrow(InvalidPropertyError);
  });
});

describe('assertRecordBytes65HexArrayProperty', () => {
  const valid65Hex = '0x' + 'de'.repeat(65);

  it('does not throw for valid array property', () => {
    expect(() => assertRecordBytes65HexArrayProperty({ foo: [] }, 'foo', 'Obj', {})).not.toThrow();
    expect(() => assertRecordBytes65HexArrayProperty({ foo: [valid65Hex] }, 'foo', 'Obj', {})).not.toThrow();
  });

  it('throws for invalid values', () => {
    expect(() => assertRecordBytes65HexArrayProperty({ foo: null }, 'foo', 'Obj', {})).toThrow(InvalidPropertyError);
    expect(() => assertRecordBytes65HexArrayProperty({ foo: ['0xdeadbeef'] }, 'foo', 'Obj', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Obj',
          property: 'foo[0]',
          expectedType: 'bytes65Hex',
          type: 'string',
        },
        {},
      ),
    );
  });
});

describe('assertRecordBytesHexNo0xArrayProperty', () => {
  it('does not throw for valid array property', () => {
    expect(() => assertRecordBytesHexNo0xArrayProperty({ foo: [] }, 'foo', 'Obj', {})).not.toThrow();
    expect(() => assertRecordBytesHexNo0xArrayProperty({ foo: ['deadbeef', 'abcd'] }, 'foo', 'Obj', {})).not.toThrow();
  });

  it('throws for invalid values', () => {
    expect(() => assertRecordBytesHexNo0xArrayProperty({ foo: null }, 'foo', 'Obj', {})).toThrow(InvalidPropertyError);
    expect(() => assertRecordBytesHexNo0xArrayProperty({ foo: ['0xdeadbeef'] }, 'foo', 'Obj', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Obj',
          property: 'foo[0]',
          expectedType: 'bytesHexNo0x',
          type: 'string',
        },
        {},
      ),
    );
  });
});

describe('isRecordUint8ArrayProperty', () => {
  it('returns true for valid Uint8Array property', () => {
    expect(isRecordUint8ArrayProperty({ foo: new Uint8Array([1, 2, 3]) }, 'foo')).toBe(true);
    expect(isRecordUint8ArrayProperty({ foo: new Uint8Array() }, 'foo')).toBe(true);
  });

  it('returns false for invalid values', () => {
    expect(isRecordUint8ArrayProperty({ foo: [1, 2, 3] }, 'foo')).toBe(false);
    expect(isRecordUint8ArrayProperty({ foo: null }, 'foo')).toBe(false);
    expect(isRecordUint8ArrayProperty({}, 'foo')).toBe(false);
    expect(isRecordUint8ArrayProperty(null, 'foo')).toBe(false);
  });
});

describe('assertRecordUint8ArrayProperty', () => {
  it('does not throw for valid Uint8Array property', () => {
    expect(() => assertRecordUint8ArrayProperty({ foo: new Uint8Array([1, 2, 3]) }, 'foo', 'Obj', {})).not.toThrow();
  });

  it('throws for invalid values', () => {
    expect(() => assertRecordUint8ArrayProperty({ foo: [1, 2, 3] }, 'foo', 'Obj', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Obj',
          property: 'foo',
          expectedType: 'Uint8Array',
          type: 'object',
        },
        {},
      ),
    );
    expect(() => assertRecordUint8ArrayProperty({}, 'foo', 'Obj', {})).toThrow(InvalidPropertyError);
  });
});

describe('bytesToHexNo0x', () => {
  it('converts bytes to hex without 0x prefix', () => {
    expect(bytesToHexNo0x(new Uint8Array([255]))).toBe('ff');
    expect(bytesToHexNo0x(new Uint8Array([0xde, 0xad, 0xbe, 0xef]))).toBe('deadbeef');
    expect(bytesToHexNo0x(new Uint8Array([0, 1, 255]))).toBe('0001ff');
  });

  it('handles empty and undefined', () => {
    expect(bytesToHexNo0x(new Uint8Array())).toBe('');
    expect(bytesToHexNo0x(undefined)).toBe('');
  });
});

describe('hexToBytes32', () => {
  it('pads hex to 32 bytes', () => {
    const result = hexToBytes32('0x1234');
    expect(result.length).toBe(32);
    expect(result[30]).toBe(0x12);
    expect(result[31]).toBe(0x34);
    // Leading bytes should be 0
    for (let i = 0; i < 30; i++) {
      expect(result[i]).toBe(0);
    }
  });

  it('handles full 32-byte hex', () => {
    const hex = '0x' + 'ab'.repeat(32);
    const result = hexToBytes32(hex);
    expect(result.length).toBe(32);
    for (let i = 0; i < 32; i++) {
      expect(result[i]).toBe(0xab);
    }
  });

  it('handles hex without 0x prefix', () => {
    const result = hexToBytes32('ff');
    expect(result.length).toBe(32);
    expect(result[31]).toBe(0xff);
  });
});

describe('concatBytes', () => {
  it('concatenates multiple Uint8Arrays', () => {
    const a = new Uint8Array([1, 2]);
    const b = new Uint8Array([3, 4, 5]);
    const c = new Uint8Array([6]);
    const result = concatBytes(a, b, c);
    expect(result).toEqual(new Uint8Array([1, 2, 3, 4, 5, 6]));
  });

  it('handles empty arrays', () => {
    const a = new Uint8Array([1, 2]);
    const b = new Uint8Array([]);
    const c = new Uint8Array([3]);
    expect(concatBytes(a, b, c)).toEqual(new Uint8Array([1, 2, 3]));
    expect(concatBytes()).toEqual(new Uint8Array([]));
  });

  it('handles single array', () => {
    const a = new Uint8Array([1, 2, 3]);
    expect(concatBytes(a)).toEqual(new Uint8Array([1, 2, 3]));
  });
});

describe('unsafeBytesEquals', () => {
  it('returns true for equal byte arrays', () => {
    expect(unsafeBytesEquals(new Uint8Array([1, 2, 3]), new Uint8Array([1, 2, 3]))).toBe(true);
    expect(unsafeBytesEquals(new Uint8Array([]), new Uint8Array([]))).toBe(true);
    expect(unsafeBytesEquals(new Uint8Array([0xff]), new Uint8Array([0xff]))).toBe(true);
  });

  it('returns false for different byte arrays', () => {
    expect(unsafeBytesEquals(new Uint8Array([1, 2, 3]), new Uint8Array([1, 2, 4]))).toBe(false);
    expect(unsafeBytesEquals(new Uint8Array([1, 2, 3]), new Uint8Array([1, 2]))).toBe(false);
    expect(unsafeBytesEquals(new Uint8Array([1]), new Uint8Array([1, 2]))).toBe(false);
  });

  it('returns false for non-Uint8Array values', () => {
    expect(unsafeBytesEquals(null as unknown as Uint8Array, new Uint8Array([]))).toBe(false);
    expect(unsafeBytesEquals(new Uint8Array([]), null as unknown as Uint8Array)).toBe(false);
    expect(unsafeBytesEquals([1, 2, 3] as unknown as Uint8Array, new Uint8Array([1, 2, 3]))).toBe(false);
  });
});

describe('toBytes32HexArray', () => {
  const validBytes32Hex = '0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef';
  const validBytes32Hex2 = '0x1234567812345678123456781234567812345678123456781234567812345678';

  it('converts array of Bytes32Hex strings', () => {
    const result = toBytes32HexArray([validBytes32Hex, validBytes32Hex2]);
    expect(result).toEqual([validBytes32Hex, validBytes32Hex2]);
  });

  it('converts array of Bytes32 Uint8Arrays', () => {
    const bytes32 = new Uint8Array(32).fill(0xde);
    const result = toBytes32HexArray([bytes32]);
    expect(result.length).toBe(1);
    expect(result[0]).toBe('0x' + 'de'.repeat(32));
  });

  it('converts mixed array of Bytes32 and Bytes32Hex', () => {
    const bytes32 = new Uint8Array(32).fill(0xab);
    const result = toBytes32HexArray([validBytes32Hex, bytes32]);
    expect(result.length).toBe(2);
    expect(result[0]).toBe(validBytes32Hex);
    expect(result[1]).toBe('0x' + 'ab'.repeat(32));
  });

  it('returns empty array for empty input', () => {
    expect(toBytes32HexArray([])).toEqual([]);
  });

  it('throws for non-array input', () => {
    expect(() => toBytes32HexArray('string' as any)).toThrow();
    expect(() => toBytes32HexArray(123 as any)).toThrow();
  });

  it('throws for invalid Bytes32Hex string', () => {
    expect(() => toBytes32HexArray(['0xdeadbeef'])).toThrow(InvalidTypeError);
    expect(() => toBytes32HexArray(['not-hex' as any])).toThrow(InvalidTypeError);
    expect(() => toBytes32HexArray([validBytes32Hex, '0xinvalid'])).toThrow(InvalidTypeError);
  });

  it('throws for invalid Bytes32 Uint8Array (wrong length)', () => {
    const bytes31 = new Uint8Array(31).fill(0xde);
    const bytes33 = new Uint8Array(33).fill(0xde);
    expect(() => toBytes32HexArray([bytes31])).toThrow(InvalidTypeError);
    expect(() => toBytes32HexArray([bytes33])).toThrow(InvalidTypeError);
  });

  it('throws for invalid element types', () => {
    expect(() => toBytes32HexArray([123 as any])).toThrow(InvalidTypeError);
    expect(() => toBytes32HexArray([null as any])).toThrow(InvalidTypeError);
    expect(() => toBytes32HexArray([{} as any])).toThrow(InvalidTypeError);
  });
});

////////////////////////////////////////////////////////////////////////////////

describe('bigIntToBytesHex', () => {
  it('converts 0n', () => {
    expect(bigIntToBytesHex(0n)).toBe('0x00');
  });

  it('converts single-byte values', () => {
    expect(bigIntToBytesHex(1n)).toBe('0x01');
    expect(bigIntToBytesHex(255n)).toBe('0xff');
  });

  it('converts multi-byte values', () => {
    expect(bigIntToBytesHex(256n)).toBe('0x0100');
    expect(bigIntToBytesHex(0xdeadbeefn)).toBe('0xdeadbeef');
  });

  it('pads to byteLength', () => {
    expect(bigIntToBytesHex(0n, { byteLength: 4 })).toBe('0x00000000');
    expect(bigIntToBytesHex(1n, { byteLength: 32 })).toBe('0x' + '00'.repeat(31) + '01');
    expect(bigIntToBytesHex(255n, { byteLength: 1 })).toBe('0xff');
  });

  it('round-trips with bytesToBigInt', () => {
    for (const v of [0n, 1n, 255n, 256n, 0xdeadbeefn, MAX_UINT256]) {
      expect(bytesToBigInt(hexToBytes(bigIntToBytesHex(v)))).toBe(v);
    }
  });

  it('throws InvalidTypeError for negative values', () => {
    expect(() => bigIntToBytesHex(-1n)).toThrow(InvalidTypeError);
    expect(() => bigIntToBytesHex(-1n, { byteLength: 32 })).toThrow(InvalidTypeError);
  });

  it('throws InvalidTypeError when value exceeds byteLength', () => {
    expect(() => bigIntToBytesHex(256n, { byteLength: 1 })).toThrow(InvalidTypeError);
    expect(() => bigIntToBytesHex(BigInt(2 ** 32), { byteLength: 4 })).toThrow(InvalidTypeError);
  });
});

////////////////////////////////////////////////////////////////////////////////

describe('toBytes32', () => {
  const valid32Hex = ('0x' + 'de'.repeat(32)) as BytesHex;
  const valid32Bytes = new Uint8Array(32).fill(0xde);

  it('accepts a 32-byte Uint8Array', () => {
    expect(toBytes32(valid32Bytes)).toEqual(valid32Bytes);
  });

  it('accepts a Bytes32Hex string', () => {
    expect(toBytes32(valid32Hex)).toEqual(valid32Bytes);
  });

  it('accepts a Bytes32Able object with .bytes32 property', () => {
    expect(toBytes32({ bytes32: valid32Bytes })).toEqual(valid32Bytes);
  });

  it('throws for wrong-length Uint8Array', () => {
    expect(() => toBytes32(new Uint8Array(31))).toThrow();
    expect(() => toBytes32(new Uint8Array(33))).toThrow();
  });

  it('throws for wrong-length hex string', () => {
    expect(() => toBytes32('0xdeadbeef')).toThrow();
  });

  it('throws InvalidTypeError for unrecognized types', () => {
    expect(() => toBytes32(123)).toThrow(InvalidTypeError);
    expect(() => toBytes32(null)).toThrow(InvalidTypeError);
    expect(() => toBytes32({})).toThrow(InvalidTypeError);
  });
});

////////////////////////////////////////////////////////////////////////////////

describe('toBytes', () => {
  it('returns the same Uint8Array reference without copy', () => {
    const arr = new Uint8Array([1, 2, 3]);
    expect(toBytes(arr)).toBe(arr);
  });

  it('returns a new copy when copy: true', () => {
    const arr = new Uint8Array([1, 2, 3]);
    const result = toBytes(arr, { copy: true });
    expect(result).toEqual(arr);
    expect(result).not.toBe(arr);
  });

  it('parses a 0x-prefixed hex string', () => {
    expect(toBytes('0xdeadbeef')).toEqual(new Uint8Array([0xde, 0xad, 0xbe, 0xef]));
    expect(toBytes('0x')).toEqual(new Uint8Array([]));
  });

  it('parses a hex string without 0x prefix', () => {
    expect(toBytes('deadbeef')).toEqual(new Uint8Array([0xde, 0xad, 0xbe, 0xef]));
  });

  it('throws for hex string with invalid characters (strict mode)', () => {
    expect(() => toBytes('0xzzzzzzzz')).toThrow();
  });

  it('throws InvalidTypeError for non-bytes non-string values', () => {
    expect(() => toBytes(123)).toThrow(InvalidTypeError);
    expect(() => toBytes(null)).toThrow(InvalidTypeError);
    expect(() => toBytes({})).toThrow(InvalidTypeError);
  });
});

////////////////////////////////////////////////////////////////////////////////

describe('concatBytesHex', () => {
  it('concatenates two hex strings', () => {
    expect(concatBytesHex(['0xdead', '0xbeef'] as unknown as BytesHex[])).toBe('0xdeadbeef');
  });

  it('handles empty array', () => {
    expect(concatBytesHex([])).toBe('0x');
  });

  it('handles 0x-only value', () => {
    expect(concatBytesHex(['0x', '0xab'] as unknown as BytesHex[])).toBe('0xab');
  });

  it('handles single value', () => {
    expect(concatBytesHex(['0xdeadbeef'] as unknown as BytesHex[])).toBe('0xdeadbeef');
  });

  it('concatenates three values', () => {
    expect(concatBytesHex(['0xde', '0xad', '0xbeef'] as unknown as BytesHex[])).toBe('0xdeadbeef');
  });
});

////////////////////////////////////////////////////////////////////////////////

describe('normalizeBytes', () => {
  it('returns a same-realm Uint8Array as-is', () => {
    const arr = new Uint8Array([1, 2, 3]);
    expect(normalizeBytes(arr)).toBe(arr);
  });

  it('wraps an ArrayBuffer in a new Uint8Array', () => {
    const buf = new Uint8Array([1, 2, 3]).buffer;
    const result = normalizeBytes(buf);
    expect(result).toBeInstanceOf(Uint8Array);
    expect(result).toEqual(new Uint8Array([1, 2, 3]));
  });

  it('wraps an empty ArrayBuffer', () => {
    expect(normalizeBytes(new ArrayBuffer(0))).toEqual(new Uint8Array([]));
  });

  it('throws TypeError for Uint16Array', () => {
    expect(() => normalizeBytes(new Uint16Array([1, 2]))).toThrow(TypeError);
  });

  it('throws TypeError for string', () => {
    expect(() => normalizeBytes('0xdeadbeef')).toThrow(TypeError);
  });

  it('throws TypeError for number', () => {
    expect(() => normalizeBytes(42)).toThrow(TypeError);
  });

  it('throws TypeError for null', () => {
    expect(() => normalizeBytes(null)).toThrow(TypeError);
  });
});

////////////////////////////////////////////////////////////////////////////////

describe('bytesToHexLarge (no0x path)', () => {
  it('converts bytes to lowercase hex without 0x prefix', () => {
    expect(bytesToHexLarge(new Uint8Array([0xde, 0xad, 0xbe, 0xef]), true)).toBe('deadbeef');
    expect(bytesToHexLarge(new Uint8Array([0, 255]), true)).toBe('00ff');
  });

  it('returns empty string for empty input', () => {
    expect(bytesToHexLarge(new Uint8Array(), true)).toBe('');
  });

  it('round-trips with hexToBytesFaster', () => {
    const input = new Uint8Array([0xde, 0xad, 0xbe, 0xef, 0x00, 0xff]);
    expect(hexToBytesFaster(bytesToHexLarge(input))).toEqual(input);
  });
});

////////////////////////////////////////////////////////////////////////////////

describe('isBytesHex with byteLength', () => {
  it('returns true when byte count matches byteLength', () => {
    expect(isBytesHex('0xdeadbeef', 4)).toBe(true);
    expect(isBytesHex('0x' + 'de'.repeat(32), 32)).toBe(true);
    expect(isBytesHex('0xde', 1)).toBe(true);
  });

  it('returns false when byte count does not match byteLength', () => {
    expect(isBytesHex('0xdeadbeef', 1)).toBe(false);
    expect(isBytesHex('0xdeadbeef', 32)).toBe(false);
    expect(isBytesHex('0x', 1)).toBe(false);
  });
});

////////////////////////////////////////////////////////////////////////////////

describe('assertIsBytesOrBytesHex', () => {
  it('does not throw for Uint8Array', () => {
    expect(() => assertIsBytesOrBytesHex(new Uint8Array([1, 2, 3]), {})).not.toThrow();
    expect(() => assertIsBytesOrBytesHex(new Uint8Array(), {})).not.toThrow();
  });

  it('does not throw for valid BytesHex string', () => {
    expect(() => assertIsBytesOrBytesHex('0xdeadbeef', {})).not.toThrow();
    expect(() => assertIsBytesOrBytesHex('0x', {})).not.toThrow();
  });

  it('throws InvalidTypeError for hex string without 0x prefix', () => {
    expect(() => assertIsBytesOrBytesHex('deadbeef', {})).toThrow(
      new InvalidTypeError({ type: 'string', expectedType: 'Bytes | BytesHex' }, {}),
    );
  });

  it('throws InvalidTypeError for number', () => {
    expect(() => assertIsBytesOrBytesHex(123, {})).toThrow(
      new InvalidTypeError({ type: 'number', expectedType: 'Bytes | BytesHex' }, {}),
    );
  });

  it('throws InvalidTypeError for null', () => {
    expect(() => assertIsBytesOrBytesHex(null, {})).toThrow(InvalidTypeError);
  });
});

////////////////////////////////////////////////////////////////////////////////

describe('bytesHexSlice', () => {
  const hex = '0xdeadbeef' as BytesHex;

  it('extracts 1 byte at position 0', () => {
    expect(bytesHexSlice(hex, 0 as any, 1)).toBe('0xde');
  });

  it('extracts 1 byte at an interior position', () => {
    expect(bytesHexSlice(hex, 1 as any, 1)).toBe('0xad');
    expect(bytesHexSlice(hex, 3 as any, 1)).toBe('0xef');
  });

  it('extracts multiple bytes', () => {
    expect(bytesHexSlice(hex, 2 as any, 2 as any)).toBe('0xbeef');
    expect(bytesHexSlice(hex, 0 as any, 4 as any)).toBe('0xdeadbeef');
  });

  it('extracts from a longer hex string at an offset', () => {
    const long = ('0x' + '00'.repeat(4) + 'deadbeef') as BytesHex; // 8 bytes
    expect(bytesHexSlice(long, 4 as any, 4 as any)).toBe('0xdeadbeef');
  });

  it('throws RangeError when position + length exceeds bounds', () => {
    expect(() => bytesHexSlice(hex, 5 as any, 1)).toThrow(RangeError);
    expect(() => bytesHexSlice(hex, 0 as any, 5 as any)).toThrow(RangeError);
    expect(() => bytesHexSlice('0x' as BytesHex, 0 as any, 1)).toThrow(RangeError);
  });
});

////////////////////////////////////////////////////////////////////////////////

describe('bytesUint8At', () => {
  const bytes = new Uint8Array([0xde, 0xad, 0xbe, 0xef]) as any;

  it('reads the byte at each position', () => {
    expect(bytesUint8At(bytes, 0 as any)).toBe(0xde);
    expect(bytesUint8At(bytes, 1 as any)).toBe(0xad);
    expect(bytesUint8At(bytes, 2 as any)).toBe(0xbe);
    expect(bytesUint8At(bytes, 3 as any)).toBe(0xef);
  });

  it('throws RangeError for out-of-bounds position', () => {
    expect(() => bytesUint8At(bytes, 4 as any)).toThrow(RangeError);
    expect(() => bytesUint8At(bytes, 100 as any)).toThrow(RangeError);
  });

  it('throws RangeError for any position on an empty array', () => {
    expect(() => bytesUint8At(new Uint8Array() as any, 0 as any)).toThrow(RangeError);
  });
});

////////////////////////////////////////////////////////////////////////////////

describe('bytesHexUint8At', () => {
  const hex = '0xdeadbeef' as BytesHex;

  it('reads the byte at each position', () => {
    expect(bytesHexUint8At(hex, 0 as any)).toBe(0xde);
    expect(bytesHexUint8At(hex, 1 as any)).toBe(0xad);
    expect(bytesHexUint8At(hex, 3 as any)).toBe(0xef);
  });

  it('throws RangeError when position is out of bounds', () => {
    expect(() => bytesHexUint8At('0xde' as BytesHex, 1 as any)).toThrow(RangeError);
  });
});

////////////////////////////////////////////////////////////////////////////////

describe('bytesHexUint64At', () => {
  it('reads all-zero 8 bytes as 0n', () => {
    expect(bytesHexUint64At(('0x' + '00'.repeat(8)) as BytesHex, 0 as any)).toBe(0n);
  });

  it('reads 8 bytes with last byte = 0xff as 255n', () => {
    const hex = ('0x' + '00'.repeat(7) + 'ff') as BytesHex;
    expect(bytesHexUint64At(hex, 0 as any)).toBe(255n);
  });

  it('reads from an interior position', () => {
    // 16-byte value: first 8 bytes zero, next 8 bytes = 0x00...ff
    const hex = ('0x' + '00'.repeat(8) + '00'.repeat(7) + 'ff') as BytesHex;
    expect(bytesHexUint64At(hex, 8 as any)).toBe(255n);
  });

  it('throws RangeError when fewer than 8 bytes remain', () => {
    expect(() => bytesHexUint64At('0xde' as BytesHex, 0 as any)).toThrow(RangeError);
    expect(() => bytesHexUint64At(('0x' + '00'.repeat(8)) as BytesHex, 1 as any)).toThrow(RangeError);
  });
});

////////////////////////////////////////////////////////////////////////////////

describe('createDeadbeefBytes', () => {
  it('returns an array of the requested length', () => {
    expect(createDeadbeefBytes(0)).toHaveLength(0);
    expect(createDeadbeefBytes(1)).toHaveLength(1);
    expect(createDeadbeefBytes(4)).toHaveLength(4);
    expect(createDeadbeefBytes(7)).toHaveLength(7);
    expect(createDeadbeefBytes(32)).toHaveLength(32);
  });

  it('fills with the 0xdeadbeef byte pattern', () => {
    const result = createDeadbeefBytes(4);
    expect(Array.from(result)).toEqual([0xde, 0xad, 0xbe, 0xef]);
  });

  it('repeats the pattern for lengths beyond 4 bytes', () => {
    const result = createDeadbeefBytes(8);
    expect(Array.from(result)).toEqual([0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef]);
  });

  it('returns a Uint8Array instance', () => {
    expect(createDeadbeefBytes(4)).toBeInstanceOf(Uint8Array);
  });
});
