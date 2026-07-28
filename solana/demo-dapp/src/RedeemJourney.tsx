import { useState } from 'react';

import { ActionError, JourneyTimeline, SettlementProgress } from './JourneyPrimitives';
import type { RedeemStage } from './redeem';
import { formatUsdc } from './format';
import type { DemoController } from './useDemoController';

const stageCopy = (percentage: number): Record<'decrypting' | RedeemStage, string> => ({
  decrypting: 'Authorizing a one-time balance reveal…',
  proving: `Creating your private ${percentage}% redeem proof…`,
  joining: 'Signing one private redeem transaction…',
  joined: 'Redemption joined · Waiting for private settlement',
});

export function RedeemJourney({ controller }: { readonly controller: DemoController }) {
  const { state, derived, actions } = controller;
  const {
    redeem,
    redeemLifecycle: lifecycle,
    redeemOperatorAction: operatorAction,
    redeemOperatorError: operatorError,
    revealedShares,
    revealedUsdc,
    revealingUsdc,
    revealUsdcError,
  } = state;
  const { connected, hasPrivateShares, redeemJoined } = derived;
  const [percentage, setPercentage] = useState(50);
  const settled = lifecycle?.kind === 'settled';
  if (!connected || !hasPrivateShares) return null;
  const copy = stageCopy(percentage);

  return (
    <section className="redeem-card">
      <div className="redeem-heading">
        <div>
          <p className="eyebrow">Redeem</p>
          <h2>Withdraw from the vault</h2>
          <p>Your amount stays encrypted. Settlement publishes aggregate batch totals.</p>
        </div>
        <div className="redeem-amount">
          <span>Amount</span>
          <strong>{redeemJoined ? 'Private' : `${percentage}%`}</strong>
          <small>{redeemJoined ? 'encrypted on-chain' : 'of your position'}</small>
        </div>
      </div>

      <JourneyTimeline
        framed
        steps={[
          {
            state: redeemJoined ? 'complete' : 'active',
            title: 'Redeem',
            detail: 'One signature · one transaction',
          },
          {
            state: settled ? 'complete' : redeemJoined ? 'active' : 'idle',
            title: settled ? 'Settled on Solana' : 'Automatic settlement',
            detail: settled ? 'Batch total is now public' : 'Your redemption stays private',
          },
          {
            state: settled && lifecycle.claimed ? 'complete' : settled ? 'active' : 'idle',
            title: settled && lifecycle.claimed ? 'cUSDC received' : 'Receiving cUSDC',
            detail: settled && lifecycle.claimed ? 'Balance remains encrypted' : 'Automatic',
          },
        ]}
      />

      <div className="redeem-action">
        <div>
          <strong>
            {redeem.kind === 'running'
              ? copy[redeem.stage]
              : redeemJoined
                ? copy.joined
                : `Redeem ${percentage}% of your position`}
          </strong>
          <small>
            {redeemJoined
              ? 'The clear redeem amount was discarded immediately after the private join.'
              : revealedShares === null
                ? 'The one-time decrypt signature is requested inside this intent.'
                : `Uses the ${formatUsdc(revealedShares.value)} cShares revealed in this view, then remasks it.`}
          </small>
          {!redeemJoined && (
            <label className="redeem-slider">
              <input
                aria-label="Percentage to redeem"
                type="range"
                min="1"
                max="100"
                step="1"
                value={percentage}
                disabled={redeem.kind === 'running'}
                onChange={(event) => setPercentage(Number(event.target.value))}
              />
              <output>{percentage}%</output>
            </label>
          )}
        </div>
        <button
          className="panel-action"
          type="button"
          disabled={redeem.kind === 'running' || redeemJoined}
          onClick={() => actions.redeem(percentage)}
        >
          {redeem.kind === 'running'
            ? 'Redeeming…'
            : redeemJoined
              ? 'Redemption joined'
              : 'Redeem'}
        </button>
      </div>

      {redeem.kind === 'error' && (
        <ActionError retryLabel="Retry redeem" onRetry={() => actions.redeem(percentage)}>
          {redeem.message}
        </ActionError>
      )}

      {(lifecycle?.kind === 'awaiting-dispatch' || lifecycle?.kind === 'proving') && (
        <SettlementProgress
          lifecycle={lifecycle}
          action={operatorAction}
        />
      )}

      {settled && (
        <div className="privacy-split redeem-privacy-split">
          <div>
            <span>Your redeemed amount</span>
            <strong>••• cShares</strong>
          </div>
          <div>
            <span>Public redeem batch total</span>
            <strong>{formatUsdc(lifecycle.totalJoined)} cShares</strong>
          </div>
          <div>
            <span>Public USDC returned</span>
            <strong>{formatUsdc(lifecycle.payoutReceived)} USDC</strong>
          </div>
        </div>
      )}

      {settled && lifecycle.claimed && (
        <div className="redeem-complete">
          <div className="verified-settlement" role="status">
            <span>✓</span>
            <div>
              <strong>Redemption complete</strong>
              <small>The remaining cShares and claimed cUSDC are both still encrypted.</small>
            </div>
          </div>
          <div className="revealed-value">
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
            className="panel-action"
            type="button"
            disabled={revealingUsdc}
            onClick={revealedUsdc === null ? actions.revealRedeemedUsdc : actions.hideRedeemedUsdc}
          >
            {revealedUsdc === null ? 'Reveal current cUSDC balance' : 'Hide cUSDC'}
          </button>
        </div>
      )}

      {settled && (
        <details className="privacy-note">
          <summary>Privacy detail</summary>
          Settlement publishes the batch total. If you are the only participant, that total reveals your amount.
          Privacy strengthens with more independent participants.
        </details>
      )}

      {operatorError && (
        <ActionError>
          {operatorAction === 'claim' ? 'Receiving cUSDC' : 'Settlement'} is retrying automatically: {operatorError}
        </ActionError>
      )}
      {revealUsdcError && (
        <ActionError retryLabel="Retry reveal" onRetry={actions.revealRedeemedUsdc}>
          {revealUsdcError}
        </ActionError>
      )}
    </section>
  );
}
