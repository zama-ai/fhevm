import { address } from '@solana/kit';
import { act, create, type ReactTestRenderer } from 'react-test-renderer';
import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';

const mocks = vi.hoisted(() => ({
  findDeposit: vi.fn(),
  findRedeem: vi.fn(),
  findCompletedRedeem: vi.fn(),
  fund: vi.fn(),
  harvest: vi.fn(),
  hasShares: vi.fn(),
  joinDeposit: vi.fn(),
  joinRedeem: vi.fn(),
  lifecycle: vi.fn(),
  metrics: vi.fn(),
  operator: vi.fn(),
  prepareBatch: vi.fn(),
  readBalances: vi.fn(),
  recordCompletedRedeem: vi.fn(),
  hasConfidentialUsdc: vi.fn(),
  revealShares: vi.fn(),
  revealUsdc: vi.fn(),
}));

vi.mock('./demoSession', () => ({
  connectDemoSession: vi.fn(),
  describeWalletError: (error: unknown) => (error instanceof Error ? error.message : String(error)),
  ensureDemoFunding: mocks.fund,
  readDemoWalletBalances: mocks.readBalances,
}));
vi.mock('./deposit', () => ({
  depositToVault: mocks.joinDeposit,
  findExistingDeposit: mocks.findDeposit,
  hasClaimedDeposit: mocks.hasShares,
  usdcToBaseUnits: (amount: number) => BigInt(amount * 1_000_000),
}));
vi.mock('./mutationLock', () => ({
  withWalletMutationLock: async (_session: unknown, action: () => Promise<unknown>) => action(),
}));
vi.mock('./operatorClient', () => ({
  prepareDemoBatch: mocks.prepareBatch,
  prepareDemoDepositBatch: mocks.prepareBatch,
  runDemoOperatorAction: mocks.operator,
}));
vi.mock('./redeem', () => ({
  findCompletedRedeem: mocks.findCompletedRedeem,
  findExistingRedeem: mocks.findRedeem,
  joinRedeemBatch: mocks.joinRedeem,
  recordCompletedRedeem: mocks.recordCompletedRedeem,
}));
vi.mock('./revealShares', () => ({
  hasConfidentialBalanceAccount: mocks.hasConfidentialUsdc,
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
        config: {
          mints: {
            joinConfidential: position.batch,
            payoutConfidential: position.batch,
          },
        },
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
    mocks.findCompletedRedeem.mockResolvedValue(null);
    mocks.fund.mockResolvedValue(undefined);
    mocks.hasShares.mockResolvedValue(true);
    mocks.lifecycle.mockResolvedValue(awaiting);
    mocks.metrics.mockResolvedValue({ totalAssets: 125n, totalShares: 100n });
    mocks.prepareBatch.mockResolvedValue(position);
    mocks.readBalances.mockResolvedValue([5_000_000_000n, 1_000_000_000n]);
    mocks.hasConfidentialUsdc.mockResolvedValue(true);
    mocks.revealUsdc.mockResolvedValue({ handle: '0xcurrent', value: 1_000_000_000n });
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

    expect(mocks.operator).toHaveBeenCalledWith({ action: 'dispatch', position, direction: 'deposit' });
    expect(controller.state.depositOperatorAction).toBe(null);
  });

  test('automatically settles when the proof is ready', async () => {
    mocks.lifecycle.mockResolvedValue(proofReady);
    await connect(controller);
    await flush();

    expect(mocks.operator).toHaveBeenCalledWith({ action: 'settle', position, direction: 'deposit' });
    expect(controller.state.depositOperatorAction).toBe(null);
  });

  test('automatically advances the redeem batch too', async () => {
    mocks.findDeposit.mockResolvedValue(null);
    mocks.findRedeem.mockResolvedValue(position);
    mocks.lifecycle.mockResolvedValue(dispatchable);
    await connect(controller);
    await flush();

    expect(mocks.operator).toHaveBeenCalledWith({ action: 'dispatch', position, direction: 'redeem' });
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

  test('prepares a new batch and preserves the existing private position for another deposit', async () => {
    const nextPosition = { ...position, batchIndex: 2n };
    mocks.lifecycle.mockResolvedValue(settled);
    mocks.prepareBatch.mockResolvedValue(nextPosition);
    mocks.joinDeposit.mockResolvedValue({ ...nextPosition, amountBaseUnits: 50_000_000n });
    await connect(controller);
    await flush();

    await act(async () => {
      controller.actions.deposit(50, 'usdc');
    });
    await flush();

    expect(mocks.fund).toHaveBeenCalledWith(
      expect.anything(),
      position.batch,
      50_000_000n,
    );
    expect(mocks.prepareBatch).toHaveBeenCalledWith();
    expect(mocks.joinDeposit).toHaveBeenCalledWith(
      expect.anything(),
      50,
      expect.any(Function),
      nextPosition,
      'usdc',
      undefined,
    );
    expect(controller.state.hasPrivateShares).toBe(true);
    expect(controller.state.deposit).toEqual({
      kind: 'joined',
      result: { ...nextPosition, amountBaseUnits: 50_000_000n },
    });
  });

  test('deposits existing cUSDC without funding public USDC', async () => {
    mocks.lifecycle.mockResolvedValue(settled);
    mocks.joinDeposit.mockResolvedValue(position);
    await connect(controller);
    await flush();
    mocks.fund.mockClear();

    await act(async () => {
      controller.actions.revealUsdc();
    });
    await flush();
    await act(async () => {
      controller.actions.deposit(25, 'cusdc');
    });
    await flush();

    expect(mocks.fund).toHaveBeenCalledWith(expect.anything(), position.batch, 0n);
    expect(mocks.joinDeposit).toHaveBeenCalledWith(
      expect.anything(),
      25,
      expect.any(Function),
      position,
      'cusdc',
      '0xcurrent',
    );
  });

  test('does not join cUSDC without a revealed sufficient balance', async () => {
    mocks.lifecycle.mockResolvedValue(settled);
    mocks.joinDeposit.mockResolvedValue(position);
    await connect(controller);
    await flush();
    mocks.fund.mockClear();

    await act(async () => {
      controller.actions.deposit(25, 'cusdc');
    });
    await flush();

    expect(controller.state.deposit).toEqual({
      kind: 'error',
      message: 'Reveal your cUSDC balance before depositing.',
    });
    expect(mocks.fund).not.toHaveBeenCalled();
    expect(mocks.joinDeposit).not.toHaveBeenCalled();
  });

  test('does not keep a stale public balance when only that refresh fails', async () => {
    mocks.lifecycle.mockResolvedValue(settled);
    mocks.joinDeposit.mockResolvedValue(position);
    await connect(controller);
    await flush();
    expect(controller.state.publicUsdcBalance).toBe(1_000_000_000n);

    mocks.readBalances.mockRejectedValueOnce(new Error('public balance unavailable'));
    await act(async () => {
      controller.actions.deposit(50, 'usdc');
    });
    await flush();

    expect(controller.state.publicUsdcBalance).toBe(null);
    expect(controller.state.hasConfidentialUsdc).toBe(true);
    expect(controller.state.walletBalancesError).toBe('public balance unavailable');
  });

  test('releases the wallet mutation lock before a post-deposit balance refresh completes', async () => {
    const pendingBalance = deferred<readonly [bigint, bigint]>();
    mocks.lifecycle.mockResolvedValue(settled);
    mocks.joinDeposit.mockResolvedValue(position);
    mocks.revealShares.mockResolvedValue({ value: 100_000_000n, handle: '0x01' });
    await connect(controller);
    await flush();

    mocks.readBalances.mockReturnValueOnce(pendingBalance.promise);
    await act(async () => {
      controller.actions.deposit(50, 'usdc');
      await Promise.resolve();
      await Promise.resolve();
      await Promise.resolve();
      await Promise.resolve();
    });
    expect(controller.state.deposit.kind).toBe('joined');
    await act(async () => {
      controller.actions.revealShares();
      await Promise.resolve();
      await Promise.resolve();
    });
    await flush();

    expect(mocks.revealShares).toHaveBeenCalledTimes(1);
    await act(async () => {
      pendingBalance.resolve([5_000_000_000n, 950_000_000n]);
      await Promise.resolve();
      await Promise.resolve();
    });
  });

  test('an older wallet refresh cannot overwrite a newer one in the same session', async () => {
    const oldBalance = deferred<readonly [bigint, bigint]>();
    mocks.lifecycle.mockResolvedValue(settled);
    mocks.joinDeposit.mockResolvedValue(position);
    await connect(controller);
    await flush();

    mocks.readBalances
      .mockReturnValueOnce(oldBalance.promise)
      .mockResolvedValueOnce([5_000_000_000n, 800_000_000n]);
    await act(async () => {
      await controller.actions.deposit(50, 'usdc');
    });
    await act(async () => {
      await controller.actions.deposit(25, 'usdc');
    });
    await flush();
    expect(controller.state.publicUsdcBalance).toBe(800_000_000n);

    oldBalance.resolve([5_000_000_000n, 950_000_000n]);
    await flush();
    expect(controller.state.publicUsdcBalance).toBe(800_000_000n);
  });

  test('does not prepare a redeem batch for a known-zero share balance', async () => {
    mocks.findDeposit.mockResolvedValue(null);
    mocks.revealShares.mockResolvedValue({ value: 0n, handle: '0x01' });
    await connect(controller);
    await flush();
    mocks.prepareBatch.mockClear();

    await act(async () => {
      await controller.actions.redeem(50);
    });
    await flush();

    expect(mocks.prepareBatch).not.toHaveBeenCalled();
    expect(mocks.joinRedeem).not.toHaveBeenCalled();
    expect(controller.state.redeem).toMatchObject({ kind: 'error' });
  });

  test('does not commit a reveal that predates an automatic claim', async () => {
    const reveal = deferred<{ value: bigint; handle: string }>();
    mocks.revealShares.mockReturnValue(reveal.promise);
    mocks.lifecycle
      .mockResolvedValueOnce(awaiting)
      .mockResolvedValueOnce(settledUnclaimed)
      .mockResolvedValue(settled);
    await connect(controller);
    await flush();

    act(() => {
      void controller.actions.revealShares();
    });
    await act(async () => {
      await vi.advanceTimersByTimeAsync(2_500);
    });
    await flush();
    reveal.resolve({ value: 100_000_000n, handle: '0x01' });
    await flush();

    expect(mocks.operator).toHaveBeenCalledWith({
      action: 'claim',
      position,
      direction: 'deposit',
      user: position.batch,
    });
    expect(controller.state.revealedShares).toBe(null);
  });

  test('completes a partial redemption and leaves the next redemption available', async () => {
    mocks.findDeposit.mockResolvedValue(null);
    mocks.lifecycle.mockResolvedValue(settled);
    mocks.revealShares.mockResolvedValue({ value: 100_000_000n, handle: '0x01' });
    mocks.joinRedeem.mockResolvedValue(position);
    await connect(controller);
    await flush();

    await act(async () => {
      controller.actions.redeem(50);
    });
    await flush();

    expect(mocks.prepareBatch).toHaveBeenCalledWith('redeem');
    expect(mocks.joinRedeem).toHaveBeenCalledWith(
      expect.anything(),
      50_000_000n,
      '0x01',
      expect.any(Function),
    );
    expect(mocks.recordCompletedRedeem).toHaveBeenCalledWith(expect.anything(), position);
    expect(controller.state.redeem).toEqual({ kind: 'idle' });
    expect(controller.state.completedRedeemLifecycle).toEqual(settled);

    await act(async () => {
      controller.actions.redeem(25);
    });
    await flush();

    expect(mocks.prepareBatch).toHaveBeenCalledTimes(2);
    expect(mocks.joinRedeem).toHaveBeenCalledTimes(2);
  });

  test('restores the latest completed redemption for activity and evidence after reconnect', async () => {
    mocks.findDeposit.mockResolvedValue(null);
    mocks.findCompletedRedeem.mockResolvedValue(position);
    mocks.lifecycle.mockResolvedValue(settled);

    await connect(controller);
    await flush();

    expect(controller.state.redeem).toEqual({ kind: 'idle' });
    expect(controller.state.completedRedeemPosition).toEqual(position);
    expect(controller.state.completedRedeemLifecycle).toEqual(settled);
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

  test('an old automatic claim completion cannot clear the new generation action', async () => {
    const oldClaim = deferred<void>();
    const currentClaim = deferred<void>();
    mocks.operator.mockReturnValueOnce(oldClaim.promise).mockReturnValueOnce(currentClaim.promise);
    mocks.lifecycle.mockResolvedValue(settledUnclaimed);
    await connect(controller);
    await flush();
    expect(mocks.operator).toHaveBeenCalledWith({
      action: 'claim',
      position,
      direction: 'deposit',
      user: position.batch,
    });

    act(() => controller.actions.disconnect());
    await connect(controller);
    await flush();
    expect(controller.state.depositOperatorAction).toBe('claim');

    oldClaim.resolve();
    await flush();
    expect(controller.state.depositOperatorAction).toBe('claim');

    currentClaim.resolve();
    await flush();
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

  test('fast-forward can be repeated independently and compounds the vault metrics', async () => {
    const initial = { totalAssets: 100_000_000n, totalShares: 100_000_000n };
    const yearOne = { totalAssets: 107_000_000n, totalShares: 100_000_000n };
    const yearTwo = { totalAssets: 114_490_000n, totalShares: 100_000_000n };
    mocks.lifecycle.mockResolvedValue(settled);
    mocks.metrics.mockResolvedValue(initial);
    mocks.harvest
      .mockResolvedValueOnce({ before: initial, after: yearOne })
      .mockResolvedValueOnce({ before: yearOne, after: yearTwo });
    await connect(controller);
    await flush();

    controller.actions.fastForwardOneYear();
    await flush();
    controller.actions.fastForwardOneYear();
    await flush();

    expect(mocks.harvest).toHaveBeenCalledTimes(2);
    expect(controller.state.vaultMetrics).toEqual(yearTwo);
    expect(controller.state.harvestFromPrice).toBe(1.07);
  });

  test('coalesces concurrent fast-forward actions', async () => {
    const result = deferred<{
      before: { totalAssets: bigint; totalShares: bigint };
      after: { totalAssets: bigint; totalShares: bigint };
    }>();
    mocks.lifecycle.mockResolvedValue(settled);
    mocks.harvest.mockReturnValue(result.promise);
    await connect(controller);
    await flush();

    controller.actions.fastForwardOneYear();
    controller.actions.fastForwardOneYear();
    await flush();

    expect(mocks.harvest).toHaveBeenCalledTimes(1);
    result.resolve({
      before: { totalAssets: 100_000_000n, totalShares: 100_000_000n },
      after: { totalAssets: 107_000_000n, totalShares: 100_000_000n },
    });
    await flush();
  });

  test('refreshes vault metrics after a completed redemption', async () => {
    const afterRedeem = { totalAssets: 0n, totalShares: 0n };
    mocks.findDeposit.mockResolvedValue(null);
    mocks.lifecycle.mockResolvedValue(settled);
    mocks.revealShares.mockResolvedValue({ value: 100_000_000n, handle: '0x01' });
    mocks.joinRedeem.mockResolvedValue(position);
    mocks.metrics.mockResolvedValueOnce({ totalAssets: 100_000_000n, totalShares: 100_000_000n });
    mocks.metrics.mockResolvedValueOnce(afterRedeem);
    await connect(controller);
    await flush();

    controller.actions.redeem(50);
    await flush();

    expect(controller.state.vaultMetrics).toEqual(afterRedeem);
  });

  test('does not let a redemption refresh overwrite a newer fast-forward result', async () => {
    const staleRefresh = deferred<{ totalAssets: bigint; totalShares: bigint }>();
    const initial = { totalAssets: 100_000_000n, totalShares: 100_000_000n };
    const afterHarvest = { totalAssets: 107_000_000n, totalShares: 100_000_000n };
    mocks.findDeposit.mockResolvedValue(null);
    mocks.lifecycle.mockResolvedValue(settled);
    mocks.revealShares.mockResolvedValue({ value: 100_000_000n, handle: '0x01' });
    mocks.joinRedeem.mockResolvedValue(position);
    mocks.metrics.mockResolvedValueOnce(initial).mockReturnValueOnce(staleRefresh.promise);
    mocks.harvest.mockResolvedValue({ before: initial, after: afterHarvest });
    await connect(controller);
    await flush();

    controller.actions.redeem(50);
    await flush();
    controller.actions.fastForwardOneYear();
    await flush();
    expect(controller.state.vaultMetrics).toEqual(afterHarvest);

    staleRefresh.resolve({ totalAssets: 50_000_000n, totalShares: 50_000_000n });
    await flush();
    expect(controller.state.vaultMetrics).toEqual(afterHarvest);
  });

  test('clears the private position after a completed full redemption', async () => {
    mocks.findDeposit.mockResolvedValue(null);
    mocks.lifecycle.mockResolvedValue(settled);
    mocks.revealShares.mockResolvedValue({ value: 100_000_000n, handle: '0x01' });
    mocks.joinRedeem.mockResolvedValue(position);
    await connect(controller);
    await flush();

    controller.actions.redeem(100);
    await flush();

    expect(controller.state.hasPrivateShares).toBe(false);
    expect(controller.state.vaultMetrics).toBe(null);
    expect(controller.state.redeemPercentage).toBe(null);
  });
});
