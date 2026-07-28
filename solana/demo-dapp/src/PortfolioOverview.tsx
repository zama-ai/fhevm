import { ActionError } from './JourneyPrimitives';
import { formatUsdc } from './format';
import type { DemoController } from './useDemoController';

const DEPOSIT_AMOUNT_USDC = 100;

export function PortfolioOverview({ controller }: { readonly controller: DemoController }) {
  const { state, derived, actions } = controller;
  const { deposit, depositLifecycle, revealedShares, revealingShares, revealSharesError } = state;
  const { connected, depositRunning, depositJoined } = derived;
  const phantomLocalnet =
    state.connection.kind === 'ready' &&
    state.connection.session.wallet.kind === 'wallet-standard' &&
    state.connection.session.wallet.name.toLowerCase() === 'phantom';
  const settled = depositLifecycle?.kind === 'settled';
  const status =
    deposit.kind === 'running'
      ? {
          preparing: 'Checking your confidential account…',
          shielding: 'Transaction 1 of 2 · Shielding 100 USDC…',
          proving: 'Creating your private deposit proof…',
          joining: 'Transaction 2 of 2 · Depositing 100 USDC…',
          joined: 'Deposit complete · Waiting for settlement',
        }[deposit.stage]
      : deposit.kind === 'joined'
        ? settled
          ? depositLifecycle.claimed
            ? 'Private cShares claimed'
            : 'Settlement complete · Claim available'
          : 'Deposit complete · Waiting for settlement'
        : null;

  return (
    <section className="portfolio-grid" aria-label="Portfolio overview">
      <article className="balance-card">
        <div className="card-heading">
          <span>Private vault shares</span>
          <button
            className="icon-button"
            type="button"
            aria-label={revealedShares === null ? 'Reveal confidential balance' : 'Hide confidential balance'}
            disabled={!settled || !depositLifecycle.claimed || revealingShares}
            onClick={revealedShares === null ? actions.revealShares : actions.hideShares}
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
            : settled && depositLifecycle.claimed
              ? 'Private cShares received'
              : connected
                ? 'Ready to decrypt'
                : 'Connect to view your private position'}
        </p>
        {revealSharesError && <p className="balance-error">{revealSharesError}</p>}
      </article>

      <article className="vault-card">
        <div className="vault-symbol">USDC</div>
        <div>
          <span className="muted">Vault</span>
          <h2>Confidential USDC</h2>
          <p>Shield USDC and earn yield.</p>
        </div>
        <div className="vault-metric">
          <span>{phantomLocalnet ? 'Wallet approvals' : 'Transactions'}</span>
          <strong>{phantomLocalnet ? '2 required' : '2'}</strong>
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
            <span>2 · Deposit</span>
          </div>
          {phantomLocalnet && !depositJoined && (
            <p className="wallet-scan-note">
              <strong>Phantom localnet · developer mode.</strong> The app simulates each transaction on this local
              validator before and after signing. Phantom&apos;s scanner cannot reach this local validator and may still
              show an unresolved warning. Use Demo wallet for the supported warning-free local flow.
            </p>
          )}
          <button
            className="primary-action"
            type="button"
            disabled={!connected || depositRunning || depositJoined}
            onClick={actions.shieldAndDeposit}
          >
            {depositRunning ? 'Deposit in progress…' : depositJoined ? 'Deposited' : 'Shield & deposit'}
          </button>
          {status && (
            <p className={`action-status ${depositJoined ? 'success' : ''}`} role="status">
              <span className="status-dot" />
              {status}
            </p>
          )}
          {deposit.kind === 'error' && (
            <ActionError retryLabel="Retry" onRetry={actions.shieldAndDeposit}>
              {deposit.message}
            </ActionError>
          )}
        </div>
      </article>
    </section>
  );
}
