import { describe, expect, test } from 'bun:test';

import { expectCleartext, userDecryptScalar } from './user-decrypt-result';

describe('userDecryptScalar', () => {
  test('returns the one scalar cleartext', () => {
    expect(userDecryptScalar([{ value: 42n }])).toBe(42n);
    expect(userDecryptScalar([{ value: true }])).toBe(1n);
  });

  test('rejects anything but exactly one clear value', () => {
    expect(() => userDecryptScalar([])).toThrow('returned 0 clear values');
    expect(() => userDecryptScalar([{ value: 42n }, { value: 42n }])).toThrow('returned 2 clear values');
  });

  test('rejects a non-scalar cleartext', () => {
    expect(() => userDecryptScalar([{ value: { amount: 42n } }])).toThrow('non-scalar cleartext');
  });
});

describe('expectCleartext', () => {
  test('returns the cleartext when it equals the expected value, and rejects any other', () => {
    expect(expectCleartext(42n, 42n)).toBe(42n);
    expect(() => expectCleartext(41n, 42n)).toThrow('cleartext 41 != expected 42');
  });
});
