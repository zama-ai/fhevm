import { describe, it, expect } from 'vitest';
import { InternalError } from './InternalError.js';
import { ensureError, assertNever, getErrorMessage } from './utils.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/base/errors/utils.test.ts
////////////////////////////////////////////////////////////////////////////////

describe('errors/utils', () => {
  //////////////////////////////////////////////////////////////////////////////

  it('ensureError returns the same instance when given an Error', () => {
    const error = new Error('boom');
    expect(ensureError(error)).toBe(error);

    const internalError = new InternalError({ message: 'internal boom' });
    expect(ensureError(internalError)).toBe(internalError);
  });

  it('ensureError wraps an error-shaped object, preserving message/name/cause', () => {
    const shaped = { message: 'shaped message', name: 'ShapedError', cause: 'root cause' };
    const err = ensureError(shaped);

    expect(err).toBeInstanceOf(Error);
    expect(err.message).toBe('shaped message');
    expect(err.name).toBe('ShapedError');
    expect(err.cause).toBe('root cause');
  });

  it('ensureError falls back to defaults for missing message/name', () => {
    const err = ensureError({});

    expect(err.message).toBe('Non-Error value caught in exception handler');
    expect(err.name).toBe('ErrorWrapper');
    expect(err.cause).toEqual({});
  });

  it('ensureError uses the caught value itself as cause when no cause is provided', () => {
    const shaped = { message: 'no cause here' };
    const err = ensureError(shaped);

    expect(err.cause).toBe(shaped);
  });

  it('ensureError wraps primitive non-Error values', () => {
    const err = ensureError('plain string');

    expect(err).toBeInstanceOf(Error);
    expect(err.message).toBe('Non-Error value caught in exception handler');
    expect(err.name).toBe('ErrorWrapper');
    expect(err.cause).toBe('plain string');
  });

  // NOTE: ensureError reads `e.message`/`e.name`/`e.cause` without a null-check, so it
  // throws a TypeError instead of returning a wrapped Error for null/undefined input.
  it('ensureError throws when given null or undefined', () => {
    expect(() => ensureError(null)).toThrow(TypeError);
    expect(() => ensureError(undefined)).toThrow(TypeError);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('assertNever throws an InternalError with the given message', () => {
    expect(() => assertNever('unreachable' as never, 'unexpected value')).toThrow(
      new InternalError({ message: 'unexpected value' }),
    );
  });

  //////////////////////////////////////////////////////////////////////////////

  it('getErrorMessage extracts the message from strings, Errors, and other values', () => {
    expect(getErrorMessage('plain string')).toBe('plain string');
    expect(getErrorMessage(new Error('error message'))).toBe('error message');
    expect(getErrorMessage(new InternalError({ message: 'internal message' }))).toBe('internal message');
    expect(getErrorMessage(123)).toBe('123');
    expect(getErrorMessage(null)).toBe('null');
    expect(getErrorMessage(undefined)).toBe('undefined');
  });

  it('getErrorMessage strips leading and trailing quotes', () => {
    expect(getErrorMessage('"quoted"')).toBe('quoted');
    expect(getErrorMessage("'quoted'")).toBe('quoted');
    expect(getErrorMessage(`"'mixed'"`)).toBe('mixed');
    expect(getErrorMessage('unquoted')).toBe('unquoted');
    expect(getErrorMessage('""')).toBe('');
    expect(getErrorMessage(new Error('"error message"'))).toBe('error message');
  });

  it('getErrorMessage only strips quotes at the boundaries, not inside the message', () => {
    expect(getErrorMessage('say "hello" to me')).toBe('say "hello" to me');
    expect(getErrorMessage('"say "hello" to me"')).toBe('say "hello" to me');
  });
});
