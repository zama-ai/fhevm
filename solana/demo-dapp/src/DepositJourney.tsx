import { ActionError, JourneyTimeline, SettlementProgress } from './JourneyPrimitives';
import { formatUsdc } from './format';
import type { DemoController } from './useDemoController';

export function DepositJourney({ controller }: { readonly controller: DemoController }) {
  const { state, derived, actions } = controller;
  const {
    depositLifecycle: lifecycle,
    depositLifecycleError: lifecycleError,
    depositOperatorAction: operatorAction,
    depositOperatorError: operatorError,
    vaultMetrics,
    harvesting,
    harvestError,
    harvestFromPrice,
  } = state;
  const { depositJoined, hasPrivateShares, sharePrice } = derived;
  if (!depositJoined && !hasPrivateShares) return null;

  const settled = lifecycle?.kind === 'settled';
  const complete = settled && lifecycle.claimed;
  const activityLabel = complete
    ? 'Deposit complete'
    : lifecycle?.kind === 'proving'
      ? 'Settlement in progress'
      : settled
        ? 'Receiving cShares'
        : 'Deposit in progress';

  return (
    <>
      {depositJoined && (
        <details className="activity-ledger" open={!complete}>
          <summary>
            <span>
              <strong>Latest activity</strong>
              <small>{activityLabel}</small>
            </span>
            <span className={`activity-state${complete ? ' complete' : ''}`}>{complete ? 'Completed' : 'Live'}</span>
          </summary>
          <div className="activity-content">
            <JourneyTimeline
              steps={[
                {
                  state: depositJoined ? 'complete' : 'active',
                  title: 'Deposit',
                  detail: 'Wallet submission complete',
                },
                {
                  state: settled ? 'complete' : depositJoined ? 'active' : 'idle',
                  title: settled ? 'Settled on Solana' : 'Private settlement',
                  detail: settled ? 'Batch total published' : 'Automatic',
                },
                {
                  state: complete ? 'complete' : settled ? 'active' : 'idle',
                  title: complete ? 'cShares received' : 'Receiving cShares',
                  detail: complete ? 'Balance stays private' : 'Automatic',
                },
              ]}
            />

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
              <SettlementProgress lifecycle={lifecycle} action={operatorAction} />
            )}

            {operatorError && (
              <ActionError>
                {operatorAction === 'claim' ? 'Receiving cShares' : 'Settlement'} is retrying automatically:{' '}
                {operatorError}
              </ActionError>
            )}
            {lifecycleError && !operatorError && (
              <ActionError>Live batch status is temporarily unavailable: {lifecycleError}</ActionError>
            )}

            {settled && (
              <details className="privacy-note">
                <summary>Privacy detail</summary>
                Settlement publishes the batch total. If you are the only participant, that total reveals your amount.
                Privacy strengthens with more independent participants.
              </details>
            )}
          </div>
        </details>
      )}

      {hasPrivateShares && (
        <details className="demo-tools">
          <summary>Demo controls</summary>
          <div>
            <span>
              <strong>Fast-forward vault time</strong>
              <small>
                {harvestFromPrice !== null && sharePrice !== null
                  ? `${harvestFromPrice.toFixed(2)} → ${sharePrice?.toFixed(2)} USDC`
                  : 'Applies one year of demo yield without a wallet approval.'}
              </small>
            </span>
            <button
              className="panel-action"
              type="button"
              disabled={vaultMetrics === null || vaultMetrics.totalShares === 0n || harvesting}
              onClick={actions.fastForwardOneYear}
            >
              {harvesting ? 'Fast-forwarding…' : 'Fast-forward 1 year'}
            </button>
          </div>
          {harvestError && (
            <ActionError retryLabel="Retry" onRetry={actions.fastForwardOneYear}>
              {harvestError}
            </ActionError>
          )}
        </details>
      )}
    </>
  );
}
