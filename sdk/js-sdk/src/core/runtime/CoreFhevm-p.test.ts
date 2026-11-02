import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { FhevmBase } from '../types/coreFhevmClient.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import { describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { sepolia } from '../chains/definitions/sepolia.js';
import { createFhevmClientFrozenContext } from '../frozenContext/fhevmClientFrozenContext-p.js';
import { createCoreFhevm, initPublicAction, setFrozenContext } from './CoreFhevm-p.js';
import { createFhevmRuntime } from './CoreFhevmRuntime-p.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/runtime/CoreFhevm-p.test.ts
////////////////////////////////////////////////////////////////////////////////

/** A core client whose init runs `init` (the frozen context is set on success). */
function makeClient(init: () => Promise<void>) {
  const runtime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
    ethereum: {} as EthereumModule,
    relayer: {} as RelayerModule,
    config: {},
  });
  const initFn = vi.fn(async (client: FhevmBase<FhevmChain>) => {
    await init();
    setFrozenContext(client, createFhevmClientFrozenContext({}));
  });
  const client = createCoreFhevm(PRIVATE_ETHERS_TOKEN, { chain: sepolia, runtime }).extend((c) => ({
    actions: {},
    runtime: c.runtime,
    init: initFn,
  }));
  return { client, initFn };
}

////////////////////////////////////////////////////////////////////////////////
// init / ready
////////////////////////////////////////////////////////////////////////////////

describe('CoreFhevm init', () => {
  it('runs init once when it succeeds', async () => {
    const { client, initFn } = makeClient(async () => undefined);

    await client.ready;
    await client.init();
    await expect(initPublicAction(client)).resolves.toBeDefined();
    expect(initFn).toHaveBeenCalledOnce();
  });

  it('forgets a failed init: the next ready runs init again and succeeds', async () => {
    const networkError = new Error('fetch failed');
    let fail = true;
    const { client, initFn } = makeClient(async () => {
      if (fail) {
        throw networkError;
      }
    });

    await expect(client.ready).rejects.toBe(networkError);
    fail = false;
    await expect(client.ready).resolves.toBeUndefined();
    expect(initFn).toHaveBeenCalledTimes(2);
  });

  it('forgets a failed init: the next action runs init again and succeeds', async () => {
    const networkError = new Error('fetch failed');
    let fail = true;
    const { client, initFn } = makeClient(async () => {
      if (fail) {
        throw networkError;
      }
    });

    await expect(initPublicAction(client)).rejects.toBe(networkError);
    fail = false;
    await expect(initPublicAction(client)).resolves.toBeDefined();
    expect(initFn).toHaveBeenCalledTimes(2);
  });

  it('forgets a failed init only once every init fn has settled: a retry meanwhile shares it', async () => {
    const networkError = new Error('fetch failed');
    let release!: () => void;
    const { client, initFn } = makeClient(async () => {
      throw networkError;
    });
    const slowInitFn = vi.fn(
      () =>
        new Promise<void>((resolve) => {
          release = resolve;
        }),
    );
    client.extend((c) => ({ actions: {}, runtime: c.runtime, init: slowInitFn }));

    const first = client.ready;
    await vi.waitFor(() => expect(initFn).toHaveBeenCalledOnce());
    // The first init fn failed, the second is still running: same attempt.
    expect(client.ready).toBe(first);
    expect(initFn).toHaveBeenCalledOnce();
    expect(slowInitFn).toHaveBeenCalledOnce();

    release();
    await expect(first).rejects.toBe(networkError);
    // Settled: the next ready starts a new attempt.
    const second = client.ready;
    expect(second).not.toBe(first);
    await vi.waitFor(() => expect(slowInitFn).toHaveBeenCalledTimes(2));
    release();
    await expect(second).rejects.toBe(networkError);
    expect(initFn).toHaveBeenCalledTimes(2);
    expect(slowInitFn).toHaveBeenCalledTimes(2);
  });

  it('shares one in-flight init between concurrent callers, failed or not', async () => {
    const networkError = new Error('fetch failed');
    let fail = true;
    const { client, initFn } = makeClient(async () => {
      if (fail) {
        throw networkError;
      }
    });

    const failed = await Promise.allSettled([client.ready, client.init(), initPublicAction(client)]);
    expect(failed.map((r) => (r as PromiseRejectedResult).reason as unknown)).toEqual([
      networkError,
      networkError,
      networkError,
    ]);
    expect(initFn).toHaveBeenCalledOnce();

    fail = false;
    expect(client.init()).toBe(client.ready);
    await Promise.all([client.ready, client.init(), initPublicAction(client)]);
    expect(initFn).toHaveBeenCalledTimes(2);
  });
});
