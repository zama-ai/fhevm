import type { ChecksummedAddress } from '../types/primitives.js';
import { describe, it, expect } from 'vitest';
import { AddressError } from './errors/AddressError.js';
import { ChecksummedAddressError } from './errors/ChecksummedAddressError.js';
import { InvalidPropertyError } from './errors/InvalidPropertyError.js';
import {
  assertIsAddressArray,
  assertRecordAddressProperty,
  assertRecordChecksummedAddressArrayProperty,
  assertRecordChecksummedAddressProperty,
  assertIsAddress,
  assertIsChecksummedAddress,
  assertIsChecksummedAddressArray,
  checksummedAddressToBytes20,
  isAddress,
  isChecksummedAddress,
  isRecordAddressProperty,
  isRecordChecksummedAddressProperty,
  ZERO_ADDRESS,
} from './address.js';
import { InvalidTypeError } from './errors/InvalidTypeError.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/base/address.test.ts
////////////////////////////////////////////////////////////////////////////////

describe('address', () => {
  it('isAddress', () => {
    // True
    expect(isAddress('0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef')).toEqual(true);

    // False
    expect(isAddress('deadbeefdeadbeefdeadbeefdeadbeefdeadbeef')).toEqual(false);
    expect(isAddress('0x')).toEqual(false);
    expect(isAddress('0xdeadbeef')).toEqual(false);
    expect(isAddress('deadbee')).toEqual(false);
    expect(isAddress('0x0')).toEqual(false);
    expect(isAddress('0xhello')).toEqual(false);
    expect(isAddress('0xdeadbee')).toEqual(false);
    expect(isAddress('0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbezz')).toEqual(false);
    expect(isAddress('0xdeadbeefzz')).toEqual(false);
    expect(isAddress('hello')).toEqual(false);
    expect(isAddress(null)).toEqual(false);
    expect(isAddress(undefined)).toEqual(false);
    expect(isAddress('')).toEqual(false);
    expect(isAddress('123')).toEqual(false);
    expect(isAddress(123)).toEqual(false);
    expect(isAddress(BigInt(123))).toEqual(false);
    expect(isAddress(123.0)).toEqual(false);
    expect(isAddress(123.1)).toEqual(false);
    expect(isAddress(-123)).toEqual(false);
    expect(isAddress(0)).toEqual(false);
    expect(isAddress({})).toEqual(false);
    expect(isAddress([])).toEqual(false);
    expect(isAddress([123])).toEqual(false);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('isChecksummedAddress', () => {
    // True
    expect(isChecksummedAddress('0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF')).toEqual(true);

    // False
    expect(isChecksummedAddress('0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef')).toEqual(false);

    expect(isChecksummedAddress('DeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF')).toEqual(false);

    expect(isChecksummedAddress('0xDeaDbeefdEAdbeefdEadbEEFdeadbeEF')).toEqual(false);
    expect(isChecksummedAddress('0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbezz')).toEqual(false);

    expect(isChecksummedAddress([123])).toEqual(false);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertIsAddress', () => {
    // True
    expect(() => assertIsAddress('0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef', {})).not.toThrow();

    // False
    expect(() => assertIsAddress('0x', {})).toThrow(new AddressError({ address: '0x' }, {}));

    expect(() => assertIsAddress('deadbeefdeadbeefdeadbeefdeadbeefdeadbeef', {})).toThrow(
      new AddressError({ address: 'deadbeefdeadbeefdeadbeefdeadbeefdeadbeef' }, {}),
    );
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertIsChecksummedAddress', () => {
    // True
    expect(() => assertIsChecksummedAddress('0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF', {})).not.toThrow();

    // False
    expect(() => assertIsChecksummedAddress('0x', {})).toThrow(new ChecksummedAddressError({ address: '0x' }, {}));

    expect(() => assertIsChecksummedAddress('0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef', {})).toThrow(
      new ChecksummedAddressError(
        {
          address: '0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef',
        },
        {},
      ),
    );

    expect(() => assertIsChecksummedAddress('DeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF', {})).toThrow(
      new ChecksummedAddressError(
        {
          address: 'DeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF',
        },
        {},
      ),
    );

    expect(() => assertIsChecksummedAddress('0xDeaDbeefdEAdbeefdEadbEEFdeadbeEF', {})).toThrow(
      new ChecksummedAddressError(
        {
          address: '0xDeaDbeefdEAdbeefdEadbEEFdeadbeEF',
        },
        {},
      ),
    );
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertChecksummedAddressProperty', () => {
    // True
    expect(() =>
      assertRecordChecksummedAddressProperty({ foo: '0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF' }, 'foo', 'Foo', {}),
    ).not.toThrow();

    // False
    expect(() => assertRecordChecksummedAddressProperty({ foo: null }, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'checksummedAddress',
          type: 'undefined',
          property: 'foo',
        },
        {},
      ),
    );

    expect(() => assertRecordChecksummedAddressProperty({ foo: undefined }, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'checksummedAddress',
          type: 'undefined',
          property: 'foo',
        },
        {},
      ),
    );

    expect(() =>
      assertRecordChecksummedAddressProperty({ foo: 'DeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF' }, 'foo', 'Foo', {}),
    ).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'checksummedAddress',
          type: 'string',
          property: 'foo',
          value: 'DeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF',
        },
        {},
      ),
    );

    expect(() => assertRecordChecksummedAddressProperty({}, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          expectedType: 'checksummedAddress',
          property: 'foo',
          type: 'undefined',
        },
        {},
      ),
    );
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertChecksummedAddressArrayProperty', () => {
    // True
    expect(() =>
      assertRecordChecksummedAddressArrayProperty(
        { foo: ['0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF'] },
        'foo',
        'Foo',
        {},
      ),
    ).not.toThrow();

    expect(() => assertRecordChecksummedAddressArrayProperty({ foo: [] }, 'foo', 'Foo', {})).not.toThrow();

    const e = (expectedType: string, type?: string) => {
      return new InvalidPropertyError(
        {
          subject: 'Foo',
          property: 'foo',
          expectedType,
          type,
        },
        {},
      );
    };

    // False
    expect(() => assertRecordChecksummedAddressArrayProperty({ foo: null }, 'foo', 'Foo', {})).toThrow(
      e('Array', 'undefined'),
    );

    expect(() => assertRecordChecksummedAddressArrayProperty({ foo: undefined }, 'foo', 'Foo', {})).toThrow(
      e('Array', 'undefined'),
    );

    expect(() =>
      assertRecordChecksummedAddressArrayProperty(
        { foo: '0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF' },
        'foo',
        'Foo',
        {},
      ),
    ).toThrow(e('Array', 'string'));

    expect(() =>
      assertRecordChecksummedAddressArrayProperty({ foo: ['0xDeaDbeefdEAdbeef'] }, 'foo', 'Foo', {}),
    ).toThrow(new ChecksummedAddressError({ address: '0xDeaDbeefdEAdbeef' }, {}));
  });

  //////////////////////////////////////////////////////////////////////////////

  it('checksummedAddressToBytes20', () => {
    // Valid checksummed address - zero address
    const zeroAddress = ZERO_ADDRESS;
    const zeroBytes = checksummedAddressToBytes20(zeroAddress);
    expect(zeroBytes).toBeInstanceOf(Uint8Array);
    expect(zeroBytes.length).toBe(20);
    expect(zeroBytes.every((b) => b === 0)).toBe(true);

    // Valid checksummed address - all 0xff
    const maxAddress = '0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF' as ChecksummedAddress;
    const maxBytes = checksummedAddressToBytes20(maxAddress);
    expect(maxBytes.length).toBe(20);
    expect(maxBytes.every((b) => b === 0xff)).toBe(true);

    // Valid checksummed address - specific pattern
    const deadbeefAddress = '0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF' as ChecksummedAddress;
    const deadbeefBytes = checksummedAddressToBytes20(deadbeefAddress);
    expect(deadbeefBytes.length).toBe(20);
    expect(deadbeefBytes[0]).toBe(0xde);
    expect(deadbeefBytes[1]).toBe(0xad);
    expect(deadbeefBytes[2]).toBe(0xbe);
    expect(deadbeefBytes[3]).toBe(0xef);

    // Valid checksummed address - verify full byte array
    const testAddress = '0x1234567890AbcdEF1234567890aBcdef12345678' as ChecksummedAddress;
    const testBytes = checksummedAddressToBytes20(testAddress);
    expect(Array.from(testBytes)).toEqual([
      0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef, 0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef, 0x12, 0x34, 0x56,
      0x78,
    ]);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertIsChecksummedAddressArray', () => {
    const validAddress = '0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF';

    // Valid arrays
    expect(() => assertIsChecksummedAddressArray([], {})).not.toThrow();
    expect(() => assertIsChecksummedAddressArray([validAddress], {})).not.toThrow();
    expect(() => assertIsChecksummedAddressArray([validAddress, ZERO_ADDRESS], {})).not.toThrow();

    // Invalid: not an array
    expect(() => assertIsChecksummedAddressArray(null, {})).toThrow(
      new InvalidTypeError(
        {
          type: 'object',
          expectedType: 'checksummedAddress[]',
        },
        {},
      ),
    );
    expect(() => assertIsChecksummedAddressArray(undefined, {})).toThrow(
      new InvalidTypeError(
        {
          type: 'undefined',
          expectedType: 'checksummedAddress[]',
        },
        {},
      ),
    );
    expect(() => assertIsChecksummedAddressArray('string', {})).toThrow(
      new InvalidTypeError(
        {
          type: 'string',
          expectedType: 'checksummedAddress[]',
        },
        {},
      ),
    );
    expect(() => assertIsChecksummedAddressArray(validAddress, {})).toThrow(InvalidTypeError);

    // Invalid: array with invalid elements
    expect(() => assertIsChecksummedAddressArray(['0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef'], {})).toThrow(
      ChecksummedAddressError,
    );

    expect(() => assertIsChecksummedAddressArray([validAddress, '0xinvalid'], {})).toThrow(ChecksummedAddressError);

    expect(() => assertIsChecksummedAddressArray(['not-an-address'], {})).toThrow(ChecksummedAddressError);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('isRecordChecksummedAddressProperty', () => {
    const validAddress = '0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF';

    // True
    expect(isRecordChecksummedAddressProperty({ foo: validAddress }, 'foo')).toBe(true);
    expect(isRecordChecksummedAddressProperty({ foo: ZERO_ADDRESS }, 'foo')).toBe(true);

    // False - not checksummed
    expect(isRecordChecksummedAddressProperty({ foo: '0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef' }, 'foo')).toBe(
      false,
    );

    // False - missing property
    expect(isRecordChecksummedAddressProperty({}, 'foo')).toBe(false);

    // False - null/undefined
    expect(isRecordChecksummedAddressProperty({ foo: null }, 'foo')).toBe(false);
    expect(isRecordChecksummedAddressProperty({ foo: undefined }, 'foo')).toBe(false);

    // False - not an object
    expect(isRecordChecksummedAddressProperty(null, 'foo')).toBe(false);
    expect(isRecordChecksummedAddressProperty(undefined, 'foo')).toBe(false);

    // False - wrong type
    expect(isRecordChecksummedAddressProperty({ foo: 123 }, 'foo')).toBe(false);
    expect(isRecordChecksummedAddressProperty({ foo: [] }, 'foo')).toBe(false);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertIsAddressArray', () => {
    const validAddress = '0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef';

    // Valid arrays
    expect(() => assertIsAddressArray([], {})).not.toThrow();
    expect(() => assertIsAddressArray([validAddress], {})).not.toThrow();
    expect(() => assertIsAddressArray([validAddress, ZERO_ADDRESS], {})).not.toThrow();

    // Invalid: not an array
    expect(() => assertIsAddressArray(null, {})).toThrow(
      new InvalidTypeError({ type: 'object', expectedType: 'address[]' }, {}),
    );
    expect(() => assertIsAddressArray(undefined, {})).toThrow(
      new InvalidTypeError({ type: 'undefined', expectedType: 'address[]' }, {}),
    );
    expect(() => assertIsAddressArray('string', {})).toThrow(
      new InvalidTypeError({ type: 'string', expectedType: 'address[]' }, {}),
    );
    expect(() => assertIsAddressArray(validAddress, {})).toThrow(InvalidTypeError);

    // Invalid: array with invalid elements
    expect(() => assertIsAddressArray(['not-an-address'], {})).toThrow(
      new AddressError({ address: 'not-an-address' }, {}),
    );
    expect(() => assertIsAddressArray([validAddress, 'not-an-address'], {})).toThrow(AddressError);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('isRecordAddressProperty', () => {
    const validAddress = '0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef';

    // True
    expect(isRecordAddressProperty({ foo: validAddress }, 'foo')).toBe(true);
    expect(isRecordAddressProperty({ foo: ZERO_ADDRESS }, 'foo')).toBe(true);

    // False - not a valid address
    expect(isRecordAddressProperty({ foo: 'not-an-address' }, 'foo')).toBe(false);

    // False - missing property
    expect(isRecordAddressProperty({}, 'foo')).toBe(false);

    // False - null/undefined
    expect(isRecordAddressProperty({ foo: null }, 'foo')).toBe(false);
    expect(isRecordAddressProperty({ foo: undefined }, 'foo')).toBe(false);

    // False - not an object
    expect(isRecordAddressProperty(null, 'foo')).toBe(false);
    expect(isRecordAddressProperty(undefined, 'foo')).toBe(false);

    // False - wrong type
    expect(isRecordAddressProperty({ foo: 123 }, 'foo')).toBe(false);
    expect(isRecordAddressProperty({ foo: [] }, 'foo')).toBe(false);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertRecordAddressProperty', () => {
    const validAddress = '0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef';

    // True
    expect(() => assertRecordAddressProperty({ foo: validAddress }, 'foo', 'Foo', {})).not.toThrow();
    expect(() => assertRecordAddressProperty({ foo: ZERO_ADDRESS }, 'foo', 'Foo', {})).not.toThrow();

    // False - null/undefined property
    expect(() => assertRecordAddressProperty({ foo: null }, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError({ subject: 'Foo', property: 'foo', type: 'undefined', expectedType: 'address' }, {}),
    );
    expect(() => assertRecordAddressProperty({ foo: undefined }, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError({ subject: 'Foo', property: 'foo', type: 'undefined', expectedType: 'address' }, {}),
    );
    expect(() => assertRecordAddressProperty({}, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError({ subject: 'Foo', property: 'foo', type: 'undefined', expectedType: 'address' }, {}),
    );

    // False - string that is not a valid address
    expect(() => assertRecordAddressProperty({ foo: 'not-an-address' }, 'foo', 'Foo', {})).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          property: 'foo',
          type: 'string',
          expectedType: 'address',
          value: 'not-an-address',
        },
        {},
      ),
    );
  });
});
