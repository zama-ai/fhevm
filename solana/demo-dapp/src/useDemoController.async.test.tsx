import { address } from '@solana/kit';
import { act, create, type ReactTestRenderer } from 'react-test-renderer';
import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';

const mocks = vi.hoisted(() => ({
  claim: vi.fn(),
  findDeposit: vi.fn(),
  findRedeem: vi.fn(),
  harvest: vi.fn(),
  joinDeposit: vi.fn(),
  joinRedeem: vi.fn(),
  lifecycle: vi.fn(),
  metrics: vi.fn(),
  operator: vi.fn(),
  revealShares: vi.fn(),
  revealUsdc: vi.fn(),
}));

vi.mock('./claim', () => ({ claimBatchPayout: mocks.claim }));
vi.mock('./demoSession', () => ({
  connectDemoSession: vi.fn(),
  describeWalletError: (error: unknown) => (error instanceof Error ? error.message : String(error)),
}));
vi.mock('./deposit', () => ({ depositToVault: mocks.joinDeposit, findExistingDeposit: mocks.findDeposit }));
vi.mock('./mutationLock', () => ({
  withWalletMutationLock: async (_session: unknown, action: () => Promise<unknown>) => action(),
}));
vi.mock('./operatorClient', () => ({ runDemoOperatorAction: mocks.operator }));
vi.mock('./redeem', () => ({ findExistingRedeem: mocks.findRedeem, joinRedeemBatch: mocks.joinRedeem }));
vi.mock('./revealShares', () => ({
  revealClaimedShares: mocks.revealShares,
  revealClaimedUsdc: mocks.revealUsdc,
}));
vi.mock('./settlement', () => ({ readVaultLifecycle: mocks.lifecycle }));
vi.mock('./vaultYield', () => ({ harvestDemoVault: mocks.harvest, readDemoVaultMetrics: mocks.metrics }));

import type { DemoSession } from './demoSession';
import { useDemoController, type DemoController } from './useDemoController';

const position = {
  batchIndex: 1n,
  batch: address('11111111111111111111111111111111'),
  amountBaseUnits: 100_000_000n,
};
const awaiting = { kind: 'awaiting-dispatch' as const, remainingSlots: 1n };
const dispatchable = { kind: 'awaiting-dispatch' as const, remainingSlots: 0n };
const proofReady = { kind: 'proving' as const, proofReady: true };
const settled = {
  kind: 'settled' as const,
  totalJoined: 100_000_000n,
  payoutReceived: 100_000_000n,
  claimed: true,
};
const settledUnclaimed = { ...settled, claimed: false };

const deferred = <T,>() => {
  let resolve!: (value: T) => void;
  let reject!: (reason: unknown) => void;
  const promise = new Promise<T>((resolvePromise, rejectPromise) => {
    resolve = resolvePromise;
    reject = rejectPromise;
  });
  return { promise, reject, resolve };
};

const flush = async () => {
  await act(async () => {
    await Promise.resolve();
    await Promise.resolve();
  });
};

const connect = async (controller: DemoController) => {
  await act(async () => {
    await controller.actions.connect(async (isActive) => {
      const session = {
        signer: { address: position.batch },
        isActive,
        assertActive: () => {
          if (!isActive()) throw new Error('stale session');
        },
      };
      return session as unknown as DemoSession;
    });
  });
};

describe('useDemoController generation safety', () => {
  let controller: DemoController;
  let renderer: ReactTestRenderer;

  function Harness() {
    controller = useDemoController();
    return null;
  }

  beforeEach(async () => {
    vi.clearAllMocks();
    vi.useFakeTimers();
    Object.assign(globalThis, { IS_REACT_ACT_ENVIRONMENT: true, window: globalThis });
    mocks.findDeposit.mockResolvedValue(position);
    mocks.findRedeem.mockResolvedValue(null);
    mocks.lifecycle.mockResolvedValue(awaiting);
    mocks.metrics.mockResolvedValue({ totalAssets: 125n, totalShares: 100n });
    await act(async () => {
      renderer = create(<Harness />);
    });
  });

  afterEach(() => {
    act(() => renderer.unmount());
    vi.useRealTimers();
  });

  test('an old polling response cannot overwrite the reconnected lifecycle', async () => {
    const oldPoll = deferred<typeof awaiting>();
    mocks.lifecycle.mockReturnValueOnce(oldPoll.promise).mockResolvedValue(settled);
    await connect(controller);
    await flush();

    act(() => controller.actions.disconnect());
    await connect(controller);
    await flush();
    expect(controller.state.depositLifecycle).toEqual(settled);

    oldPoll.resolve(awaiting);
    await flush();
    expect(controller.state.depositLifecycle).toEqual(settled);
  });

  test('automatically dispatches an eligible batch', async () => {
    mocks.lifecycle.mockResolvedValue(dispatchable);
    await connect(controller);
    await flush();

    expect(mocks.operator).toHaveBeenCalledWith('dispatch', position, 'deposit');
    expect(controller.state.depositOperatorAction).toBe(null);
  });

  test('automatically settles when the proof is ready', async () => {
    mocks.lifecycle.mockResolvedValue(proofReady);
    await connect(controller);
    await flush();

    expect(mocks.operator).toHaveBeenCalledWith('settle', position, 'deposit');
    expect(controller.state.depositOperatorAction).toBe(null);
  });

  test('automatically advances the redeem batch too', async () => {
    mocks.findDeposit.mockResolvedValue(null);
    mocks.findRedeem.mockResolvedValue(position);
    mocks.lifecycle.mockResolvedValue(dispatchable);
    await connect(controller);
    await flush();

    expect(mocks.operator).toHaveBeenCalledWith('dispatch', position, 'redeem');
    expect(controller.state.redeemOperatorAction).toBe(null);
  });

  test('refreshes the lifecycle after an automatic action succeeds', async () => {
    const proving = { kind: 'proving' as const, proofReady: false };
    mocks.lifecycle.mockResolvedValueOnce(dispatchable).mockResolvedValueOnce(proving);
    await connect(controller);
    await flush();

    expect(controller.state.depositLifecycle).toEqual(proving);
  });

  test('does not advance a batch before its slot boundary', async () => {
    await connect(controller);
    await flush();

    expect(mocks.operator).not.toHaveBeenCalled();
  });

  test('retries an automatic operator failure on the next lifecycle poll', async () => {
    mocks.lifecycle.mockResolvedValue(dispatchable);
    mocks.operator.mockRejectedValueOnce(new Error('temporary operator failure')).mockResolvedValue(undefined);
    await connect(controller);
    await flush();
    expect(mocks.operator).toHaveBeenCalledTimes(1);

    await act(async () => {
      await vi.advanceTimersByTimeAsync(2_500);
    });
    await flush();
    expect(mocks.operator).toHaveBeenCalledTimes(2);
  });

  test('an old automatic operator completion cannot clear the new generation action', async () => {
    const oldOperator = deferred<void>();
    const currentOperator = deferred<void>();
    mocks.operator.mockReturnValueOnce(oldOperator.promise).mockReturnValueOnce(currentOperator.promise);
    mocks.lifecycle.mockResolvedValue(dispatchable);
    await connect(controller);
    await flush();

    act(() => controller.actions.disconnect());
    await connect(controller);
    await flush();
    expect(controller.state.depositOperatorAction).toBe('dispatch');

    oldOperator.resolve();
    await flush();
    expect(controller.state.depositOperatorAction).toBe('dispatch');

    currentOperator.resolve();
    await flush();
  });

  test('an old claim completion cannot clear the new generation claiming state', async () => {
    const oldClaim = deferred<void>();
    const currentClaim = deferred<void>();
    mocks.claim.mockReturnValueOnce(oldClaim.promise).mockReturnValueOnce(currentClaim.promise);
    mocks.lifecycle.mockResolvedValue(settledUnclaimed);
    await connect(controller);
    await flush();
    controller.actions.claim('deposit');
    await flush();

    act(() => controller.actions.disconnect());
    await connect(controller);
    await flush();
    controller.actions.claim('deposit');
    await flush();
    expect(controller.state.depositClaiming).toBe(true);

    oldClaim.resolve();
    await flush();
    expect(controller.state.depositClaiming).toBe(true);

    currentClaim.resolve();
    await flush();
    expect(controller.state.depositClaiming).toBe(false);
  });

  test('an old harvest result cannot overwrite the reconnected generation', async () => {
    const oldHarvest = deferred<{
      before: { totalAssets: bigint; totalShares: bigint };
      after: { totalAssets: bigint; totalShares: bigint };
    }>();
    mocks.lifecycle.mockResolvedValue(settled);
    mocks.metrics.mockResolvedValueOnce({ totalAssets: 125n, totalShares: 100n }).mockResolvedValue({
      totalAssets: 200n,
      totalShares: 100n,
    });
    mocks.harvest.mockReturnValue(oldHarvest.promise);
    await connect(controller);
    await flush();
    controller.actions.fastForwardOneYear();
    await flush();

    act(() => controller.actions.disconnect());
    await connect(controller);
    await flush();
    expect(controller.state.vaultMetrics).toEqual({ totalAssets: 200n, totalShares: 100n });

    oldHarvest.resolve({
      before: { totalAssets: 125n, totalShares: 100n },
      after: { totalAssets: 999n, totalShares: 100n },
    });
    await flush();
    expect(controller.state.vaultMetrics).toEqual({ totalAssets: 200n, totalShares: 100n });
    expect(controller.state.harvesting).toBe(false);
  });

  test('fast-forward recovers from an initial metrics read failure', async () => {
    const before = { totalAssets: 100n, totalShares: 100n };
    const after = { totalAssets: 107n, totalShares: 100n };
    mocks.lifecycle.mockResolvedValue(settled);
    mocks.metrics.mockRejectedValueOnce(new Error('metrics unavailable')).mockResolvedValue(before);
    mocks.harvest.mockResolvedValue({ before, after });
    await connect(controller);
    await flush();
    expect(controller.state.vaultMetrics).toBe(null);
    expect(controller.state.harvestError).toBe('metrics unavailable');

    controller.actions.fastForwardOneYear();
    await flush();
    expect(mocks.harvest).toHaveBeenCalledTimes(1);
    expect(controller.state.vaultMetrics).toEqual(after);
    expect(controller.state.harvestError).toBe(null);
  });
});
