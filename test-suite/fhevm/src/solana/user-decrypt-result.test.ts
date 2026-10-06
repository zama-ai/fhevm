import { describe, expect, test } from 'bun:test';

import { expectUserDecryptValue } from './user-decrypt-result';

describe('expectUserDecryptValue', () => {
  test('returns the one cleartext when it equals the expected value', () => {
    expect(expectUserDecryptValue([{ value: 42n }], 42n)).toBe(42n);
    expect(expectUserDecryptValue([{ value: true }], 1n)).toBe(1n);
  });

  test('rejects anything but exactly one clear value', () => {
    expect(() => expectUserDecryptValue([], 42n)).toThrow('returned 0 clear values');
    expect(() => expectUserDecryptValue([{ value: 42n }, { value: 42n }], 42n)).toThrow('returned 2 clear values');
  });

  test('rejects a wrong or non-scalar cleartext', () => {
    expect(() => expectUserDecryptValue([{ value: 41n }], 42n)).toThrow('cleartext 41 != expected 42');
    expect(() => expectUserDecryptValue([{ value: { amount: 42n } }], 42n)).toThrow('non-scalar cleartext');
  });
});
