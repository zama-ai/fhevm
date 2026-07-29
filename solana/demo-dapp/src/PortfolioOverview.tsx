import { useEffect, useRef, useState } from 'react';

import { ActionError } from './JourneyPrimitives';
import { formatUsdc } from './format';
import type { DemoController } from './useDemoController';
import { DEMO_APY_PERCENT, DEMO_RATE_WINDOW_DAYS } from './yieldPolicy';

const DEFAULT_DEPOSIT_AMOUNT = '100';

export function PortfolioOverview({ controller }: { readonly controller: DemoController }) {
  const { state, derived, actions } = controller;
  const {
    deposit,
    depositLifecycle,
    hasConfidentialUsdc,
    hasConfidentialShares,
    publicUsdcBalance,
    walletBalancesError,
    revealedShares,
    revealingShares,
    revealSharesError,
    revealedUsdc,
    revealingUsdc,
    revealUsdcError,
  } = state;
  const { connected, depositRunning, sharePrice } = derived;
  const hasConfidentialPositionAccount = hasConfidentialShares === true;
  const privateActionRunning =
    depositRunning ||
    state.redeem.kind === 'running' ||
    state.depositOperatorAction !== null ||
    state.redeemOperatorAction !== null ||
    revealingShares ||
    revealingUsdc;
  const [amount, setAmount] = useState(DEFAULT_DEPOSIT_AMOUNT);
  const [depositComplete, setDepositComplete] = useState(false);
  const pendingDeposit = useRef(false);
  const parsedAmount = Number(amount);
  const validAmount =
    amount.trim() !== '' &&
    Number.isFinite(parsedAmount) &&
    parsedAmount > 0 &&
    parsedAmount <= 1_000 &&
    /^\d+(\.\d{0,6})?$/.test(amount);
  const settled = depositLifecycle?.kind === 'settled';
  const currentDepositClaimed = settled && depositLifecycle.claimed;
  const canDeposit = deposit.kind === 'idle' || deposit.kind === 'error' || currentDepositClaimed;
  const phantomLocalnet =
    state.connection.kind === 'ready' &&
    state.connection.session.wallet.kind === 'wallet-standard' &&
    state.connection.session.wallet.name.toLowerCase() === 'phantom';
  const externalWallet =
    state.connection.kind === 'ready' && state.connection.session.wallet.kind === 'wallet-standard';
  const status =
    deposit.kind === 'running'
      ? {
          preparing: 'Preparing the next private batch…',
          shielding: 'Approval 1 of 2 · Shielding USDC…',
          proving: 'Creating the private deposit proof…',
          joining: 'Approval 2 of 2 · Depositing…',
          joined: 'Deposit submitted',
        }[deposit.stage]
      : deposit.kind === 'joined'
        ? settled
          ? 'Receiving cShares…'
          : 'Settlement in progress'
        : null;
  const publicUsdcLabel =
    !connected
      ? '—'
      : publicUsdcBalance === null
        ? walletBalancesError
          ? 'Unavailable'
          : 'Loading…'
        : `${formatUsdc(publicUsdcBalance)} USDC`;
  const confidentialUsdcLabel =
    !connected
      ? '—'
      : hasConfidentialUsdc === null
        ? walletBalancesError
          ? 'Unavailable'
          : 'Loading…'
        : !hasConfidentialUsdc
          ? '0 cUSDC'
          : revealingUsdc
            ? 'Decrypting…'
            : revealedUsdc === null
              ? '•••• cUSDC'
              : `${formatUsdc(revealedUsdc.value)} cUSDC`;
  const confidentialSharesLabel =
    !connected
      ? '—'
      : hasConfidentialShares === null
        ? walletBalancesError
          ? 'Unavailable'
          : 'Loading…'
        : !hasConfidentialShares
          ? '0 cShares'
          : revealingShares
            ? 'Decrypting…'
            : revealedShares === null
              ? '•••• cShares'
              : `${formatUsdc(revealedShares.value)} cShares`;

  useEffect(() => {
    pendingDeposit.current = false;
    setDepositComplete(false);
    setAmount(DEFAULT_DEPOSIT_AMOUNT);
  }, [state.generation]);

  useEffect(() => {
    if (hasConfidentialPositionAccount && !pendingDeposit.current) setAmount('');
  }, [hasConfidentialPositionAccount]);

  useEffect(() => {
    if (deposit.kind === 'joined' && depositLifecycle !== null && !currentDepositClaimed) {
      pendingDeposit.current = true;
    }
  }, [currentDepositClaimed, deposit.kind, depositLifecycle]);

  useEffect(() => {
    if (!currentDepositClaimed || !pendingDeposit.current) return;
    pendingDeposit.current = false;
    setAmount('');
    setDepositComplete(true);
    const timeout = globalThis.setTimeout(() => setDepositComplete(false), 5_000);
    return () => globalThis.clearTimeout(timeout);
  }, [currentDepositClaimed]);

  return (
    <section className="vault-workspace" aria-label="Confidential USDC vault">
      {depositComplete && (
        <div className="success-toast" role="status" aria-live="polite">
          <span>✓</span>
          <strong>Deposit complete · cShares received</strong>
        </div>
      )}
      <article className="vault-identity">
        <div className="vault-title">
          <div className="vault-symbol">USDC</div>
          <div>
            <span className="muted">Vault</span>
            <h2>Confidential USDC</h2>
            <p>Shield USDC and earn yield.</p>
          </div>
        </div>

        <div className="vault-stats" aria-label="Vault metrics">
          <div>
            <span>APY</span>
            <strong>{DEMO_APY_PERCENT.toFixed(1)}%</strong>
            <small>{DEMO_RATE_WINDOW_DAYS}-day average · annualized</small>
          </div>
          <div>
            <span>Share price</span>
            <strong>{sharePrice == null ? '1.00 USDC' : `${sharePrice.toFixed(2)} USDC`}</strong>
            <small>Live vault ratio</small>
          </div>
        </div>

        <div className="asset-inventory">
          <h3>Your assets</h3>
          <div className="asset-list">
            <div className="asset-row">
              <div>
                <strong>USDC</strong>
                <span>Wallet · Public</span>
              </div>
              <strong className="asset-balance">{publicUsdcLabel}</strong>
            </div>
            <div className="asset-row">
              <div>
                <strong>cUSDC</strong>
                <span>Shielded balance · Private</span>
              </div>
              <div className="asset-value">
                <strong className="asset-balance" aria-live="polite">{confidentialUsdcLabel}</strong>
                {hasConfidentialUsdc && (
                  <button
                    className="balance-action"
                    type="button"
                    aria-label={revealedUsdc === null ? 'Reveal cUSDC balance' : 'Hide cUSDC balance'}
                    disabled={privateActionRunning}
                    onClick={revealedUsdc === null ? actions.revealUsdc : actions.hideUsdc}
                  >
                    {revealedUsdc === null ? 'Reveal' : 'Hide'}
                  </button>
                )}
              </div>
            </div>
            <div className="asset-row">
              <div>
                <strong>cShares</strong>
                <span>Vault position · Private</span>
              </div>
              <div className="asset-value">
                <strong className="asset-balance" aria-live="polite">{confidentialSharesLabel}</strong>
                {hasConfidentialPositionAccount && (
                  <button
                    className="balance-action"
                    type="button"
                    aria-label={revealedShares === null ? 'Reveal cShares balance' : 'Hide cShares balance'}
                    disabled={privateActionRunning}
                    onClick={revealedShares === null ? actions.revealShares : actions.hideShares}
                  >
                    {revealedShares === null ? 'Reveal' : 'Hide'}
                  </button>
                )}
              </div>
            </div>
          </div>
          {(walletBalancesError || revealUsdcError || revealSharesError) && (
            <div className="balance-errors" role="alert">
              {walletBalancesError && <p className="balance-error">{walletBalancesError}</p>}
              {revealUsdcError && <p className="balance-error">{revealUsdcError}</p>}
              {revealSharesError && <p className="balance-error">{revealSharesError}</p>}
            </div>
          )}
        </div>
      </article>

      <article className="deposit-panel">
        <div className="deposit-panel-heading">
          <div>
            <span className="muted">{hasConfidentialPositionAccount ? 'Add to position' : 'Deposit'}</span>
            <h2>{hasConfidentialPositionAccount ? 'Deposit more USDC' : 'Start earning privately'}</h2>
          </div>
          <span className="approval-count">{externalWallet ? '2 approvals' : '2 transactions'}</span>
        </div>

        {canDeposit ? (
          <>
            <label className="amount-input" htmlFor="deposit-amount">
              <span>Amount</span>
              <div>
                <input
                  id="deposit-amount"
                  name="deposit-amount"
                  type="text"
                  inputMode="decimal"
                  autoComplete="off"
                  value={amount}
                  aria-describedby="deposit-amount-help"
                  aria-invalid={amount !== '' && !validAmount}
                  aria-errormessage={amount !== '' && !validAmount ? 'deposit-amount-error' : undefined}
                  onChange={(event) => setAmount(event.target.value)}
                />
                <strong>USDC</strong>
              </div>
              <small id="deposit-amount-help">Funded automatically on localnet</small>
            </label>
            <div
              className="transaction-preview"
              aria-label={externalWallet ? 'Two wallet approvals' : 'Two transactions'}
            >
              <span>1 · Shield</span>
              <span>2 · Deposit</span>
            </div>
            {phantomLocalnet && (
              <p className="wallet-scan-note">
                Phantom may show an unresolved simulation warning because its scanner cannot reach this local validator.
              </p>
            )}
            <button
              className="primary-action"
              type="button"
              disabled={!connected || depositRunning || !validAmount}
              onClick={() => {
                pendingDeposit.current = true;
                setDepositComplete(false);
                actions.shieldAndDeposit(parsedAmount);
              }}
            >
              {depositRunning ? 'Depositing…' : 'Shield & deposit'}
            </button>
            {!validAmount && amount !== '' && (
              <p className="input-error" id="deposit-amount-error">
                Enter 0.000001–1,000 USDC, with up to 6 decimals.
              </p>
            )}
          </>
        ) : (
          <div className="active-deposit">
            <span className="status-dot" />
            <div>
              <strong role="status" aria-live="polite">{status ?? 'Reading deposit status…'}</strong>
              <small>You can close this page. Progress is saved.</small>
            </div>
          </div>
        )}

        {deposit.kind === 'error' && (
          <ActionError>{deposit.message}</ActionError>
        )}
      </article>
    </section>
  );
}
