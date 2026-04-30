import { describe, it, expect } from 'vitest';
import {
  ensure0x,
  removeSuffix,
  is0x,
  isNo0x,
  remove0x,
  assertIs0xString,
  isRecordStringProperty,
  assertRecordStringProperty,
  assertRecordStringArrayProperty,
  safeJSONstringify,
  isNonEmptyString,
} from './string.js';
import { InternalError } from './errors/InternalError.js';
import { InvalidPropertyError } from './errors/InvalidPropertyError.js';

////////////////////////////////////////////////////////////////////////////////
//
// Jest Command line
// =================
//
// npx jest --colors --passWithNoTests ./src/base/string.test.ts
// npx jest --colors --passWithNoTests --coverage ./src/base/string.test.ts --collectCoverageFrom=./src/base/string.ts
//
////////////////////////////////////////////////////////////////////////////////

describe('string', () => {
  //////////////////////////////////////////////////////////////////////////////

  it('ensure0x', () => {
    expect(ensure0x('hello')).toEqual('0xhello');
    expect(ensure0x('0xhello')).toEqual('0xhello');
  });

  //////////////////////////////////////////////////////////////////////////////

  it('removeSuffix', () => {
    expect(removeSuffix('hello/', '/')).toEqual('hello');
    expect(removeSuffix('hello/', 'o/')).toEqual('hell');
    expect(removeSuffix('hello/', '')).toEqual('hello/');
    expect(removeSuffix('hello/', 'o')).toEqual('hello/');
    expect(removeSuffix(undefined, '/')).toEqual('');
    expect(removeSuffix('', '/')).toEqual('');
  });

  //////////////////////////////////////////////////////////////////////////////

  it('is0x', () => {
    expect(is0x('0xhello')).toBe(true);
    expect(is0x('0x')).toBe(true);
    expect(is0x('0x1234')).toBe(true);

    expect(is0x('hello')).toBe(false);
    expect(is0x('')).toBe(false);
    expect(is0x(123)).toBe(false);
    expect(is0x(null)).toBe(false);
    expect(is0x(undefined)).toBe(false);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('isNo0x', () => {
    expect(isNo0x('hello')).toBe(true);
    expect(isNo0x('')).toBe(true);
    expect(isNo0x('1234')).toBe(true);

    expect(isNo0x('0xhello')).toBe(false);
    expect(isNo0x('0x')).toBe(false);
    expect(isNo0x(123)).toBe(false);
    expect(isNo0x(null)).toBe(false);
    expect(isNo0x(undefined)).toBe(false);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('remove0x', () => {
    expect(remove0x('0xhello')).toEqual('hello');
    expect(remove0x('0x')).toEqual('');
    expect(remove0x('hello')).toEqual('hello');
    expect(remove0x('')).toEqual('');
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertIs0xString', () => {
    expect(() => assertIs0xString('0xhello')).not.toThrow();
    expect(() => assertIs0xString('0x')).not.toThrow();

    expect(() => assertIs0xString('hello')).toThrow(InternalError);
    expect(() => assertIs0xString('')).toThrow(InternalError);
    expect(() => assertIs0xString(123)).toThrow(InternalError);
    expect(() => assertIs0xString(null)).toThrow(InternalError);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('isRecordStringProperty', () => {
    expect(isRecordStringProperty({ foo: 'bar' }, 'foo')).toBe(true);
    expect(isRecordStringProperty({ foo: '' }, 'foo')).toBe(true);

    expect(isRecordStringProperty({ foo: 123 }, 'foo')).toBe(false);
    expect(isRecordStringProperty({ foo: null }, 'foo')).toBe(false);
    expect(isRecordStringProperty({ foo: undefined }, 'foo')).toBe(false);
    expect(isRecordStringProperty({}, 'foo')).toBe(false);
    expect(isRecordStringProperty(null, 'foo')).toBe(false);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertRecordStringProperty', () => {
    expect(() => assertRecordStringProperty({ foo: 'bar' }, 'foo', 'Obj', {})).not.toThrow();
    expect(() => assertRecordStringProperty({ foo: '' }, 'foo', 'Obj', {})).not.toThrow();

    expect(() => assertRecordStringProperty({ foo: 123 }, 'foo', 'Obj', {})).toThrow(InvalidPropertyError);
    expect(() => assertRecordStringProperty({}, 'foo', 'Obj', {})).toThrow(InvalidPropertyError);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertRecordStringProperty with expectedValue (string)', () => {
    expect(() => assertRecordStringProperty({ foo: 'bar' }, 'foo', 'Obj', { expectedValue: 'bar' })).not.toThrow();

    expect(() => assertRecordStringProperty({ foo: 'bar' }, 'foo', 'Obj', { expectedValue: 'baz' })).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Obj',
          property: 'foo',
          expectedType: 'string',
          expectedValue: 'baz',
          type: 'string',
          value: 'bar',
        },
        {},
      ),
    );
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertRecordStringProperty with expectedValue (array)', () => {
    expect(() =>
      assertRecordStringProperty({ foo: 'bar' }, 'foo', 'Obj', { expectedValue: ['bar', 'baz'] }),
    ).not.toThrow();
    expect(() =>
      assertRecordStringProperty({ foo: 'baz' }, 'foo', 'Obj', { expectedValue: ['bar', 'baz'] }),
    ).not.toThrow();

    expect(() => assertRecordStringProperty({ foo: 'qux' }, 'foo', 'Obj', { expectedValue: ['bar', 'baz'] })).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Obj',
          property: 'foo',
          expectedType: 'string',
          expectedValue: ['bar', 'baz'],
          type: 'string',
          value: 'qux',
        },
        {},
      ),
    );
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertRecordStringArrayProperty', () => {
    expect(() => assertRecordStringArrayProperty({ foo: [] }, 'foo', 'Obj', {})).not.toThrow();
    expect(() => assertRecordStringArrayProperty({ foo: ['a', 'b', 'c'] }, 'foo', 'Obj', {})).not.toThrow();

    expect(() => assertRecordStringArrayProperty({ foo: [1, 2, 3] }, 'foo', 'Obj', {})).toThrow(InvalidPropertyError);
    expect(() => assertRecordStringArrayProperty({ foo: ['a', 123, 'c'] }, 'foo', 'Obj', {})).toThrow(
      InvalidPropertyError,
    );
    expect(() => assertRecordStringArrayProperty({ foo: 'not-array' }, 'foo', 'Obj', {})).toThrow(InvalidPropertyError);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('safeJSONstringify', () => {
    expect(safeJSONstringify({ a: 1, b: 'hello' })).toBe('{"a":1,"b":"hello"}');
    expect(safeJSONstringify({ a: 1 }, 2)).toBe('{\n  "a": 1\n}');
    expect(safeJSONstringify({ big: BigInt(123) })).toBe('{"big":"123"}');
    expect(safeJSONstringify(null)).toBe('null');
    expect(safeJSONstringify(undefined)).toBe(undefined);

    // Test circular reference returns empty string
    const circular: Record<string, unknown> = { a: 1 };
    circular.self = circular;
    expect(safeJSONstringify(circular)).toBe('');
  });

  //////////////////////////////////////////////////////////////////////////////

  it('isNonEmptyString', () => {
    // Valid non-empty strings
    expect(isNonEmptyString('hello')).toBe(true);
    expect(isNonEmptyString('a')).toBe(true);
    expect(isNonEmptyString(' ')).toBe(true); // whitespace is still non-empty
    expect(isNonEmptyString('0')).toBe(true);

    // Empty string
    expect(isNonEmptyString('')).toBe(false);

    // Non-string types
    const nonStringTypes = [null, undefined, 123, 0, true, false, {}, [], ['a']];
    for (const value of nonStringTypes) {
      expect(isNonEmptyString(value)).toBe(false);
    }
  });
});
