import { describe, it, expect } from 'vitest';
import { toArray, simpleDeepFreeze, isDeepEqual, addInternalFunction } from './object.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/base/object.test.ts
////////////////////////////////////////////////////////////////////////////////

describe('object', () => {
  //////////////////////////////////////////////////////////////////////////////

  it('toArray', () => {
    // Non-array values are wrapped in an array
    expect(toArray('foo')).toEqual(['foo']);
    expect(toArray(123)).toEqual([123]);
    expect(toArray(null)).toEqual([null]);
    expect(toArray(undefined)).toEqual([undefined]);
    expect(toArray({ foo: 'bar' })).toEqual([{ foo: 'bar' }]);

    // Arrays are returned as-is
    const arr = [1, 2, 3];
    expect(toArray(arr)).toBe(arr);
    expect(toArray([])).toEqual([]);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('simpleDeepFreeze', () => {
    const obj = { foo: 'bar', nested: { baz: 'qux' }, arr: [1, 2, 3] };
    const frozen = simpleDeepFreeze(obj);

    expect(frozen).toBe(obj);
    expect(Object.isFrozen(frozen)).toBe(true);
    expect(Object.isFrozen(frozen.nested)).toBe(true);

    // Arrays are not recursed into
    expect(Object.isFrozen(frozen.arr)).toBe(false);

    // Frozen object cannot be mutated
    expect(() => {
      'use strict';
      // @ts-expect-error - testing runtime immutability
      frozen.foo = 'changed';
    }).toThrow();
  });

  it('simpleDeepFreeze does not recurse into null values', () => {
    const obj = { foo: null };
    expect(() => simpleDeepFreeze(obj)).not.toThrow();
    expect(Object.isFrozen(obj)).toBe(true);
  });

  it('simpleDeepFreeze skips already-frozen nested objects', () => {
    const nested = Object.freeze({ baz: 'qux' });
    const obj = { nested };
    simpleDeepFreeze(obj);
    expect(Object.isFrozen(obj.nested)).toBe(true);
    expect(obj.nested).toBe(nested);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('isDeepEqual with primitives', () => {
    // True
    expect(isDeepEqual('foo', 'foo')).toBe(true);
    expect(isDeepEqual(123, 123)).toBe(true);
    expect(isDeepEqual(true, true)).toBe(true);
    expect(isDeepEqual(null, null)).toBe(true);
    expect(isDeepEqual(undefined, undefined)).toBe(true);
    expect(isDeepEqual(NaN, NaN)).toBe(false); // uses === , not Object.is

    // False
    expect(isDeepEqual('foo', 'bar')).toBe(false);
    expect(isDeepEqual(123, 456)).toBe(false);
    expect(isDeepEqual(null, undefined)).toBe(false);
    expect(isDeepEqual(0, false)).toBe(false);
    expect(isDeepEqual('123', 123)).toBe(false);
  });

  it('isDeepEqual with arrays', () => {
    // True
    expect(isDeepEqual([1, 2, 3], [1, 2, 3])).toBe(true);
    expect(isDeepEqual([], [])).toBe(true);
    expect(isDeepEqual([{ foo: 'bar' }], [{ foo: 'bar' }])).toBe(true);
    expect(isDeepEqual([[1, 2], [3]], [[1, 2], [3]])).toBe(true);

    // False - order sensitive
    expect(isDeepEqual([1, 2, 3], [3, 2, 1])).toBe(false);

    // False - length mismatch
    expect(isDeepEqual([1, 2], [1, 2, 3])).toBe(false);
    expect(isDeepEqual([1, 2, 3], [1, 2])).toBe(false);

    // False - not an array
    expect(isDeepEqual({ length: 0 }, [])).toBe(false);
    expect(isDeepEqual('123', [1, 2, 3])).toBe(false);
    expect(isDeepEqual(null, [])).toBe(false);
  });

  it('isDeepEqual with objects', () => {
    // True
    expect(isDeepEqual({ foo: 'bar' }, { foo: 'bar' })).toBe(true);
    expect(isDeepEqual({}, {})).toBe(true);
    expect(isDeepEqual({ a: 1, b: 2 }, { b: 2, a: 1 })).toBe(true); // key order doesn't matter
    expect(isDeepEqual({ a: { b: { c: 1 } } }, { a: { b: { c: 1 } } })).toBe(true);
    expect(isDeepEqual({ a: [1, 2], b: 'x' }, { a: [1, 2], b: 'x' })).toBe(true);

    // False - different values
    expect(isDeepEqual({ foo: 'bar' }, { foo: 'baz' })).toBe(false);

    // False - different keys
    expect(isDeepEqual({ foo: 'bar' }, { baz: 'bar' })).toBe(false);

    // False - different number of keys
    expect(isDeepEqual({ a: 1 }, { a: 1, b: 2 })).toBe(false);
    expect(isDeepEqual({ a: 1, b: 2 }, { a: 1 })).toBe(false);

    // False - not an object
    expect(isDeepEqual(null, { foo: 'bar' })).toBe(false);
    expect(isDeepEqual([1, 2], { 0: 1, 1: 2 })).toBe(false);
    expect(isDeepEqual('foo', { foo: 'bar' })).toBe(false);

    // False - own property required, inherited properties don't count
    const withInherited = Object.create({ foo: 'bar' });
    withInherited.baz = 'qux';
    expect(isDeepEqual(withInherited, { foo: 'bar' })).toBe(false);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('addInternalFunction', () => {
    const target = { foo: 'bar' };
    const fn = () => 'result';

    const result = addInternalFunction(target, 'myFn', fn);

    expect(result).toBe(target);
    expect(result.myFn).toBe(fn);
    expect(result.myFn()).toBe('result');

    // Non-enumerable: hidden from Object.keys()/entries()
    expect(Object.keys(result)).toEqual(['foo']);
    expect(Object.keys(result)).not.toContain('myFn');

    // Non-writable
    expect(() => {
      'use strict';
      // @ts-expect-error - testing runtime immutability
      result.myFn = () => 'other';
    }).toThrow();

    // Non-configurable
    expect(() => {
      Object.defineProperty(result, 'myFn', { value: () => 'other' });
    }).toThrow();
    expect(() => delete (result as Record<string, unknown>).myFn).toThrow();
  });
});
