import { ActionError, ClaimPanel, JourneyTimeline, SettlementProgress } from './JourneyPrimitives';
import type { RedeemStage } from './redeem';
import { formatUsdc } from './format';
import type { DemoController } from './useDemoController';

const stageCopy: Record<'decrypting' | RedeemStage, string> = {
  decrypting: 'Authorizing a one-time balance reveal…',
  proving: 'Creating your private 50% redeem proof…',
  joining: 'Signing one private redeem transaction…',
  joined: 'Redemption joined · Waiting for private settlement',
};

export function RedeemJourney({ controller }: { readonly controller: DemoController }) {
  const { state, derived, actions } = controller;
  const {
    redeem,
    redeemLifecycle: lifecycle,
    redeemOperatorAction: operatorAction,
    redeemOperatorError: operatorError,
    redeemClaiming: claiming,
    redeemClaimError: claimError,
    revealedShares,
    revealedUsdc,
    revealingUsdc,
    revealUsdcError,
  } = state;
  const { sharesClaimed, yieldApplied, redeemJoined } = derived;
  const settled = lifecycle?.kind === 'settled';
  if (!sharesClaimed) return null;

  return (
    <section className="redeem-card">
      <div className="redeem-heading">
        <div>
          <p className="eyebrow">Private exit, same Solana rhythm</p>
          <h2>Redeem half your position</h2>
          <p>
            A one-time balance signature calculates exactly 50%, then one transaction joins the private redeem batch.
            The clear balance is remasked immediately.
          </p>
        </div>
        <div className="redeem-amount">
          <span>Intent</span>
          <strong>50%</strong>
          <small>of private cShares</small>
        </div>
      </div>

      <JourneyTimeline
        framed
        steps={[
          {
            state: yieldApplied ? 'complete' : 'active',
            title: 'Yield accrues',
            detail: 'Public share price rises on-chain',
          },
          {
            state: redeemJoined ? 'complete' : yieldApplied ? 'active' : 'idle',
            title: 'Redeem half privately',
            detail: 'One signature · one transaction',
          },
          {
            state: settled ? 'complete' : redeemJoined ? 'active' : 'idle',
            title: settled ? 'Settled on Solana' : 'Automatic settlement',
            detail: settled ? 'Batch total is now public' : 'Your redemption stays private',
          },
          {
            state: settled && lifecycle.claimed ? 'complete' : settled ? 'active' : 'idle',
            title: 'Claim private cUSDC',
            detail: 'Redeemed value stays encrypted',
          },
        ]}
      />

      <div className="redeem-action">
        <div>
          <strong>
            {redeem.kind === 'running'
              ? stageCopy[redeem.stage]
              : redeemJoined
                ? stageCopy.joined
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
          className="panel-action"
          type="button"
          disabled={!yieldApplied || redeem.kind === 'running' || redeemJoined}
          onClick={actions.redeemHalf}
        >
          {redeem.kind === 'running'
            ? 'Redeeming half…'
            : redeemJoined
              ? 'Redemption joined'
              : 'Redeem half privately'}
        </button>
      </div>

      {redeem.kind === 'error' && (
        <ActionError retryLabel="Retry redeem" onRetry={actions.redeemHalf}>
          {redeem.message}
        </ActionError>
      )}

      {(lifecycle?.kind === 'awaiting-dispatch' || lifecycle?.kind === 'proving') && (
        <SettlementProgress
          lifecycle={lifecycle}
          action={operatorAction}
        />
      )}

      {settled && !lifecycle.claimed && (
        <ClaimPanel
          title="Claim your private cUSDC"
          detail="One transaction. The redeemed value remains encrypted."
          label="Claim private cUSDC"
          busyLabel="Claiming private cUSDC…"
          busy={claiming}
          onClaim={() => actions.claim('redeem')}
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
              <strong>Half redeemed and cUSDC claimed privately</strong>
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

      {operatorError && <ActionError>Settlement is retrying automatically: {operatorError}</ActionError>}
      {claimError && (
        <ActionError retryLabel="Retry claim" onRetry={() => actions.claim('redeem')}>
          {claimError}
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
