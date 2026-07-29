import { useCallback, useEffect, useReducer, useRef } from 'react';

import type { BatchLifecycle, BatchPosition, OperatorAction, VaultDirection, VaultMetrics } from './batchTypes';
import {
  connectDemoSession,
  describeWalletError,
  ensureDemoFunding,
  readDemoWalletBalances,
  type DemoSession,
} from './demoSession';
import {
  depositToVault,
  findExistingDeposit,
  hasClaimedDeposit,
  usdcToBaseUnits,
  type DepositStage,
} from './deposit';
import { recordTransactionEvidence } from './evidenceStore';
import { withWalletMutationLock } from './mutationLock';
import { prepareDemoBatch, prepareDemoDepositBatch, runDemoOperatorAction } from './operatorClient';
import {
  findCompletedRedeem,
  findExistingRedeem,
  joinRedeemBatch,
  recordCompletedRedeem,
  type RedeemStage,
} from './redeem';
import {
  hasConfidentialBalanceAccount,
  revealClaimedShares,
  revealClaimedUsdc,
  type RevealedBalance,
} from './revealShares';
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
  readonly hasPrivateShares: boolean;
  readonly hasConfidentialUsdc: boolean | null;
  readonly hasConfidentialShares: boolean | null;
  readonly publicUsdcBalance: bigint | null;
  readonly walletBalancesError: string | null;
  readonly revealedShares: RevealedBalance | null;
  readonly revealingShares: boolean;
  readonly revealSharesError: string | null;
  readonly vaultMetrics: VaultMetrics | null;
  readonly harvesting: boolean;
  readonly harvestError: string | null;
  readonly harvestFromPrice: number | null;
  readonly redeem: JoinState<'decrypting' | RedeemStage>;
  readonly redeemPercentage: number | null;
  readonly redeemLifecycle: BatchLifecycle | null;
  readonly completedRedeemLifecycle: Extract<BatchLifecycle, { readonly kind: 'settled' }> | null;
  readonly completedRedeemPosition: BatchPosition | null;
  readonly redeemOperatorAction: OperatorAction | null;
  readonly redeemOperatorError: string | null;
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
  hasPrivateShares: false,
  hasConfidentialUsdc: null,
  hasConfidentialShares: null,
  publicUsdcBalance: null,
  walletBalancesError: null,
  revealedShares: null,
  revealingShares: false,
  revealSharesError: null,
  vaultMetrics: null,
  harvesting: false,
  harvestError: null,
  harvestFromPrice: null,
  redeem: { kind: 'idle' },
  redeemPercentage: null,
  redeemLifecycle: null,
  completedRedeemLifecycle: null,
  completedRedeemPosition: null,
  redeemOperatorAction: null,
  redeemOperatorError: null,
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
  const harvestInFlight = useRef(false);
  const metricsRequestGeneration = useRef(0);
  const walletBalancesRequestGeneration = useRef(0);
  const privateBalanceMutationGeneration = useRef(0);
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
  const refreshVaultMetrics = useCallback(
    async (generation: number) => {
      const requestGeneration = ++metricsRequestGeneration.current;
      try {
        const vaultMetrics = await readDemoVaultMetrics();
        if (metricsRequestGeneration.current === requestGeneration) {
          commit(generation, { vaultMetrics, harvestError: null });
        }
      } catch (error) {
        if (metricsRequestGeneration.current === requestGeneration) {
          commit(generation, { harvestError: errorMessage(error) });
        }
      }
    },
    [commit],
  );
  const refreshWalletBalances = useCallback(
    async (session: DemoSession, generation: number) => {
      const requestGeneration = ++walletBalancesRequestGeneration.current;
      const isCurrent = () =>
        session.isActive() && walletBalancesRequestGeneration.current === requestGeneration;
      commit(generation, {
        publicUsdcBalance: null,
        hasConfidentialUsdc: null,
        hasConfidentialShares: null,
        walletBalancesError: null,
      });
      const errors: string[] = [];
      const fail = (patch: Partial<DemoState>, error: unknown) => {
        if (!isCurrent()) return;
        errors.push(errorMessage(error));
        commit(generation, { ...patch, walletBalancesError: errors.join(' · ') });
      };
      await Promise.all([
        readDemoWalletBalances(session.config, session.signer.address)
          .then(([, publicUsdcBalance]) => {
            if (isCurrent()) commit(generation, { publicUsdcBalance });
          })
          .catch((error: unknown) => fail({ publicUsdcBalance: null }, error)),
        hasConfidentialBalanceAccount(session, session.config.mints.joinConfidential)
          .then((hasConfidentialUsdc) => {
            if (isCurrent()) commit(generation, { hasConfidentialUsdc });
          })
          .catch((error: unknown) => fail({ hasConfidentialUsdc: null }, error)),
        hasConfidentialBalanceAccount(session, session.config.mints.payoutConfidential)
          .then((hasConfidentialShares) => {
            if (isCurrent()) commit(generation, { hasConfidentialShares });
          })
          .catch((error: unknown) => fail({ hasConfidentialShares: null }, error)),
      ]);
    },
    [commit],
  );

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
      if (action === 'claim') {
        privateBalanceMutationGeneration.current += 1;
        commit(generation, direction === 'deposit' ? { revealedShares: null } : { revealedUsdc: null });
      }
      commit(generation, { [actionKey]: action, [errorKey]: null });
      let actionError: string | null = null;
      try {
        const signature =
          action === 'claim'
            ? await runDemoOperatorAction({ action, position, direction, user: session.signer.address })
            : await runDemoOperatorAction({ action, position, direction });
        if (typeof signature === 'string') {
          recordTransactionEvidence(session, {
            label: `${direction === 'deposit' ? 'Deposit' : 'Redeem'} ${action}`,
            signature,
          });
        }
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
                : action === 'settle'
                  ? next.kind === 'settled' || next.kind === 'canceled'
                  : next.kind === 'settled' && next.claimed;
            commit(generation, {
              [lifecycleKey]: next,
              [errorKey]: completed ? null : actionError,
              [actionKey]: null,
              ...(direction === 'deposit' && next.kind === 'settled' && next.claimed
                ? { hasPrivateShares: true, revealedShares: null }
                : {}),
            });
            if (action === 'claim' && completed) {
              void refreshWalletBalances(session, generation);
            }
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
    [commit, refreshWalletBalances],
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
          } else if (next.kind === 'settled' && !next.claimed) {
            await advanceOperator(session, position, 'deposit', 'claim', generation);
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
          if (next.kind === 'settled' && next.claimed) {
            recordCompletedRedeem(session, position);
            commit(generation, {
              redeem: { kind: 'idle' },
              redeemPercentage: null,
              redeemLifecycle: null,
              completedRedeemLifecycle: next,
              completedRedeemPosition: position,
              redeemOperatorError: null,
              revealedUsdc: null,
              revealUsdcError: null,
              ...(state.redeemPercentage === 100
                ? { hasPrivateShares: false, revealedShares: null, vaultMetrics: null }
                : {}),
            });
            if (state.redeemPercentage !== 100) void refreshVaultMetrics(generation);
            return;
          }
          commit(generation, { redeemLifecycle: next, redeemOperatorError: null });
          if (next.kind === 'awaiting-dispatch' && next.remainingSlots === 0n) {
            await advanceOperator(session, position, 'redeem', 'dispatch', generation);
          } else if (next.kind === 'proving' && next.proofReady) {
            await advanceOperator(session, position, 'redeem', 'settle', generation);
          } else if (next.kind === 'settled' && !next.claimed) {
            await advanceOperator(session, position, 'redeem', 'claim', generation);
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
  }, [advanceOperator, commit, refreshVaultMetrics, state.connection, state.generation, state.redeem, state.redeemPercentage]);

  useEffect(() => {
    if (!state.hasPrivateShares) {
      commit(state.generation, { vaultMetrics: null });
      return;
    }
    const generation = state.generation;
    void refreshVaultMetrics(generation);
  }, [commit, refreshVaultMetrics, state.generation, state.hasPrivateShares]);

  const disconnect = useCallback(() => {
    const generation = sessionGeneration.current + 1;
    sessionGeneration.current = generation;
    operationInFlight.current = false;
    harvestInFlight.current = false;
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
      await refreshWalletBalances(session, generation);
      session.assertActive();
      try {
        const existingRedeem = await findExistingRedeem(session);
        session.assertActive();
        const completedRedeemPosition = await findCompletedRedeem(session);
        session.assertActive();
        const completedRedeemLifecycle =
          completedRedeemPosition === null
            ? null
            : await readVaultLifecycle(session, completedRedeemPosition, 'redeem');
        session.assertActive();
        commit(generation, {
          redeem: existingRedeem === null ? { kind: 'idle' } : { kind: 'joined', result: existingRedeem },
          completedRedeemPosition,
          completedRedeemLifecycle:
            completedRedeemLifecycle?.kind === 'settled' && completedRedeemLifecycle.claimed
              ? completedRedeemLifecycle
              : null,
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
      revealSharesError: null,
      revealedUsdc: null,
      revealUsdcError: null,
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
      void refreshWalletBalances(session, generation);
    }
  };

  const revealShares = async () => {
    if (
      state.connection.kind !== 'ready' ||
      state.hasConfidentialShares !== true ||
      operationInFlight.current
    )
      return;
    const session = state.connection.session;
    const generation = state.generation;
    const mutationGeneration = privateBalanceMutationGeneration.current;
    operationInFlight.current = true;
    commit(generation, { revealingShares: true, revealSharesError: null });
    try {
      const revealedShares = await revealClaimedShares(session);
      session.assertActive();
      if (privateBalanceMutationGeneration.current === mutationGeneration) {
        commit(generation, { revealedShares });
      }
    } catch (error) {
      commit(generation, { revealSharesError: describeWalletError(error, 'reveal') });
    } finally {
      commit(generation, { revealingShares: false });
      finishOperation(generation);
    }
  };

  const fastForwardOneYear = async () => {
    if (harvestInFlight.current) return;
    const generation = state.generation;
    const metricsRequest = ++metricsRequestGeneration.current;
    harvestInFlight.current = true;
    commit(generation, {
      harvesting: true,
      harvestError: null,
    });
    try {
      const metrics = state.vaultMetrics ?? (await readDemoVaultMetrics());
      if (metrics.totalShares === 0n) throw new Error('The vault has no shares to accrue yield to');
      const result = await harvestDemoVault();
      if (metricsRequestGeneration.current === metricsRequest) {
        commit(generation, {
          vaultMetrics: result.after,
          harvestFromPrice: Number(result.before.totalAssets) / Number(result.before.totalShares),
        });
      }
    } catch (error) {
      commit(generation, { harvestFromPrice: null, harvestError: errorMessage(error) });
    } finally {
      commit(generation, { harvesting: false });
      if (sessionGeneration.current === generation) harvestInFlight.current = false;
    }
  };

  const redeemPosition = async (percentage: number) => {
    if (state.connection.kind !== 'ready' || operationInFlight.current || state.redeem.kind === 'running') return;
    if (!Number.isInteger(percentage) || percentage < 1 || percentage > 100) return;
    const session = state.connection.session;
    const generation = state.generation;
    operationInFlight.current = true;
    commit(generation, {
      redeem: { kind: 'running', stage: 'decrypting' },
      redeemPercentage: percentage,
      revealedUsdc: null,
      revealUsdcError: null,
    });
    try {
      const shares = state.revealedShares ?? (await revealClaimedShares(session));
      session.assertActive();
      commit(generation, { revealedShares: null });
      const amount = (shares.value * BigInt(percentage)) / 100n;
      if (amount === 0n) throw new Error(`The private share balance is too small to redeem ${percentage}%`);
      await prepareDemoBatch('redeem');
      session.assertActive();
      const result = await withWalletMutationLock(session, () =>
        joinRedeemBatch(session, amount, shares.handle, (stage) => {
          commit(generation, { redeem: { kind: 'running', stage } });
        }),
      );
      session.assertActive();
      commit(generation, { redeem: { kind: 'joined', result } });
    } catch (error) {
      commit(generation, {
        redeem: { kind: 'error', message: describeWalletError(error, 'transaction') },
        redeemPercentage: null,
      });
    } finally {
      finishOperation(generation);
    }
  };

  const revealUsdc = async () => {
    if (state.connection.kind !== 'ready' || operationInFlight.current) return;
    const session = state.connection.session;
    const generation = state.generation;
    const mutationGeneration = privateBalanceMutationGeneration.current;
    operationInFlight.current = true;
    commit(generation, { revealingUsdc: true, revealUsdcError: null });
    try {
      const revealedUsdc = await revealClaimedUsdc(session);
      session.assertActive();
      if (privateBalanceMutationGeneration.current === mutationGeneration) {
        commit(generation, { revealedUsdc });
      }
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
      hasConfidentialShares: state.hasConfidentialShares,
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
      revealUsdc: () => void revealUsdc(),
      hideUsdc: () => commit(state.generation, { revealedUsdc: null }),
    },
  };
}

export type DemoController = ReturnType<typeof useDemoController>;
