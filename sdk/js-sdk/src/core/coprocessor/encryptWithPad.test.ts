import { describe, expect, it, vi } from 'vitest';
import { reconstructWithOneRetry } from './encryptWithPad.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/coprocessor/encryptWithPad.test.ts
////////////////////////////////////////////////////////////////////////////////

describe('reconstructWithOneRetry', () => {
  it('reconstructs from the first responses without fetching again', async () => {
    const fetch = vi.fn(() => Promise.resolve('second'));
    const reconstruct = vi.fn((r: string) => Promise.resolve(`pad from ${r}`));
    await expect(reconstructWithOneRetry('first', fetch, reconstruct)).resolves.toBe('pad from first');
    expect(fetch).not.toHaveBeenCalled();
  });

  it('fetches once more when the first reconstruction fails', async () => {
    const fetch = vi.fn(() => Promise.resolve('second'));
    const reconstruct = vi.fn((r: string) =>
      r === 'first' ? Promise.reject(new Error('not enough consistent shares')) : Promise.resolve(`pad from ${r}`),
    );
    await expect(reconstructWithOneRetry('first', fetch, reconstruct)).resolves.toBe('pad from second');
    expect(fetch).toHaveBeenCalledTimes(1);
    expect(reconstruct).toHaveBeenCalledTimes(2);
  });

  it('surfaces the second failure, and never retries a failed fetch', async () => {
    const reconstruct = vi.fn(() => Promise.reject(new Error('still inconsistent')));
    await expect(reconstructWithOneRetry('first', () => Promise.resolve('second'), reconstruct)).rejects.toThrow(
      'still inconsistent',
    );
    expect(reconstruct).toHaveBeenCalledTimes(2);

    const fetch = vi.fn(() => Promise.reject(new Error('relayer down')));
    await expect(reconstructWithOneRetry('first', fetch, reconstruct)).rejects.toThrow('relayer down');
    expect(fetch).toHaveBeenCalledTimes(1);
  });
});
