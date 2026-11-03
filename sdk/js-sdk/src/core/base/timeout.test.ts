import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { abortableSleep } from './timeout.js';

////////////////////////////////////////////////////////////////////////////////
//
// Jest Command line
// =================
//
// npx jest --colors --passWithNoTests ./src/base/timeout.test.ts
// npx jest --colors --passWithNoTests --coverage ./src/base/timeout.test.ts --collectCoverageFrom=./src/base/timeout.ts
//
////////////////////////////////////////////////////////////////////////////////

describe('abortableSleep', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  //////////////////////////////////////////////////////////////////////////////
  // Basic sleep behavior
  //////////////////////////////////////////////////////////////////////////////

  it('resolves after specified delay', async () => {
    const promise = abortableSleep(1000);

    vi.advanceTimersByTime(999);
    // Should not resolve yet
    let resolved = false;
    promise.then(() => {
      resolved = true;
    });

    await Promise.resolve(); // flush microtasks
    expect(resolved).toBe(false);

    vi.advanceTimersByTime(1);
    await promise;
  });

  it('resolves without signal', async () => {
    const promise = abortableSleep(100);
    vi.advanceTimersByTime(100);
    await expect(promise).resolves.toBeUndefined();
  });

  it('resolves with undefined signal', async () => {
    const promise = abortableSleep(100, undefined);
    vi.advanceTimersByTime(100);
    await expect(promise).resolves.toBeUndefined();
  });

  //////////////////////////////////////////////////////////////////////////////
  // Abort before sleep starts
  //////////////////////////////////////////////////////////////////////////////

  it('throws immediately if signal already aborted', () => {
    const controller = new AbortController();
    controller.abort();

    // throwIfAborted throws synchronously before Promise is created
    expect(() => abortableSleep(1000, controller.signal)).toThrow();
  });

  it('throws AbortError if signal already aborted', () => {
    const controller = new AbortController();
    controller.abort();

    try {
      abortableSleep(1000, controller.signal);
      throw new Error('Expected to throw');
    } catch (error) {
      expect((error as Error).name).toBe('AbortError');
    }
  });

  // Unlike an abort during the sleep, the raw reason is thrown, not wrapped.
  it.each([
    ['a string', 'early'],
    ['null', null],
  ])('throws the raw reason (%s) synchronously if signal already aborted', (_label, reason) => {
    const controller = new AbortController();
    controller.abort(reason);

    let thrown: unknown = 'nothing thrown';
    try {
      void abortableSleep(1000, controller.signal);
    } catch (error) {
      thrown = error;
    }

    expect(thrown).toBe(reason);
  });

  //////////////////////////////////////////////////////////////////////////////
  // Abort during sleep
  //////////////////////////////////////////////////////////////////////////////

  it.each([
    ['a string', 'stop'],
    ['null', null],
  ])(
    'wraps a non-Error reason (%s) in an AbortError with that cause when aborted during sleep',
    async (_label, reason) => {
      const controller = new AbortController();
      const promise = abortableSleep(1000, controller.signal);

      vi.advanceTimersByTime(500);
      controller.abort(reason);

      const error = await promise.then(
        () => undefined,
        (e: unknown) => e,
      );
      expect(error).toBeInstanceOf(Error);
      expect((error as Error).name).toBe('AbortError');
      expect((error as Error).cause).toBe(reason);
    },
  );

  it('rejects when signal is aborted during sleep', async () => {
    const controller = new AbortController();
    const promise = abortableSleep(1000, controller.signal);

    // Advance halfway
    vi.advanceTimersByTime(500);

    // Abort
    controller.abort();

    await expect(promise).rejects.toThrow();
  });

  it('rejected error has name AbortError when aborted during sleep', async () => {
    const controller = new AbortController();
    const promise = abortableSleep(1000, controller.signal);

    vi.advanceTimersByTime(500);
    controller.abort();

    try {
      await promise;
      throw new Error('Expected to throw');
    } catch (error) {
      expect((error as Error).name).toBe('AbortError');
    }
  });

  it('preserves abort reason as cause', async () => {
    const controller = new AbortController();
    const customReason = new Error('Custom reason');

    const promise = abortableSleep(1000, controller.signal);

    vi.advanceTimersByTime(500);
    controller.abort(customReason);

    try {
      await promise;
      throw new Error('Expected to throw');
    } catch (error) {
      expect((error as Error).cause).toBe(customReason);
    }
  });

  it('reuses the abort reason name (e.g. TimeoutError) instead of always AbortError', async () => {
    const controller = new AbortController();
    const timeoutReason = new DOMException('Timed out', 'TimeoutError');

    const promise = abortableSleep(1000, controller.signal);

    vi.advanceTimersByTime(500);
    controller.abort(timeoutReason);

    try {
      await promise;
      throw new Error('Expected to throw');
    } catch (error) {
      expect((error as Error).name).toBe('TimeoutError');
      expect((error as Error).cause).toBe(timeoutReason);
    }
  });

  it('rejects when aborted during sleep with a reason whose name getter throws', async () => {
    const controller = new AbortController();
    const hostileReason = {
      get name(): string {
        throw new Error('hostile name getter');
      },
    };

    let outcome: unknown = 'pending';
    abortableSleep(1000, controller.signal).then(
      () => (outcome = 'resolved'),
      (e: unknown) => (outcome = e),
    );

    vi.advanceTimersByTime(500);
    controller.abort(hostileReason);
    // Flush promise jobs only: the sleep timer must not be what settles it.
    await vi.advanceTimersByTimeAsync(0);

    expect(outcome).toBeInstanceOf(Error);
    expect((outcome as Error).cause).toBe(hostileReason);
  });

  //////////////////////////////////////////////////////////////////////////////
  // Timer cleanup
  //////////////////////////////////////////////////////////////////////////////

  it('clears timeout when aborted', async () => {
    const clearTimeoutSpy = vi.spyOn(global, 'clearTimeout');
    const controller = new AbortController();

    const promise = abortableSleep(1000, controller.signal);

    controller.abort();

    try {
      await promise;
    } catch {
      // Expected
    }

    expect(clearTimeoutSpy).toHaveBeenCalled();
    clearTimeoutSpy.mockRestore();
  });

  //////////////////////////////////////////////////////////////////////////////
  // Edge cases
  //////////////////////////////////////////////////////////////////////////////

  it('works with zero delay', async () => {
    const promise = abortableSleep(0);
    vi.advanceTimersByTime(0);
    await expect(promise).resolves.toBeUndefined();
  });

  it('non-aborted signal allows sleep to complete', async () => {
    const controller = new AbortController();
    const promise = abortableSleep(100, controller.signal);

    vi.advanceTimersByTime(100);

    await expect(promise).resolves.toBeUndefined();
  });

  it('removes its abort listener after resolving normally', async () => {
    const controller = new AbortController();
    const addSpy = vi.spyOn(controller.signal, 'addEventListener');
    const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');
    const promise = abortableSleep(100, controller.signal);

    vi.advanceTimersByTime(100);
    await promise;

    expect(addSpy).toHaveBeenCalledTimes(1);
    expect(removeSpy).toHaveBeenCalledTimes(1);
    expect(removeSpy).toHaveBeenCalledWith('abort', addSpy.mock.calls[0]![1]);
    expect(vi.getTimerCount()).toBe(0);
  });
});
