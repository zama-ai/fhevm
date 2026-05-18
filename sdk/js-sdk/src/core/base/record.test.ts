import { describe, it, expect } from 'vitest';
import { InvalidPropertyError } from './errors/InvalidPropertyError.js';
import {
  isRecordNonNullableProperty,
  assertRecordNonNullableProperty,
  isRecordArrayProperty,
  assertRecordArrayProperty,
  isRecordBooleanProperty,
  assertRecordBooleanProperty,
  typeofProperty,
} from './record.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/base/record.test.ts
////////////////////////////////////////////////////////////////////////////////

describe('record', () => {
  //////////////////////////////////////////////////////////////////////////////

  it('isRecordNonNullableProperty', () => {
    // True
    expect(isRecordNonNullableProperty({ foo: 'bar' }, 'foo')).toBe(true);
    expect(isRecordNonNullableProperty({ foo: 0 }, 'foo')).toBe(true);
    expect(isRecordNonNullableProperty({ foo: false }, 'foo')).toBe(true);
    expect(isRecordNonNullableProperty({ foo: [] }, 'foo')).toBe(true);
    expect(isRecordNonNullableProperty({ foo: {} }, 'foo')).toBe(true);

    // False
    expect(isRecordNonNullableProperty({ foo: null }, 'foo')).toBe(false);
    expect(isRecordNonNullableProperty({ foo: undefined }, 'foo')).toBe(false);
    expect(isRecordNonNullableProperty({}, 'foo')).toBe(false);
    expect(isRecordNonNullableProperty(null, 'foo')).toBe(false);
    expect(isRecordNonNullableProperty(undefined, 'foo')).toBe(false);
    expect(isRecordNonNullableProperty('string', 'foo')).toBe(false);
    expect(isRecordNonNullableProperty(123, 'foo')).toBe(false);
    expect(isRecordNonNullableProperty([], 'foo')).toBe(false);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertRecordNonNullableProperty', () => {
    // True
    expect(() => assertRecordNonNullableProperty({ foo: 'bar' }, 'foo', 'Foo', {})).not.toThrow();

    expect(() => assertRecordNonNullableProperty({ foo: 0 }, 'foo', 'Foo', {})).not.toThrow();

    const missing = () =>
      new InvalidPropertyError(
        {
          subject: 'Foo',
          property: 'foo',
          expectedType: 'non-nullable',
          type: 'undefined',
        },
        {},
      );

    // False
    expect(() => assertRecordNonNullableProperty({ foo: null }, 'foo', 'Foo', {})).toThrow(missing());

    expect(() => assertRecordNonNullableProperty({ foo: undefined }, 'foo', 'Foo', {})).toThrow(missing());

    expect(() => assertRecordNonNullableProperty('', 'foo', 'Foo', {})).toThrow(missing());

    expect(() => assertRecordNonNullableProperty(null, 'foo', 'Foo', {})).toThrow(missing());

    expect(() => assertRecordNonNullableProperty(undefined, 'foo', 'Foo', {})).toThrow(missing());

    expect(() => assertRecordNonNullableProperty({}, 'foo', 'Foo', {})).toThrow(missing());

    expect(() => assertRecordNonNullableProperty([], 'foo', 'Foo', {})).toThrow(missing());

    expect(() => assertRecordNonNullableProperty(['foo'], 'foo', 'Foo', {})).toThrow(missing());

    expect(() => assertRecordNonNullableProperty('foo', 'foo', 'Foo', {})).toThrow(missing());
  });

  //////////////////////////////////////////////////////////////////////////////

  it('typeofProperty', () => {
    expect(typeofProperty({ foo: [] }, 'foo')).toBe('object');
    expect(typeofProperty({ foo: '123' }, 'foo')).toBe('string');
    expect(typeofProperty({ foo: 123 }, 'foo')).toBe('number');
    expect(typeofProperty('123', 'foo')).toBe('undefined');
    expect(typeofProperty(123, 'foo')).toBe('undefined');
    expect(typeofProperty(null, 'foo')).toBe('undefined');
    expect(typeofProperty(undefined, 'foo')).toBe('undefined');
  });

  //////////////////////////////////////////////////////////////////////////////

  it('isRecordArrayProperty', () => {
    // True
    expect(isRecordArrayProperty({ foo: [] }, 'foo')).toBe(true);
    expect(isRecordArrayProperty({ foo: [1, 2, 3] }, 'foo')).toBe(true);
    expect(isRecordArrayProperty({ foo: ['a', 'b'] }, 'foo')).toBe(true);

    // False
    expect(isRecordArrayProperty({ foo: null }, 'foo')).toBe(false);
    expect(isRecordArrayProperty({ foo: undefined }, 'foo')).toBe(false);
    expect(isRecordArrayProperty({ foo: {} }, 'foo')).toBe(false);
    expect(isRecordArrayProperty({ foo: 'string' }, 'foo')).toBe(false);
    expect(isRecordArrayProperty({ foo: 123 }, 'foo')).toBe(false);
    expect(isRecordArrayProperty({}, 'foo')).toBe(false);
    expect(isRecordArrayProperty(null, 'foo')).toBe(false);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertRecordArrayProperty', () => {
    // True
    expect(() => assertRecordArrayProperty({ foo: [] }, 'foo', 'Foo', {})).not.toThrow();

    expect(() => assertRecordArrayProperty({ foo: ['123', 123] }, 'foo', 'Foo', {})).not.toThrow();

    const e = (type: string) =>
      new InvalidPropertyError(
        {
          subject: 'Foo',
          property: 'foo',
          expectedType: 'Array',
          type,
        },
        {},
      );

    const missing = () =>
      new InvalidPropertyError(
        {
          subject: 'Foo',
          property: 'foo',
          expectedType: 'Array',
          type: 'undefined',
        },
        {},
      );

    // False
    expect(() => assertRecordArrayProperty({ foo: null }, 'foo', 'Foo', {})).toThrow(missing());

    expect(() => assertRecordArrayProperty({ foo: undefined }, 'foo', 'Foo', {})).toThrow(missing());

    expect(() => assertRecordArrayProperty({}, 'foo', 'Foo', {})).toThrow(missing());

    expect(() => assertRecordArrayProperty(null, 'foo', 'Foo', {})).toThrow(missing());

    expect(() => assertRecordArrayProperty({ foo: 'hello' }, 'foo', 'Foo', {})).toThrow(e('string'));

    expect(() => assertRecordArrayProperty({ foo: 123 }, 'foo', 'Foo', {})).toThrow(e('number'));
  });

  //////////////////////////////////////////////////////////////////////////////

  it('isRecordBooleanProperty', () => {
    // True
    expect(isRecordBooleanProperty({ foo: true }, 'foo')).toBe(true);
    expect(isRecordBooleanProperty({ foo: false }, 'foo')).toBe(true);

    // False
    expect(isRecordBooleanProperty({ foo: null }, 'foo')).toBe(false);
    expect(isRecordBooleanProperty({ foo: undefined }, 'foo')).toBe(false);
    expect(isRecordBooleanProperty({ foo: 'true' }, 'foo')).toBe(false);
    expect(isRecordBooleanProperty({ foo: 1 }, 'foo')).toBe(false);
    expect(isRecordBooleanProperty({ foo: 0 }, 'foo')).toBe(false);
    expect(isRecordBooleanProperty({}, 'foo')).toBe(false);
    expect(isRecordBooleanProperty(null, 'foo')).toBe(false);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertRecordBooleanProperty', () => {
    // True
    expect(() => assertRecordBooleanProperty({ foo: true }, 'foo', 'Foo', {})).not.toThrow();

    expect(() => assertRecordBooleanProperty({ foo: false }, 'foo', 'Foo', {})).not.toThrow();

    const e = (type: string) =>
      new InvalidPropertyError(
        {
          subject: 'Foo',
          property: 'foo',
          expectedType: 'boolean',
          type,
        },
        {},
      );

    const missing = () =>
      new InvalidPropertyError(
        {
          subject: 'Foo',
          property: 'foo',
          expectedType: 'boolean',
          type: 'undefined',
        },
        {},
      );

    // False
    expect(() => assertRecordBooleanProperty({ foo: null }, 'foo', 'Foo', {})).toThrow(missing());

    expect(() => assertRecordBooleanProperty({ foo: undefined }, 'foo', 'Foo', {})).toThrow(missing());

    expect(() => assertRecordBooleanProperty({}, 'foo', 'Foo', {})).toThrow(missing());

    expect(() => assertRecordBooleanProperty(null, 'foo', 'Foo', {})).toThrow(missing());

    expect(() => assertRecordBooleanProperty({ foo: 'hello' }, 'foo', 'Foo', {})).toThrow(e('string'));

    expect(() => assertRecordBooleanProperty({ foo: 123 }, 'foo', 'Foo', {})).toThrow(e('number'));
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertRecordBooleanProperty with expectedValue', () => {
    // Valid with expectedValue
    expect(() => assertRecordBooleanProperty({ foo: true }, 'foo', 'Foo', { expectedValue: true })).not.toThrow();

    expect(() => assertRecordBooleanProperty({ foo: false }, 'foo', 'Foo', { expectedValue: false })).not.toThrow();

    // Invalid: value doesn't match expectedValue
    expect(() => assertRecordBooleanProperty({ foo: false }, 'foo', 'Foo', { expectedValue: true })).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          property: 'foo',
          expectedType: 'boolean',
          expectedValue: 'true',
          type: 'boolean',
          value: 'false',
        },
        {},
      ),
    );

    expect(() => assertRecordBooleanProperty({ foo: true }, 'foo', 'Foo', { expectedValue: false })).toThrow(
      new InvalidPropertyError(
        {
          subject: 'Foo',
          property: 'foo',
          expectedType: 'boolean',
          expectedValue: 'false',
          type: 'boolean',
          value: 'true',
        },
        {},
      ),
    );
  });
});
