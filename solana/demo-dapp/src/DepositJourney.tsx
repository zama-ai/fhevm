import { ActionError, ClaimPanel, JourneyTimeline, OperatorPanel } from './JourneyPrimitives';
import { formatUsdc } from './format';
import type { DemoController } from './useDemoController';

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
        <p className="eyebrow">One intent, clearly tracked</p>
        <h2>Deposit journey</h2>
      </div>
      <JourneyTimeline
        steps={[
          {
            state: depositJoined ? 'complete' : 'active',
            title: 'Shield & join',
            detail: 'One click · two sequential transactions',
          },
          {
            state: settled ? 'complete' : depositJoined ? 'active' : 'idle',
            title: settled
              ? 'KMS settlement verified'
              : lifecycle?.kind === 'proving'
                ? lifecycle.proofReady
                  ? 'Private proof ready'
                  : 'Proving privately'
                : depositJoined
                  ? 'Awaiting dispatch'
                  : 'Private batch settles',
            detail: settled ? 'Public total revealed on-chain' : 'Your contribution remains masked',
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
            <OperatorPanel
              lifecycle={lifecycle}
              action={operatorAction}
              copy={{
                awaitingTitle: 'The local keeper advances this permissionless batch.',
                provingPendingTitle: 'Proving the encrypted batch total…',
                provingReadyTitle: 'Private proof is ready.',
                provingReadyDetail: 'The KMS certificate can now be verified on Solana.',
                settleLabel: 'Settle with KMS certificate',
              }}
              onAction={(action) => actions.runOperator('deposit', action)}
            />
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
                className="panel-action"
                type="button"
                disabled={vaultMetrics === null || harvesting || yieldApplied}
                onClick={actions.applyDemoYield}
              >
                {harvesting ? 'Applying yield…' : yieldApplied ? 'Yield applied' : 'Simulate +25% yield'}
              </button>
            </div>
          )}

          {operatorError && <ActionError>{operatorError}</ActionError>}
          {lifecycleError && !operatorError && (
            <ActionError>Live batch status is temporarily unavailable: {lifecycleError}</ActionError>
          )}
          {claimError && (
            <ActionError retryLabel="Retry claim" onRetry={() => actions.claim('deposit')}>
              {claimError}
            </ActionError>
          )}
          {harvestError && (
            <ActionError retryLabel="Retry yield" onRetry={actions.applyDemoYield}>
              {harvestError}
            </ActionError>
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
  );
}
