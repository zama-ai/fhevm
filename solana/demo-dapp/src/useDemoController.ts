import { useCallback, useEffect, useReducer, useRef } from 'react';

import type { BatchLifecycle, BatchPosition, OperatorAction, VaultDirection, VaultMetrics } from './batchTypes';
import { claimBatchPayout } from './claim';
import { connectDemoSession, describeWalletError, ensureDemoFunding, type DemoSession } from './demoSession';
import {
  depositToVault,
  findExistingDeposit,
  hasClaimedDeposit,
  usdcToBaseUnits,
  type DepositStage,
} from './deposit';
import { withWalletMutationLock } from './mutationLock';
import { prepareDemoDepositBatch, runDemoOperatorAction } from './operatorClient';
import { findExistingRedeem, joinRedeemBatch, type RedeemStage } from './redeem';
import { revealClaimedShares, revealClaimedUsdc, type RevealedBalance } from './revealShares';
import { readVaultLifecycle } from './settlement';
import { harvestDemoVault, readDemoVaultMetrics } from './vaultYield';

export type ConnectionState =
  | { readonly kind: 'disconnected' }
  | { readonly kind: 'connecting' }
  | { readonly kind: 'ready'; readonly session: DemoSession }
  | { readonly kind: 'error'; readonly message: string };

export type JoinState<TStage extends string> =
  | { readonly kind: 'idle' }
  | { readonly kind: 'running'; readonly stage: TStage }
  | { readonly kind: 'joined'; readonly result: BatchPosition }
  | { readonly kind: 'error'; readonly message: string };

export type DemoState = {
  readonly generation: number;
  readonly connection: ConnectionState;
  readonly deposit: JoinState<DepositStage>;
  readonly depositLifecycle: BatchLifecycle | null;
  readonly depositLifecycleError: string | null;
  readonly depositOperatorAction: OperatorAction | null;
  readonly depositOperatorError: string | null;
  readonly depositClaiming: boolean;
  readonly depositClaimError: string | null;
  readonly hasPrivateShares: boolean;
  readonly revealedShares: RevealedBalance | null;
  readonly revealingShares: boolean;
  readonly revealSharesError: string | null;
  readonly vaultMetrics: VaultMetrics | null;
  readonly harvesting: boolean;
  readonly harvestError: string | null;
  readonly harvestFromPrice: number | null;
  readonly redeem: JoinState<'decrypting' | RedeemStage>;
  readonly redeemLifecycle: BatchLifecycle | null;
  readonly redeemOperatorAction: OperatorAction | null;
  readonly redeemOperatorError: string | null;
  readonly redeemClaiming: boolean;
  readonly redeemClaimError: string | null;
  readonly revealedUsdc: RevealedBalance | null;
  readonly revealingUsdc: boolean;
  readonly revealUsdcError: string | null;
};

const accountState = {
  deposit: { kind: 'idle' },
  depositLifecycle: null,
  depositLifecycleError: null,
  depositOperatorAction: null,
  depositOperatorError: null,
  depositClaiming: false,
  depositClaimError: null,
  hasPrivateShares: false,
  revealedShares: null,
  revealingShares: false,
  revealSharesError: null,
  vaultMetrics: null,
  harvesting: false,
  harvestError: null,
  harvestFromPrice: null,
  redeem: { kind: 'idle' },
  redeemLifecycle: null,
  redeemOperatorAction: null,
  redeemOperatorError: null,
  redeemClaiming: false,
  redeemClaimError: null,
  revealedUsdc: null,
  revealingUsdc: false,
  revealUsdcError: null,
} as const satisfies Omit<DemoState, 'connection' | 'generation'>;

export const initialDemoState: DemoState = {
  generation: 0,
  connection: { kind: 'disconnected' },
  ...accountState,
};

export type DemoStateAction =
  | { readonly type: 'reset'; readonly generation: number; readonly connection: ConnectionState }
  | { readonly type: 'update'; readonly generation: number; readonly patch: Partial<DemoState> };

export const demoStateReducer = (state: DemoState, action: DemoStateAction): DemoState => {
  if (action.type === 'reset') return { generation: action.generation, connection: action.connection, ...accountState };
  if (action.generation !== state.generation) return state;
  return { ...state, ...action.patch };
};

const DEPOSIT_AMOUNT_USDC = 100;

const errorMessage = (error: unknown): string => (error instanceof Error ? error.message : String(error));

export function useDemoController() {
  const [state, dispatch] = useReducer(demoStateReducer, initialDemoState);
  const operationInFlight = useRef(false);
  const operatorGeneration = useRef<Record<VaultDirection, number | null>>({ deposit: null, redeem: null });
  const sessionGeneration = useRef(0);
  const currentDepositClaimed = state.depositLifecycle?.kind === 'settled' && state.depositLifecycle.claimed;
  const commit = useCallback(
    (generation: number, patch: Partial<DemoState>) => dispatch({ type: 'update', generation, patch }),
    [],
  );
  const finishOperation = useCallback((generation: number) => {
    if (sessionGeneration.current === generation) operationInFlight.current = false;
  }, []);

  const advanceOperator = useCallback(
    async (
      session: DemoSession,
      position: BatchPosition,
      direction: VaultDirection,
      action: OperatorAction,
      generation: number,
    ) => {
      if (
        sessionGeneration.current !== generation ||
        operatorGeneration.current[direction] === generation
      ) {
        return;
      }
      operatorGeneration.current[direction] = generation;
      const actionKey = direction === 'deposit' ? 'depositOperatorAction' : 'redeemOperatorAction';
      const errorKey = direction === 'deposit' ? 'depositOperatorError' : 'redeemOperatorError';
      const lifecycleKey = direction === 'deposit' ? 'depositLifecycle' : 'redeemLifecycle';
      commit(generation, { [actionKey]: action, [errorKey]: null });
      let actionError: string | null = null;
      try {
        await runDemoOperatorAction(action, position, direction);
      } catch (error) {
        actionError = errorMessage(error);
      } finally {
        if (sessionGeneration.current === generation) {
          try {
            const next = await readVaultLifecycle(session, position, direction);
            session.assertActive();
            const completed =
              action === 'dispatch'
                ? next.kind !== 'awaiting-dispatch'
                : next.kind === 'settled' || next.kind === 'canceled';
            commit(generation, {
              [lifecycleKey]: next,
              [errorKey]: completed ? null : actionError,
              [actionKey]: null,
            });
          } catch (error) {
            commit(generation, {
              [errorKey]: actionError ?? errorMessage(error),
              [actionKey]: null,
            });
          }
        }
        if (operatorGeneration.current[direction] === generation) {
          operatorGeneration.current[direction] = null;
        }
      }
    },
    [commit],
  );

  useEffect(() => {
    if (state.connection.kind !== 'ready' || state.deposit.kind !== 'joined') {
      commit(state.generation, { depositLifecycle: null });
      return;
    }
    const session = state.connection.session;
    const position = state.deposit.result;
    const generation = state.generation;
    let canceled = false;
    let timeout: number | undefined;
    const refresh = async () => {
      try {
        const next = await readVaultLifecycle(session, position, 'deposit');
        if (!canceled) {
          commit(generation, { depositLifecycle: next, depositLifecycleError: null });
          if (next.kind === 'awaiting-dispatch' && next.remainingSlots === 0n) {
            await advanceOperator(session, position, 'deposit', 'dispatch', generation);
          } else if (next.kind === 'proving' && next.proofReady) {
            await advanceOperator(session, position, 'deposit', 'settle', generation);
          }
        }
      } catch (error) {
        if (!canceled) commit(generation, { depositLifecycleError: errorMessage(error) });
      } finally {
        if (!canceled) timeout = window.setTimeout(() => void refresh(), 2_500);
      }
    };
    void refresh();
    return () => {
      canceled = true;
      if (timeout !== undefined) window.clearTimeout(timeout);
    };
  }, [advanceOperator, commit, state.connection, state.deposit, state.generation]);

  useEffect(() => {
    if (state.connection.kind !== 'ready' || state.redeem.kind !== 'joined') {
      commit(state.generation, { redeemLifecycle: null });
      return;
    }
    const session = state.connection.session;
    const position = state.redeem.result;
    const generation = state.generation;
    let canceled = false;
    let timeout: number | undefined;
    const refresh = async () => {
      try {
        const next = await readVaultLifecycle(session, position, 'redeem');
        if (!canceled) {
          commit(generation, { redeemLifecycle: next, redeemOperatorError: null });
          if (next.kind === 'awaiting-dispatch' && next.remainingSlots === 0n) {
            await advanceOperator(session, position, 'redeem', 'dispatch', generation);
          } else if (next.kind === 'proving' && next.proofReady) {
            await advanceOperator(session, position, 'redeem', 'settle', generation);
          }
        }
      } catch (error) {
        if (!canceled) commit(generation, { redeemOperatorError: errorMessage(error) });
      } finally {
        if (!canceled) timeout = window.setTimeout(() => void refresh(), 2_500);
      }
    };
    void refresh();
    return () => {
      canceled = true;
      if (timeout !== undefined) window.clearTimeout(timeout);
    };
  }, [advanceOperator, commit, state.connection, state.generation, state.redeem]);

  useEffect(() => {
    if (!state.hasPrivateShares) {
      commit(state.generation, { vaultMetrics: null });
      return;
    }
    let canceled = false;
    const generation = state.generation;
    void readDemoVaultMetrics()
      .then((vaultMetrics) => {
        if (!canceled) commit(generation, { vaultMetrics });
      })
      .catch((error) => {
        if (!canceled) commit(generation, { harvestError: errorMessage(error) });
      });
    return () => {
      canceled = true;
    };
  }, [commit, state.generation, state.hasPrivateShares]);

  const disconnect = useCallback(() => {
    const generation = sessionGeneration.current + 1;
    sessionGeneration.current = generation;
    operationInFlight.current = false;
    dispatch({ type: 'reset', generation, connection: { kind: 'disconnected' } });
  }, []);

  const connect = async (createSession: (isActive: () => boolean) => Promise<DemoSession>) => {
    if (operationInFlight.current) return;
    operationInFlight.current = true;
    const generation = sessionGeneration.current + 1;
    sessionGeneration.current = generation;
    const isActive = () => sessionGeneration.current === generation;
    dispatch({ type: 'reset', generation, connection: { kind: 'connecting' } });
    try {
      const session = await createSession(isActive);
      session.assertActive();
      commit(generation, { deposit: { kind: 'running', stage: 'preparing' } });
      const existingDeposit = await findExistingDeposit(session);
      session.assertActive();
      const hasPrivateShares = await hasClaimedDeposit(session);
      session.assertActive();
      commit(generation, {
        deposit: existingDeposit === null ? { kind: 'idle' } : { kind: 'joined', result: existingDeposit },
        hasPrivateShares,
      });
      try {
        const existingRedeem = await findExistingRedeem(session);
        session.assertActive();
        commit(generation, {
          redeem: existingRedeem === null ? { kind: 'idle' } : { kind: 'joined', result: existingRedeem },
        });
      } catch (error) {
        commit(generation, { redeem: { kind: 'error', message: errorMessage(error) } });
      }
      session.assertActive();
      commit(generation, { connection: { kind: 'ready', session } });
    } catch (error) {
      if (isActive()) {
        commit(generation, {
          connection: { kind: 'error', message: describeWalletError(error, 'connect') },
          deposit: { kind: 'idle' },
        });
      }
    } finally {
      finishOperation(generation);
    }
  };

  const claim = async (direction: VaultDirection) => {
    const journey = direction === 'deposit' ? state.deposit : state.redeem;
    const lifecycle = direction === 'deposit' ? state.depositLifecycle : state.redeemLifecycle;
    if (
      state.connection.kind !== 'ready' ||
      journey.kind !== 'joined' ||
      lifecycle?.kind !== 'settled' ||
      lifecycle.claimed ||
      operationInFlight.current
    ) {
      return;
    }
    const session = state.connection.session;
    const generation = state.generation;
    const claimingKey = direction === 'deposit' ? 'depositClaiming' : 'redeemClaiming';
    const errorKey = direction === 'deposit' ? 'depositClaimError' : 'redeemClaimError';
    const lifecycleKey = direction === 'deposit' ? 'depositLifecycle' : 'redeemLifecycle';
    operationInFlight.current = true;
    commit(generation, { [claimingKey]: true, [errorKey]: null });
    let actionError: string | null = null;
    try {
      await withWalletMutationLock(session, () => claimBatchPayout(session, journey.result, direction));
    } catch (error) {
      actionError = describeWalletError(error, 'transaction');
    } finally {
      if (sessionGeneration.current === generation) {
        try {
          const next = await readVaultLifecycle(session, journey.result, direction);
          session.assertActive();
          commit(generation, {
            [lifecycleKey]: next,
            [errorKey]: next.kind === 'settled' && next.claimed ? null : actionError,
            [claimingKey]: false,
            ...(direction === 'deposit' && next.kind === 'settled' && next.claimed
              ? { hasPrivateShares: true, revealedShares: null }
              : {}),
          });
        } catch {
          commit(generation, { [errorKey]: actionError, [claimingKey]: false });
        }
      }
      finishOperation(generation);
    }
  };

  const shieldAndDeposit = async (amount: number) => {
    if (
      state.connection.kind !== 'ready' ||
      operationInFlight.current ||
      (state.deposit.kind === 'joined' && !currentDepositClaimed)
    ) {
      return;
    }
    const session = state.connection.session;
    const generation = state.generation;
    operationInFlight.current = true;
    commit(generation, {
      deposit: { kind: 'running', stage: 'preparing' },
      depositLifecycle: null,
      depositClaimError: null,
      revealSharesError: null,
    });
    try {
      await ensureDemoFunding(session.config, session.signer.address, usdcToBaseUnits(amount));
      session.assertActive();
      const target = await prepareDemoDepositBatch();
      session.assertActive();
      const result = await withWalletMutationLock(session, () =>
        depositToVault(session, amount, (stage) => {
          commit(generation, { deposit: { kind: 'running', stage } });
        }, target),
      );
      session.assertActive();
      commit(generation, { deposit: { kind: 'joined', result } });
    } catch (error) {
      commit(generation, { deposit: { kind: 'error', message: describeWalletError(error, 'transaction') } });
    } finally {
      finishOperation(generation);
    }
  };

  const revealShares = async () => {
    if (
      state.connection.kind !== 'ready' ||
      !state.hasPrivateShares ||
      operationInFlight.current
    )
      return;
    const session = state.connection.session;
    const generation = state.generation;
    operationInFlight.current = true;
    commit(generation, { revealingShares: true, revealSharesError: null });
    try {
      const revealedShares = await revealClaimedShares(session);
      session.assertActive();
      commit(generation, { revealedShares });
    } catch (error) {
      commit(generation, { revealSharesError: describeWalletError(error, 'reveal') });
    } finally {
      commit(generation, { revealingShares: false });
      finishOperation(generation);
    }
  };

  const fastForwardOneYear = async () => {
    if (state.harvesting) return;
    const generation = state.generation;
    commit(generation, {
      harvesting: true,
      harvestError: null,
    });
    try {
      const metrics = state.vaultMetrics ?? (await readDemoVaultMetrics());
      if (metrics.totalShares === 0n) throw new Error('The vault has no shares to accrue yield to');
      const result = await harvestDemoVault();
      commit(generation, {
        vaultMetrics: result.after,
        harvestFromPrice: Number(result.before.totalAssets) / Number(result.before.totalShares),
      });
    } catch (error) {
      commit(generation, { harvestFromPrice: null, harvestError: errorMessage(error) });
    } finally {
      commit(generation, { harvesting: false });
    }
  };

  const redeemPosition = async (percentage: number) => {
    if (state.connection.kind !== 'ready' || operationInFlight.current || state.redeem.kind === 'running') return;
    if (!Number.isInteger(percentage) || percentage < 1 || percentage > 100) return;
    const session = state.connection.session;
    const generation = state.generation;
    operationInFlight.current = true;
    commit(generation, { redeem: { kind: 'running', stage: 'decrypting' } });
    try {
      const shares = state.revealedShares ?? (await revealClaimedShares(session));
      session.assertActive();
      commit(generation, { revealedShares: null });
      const amount = (shares.value * BigInt(percentage)) / 100n;
      if (amount === 0n) throw new Error(`The private share balance is too small to redeem ${percentage}%`);
      const result = await withWalletMutationLock(session, () =>
        joinRedeemBatch(session, amount, shares.handle, (stage) => {
          commit(generation, { redeem: { kind: 'running', stage } });
        }),
      );
      session.assertActive();
      commit(generation, { redeem: { kind: 'joined', result } });
    } catch (error) {
      commit(generation, { redeem: { kind: 'error', message: describeWalletError(error, 'transaction') } });
    } finally {
      finishOperation(generation);
    }
  };

  const revealRedeemedUsdc = async () => {
    if (state.connection.kind !== 'ready' || operationInFlight.current) return;
    const session = state.connection.session;
    const generation = state.generation;
    operationInFlight.current = true;
    commit(generation, { revealingUsdc: true, revealUsdcError: null });
    try {
      const revealedUsdc = await revealClaimedUsdc(session);
      session.assertActive();
      commit(generation, { revealedUsdc });
    } catch (error) {
      commit(generation, { revealUsdcError: describeWalletError(error, 'reveal') });
    } finally {
      commit(generation, { revealingUsdc: false });
      finishOperation(generation);
    }
  };

  const sharePrice =
    state.vaultMetrics === null || state.vaultMetrics.totalShares === 0n
      ? null
      : Number(state.vaultMetrics.totalAssets) / Number(state.vaultMetrics.totalShares);

  return {
    state,
    derived: {
      connected: state.connection.kind === 'ready',
      depositRunning: state.deposit.kind === 'running',
      depositJoined: state.deposit.kind === 'joined',
      depositSettled: state.depositLifecycle?.kind === 'settled',
      sharesClaimed: currentDepositClaimed,
      hasPrivateShares: state.hasPrivateShares,
      sharePrice,
      redeemJoined: state.redeem.kind === 'joined',
      redeemSettled: state.redeemLifecycle?.kind === 'settled',
    },
    actions: {
      connect,
      connectBurner: () => void connect((isActive) => connectDemoSession(isActive)),
      disconnect,
      shieldAndDeposit: (amount: number = DEPOSIT_AMOUNT_USDC) => void shieldAndDeposit(amount),
      revealShares: () => void revealShares(),
      hideShares: () => commit(state.generation, { revealedShares: null }),
      fastForwardOneYear: () => void fastForwardOneYear(),
      redeem: (percentage: number) => void redeemPosition(percentage),
      revealRedeemedUsdc: () => void revealRedeemedUsdc(),
      hideRedeemedUsdc: () => commit(state.generation, { revealedUsdc: null }),
      claim: (direction: VaultDirection) => void claim(direction),
    },
  };
}

export type DemoController = ReturnType<typeof useDemoController>;
