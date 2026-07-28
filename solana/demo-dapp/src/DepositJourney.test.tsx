import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, test, vi } from 'vitest';

import { DepositJourney } from './DepositJourney';
import type { DemoController } from './useDemoController';

describe('DepositJourney yield presentation', () => {
  test('keeps the assumed APY in the product UI and time travel in demo controls', () => {
    const fastForwardOneYear = vi.fn();
    const controller = {
      state: {
        depositLifecycle: {
          kind: 'settled',
          totalJoined: 100_000_000n,
          payoutReceived: 100_000_000n,
          claimed: true,
        },
        depositLifecycleError: null,
        depositOperatorAction: null,
        depositOperatorError: null,
        depositClaiming: false,
        depositClaimError: null,
        vaultMetrics: { totalAssets: 100_000_000n, totalShares: 100_000_000n },
        harvesting: false,
        harvestError: null,
        harvestFromPrice: null,
      },
      derived: {
        depositJoined: true,
        sharePrice: 1,
        yieldApplied: false,
      },
      actions: { fastForwardOneYear },
    } as unknown as DemoController;

    const markup = renderToStaticMarkup(<DepositJourney controller={controller} />);
    expect(markup).toContain('7.0% APY');
    expect(markup).toContain('Illustrative 30-day rate · annualized');
    expect(markup).toContain('Demo control');
    expect(markup).toContain('No wallet approval · local keeper demo action');
    expect(markup).toContain('Fast-forward 1 year');
    expect(markup).not.toContain('Simulate +25% yield');
  });
});
