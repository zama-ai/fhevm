import { describe, it, expect } from 'vitest';
import { executeWithBatching } from './promise.js';

type PromiseFactory<T> = () => Promise<T>;

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/base/promise.test.ts
////////////////////////////////////////////////////////////////////////////////

describe('executeWithBatching', () => {
  it.each([true, false])('executes all factories (parallel=%s)', async (parallel) => {
    const executionOrder: number[] = [];
    const factories: PromiseFactory<number>[] = [
      async () => {
        executionOrder.push(1);
        return 1;
      },
      async () => {
        executionOrder.push(2);
        return 2;
      },
      async () => {
        executionOrder.push(3);
        return 3;
      },
    ];

    const results = await executeWithBatching(factories, parallel);

    expect(results).toEqual([1, 2, 3]);
    // All factories should have been called
    expect(executionOrder.sort()).toEqual([1, 2, 3]);
  });

  //////////////////////////////////////////////////////////////////////////////

  it.each([
    {
      parallel: true,
      expectedCompletionOrder: [2, 3, 1], // parallel: completes by delay (10ms, 20ms, 30ms)
    },
    {
      parallel: false,
      expectedCompletionOrder: [1, 2, 3], // sequential: completes in order regardless of delay
    },
  ])(
    'returns results in original order even with varying delays (parallel=$parallel)',
    async ({ parallel, expectedCompletionOrder }) => {
      const executionOrder: number[] = [];
      const completionOrder: number[] = [];

      const factories: PromiseFactory<number>[] = [
        () =>
          new Promise((resolve) => {
            executionOrder.push(1);
            setTimeout(() => {
              completionOrder.push(1);
              resolve(1);
            }, 30);
          }), // slowest
        () =>
          new Promise((resolve) => {
            executionOrder.push(2);
            setTimeout(() => {
              completionOrder.push(2);
              resolve(2);
            }, 10);
          }), // fastest
        () =>
          new Promise((resolve) => {
            executionOrder.push(3);
            setTimeout(() => {
              completionOrder.push(3);
              resolve(3);
            }, 20);
          }),
      ];

      const results = await executeWithBatching(factories, parallel);

      // Results should always be in original order
      expect(results).toEqual([1, 2, 3]);
      // Execution order is always 1, 2, 3 (factories are called in order)
      expect(executionOrder).toEqual([1, 2, 3]);
      // Completion order differs based on parallel vs sequential
      expect(completionOrder).toEqual(expectedCompletionOrder);
    },
  );

  //////////////////////////////////////////////////////////////////////////////

  it.each([true, false])('handles empty array (parallel=%s)', async (parallel) => {
    const results = await executeWithBatching([], parallel);
    expect(results).toEqual([]);
  });

  //////////////////////////////////////////////////////////////////////////////

  it.each([true, false])('handles single factory (parallel=%s)', async (parallel) => {
    const results = await executeWithBatching([async () => 42], parallel);
    expect(results).toEqual([42]);
  });

  //////////////////////////////////////////////////////////////////////////////

  it.each([true, false])('propagates single error (parallel=%s)', async (parallel) => {
    const factories: PromiseFactory<number>[] = [
      async () => 1,
      async () => {
        throw new Error('Factory error');
      },
      async () => 3,
    ];

    await expect(executeWithBatching(factories, parallel)).rejects.toThrow('Factory error');
  });

  //////////////////////////////////////////////////////////////////////////////

  it.each([true, false])('propagates multiple errors (parallel=%s)', async (parallel) => {
    const factories: PromiseFactory<number>[] = [
      async () => 1,
      async () => {
        throw new Error('Factory error 2');
      },
      async () => 3,
      async () => {
        throw new Error('Factory error 4');
      },
    ];

    await expect(executeWithBatching(factories, parallel)).rejects.toThrow('Factory error 2');
  });

  //////////////////////////////////////////////////////////////////////////////

  it.each([
    { parallel: true, expectedError: 'Factory error 4' }, // parallel: error 4 rejects first (10ms < 30ms)
    { parallel: false, expectedError: 'Factory error 2' }, // sequential: error 2 is encountered first in order
  ])('propagates multiple errors with delay (parallel=$parallel)', async ({ parallel, expectedError }) => {
    const factories: PromiseFactory<number>[] = [
      async () => 1,
      async () => {
        await new Promise((r) => setTimeout(r, 30));
        throw new Error('Factory error 2');
      },
      async () => 3,
      async () => {
        await new Promise((r) => setTimeout(r, 10));
        throw new Error('Factory error 4');
      },
    ];

    await expect(executeWithBatching(factories, parallel)).rejects.toThrow(expectedError);
  });

  //////////////////////////////////////////////////////////////////////////////

  it.each([true, false])('preserves types correctly (parallel=%s)', async (parallel) => {
    interface User {
      id: number;
      name: string;
    }

    const factories: PromiseFactory<User>[] = [
      async () => ({ id: 1, name: 'Alice' }),
      async () => ({ id: 2, name: 'Bob' }),
    ];

    const results = await executeWithBatching(factories, parallel);

    expect(results[0]!.id).toBe(1);
    expect(results[0]!.name).toBe('Alice');
    expect(results[1]!.id).toBe(2);
    expect(results[1]!.name).toBe('Bob');
  });

  //////////////////////////////////////////////////////////////////////////////

  it.each([true, false])('handles mixed return values (parallel=%s)', async (parallel) => {
    const factories: PromiseFactory<string | number | null>[] = [
      async () => 'string',
      async () => 42,
      async () => null,
    ];

    const results = await executeWithBatching(factories, parallel);
    expect(results).toEqual(['string', 42, null]);
  });

  //////////////////////////////////////////////////////////////////////////////

  it('parallel is faster than sequential for independent operations', async () => {
    const delay = 20;
    const factories: PromiseFactory<number>[] = [
      () => new Promise((resolve) => setTimeout(() => resolve(1), delay)),
      () => new Promise((resolve) => setTimeout(() => resolve(2), delay)),
      () => new Promise((resolve) => setTimeout(() => resolve(3), delay)),
    ];

    const parallelStart = Date.now();
    await executeWithBatching(factories, true);
    const parallelTime = Date.now() - parallelStart;

    const sequentialStart = Date.now();
    await executeWithBatching(factories, false);
    const sequentialTime = Date.now() - sequentialStart;

    // Parallel should be roughly 1x delay, sequential should be roughly 3x delay
    expect(parallelTime).toBeLessThan(sequentialTime);
  });
});
