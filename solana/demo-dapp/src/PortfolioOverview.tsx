import { useEffect, useRef, useState } from 'react';

import { ActionError, ClaimPanel } from './JourneyPrimitives';
import { formatUsdc } from './format';
import type { DemoController } from './useDemoController';
import { DEMO_APY_PERCENT, DEMO_RATE_WINDOW_DAYS } from './yieldPolicy';

const DEFAULT_DEPOSIT_AMOUNT = '100';

export function PortfolioOverview({ controller }: { readonly controller: DemoController }) {
  const { state, derived, actions } = controller;
  const {
    deposit,
    depositLifecycle,
    depositClaiming,
    depositClaimError,
    revealedShares,
    revealingShares,
    revealSharesError,
  } = state;
  const { connected, depositRunning, hasPrivateShares, sharePrice } = derived;
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
      : deposit.kind === 'joined' && !settled
        ? 'Settlement in progress'
        : null;

  useEffect(() => {
    pendingDeposit.current = false;
    setDepositComplete(false);
    setAmount(DEFAULT_DEPOSIT_AMOUNT);
  }, [state.generation]);

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

        <div className="private-position">
          <div className="card-heading">
            <span>Your private position</span>
            <button
              className="icon-button"
              type="button"
              aria-label={revealedShares === null ? 'Reveal confidential balance' : 'Hide confidential balance'}
              disabled={!hasPrivateShares || revealingShares}
              onClick={revealedShares === null ? actions.revealShares : actions.hideShares}
            >
              {revealedShares === null ? '◉' : '○'}
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
              ? 'Visible in this browser only'
              : hasPrivateShares
                ? 'Ready to reveal'
                : connected
                  ? 'No shares yet'
                  : 'Connect to view'}
          </p>
          {revealSharesError && <p className="balance-error">{revealSharesError}</p>}
        </div>
      </article>

      <article className="deposit-panel">
        <div className="deposit-panel-heading">
          <div>
            <span className="muted">{hasPrivateShares ? 'Add to position' : 'Deposit'}</span>
            <h2>{hasPrivateShares ? 'Deposit more USDC' : 'Start earning privately'}</h2>
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
            {!validAmount && amount !== '' && <p className="input-error">Enter up to 1,000 USDC.</p>}
          </>
        ) : settled && !depositLifecycle.claimed ? (
          <ClaimPanel
            title="Your cShares are ready"
            detail="One transaction. The balance stays private."
            label="Claim private cShares"
            busyLabel="Claiming…"
            busy={depositClaiming}
            onClaim={() => actions.claim('deposit')}
          />
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
          <ActionError retryLabel="Retry" onRetry={() => actions.shieldAndDeposit(parsedAmount)}>
            {deposit.message}
          </ActionError>
        )}
        {depositClaimError && (
          <ActionError retryLabel="Retry claim" onRetry={() => actions.claim('deposit')}>
            {depositClaimError}
          </ActionError>
        )}
      </article>
    </section>
  );
}
