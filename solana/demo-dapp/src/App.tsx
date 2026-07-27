import { useCallback, useEffect, useRef, useState } from 'react';

import { claimBatchPayout } from './claim';
import { connectDemoSession, describeWalletError, type DemoSession } from './demoSession';
import { depositToVault, findExistingDeposit, type DepositResult, type DepositStage } from './deposit';
import { runDemoOperatorAction } from './operatorClient';
import { findExistingRedeem, joinRedeemBatch, type RedeemStage } from './redeem';
import { revealClaimedShares, revealClaimedUsdc, type RevealedBalance } from './revealShares';
import { readVaultLifecycle, type DepositLifecycle } from './settlement';
import { harvestDemoVault, readDemoVaultMetrics, type DemoVaultMetrics } from './vaultYield';
import { WalletControl } from './WalletControl';
import { withWalletMutationLock } from './mutationLock';

type ConnectionState =
  | { readonly kind: 'disconnected' }
  | { readonly kind: 'connecting' }
  | { readonly kind: 'ready'; readonly session: DemoSession }
  | { readonly kind: 'error'; readonly message: string };

type DepositState =
  | { readonly kind: 'idle' }
  | { readonly kind: 'running'; readonly stage: DepositStage }
  | { readonly kind: 'joined'; readonly result: DepositResult }
  | { readonly kind: 'error'; readonly message: string };

type RedeemState =
  | { readonly kind: 'idle' }
  | { readonly kind: 'running'; readonly stage: 'decrypting' | RedeemStage }
  | { readonly kind: 'joined'; readonly result: DepositResult }
  | { readonly kind: 'error'; readonly message: string };

const DEPOSIT_AMOUNT_USDC = 100;

const depositStageCopy: Record<DepositStage, string> = {
  preparing: 'Checking your confidential account…',
  shielding: 'Transaction 1 of 2 · Shielding 100 USDC…',
  proving: 'Creating your private deposit proof…',
  joining: 'Transaction 2 of 2 · Joining the batch…',
  joined: 'Deposit joined · Waiting for private settlement',
};

const redeemStageCopy: Record<'decrypting' | RedeemStage, string> = {
  decrypting: 'Authorizing a one-time balance reveal…',
  proving: 'Creating your private 50% redeem proof…',
  joining: 'Signing one private redeem transaction…',
  joined: 'Redemption joined · Waiting for private settlement',
};

const formatUsdc = (baseUnits: bigint): string =>
  new Intl.NumberFormat('en-US', { maximumFractionDigits: 6 }).format(Number(baseUnits) / 1_000_000);

export function App() {
  const [connection, setConnection] = useState<ConnectionState>({ kind: 'disconnected' });
  const [deposit, setDeposit] = useState<DepositState>({ kind: 'idle' });
  const [lifecycle, setLifecycle] = useState<DepositLifecycle | null>(null);
  const [operatorAction, setOperatorAction] = useState<'dispatch' | 'settle' | null>(null);
  const [operatorError, setOperatorError] = useState<string | null>(null);
  const [lifecycleError, setLifecycleError] = useState<string | null>(null);
  const [claiming, setClaiming] = useState(false);
  const [claimError, setClaimError] = useState<string | null>(null);
  const [revealedShares, setRevealedShares] = useState<RevealedBalance | null>(null);
  const [revealingShares, setRevealingShares] = useState(false);
  const [revealError, setRevealError] = useState<string | null>(null);
  const [vaultMetrics, setVaultMetrics] = useState<DemoVaultMetrics | null>(null);
  const [harvesting, setHarvesting] = useState(false);
  const [harvestError, setHarvestError] = useState<string | null>(null);
  const [harvestFromPrice, setHarvestFromPrice] = useState<number | null>(null);
  const [redeem, setRedeem] = useState<RedeemState>({ kind: 'idle' });
  const [redeemLifecycle, setRedeemLifecycle] = useState<DepositLifecycle | null>(null);
  const [redeemOperatorAction, setRedeemOperatorAction] = useState<'dispatch' | 'settle' | null>(null);
  const [redeemOperatorError, setRedeemOperatorError] = useState<string | null>(null);
  const [claimingRedeem, setClaimingRedeem] = useState(false);
  const [redeemClaimError, setRedeemClaimError] = useState<string | null>(null);
  const [revealedUsdc, setRevealedUsdc] = useState<RevealedBalance | null>(null);
  const [revealingUsdc, setRevealingUsdc] = useState(false);
  const [revealUsdcError, setRevealUsdcError] = useState<string | null>(null);
  const operationInFlight = useRef(false);
  const sessionGeneration = useRef(0);
  const sharesClaimed = lifecycle?.kind === 'settled' && lifecycle.claimed;

  useEffect(() => {
    if (connection.kind !== 'ready' || deposit.kind !== 'joined') {
      setLifecycle(null);
      return;
    }
    let canceled = false;
    let timeout: number | undefined;
    const refresh = async () => {
      try {
        const next = await readVaultLifecycle(connection.session, deposit.result, 'deposit');
        if (!canceled) {
          setLifecycle(next);
          setLifecycleError(null);
        }
      } catch (error) {
        if (!canceled) setLifecycleError(error instanceof Error ? error.message : String(error));
      } finally {
        if (!canceled) timeout = window.setTimeout(() => void refresh(), 2_500);
      }
    };
    void refresh();
    return () => {
      canceled = true;
      if (timeout !== undefined) window.clearTimeout(timeout);
    };
  }, [connection, deposit]);

  useEffect(() => {
    if (connection.kind !== 'ready' || redeem.kind !== 'joined') {
      setRedeemLifecycle(null);
      return;
    }
    let canceled = false;
    let timeout: number | undefined;
    const refresh = async () => {
      try {
        const next = await readVaultLifecycle(connection.session, redeem.result, 'redeem');
        if (!canceled) {
          setRedeemLifecycle(next);
          setRedeemOperatorError(null);
        }
      } catch (error) {
        if (!canceled) setRedeemOperatorError(error instanceof Error ? error.message : String(error));
      } finally {
        if (!canceled) timeout = window.setTimeout(() => void refresh(), 2_500);
      }
    };
    void refresh();
    return () => {
      canceled = true;
      if (timeout !== undefined) window.clearTimeout(timeout);
    };
  }, [connection, redeem]);

  useEffect(() => {
    if (connection.kind !== 'ready' || !sharesClaimed) {
      setVaultMetrics(null);
      return;
    }
    let canceled = false;
    void readDemoVaultMetrics()
      .then((metrics) => {
        if (!canceled) setVaultMetrics(metrics);
      })
      .catch((error) => {
        if (!canceled) setHarvestError(error instanceof Error ? error.message : String(error));
      });
    return () => {
      canceled = true;
    };
  }, [connection, sharesClaimed]);

  const clearAccountState = useCallback(() => {
    setDeposit({ kind: 'idle' });
    setLifecycle(null);
    setLifecycleError(null);
    setOperatorError(null);
    setOperatorAction(null);
    setClaiming(false);
    setClaimError(null);
    setRevealedShares(null);
    setRevealingShares(false);
    setRevealError(null);
    setVaultMetrics(null);
    setHarvesting(false);
    setHarvestError(null);
    setHarvestFromPrice(null);
    setRedeem({ kind: 'idle' });
    setRedeemLifecycle(null);
    setRedeemOperatorAction(null);
    setRedeemOperatorError(null);
    setClaimingRedeem(false);
    setRedeemClaimError(null);
    setRevealedUsdc(null);
    setRevealingUsdc(false);
    setRevealUsdcError(null);
  }, []);

  const disconnect = useCallback(() => {
    sessionGeneration.current += 1;
    operationInFlight.current = false;
    setConnection({ kind: 'disconnected' });
    clearAccountState();
  }, [clearAccountState]);

  const connect = async (createSession: (isActive: () => boolean) => Promise<DemoSession>) => {
    if (operationInFlight.current) return;
    operationInFlight.current = true;
    const generation = sessionGeneration.current + 1;
    sessionGeneration.current = generation;
    const isActive = () => sessionGeneration.current === generation;
    setConnection({ kind: 'connecting' });
    clearAccountState();
    try {
      const session = await createSession(isActive);
      session.assertActive();
      setConnection({ kind: 'ready', session });
      setDeposit({ kind: 'running', stage: 'preparing' });
      const existingDeposit = await findExistingDeposit(session, DEPOSIT_AMOUNT_USDC);
      session.assertActive();
      setDeposit(existingDeposit === null ? { kind: 'idle' } : { kind: 'joined', result: existingDeposit });
      try {
        const existingRedeem = await findExistingRedeem(session);
        session.assertActive();
        setRedeem(existingRedeem === null ? { kind: 'idle' } : { kind: 'joined', result: existingRedeem });
      } catch (error) {
        if (session.isActive()) {
          setRedeem({ kind: 'error', message: error instanceof Error ? error.message : String(error) });
        }
      }
    } catch (error) {
      if (isActive()) {
        setConnection({ kind: 'error', message: describeWalletError(error, 'connect') });
        setDeposit({ kind: 'idle' });
      }
    } finally {
      if (isActive()) operationInFlight.current = false;
    }
  };

  const connectBurner = () => {
    void connect((isActive) => connectDemoSession(isActive));
  };

  const claimShares = async () => {
    if (connection.kind !== 'ready' || deposit.kind !== 'joined' || operationInFlight.current) return;
    const session = connection.session;
    operationInFlight.current = true;
    setClaiming(true);
    setClaimError(null);
    let actionError: string | null = null;
    try {
      await withWalletMutationLock(session, () => claimBatchPayout(session, deposit.result, 'deposit'));
    } catch (error) {
      actionError = describeWalletError(error, 'transaction');
    } finally {
      if (session.isActive()) {
        try {
          const next = await readVaultLifecycle(session, deposit.result, 'deposit');
          session.assertActive();
          setLifecycle(next);
          setClaimError(next.kind === 'settled' && next.claimed ? null : actionError);
        } catch {
          setClaimError(actionError);
        }
        setClaiming(false);
        operationInFlight.current = false;
      }
    }
  };

  const runOperatorAction = async (action: 'dispatch' | 'settle') => {
    if (connection.kind !== 'ready' || deposit.kind !== 'joined' || operationInFlight.current) return;
    const session = connection.session;
    operationInFlight.current = true;
    setOperatorAction(action);
    setOperatorError(null);
    let actionError: string | null = null;
    try {
      await withWalletMutationLock(session, () => runDemoOperatorAction(action, deposit.result));
    } catch (error) {
      actionError = describeWalletError(error, 'transaction');
    } finally {
      if (session.isActive()) {
        try {
          const next = await readVaultLifecycle(session, deposit.result, 'deposit');
          session.assertActive();
          setLifecycle(next);
          const completed =
            action === 'dispatch'
              ? next.kind !== 'awaiting-dispatch'
              : next.kind === 'settled' || next.kind === 'canceled';
          setOperatorError(completed ? null : actionError);
        } catch {
          setOperatorError(actionError);
        }
        setOperatorAction(null);
        operationInFlight.current = false;
      }
    }
  };

  const shieldAndDeposit = async () => {
    if (connection.kind !== 'ready' || operationInFlight.current) return;
    const session = connection.session;
    operationInFlight.current = true;
    setDeposit({ kind: 'running', stage: 'preparing' });
    try {
      const result = await withWalletMutationLock(session, () =>
        depositToVault(session, DEPOSIT_AMOUNT_USDC, (stage) => {
          if (session.isActive()) setDeposit({ kind: 'running', stage });
        }),
      );
      session.assertActive();
      setDeposit({ kind: 'joined', result });
    } catch (error) {
      if (session.isActive()) {
        setDeposit({ kind: 'error', message: describeWalletError(error, 'transaction') });
      }
    } finally {
      if (session.isActive()) operationInFlight.current = false;
    }
  };

  const revealShares = async () => {
    if (connection.kind !== 'ready' || lifecycle?.kind !== 'settled' || !lifecycle.claimed || operationInFlight.current)
      return;
    const session = connection.session;
    operationInFlight.current = true;
    setRevealingShares(true);
    setRevealError(null);
    try {
      const balance = await revealClaimedShares(session);
      session.assertActive();
      setRevealedShares(balance);
    } catch (error) {
      if (session.isActive()) setRevealError(describeWalletError(error, 'reveal'));
    } finally {
      if (session.isActive()) {
        setRevealingShares(false);
        operationInFlight.current = false;
      }
    }
  };

  const applyDemoYield = async () => {
    if (harvesting || vaultMetrics === null) return;
    setHarvesting(true);
    setHarvestError(null);
    setHarvestFromPrice(Number(vaultMetrics.totalAssets) / Number(vaultMetrics.totalShares));
    try {
      const result = await harvestDemoVault();
      setVaultMetrics(result.after);
    } catch (error) {
      setHarvestFromPrice(null);
      setHarvestError(error instanceof Error ? error.message : String(error));
    } finally {
      setHarvesting(false);
    }
  };

  const redeemHalf = async () => {
    if (connection.kind !== 'ready' || operationInFlight.current || redeem.kind === 'running') return;
    const session = connection.session;
    operationInFlight.current = true;
    setRedeem({ kind: 'running', stage: 'decrypting' });
    try {
      const shares = revealedShares ?? (await revealClaimedShares(session));
      session.assertActive();
      setRevealedShares(null);
      const amount = shares.value / 2n;
      if (amount === 0n) throw new Error('The private share balance is too small to redeem half');
      const result = await withWalletMutationLock(session, () =>
        joinRedeemBatch(session, amount, shares.handle, (stage) => {
          if (session.isActive()) setRedeem({ kind: 'running', stage });
        }),
      );
      session.assertActive();
      setRedeem({ kind: 'joined', result });
    } catch (error) {
      if (session.isActive()) {
        setRedeem({ kind: 'error', message: describeWalletError(error, 'transaction') });
      }
    } finally {
      if (session.isActive()) operationInFlight.current = false;
    }
  };

  const runRedeemOperatorAction = async (action: 'dispatch' | 'settle') => {
    if (connection.kind !== 'ready' || redeem.kind !== 'joined' || operationInFlight.current) return;
    const session = connection.session;
    operationInFlight.current = true;
    setRedeemOperatorAction(action);
    setRedeemOperatorError(null);
    let actionError: string | null = null;
    try {
      await withWalletMutationLock(session, () => runDemoOperatorAction(action, redeem.result, 'redeem'));
    } catch (error) {
      actionError = describeWalletError(error, 'transaction');
    } finally {
      if (session.isActive()) {
        try {
          const next = await readVaultLifecycle(session, redeem.result, 'redeem');
          session.assertActive();
          setRedeemLifecycle(next);
          const completed =
            action === 'dispatch'
              ? next.kind !== 'awaiting-dispatch'
              : next.kind === 'settled' || next.kind === 'canceled';
          setRedeemOperatorError(completed ? null : actionError);
        } catch {
          setRedeemOperatorError(actionError);
        }
        setRedeemOperatorAction(null);
        operationInFlight.current = false;
      }
    }
  };

  const claimRedeemedUsdc = async () => {
    if (connection.kind !== 'ready' || redeem.kind !== 'joined' || operationInFlight.current) return;
    const session = connection.session;
    operationInFlight.current = true;
    setClaimingRedeem(true);
    setRedeemClaimError(null);
    let actionError: string | null = null;
    try {
      await withWalletMutationLock(session, () => claimBatchPayout(session, redeem.result, 'redeem'));
    } catch (error) {
      actionError = describeWalletError(error, 'transaction');
    } finally {
      if (session.isActive()) {
        try {
          const next = await readVaultLifecycle(session, redeem.result, 'redeem');
          session.assertActive();
          setRedeemLifecycle(next);
          setRedeemClaimError(next.kind === 'settled' && next.claimed ? null : actionError);
        } catch {
          setRedeemClaimError(actionError);
        }
        setClaimingRedeem(false);
        operationInFlight.current = false;
      }
    }
  };

  const revealRedeemedUsdc = async () => {
    if (connection.kind !== 'ready' || operationInFlight.current) return;
    const session = connection.session;
    operationInFlight.current = true;
    setRevealingUsdc(true);
    setRevealUsdcError(null);
    try {
      const balance = await revealClaimedUsdc(session);
      session.assertActive();
      setRevealedUsdc(balance);
    } catch (error) {
      if (session.isActive()) setRevealUsdcError(describeWalletError(error, 'reveal'));
    } finally {
      if (session.isActive()) {
        setRevealingUsdc(false);
        operationInFlight.current = false;
      }
    }
  };

  const connected = connection.kind === 'ready';
  const depositRunning = deposit.kind === 'running';
  const depositJoined = deposit.kind === 'joined';
  const settled = lifecycle?.kind === 'settled';
  const sharePrice =
    vaultMetrics === null || vaultMetrics.totalShares === 0n
      ? null
      : Number(vaultMetrics.totalAssets) / Number(vaultMetrics.totalShares);
  const yieldApplied = sharePrice !== null && sharePrice >= 1.25;
  const redeemJoined = redeem.kind === 'joined';
  const redeemSettled = redeemLifecycle?.kind === 'settled';
  const depositStatus =
    deposit.kind === 'running'
      ? depositStageCopy[deposit.stage]
      : deposit.kind === 'joined'
        ? lifecycle?.kind === 'settled'
          ? lifecycle.claimed
            ? 'Private cShares claimed'
            : 'Settlement complete · Claim available'
          : depositStageCopy.joined
        : null;

  return (
    <div className="app-shell">
      <header className="topbar">
        <a className="brand" href="/" aria-label="Confidential Vault home">
          <span className="brand-mark">Z</span>
          <span>Confidential Vault</span>
        </a>
        <div className="network-pill">
          <span className="network-dot" />
          Solana localnet
        </div>
        <WalletControl
          connection={connection}
          disabled={depositRunning}
          onBurnerConnect={connectBurner}
          onConnect={(createSession) => void connect(createSession)}
          onDisconnect={disconnect}
        />
      </header>

      <main>
        <section className="hero">
          <p className="eyebrow">Private yield, familiar Solana flow</p>
          <h1>Your confidential portfolio</h1>
          <p className="hero-copy">
            Deposit USDC into an encrypted vault. Your position stays private; settlement remains verifiable on Solana.
          </p>
        </section>

        {connection.kind === 'error' && (
          <div className="error-banner" role="alert">
            <span>{connection.message}</span>
            <button type="button" onClick={disconnect}>
              Dismiss
            </button>
          </div>
        )}

        <section className="portfolio-grid" aria-label="Portfolio overview">
          <article className="balance-card">
            <div className="card-heading">
              <span>Private vault shares</span>
              <button
                className="icon-button"
                type="button"
                aria-label={revealedShares === null ? 'Reveal confidential balance' : 'Hide confidential balance'}
                disabled={!settled || !lifecycle.claimed || revealingShares}
                onClick={() => {
                  if (revealedShares === null) void revealShares();
                  else setRevealedShares(null);
                }}
              >
                ◉
              </button>
            </div>
            <strong className="private-balance">
              {revealingShares
                ? 'Decrypting…'
                : revealedShares === null
                  ? '••••••'
                  : `${formatUsdc(revealedShares.value)} cShares`}
            </strong>
            <p>
              {revealedShares !== null
                ? 'Revealed for this view only · no transaction'
                : lifecycle?.kind === 'settled' && lifecycle.claimed
                  ? 'Private cShares received'
                  : connected
                    ? 'Ready to decrypt'
                    : 'Connect to view your private position'}
            </p>
            {revealError && <p className="balance-error">{revealError}</p>}
          </article>

          <article className="vault-card">
            <div className="vault-symbol">USDC</div>
            <div>
              <span className="muted">Vault</span>
              <h2>Confidential USDC</h2>
              <p>Shield USDC, join the next batch, and earn private vault shares.</p>
            </div>
            <div className="vault-metric">
              <span>Deposit flow</span>
              <strong>2 txs</strong>
            </div>
            <div className="deposit-action">
              <div className="amount-row">
                <div>
                  <span className="muted">Amount</span>
                  <strong>
                    {DEPOSIT_AMOUNT_USDC} <small>USDC</small>
                  </strong>
                </div>
                <span className="funding-note">{connected ? 'Funded automatically' : 'Connect to fund'}</span>
              </div>
              <div className="transaction-preview">
                <span>1 · Shield USDC</span>
                <span>2 · Join private batch</span>
              </div>
              <button
                className="primary-action"
                type="button"
                disabled={!connected || depositRunning || depositJoined}
                onClick={shieldAndDeposit}
              >
                {depositRunning ? 'Deposit in progress…' : depositJoined ? 'Deposit joined' : 'Shield & deposit'}
              </button>
              {depositStatus && (
                <p className={`action-status ${depositJoined ? 'success' : ''}`} role="status">
                  <span className="status-dot" />
                  {depositStatus}
                </p>
              )}
              {deposit.kind === 'error' && (
                <div className="action-error" role="alert">
                  <span>{deposit.message}</span>
                  <button type="button" onClick={shieldAndDeposit}>
                    Retry
                  </button>
                </div>
              )}
            </div>
          </article>
        </section>

        <section className="journey-card">
          <div>
            <p className="eyebrow">One intent, clearly tracked</p>
            <h2>Deposit journey</h2>
          </div>
          <ol className="timeline">
            <li className={depositJoined ? 'complete' : 'active'}>
              <span>{depositJoined ? '✓' : '1'}</span>
              <div>
                <strong>Shield &amp; join</strong>
                <small>One click · two sequential transactions</small>
              </div>
            </li>
            <li className={settled ? 'complete' : depositJoined ? 'active' : ''}>
              <span>{settled ? '✓' : '2'}</span>
              <div>
                <strong>
                  {settled
                    ? 'KMS settlement verified'
                    : lifecycle?.kind === 'proving'
                      ? lifecycle.proofReady
                        ? 'Private proof ready'
                        : 'Proving privately'
                      : depositJoined
                        ? 'Awaiting dispatch'
                        : 'Private batch settles'}
                </strong>
                <small>{settled ? 'Public total revealed on-chain' : 'Your contribution remains masked'}</small>
              </div>
            </li>
            <li className={settled ? (lifecycle.claimed ? 'complete' : 'active') : ''}>
              <span>{settled && lifecycle.claimed ? '✓' : '3'}</span>
              <div>
                <strong>
                  {settled ? (lifecycle.claimed ? 'cShares received' : 'Claim available') : 'Claim cShares'}
                </strong>
                <small>
                  {settled && lifecycle.claimed ? 'Your shares remain private' : 'Your payout remains encrypted'}
                </small>
              </div>
            </li>
          </ol>
          {depositJoined && (
            <div className="position-detail">
              <div className="privacy-split">
                <div>
                  <span>Your contribution</span>
                  <strong>••• USDC</strong>
                </div>
                <div>
                  <span>Public batch total</span>
                  <strong>{settled ? `${formatUsdc(lifecycle.totalJoined)} USDC` : 'Not public yet'}</strong>
                </div>
              </div>

              {lifecycle?.kind === 'awaiting-dispatch' && (
                <div className="operator-panel">
                  <div>
                    <span className="operator-label">Demo operator</span>
                    <strong>The local keeper advances this permissionless batch.</strong>
                    <small>This is not a wallet action.</small>
                  </div>
                  <button
                    type="button"
                    disabled={lifecycle.remainingSlots > 0n || operatorAction !== null}
                    onClick={() => void runOperatorAction('dispatch')}
                  >
                    {operatorAction === 'dispatch'
                      ? 'Dispatching…'
                      : lifecycle.remainingSlots > 0n
                        ? `Available in ~${lifecycle.remainingSlots.toString()} slots`
                        : 'Dispatch batch'}
                  </button>
                </div>
              )}

              {lifecycle?.kind === 'proving' && (
                <div className="operator-panel">
                  <div>
                    <span className="operator-label">Demo operator</span>
                    <strong>
                      {lifecycle.proofReady ? 'Private proof is ready.' : 'Proving the encrypted batch total…'}
                    </strong>
                    <small>
                      {lifecycle.proofReady
                        ? 'The KMS certificate can now be verified on Solana.'
                        : 'Proof readiness is checked automatically.'}
                    </small>
                  </div>
                  {lifecycle.proofReady && (
                    <button
                      type="button"
                      disabled={operatorAction !== null}
                      onClick={() => void runOperatorAction('settle')}
                    >
                      {operatorAction === 'settle' ? 'Settling…' : 'Settle with KMS certificate'}
                    </button>
                  )}
                </div>
              )}

              {settled && (
                <div className="verified-settlement" role="status">
                  <span>✓</span>
                  <div>
                    <strong>KMS certificate verified on Solana</strong>
                    <small>
                      {lifecycle.claimed
                        ? 'Your claimed cShares remain encrypted.'
                        : 'Your cShares payout is available to claim and remains encrypted.'}
                    </small>
                  </div>
                </div>
              )}

              {settled && !lifecycle.claimed && (
                <div className="claim-panel">
                  <div>
                    <strong>Claim your private cShares</strong>
                    <small>One user transaction. The received balance remains encrypted.</small>
                  </div>
                  <button type="button" disabled={claiming} onClick={() => void claimShares()}>
                    {claiming ? 'Claiming encrypted shares…' : 'Claim private cShares'}
                  </button>
                </div>
              )}

              {settled && lifecycle.claimed && (
                <div className="claim-complete" role="status">
                  <span>✓</span>
                  <strong>cShares received privately</strong>
                </div>
              )}

              {settled && lifecycle.claimed && (
                <div className={`yield-panel ${harvestFromPrice !== null && yieldApplied ? 'yield-applied' : ''}`}>
                  <div>
                    <span className="operator-label">Public vault event</span>
                    <strong>
                      {sharePrice === null ? 'Reading live share price…' : `1 cShare = ${sharePrice.toFixed(2)} USDC`}
                    </strong>
                    <small>
                      {harvestFromPrice !== null && yieldApplied
                        ? `${harvestFromPrice.toFixed(2)} → ${sharePrice?.toFixed(2)} · assets rose, share supply did not`
                        : yieldApplied
                          ? 'Yield is already reflected in the on-chain asset/share ratio.'
                          : 'The demo keeper can donate real USDC to simulate accrued yield.'}
                    </small>
                  </div>
                  <button
                    type="button"
                    disabled={vaultMetrics === null || harvesting || yieldApplied}
                    onClick={() => void applyDemoYield()}
                  >
                    {harvesting ? 'Applying yield…' : yieldApplied ? 'Yield applied' : 'Simulate +25% yield'}
                  </button>
                </div>
              )}

              {operatorError && (
                <div className="action-error" role="alert">
                  <span>{operatorError}</span>
                </div>
              )}

              {lifecycleError && !operatorError && (
                <div className="action-error" role="alert">
                  <span>Live batch status is temporarily unavailable: {lifecycleError}</span>
                </div>
              )}

              {claimError && (
                <div className="action-error" role="alert">
                  <span>{claimError}</span>
                  <button type="button" onClick={() => void claimShares()}>
                    Retry claim
                  </button>
                </div>
              )}

              {harvestError && (
                <div className="action-error" role="alert">
                  <span>{harvestError}</span>
                  <button type="button" onClick={() => void applyDemoYield()}>
                    Retry yield
                  </button>
                </div>
              )}

              {settled && (
                <details className="privacy-note">
                  <summary>Privacy detail</summary>
                  This local batch has one participant, so its public total allows the deposit to be inferred. Privacy
                  strengthens with multiple independent participants.
                </details>
              )}
            </div>
          )}
        </section>

        {sharesClaimed && (
          <section className="redeem-card">
            <div className="redeem-heading">
              <div>
                <p className="eyebrow">Private exit, same Solana rhythm</p>
                <h2>Redeem half your position</h2>
                <p>
                  A one-time balance signature calculates exactly 50%, then one transaction joins the private redeem
                  batch. The clear balance is remasked immediately.
                </p>
              </div>
              <div className="redeem-amount">
                <span>Intent</span>
                <strong>50%</strong>
                <small>of private cShares</small>
              </div>
            </div>

            <ol className="redeem-timeline">
              <li className={yieldApplied ? 'complete' : 'active'}>
                <span>{yieldApplied ? '✓' : '1'}</span>
                <div>
                  <strong>Yield accrues</strong>
                  <small>Public share price rises on-chain</small>
                </div>
              </li>
              <li className={redeemJoined ? 'complete' : yieldApplied ? 'active' : ''}>
                <span>{redeemJoined ? '✓' : '2'}</span>
                <div>
                  <strong>Redeem half privately</strong>
                  <small>One signature · one transaction</small>
                </div>
              </li>
              <li className={redeemSettled ? 'complete' : redeemJoined ? 'active' : ''}>
                <span>{redeemSettled ? '✓' : '3'}</span>
                <div>
                  <strong>KMS settlement</strong>
                  <small>Certificate verified on Solana</small>
                </div>
              </li>
              <li className={redeemSettled && redeemLifecycle.claimed ? 'complete' : redeemSettled ? 'active' : ''}>
                <span>{redeemSettled && redeemLifecycle.claimed ? '✓' : '4'}</span>
                <div>
                  <strong>Claim private cUSDC</strong>
                  <small>Redeemed value stays encrypted</small>
                </div>
              </li>
            </ol>

            <div className="redeem-action">
              <div>
                <strong>
                  {redeem.kind === 'running'
                    ? redeemStageCopy[redeem.stage]
                    : redeemJoined
                      ? redeemStageCopy.joined
                      : 'Redeem 50% without exposing the amount on-chain'}
                </strong>
                <small>
                  {redeemJoined
                    ? 'The clear redeem amount was discarded immediately after the private join.'
                    : revealedShares === null
                      ? 'The one-time decrypt signature is requested inside this intent.'
                      : `Uses the ${formatUsdc(revealedShares.value)} cShares revealed in this view, then remasks it.`}
                </small>
              </div>
              <button
                type="button"
                disabled={!yieldApplied || redeem.kind === 'running' || redeemJoined}
                onClick={() => void redeemHalf()}
              >
                {redeem.kind === 'running'
                  ? 'Redeeming half…'
                  : redeemJoined
                    ? 'Redemption joined'
                    : 'Redeem half privately'}
              </button>
            </div>

            {redeem.kind === 'error' && (
              <div className="action-error" role="alert">
                <span>{redeem.message}</span>
                <button type="button" onClick={() => void redeemHalf()}>
                  Retry redeem
                </button>
              </div>
            )}

            {redeemLifecycle?.kind === 'awaiting-dispatch' && (
              <div className="operator-panel">
                <div>
                  <span className="operator-label">Demo operator</span>
                  <strong>The private redeem batch is ready to advance.</strong>
                  <small>This is not a wallet action.</small>
                </div>
                <button
                  type="button"
                  disabled={redeemLifecycle.remainingSlots > 0n || redeemOperatorAction !== null}
                  onClick={() => void runRedeemOperatorAction('dispatch')}
                >
                  {redeemOperatorAction === 'dispatch'
                    ? 'Dispatching…'
                    : redeemLifecycle.remainingSlots > 0n
                      ? `Available in ~${redeemLifecycle.remainingSlots.toString()} slots`
                      : 'Dispatch redeem batch'}
                </button>
              </div>
            )}

            {redeemLifecycle?.kind === 'proving' && (
              <div className="operator-panel">
                <div>
                  <span className="operator-label">Demo operator</span>
                  <strong>
                    {redeemLifecycle.proofReady ? 'Private redeem proof is ready.' : 'Proving the private total…'}
                  </strong>
                  <small>
                    {redeemLifecycle.proofReady
                      ? 'The KMS certificate can now settle the redemption on Solana.'
                      : 'Proof readiness is checked automatically.'}
                  </small>
                </div>
                {redeemLifecycle.proofReady && (
                  <button
                    type="button"
                    disabled={redeemOperatorAction !== null}
                    onClick={() => void runRedeemOperatorAction('settle')}
                  >
                    {redeemOperatorAction === 'settle' ? 'Settling…' : 'Settle private redemption'}
                  </button>
                )}
              </div>
            )}

            {redeemSettled && !redeemLifecycle.claimed && (
              <div className="claim-panel">
                <div>
                  <strong>Claim your private cUSDC</strong>
                  <small>One transaction. The redeemed value remains encrypted.</small>
                </div>
                <button type="button" disabled={claimingRedeem} onClick={() => void claimRedeemedUsdc()}>
                  {claimingRedeem ? 'Claiming private cUSDC…' : 'Claim private cUSDC'}
                </button>
              </div>
            )}

            {redeemSettled && (
              <div className="privacy-split redeem-privacy-split">
                <div>
                  <span>Your redeemed amount</span>
                  <strong>••• cShares</strong>
                </div>
                <div>
                  <span>Public redeem batch total</span>
                  <strong>{formatUsdc(redeemLifecycle.totalJoined)} cShares</strong>
                </div>
                <div>
                  <span>Public USDC returned</span>
                  <strong>{formatUsdc(redeemLifecycle.payoutReceived)} USDC</strong>
                </div>
              </div>
            )}

            {redeemSettled && redeemLifecycle.claimed && (
              <div className="redeem-complete">
                <div className="verified-settlement" role="status">
                  <span>✓</span>
                  <div>
                    <strong>Half redeemed and cUSDC claimed privately</strong>
                    <small>The remaining cShares and claimed cUSDC are both still encrypted.</small>
                  </div>
                </div>
                <div>
                  <strong>
                    {revealingUsdc
                      ? 'Decrypting…'
                      : revealedUsdc === null
                        ? '•••• cUSDC'
                        : `${formatUsdc(revealedUsdc.value)} cUSDC`}
                  </strong>
                  <small>{revealedUsdc === null ? 'Exact current balance' : 'Revealed for this view only'}</small>
                </div>
                <button
                  type="button"
                  disabled={revealingUsdc}
                  onClick={() => {
                    if (revealedUsdc === null) void revealRedeemedUsdc();
                    else setRevealedUsdc(null);
                  }}
                >
                  {revealedUsdc === null ? 'Reveal current cUSDC balance' : 'Hide cUSDC'}
                </button>
              </div>
            )}

            {redeemSettled && (
              <details className="privacy-note">
                <summary>Privacy detail</summary>
                This local redeem batch has one participant, so its public total allows the redeemed amount to be
                inferred. Privacy strengthens with multiple independent participants.
              </details>
            )}

            {redeemOperatorError && (
              <div className="action-error" role="alert">
                <span>{redeemOperatorError}</span>
              </div>
            )}

            {redeemClaimError && (
              <div className="action-error" role="alert">
                <span>{redeemClaimError}</span>
                <button type="button" onClick={() => void claimRedeemedUsdc()}>
                  Retry claim
                </button>
              </div>
            )}

            {revealUsdcError && (
              <div className="action-error" role="alert">
                <span>{revealUsdcError}</span>
                <button type="button" onClick={() => void revealRedeemedUsdc()}>
                  Retry reveal
                </button>
              </div>
            )}
          </section>
        )}
      </main>
    </div>
  );
}
