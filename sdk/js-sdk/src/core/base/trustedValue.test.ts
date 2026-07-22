import { describe, it, expect } from 'vitest';
import { createTrustedValue, verifyTrustedValue } from './trustedValue.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/base/trustedValue.test.ts
////////////////////////////////////////////////////////////////////////////////

describe('trustedValue', () => {
  //////////////////////////////////////////////////////////////////////////////

  it('createTrustedValue + verifyTrustedValue roundtrips the value with the same token', () => {
    const token = Symbol('token');

    expect(verifyTrustedValue(createTrustedValue('secret', token), token)).toBe('secret');
    expect(verifyTrustedValue(createTrustedValue(123, token), token)).toBe(123);
    expect(verifyTrustedValue(createTrustedValue(null, token), token)).toBe(null);
    expect(verifyTrustedValue(createTrustedValue(undefined, token), token)).toBe(undefined);

    const obj = { foo: 'bar' };
    expect(verifyTrustedValue(createTrustedValue(obj, token), token)).toBe(obj);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('verifyTrustedValue throws on token mismatch', () => {
    const token = Symbol('token');
    const otherToken = Symbol('other-token');
    const trusted = createTrustedValue('secret', token);

    expect(() => verifyTrustedValue(trusted, otherToken)).toThrow('Token mismatch');
  });

  it('verifyTrustedValue distinguishes tokens with the same description', () => {
    const token = Symbol('token');
    const lookalikeToken = Symbol('token');
    const trusted = createTrustedValue('secret', token);

    expect(() => verifyTrustedValue(trusted, lookalikeToken)).toThrow('Token mismatch');
  });

  //////////////////////////////////////////////////////////////////////////////

  it('verifyTrustedValue throws on a forged / non-genuine value', () => {
    const token = Symbol('token');

    // Plain object shaped like a TrustedValue, but not a real instance
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const forged = { toString: () => 'TrustedValue' } as any;

    expect(() => verifyTrustedValue(forged, token)).toThrow('Invalid TrustedValue');
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    expect(() => verifyTrustedValue(null as any, token)).toThrow('Invalid TrustedValue');
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    expect(() => verifyTrustedValue(undefined as any, token)).toThrow('Invalid TrustedValue');
  });

  //////////////////////////////////////////////////////////////////////////////

  it('createTrustedValue returns a frozen, opaque instance', () => {
    const token = Symbol('token');
    const trusted = createTrustedValue('secret', token);

    expect(Object.isFrozen(trusted)).toBe(true);

    // Does not leak the inner value through string/JSON conversion
    expect(String(trusted)).toBe('TrustedValue');
    expect(JSON.stringify(trusted)).toBe('"TrustedValue"');
    expect(JSON.stringify({ trusted })).toBe('{"trusted":"TrustedValue"}');

    // Cannot be mutated
    expect(() => {
      'use strict';
      // @ts-expect-error - testing runtime immutability
      trusted.injected = 'evil';
    }).toThrow();
  });

  //////////////////////////////////////////////////////////////////////////////

  it('each trusted value is independently verifiable', () => {
    const token = Symbol('token');
    const a = createTrustedValue('a', token);
    const b = createTrustedValue('b', token);

    expect(verifyTrustedValue(a, token)).toBe('a');
    expect(verifyTrustedValue(b, token)).toBe('b');
  });
});
