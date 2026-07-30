import { useEffect, useRef, useState } from 'react';

import { ActionError } from './JourneyPrimitives';
import { usdcToBaseUnits, type DepositSource } from './deposit';
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
  const canDepositConfidential = hasConfidentialUsdc === true;
  const privateActionRunning =
    depositRunning ||
    state.redeem.kind === 'running' ||
    state.depositOperatorAction !== null ||
    state.redeemOperatorAction !== null ||
    revealingShares ||
    revealingUsdc;
  const [amount, setAmount] = useState(DEFAULT_DEPOSIT_AMOUNT);
  const [depositSource, setDepositSource] = useState<DepositSource>('usdc');
  const [depositComplete, setDepositComplete] = useState(false);
  const pendingDeposit = useRef(false);
  const parsedAmount = Number(amount);
  const validAmount =
    amount.trim() !== '' &&
    Number.isFinite(parsedAmount) &&
    parsedAmount > 0 &&
    parsedAmount <= 1_000 &&
    /^\d+(\.\d{0,6})?$/.test(amount);
  const confidentialBalanceTooLow =
    depositSource === 'cusdc' &&
    validAmount &&
    revealedUsdc !== null &&
    revealedUsdc.value < usdcToBaseUnits(parsedAmount);
  const confidentialBalanceUnknown = depositSource === 'cusdc' && revealedUsdc === null;
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
          shielding: `${externalWallet ? 'Approval' : 'Transaction'} 1 of 2 · Shielding USDC…`,
          proving: 'Creating the private deposit proof…',
          joining:
            depositSource === 'usdc'
              ? `${externalWallet ? 'Approval' : 'Transaction'} 2 of 2 · Depositing…`
              : `${externalWallet ? 'Approval' : 'Transaction'} 1 of 1 · Depositing…`,
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
    setDepositSource('usdc');
  }, [state.generation]);

  useEffect(() => {
    if (hasConfidentialUsdc === false && depositSource === 'cusdc') setDepositSource('usdc');
  }, [hasConfidentialUsdc, depositSource]);

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
                    aria-label={revealedUsdc === null ? 'Decrypt cUSDC balance' : 'Hide cUSDC balance'}
                    disabled={privateActionRunning}
                    onClick={revealedUsdc === null ? actions.revealUsdc : actions.hideUsdc}
                  >
                    {revealedUsdc === null ? 'Decrypt' : 'Hide'}
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
            <span className="muted">Deposit</span>
            <h2>{depositSource === 'usdc' ? 'Deposit USDC' : 'Deposit cUSDC'}</h2>
          </div>
          <span className="approval-count">
            {depositSource === 'usdc'
              ? externalWallet
                ? 'Up to 2 approvals'
                : 'Up to 2 transactions'
              : externalWallet
                ? revealedUsdc === null
                  ? '1 signature · 1 approval'
                  : '1 approval'
                : revealedUsdc === null
                  ? '1 signature · 1 transaction'
                  : '1 transaction'}
          </span>
        </div>

        {canDeposit ? (
          <>
            {(canDepositConfidential || depositSource === 'cusdc') && (
              <fieldset className="deposit-source">
                <legend>Deposit from</legend>
                <div>
                  <label>
                    <input
                      type="radio"
                      name="deposit-source"
                      value="usdc"
                      checked={depositSource === 'usdc'}
                      disabled={depositRunning}
                      onChange={() => setDepositSource('usdc')}
                    />
                    <span>USDC</span>
                  </label>
                  <label>
                    <input
                      type="radio"
                      name="deposit-source"
                      value="cusdc"
                      checked={depositSource === 'cusdc'}
                      disabled={depositRunning}
                      onChange={() => setDepositSource('cusdc')}
                    />
                    <span>cUSDC</span>
                  </label>
                </div>
              </fieldset>
            )}
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
                <strong>{depositSource === 'usdc' ? 'USDC' : 'cUSDC'}</strong>
              </div>
              <small id="deposit-amount-help">
                {depositSource === 'usdc'
                  ? 'Funded automatically on localnet'
                  : 'Uses your private cUSDC balance'}
              </small>
            </label>
            <div
              className="transaction-preview"
              aria-label={
                depositSource === 'usdc'
                  ? externalWallet
                    ? 'Up to two wallet approvals'
                    : 'Up to two transactions'
                  : externalWallet
                    ? revealedUsdc === null
                      ? 'One balance signature and one wallet approval'
                      : 'One wallet approval'
                    : revealedUsdc === null
                      ? 'One balance signature and one transaction'
                      : 'One transaction'
              }
            >
              {depositSource === 'cusdc' && revealedUsdc === null && <span>1 · Decrypt balance</span>}
              {depositSource === 'usdc' && <span>1 · Shield if needed</span>}
              <span>
                {depositSource === 'usdc' || (depositSource === 'cusdc' && revealedUsdc === null)
                  ? '2 · Deposit'
                  : '1 · Deposit'}
              </span>
            </div>
            {phantomLocalnet && (
              <p className="wallet-scan-note">
                Phantom may show an unresolved simulation warning because its scanner cannot reach this local validator.
              </p>
            )}
            <button
              className="primary-action"
              type="button"
              title={
                confidentialBalanceUnknown
                  ? 'Decrypts the balance only in this browser so the deposit amount can be checked.'
                  : undefined
              }
              disabled={
                !connected ||
                depositRunning ||
                revealingUsdc ||
                (!confidentialBalanceUnknown && (!validAmount || confidentialBalanceTooLow))
              }
              onClick={() => {
                if (confidentialBalanceUnknown) {
                  actions.revealUsdc();
                  return;
                }
                pendingDeposit.current = true;
                setDepositComplete(false);
                actions.deposit(parsedAmount, depositSource);
              }}
            >
              {revealingUsdc && confidentialBalanceUnknown
                ? 'Decrypting cUSDC…'
                : confidentialBalanceUnknown
                  ? 'Decrypt cUSDC balance to continue'
                  : depositRunning
                    ? 'Depositing…'
                    : depositSource === 'usdc'
                      ? 'Deposit USDC'
                      : 'Deposit cUSDC'}
            </button>
            {!validAmount && amount !== '' && (
              <p className="input-error" id="deposit-amount-error">
                Enter 0.000001–1,000 {depositSource === 'usdc' ? 'USDC' : 'cUSDC'}, with up to 6 decimals.
              </p>
            )}
            {confidentialBalanceTooLow && (
              <p className="input-error" role="alert">
                The decrypted cUSDC balance is too low for this deposit.
              </p>
            )}
            {depositSource === 'cusdc' && revealedUsdc !== null && !confidentialBalanceTooLow && (
              <p className="balance-available">Available: {formatUsdc(revealedUsdc.value)} cUSDC</p>
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
