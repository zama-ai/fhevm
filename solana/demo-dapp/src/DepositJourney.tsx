import { ActionError, ClaimPanel, JourneyTimeline, SettlementProgress } from './JourneyPrimitives';
import { formatUsdc } from './format';
import type { DemoController } from './useDemoController';
import { DEMO_APY_PERCENT, DEMO_RATE_WINDOW_DAYS } from './yieldPolicy';

export function DepositJourney({ controller }: { readonly controller: DemoController }) {
  const { state, derived, actions } = controller;
  const {
    depositLifecycle: lifecycle,
    depositLifecycleError: lifecycleError,
    depositOperatorAction: operatorAction,
    depositOperatorError: operatorError,
    depositClaiming: claiming,
    depositClaimError: claimError,
    vaultMetrics,
    harvesting,
    harvestError,
    harvestFromPrice,
  } = state;
  const { depositJoined, sharePrice, yieldApplied } = derived;
  const settled = lifecycle?.kind === 'settled';

  return (
    <section className="journey-card">
      <div>
        <h2>Your deposit</h2>
      </div>
      <JourneyTimeline
        steps={[
          {
            state: depositJoined ? 'complete' : 'active',
            title: 'Shield & deposit',
            detail: 'Two transactions',
          },
          {
            state: settled ? 'complete' : depositJoined ? 'active' : 'idle',
            title: settled
              ? 'Settled on Solana'
              : lifecycle?.kind === 'proving'
                ? lifecycle.proofReady
                  ? 'Verifying on Solana'
                  : 'Processing privately'
                : depositJoined
                  ? 'Waiting for batch close'
                  : 'Automatic settlement',
            detail: settled ? 'Batch total is now public' : 'Your contribution stays private',
          },
          {
            state: settled ? (lifecycle.claimed ? 'complete' : 'active') : 'idle',
            title: settled ? (lifecycle.claimed ? 'cShares received' : 'Claim available') : 'Claim cShares',
            detail: settled && lifecycle.claimed ? 'Your shares remain private' : 'Your payout remains encrypted',
          },
        ]}
      />
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

          {(lifecycle?.kind === 'awaiting-dispatch' || lifecycle?.kind === 'proving') && (
            <SettlementProgress
              lifecycle={lifecycle}
              action={operatorAction}
            />
          )}

          {settled && (
            <div className="verified-settlement" role="status">
              <span>✓</span>
              <div>
                <strong>Settlement verified on Solana</strong>
                <small>
                  {lifecycle.claimed
                    ? 'Your claimed cShares remain encrypted.'
                    : 'Your cShares payout is available to claim and remains encrypted.'}
                </small>
              </div>
            </div>
          )}

          {settled && !lifecycle.claimed && (
            <ClaimPanel
              title="Claim your private cShares"
              detail="One user transaction. The received balance remains encrypted."
              label="Claim private cShares"
              busyLabel="Claiming encrypted shares…"
              busy={claiming}
              onClaim={() => actions.claim('deposit')}
            />
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
                <span className="operator-label">Demo vault yield</span>
                <strong>{DEMO_APY_PERCENT.toFixed(1)}% APY</strong>
                <small>Illustrative {DEMO_RATE_WINDOW_DAYS}-day rate · annualized</small>
              </div>
              <div className="yield-price">
                <span>Share price</span>
                <strong>{sharePrice === null ? 'Reading…' : `${sharePrice.toFixed(2)} USDC`}</strong>
                <small>{yieldApplied ? 'Yield reflected on-chain' : 'Live vault ratio'}</small>
              </div>
            </div>
          )}

          {settled && lifecycle.claimed && (
            <div className="demo-controls">
              <div>
                <span className="operator-label">Demo control</span>
                <strong>Fast-forward vault time</strong>
                <small>
                  {harvestFromPrice !== null && yieldApplied
                    ? `${harvestFromPrice.toFixed(2)} → ${sharePrice?.toFixed(2)} · assets rose, share supply stayed fixed`
                    : 'No wallet approval · local keeper demo action'}
                </small>
              </div>
              <button
                className="panel-action"
                type="button"
                disabled={vaultMetrics === null || harvesting || yieldApplied}
                onClick={actions.fastForwardOneYear}
              >
                {harvesting
                  ? 'Fast-forwarding…'
                  : yieldApplied
                    ? '1 year of demo yield applied'
                    : 'Fast-forward 1 year'}
              </button>
            </div>
          )}

          {operatorError && <ActionError>Settlement is retrying automatically: {operatorError}</ActionError>}
          {lifecycleError && !operatorError && (
            <ActionError>Live batch status is temporarily unavailable: {lifecycleError}</ActionError>
          )}
          {claimError && (
            <ActionError retryLabel="Retry claim" onRetry={() => actions.claim('deposit')}>
              {claimError}
            </ActionError>
          )}
          {harvestError && (
            <ActionError retryLabel="Retry fast-forward" onRetry={actions.fastForwardOneYear}>
              {harvestError}
            </ActionError>
          )}

          {settled && (
            <details className="privacy-note">
              <summary>Privacy detail</summary>
              Settlement publishes the batch total. If you are the only participant, that total reveals your amount.
              Privacy strengthens with more independent participants.
            </details>
          )}
        </div>
      )}
    </section>
  );
}
