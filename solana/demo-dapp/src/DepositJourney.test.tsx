import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, test, vi } from 'vitest';

import { DepositJourney } from './DepositJourney';
import type { DemoController } from './useDemoController';

describe('DepositJourney activity presentation', () => {
  test('keeps completed deposit events compact and separates demo controls', () => {
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
        hasPrivateShares: true,
        sharePrice: 1,
        yieldApplied: false,
      },
      actions: { fastForwardOneYear },
    } as unknown as DemoController;

    const markup = renderToStaticMarkup(<DepositJourney controller={controller} />);
    expect(markup).toContain('Latest activity');
    expect(markup).toContain('Deposit complete');
    expect(markup).toContain('Completed');
    expect(markup).toContain('Demo controls');
    expect(markup).toContain('Applies one year of demo yield without a wallet approval.');
    expect(markup).toContain('Fast-forward 1 year');
    expect(markup).not.toContain('Settlement verified on Solana');
    expect(markup).not.toContain('cShares received privately');
    expect(markup).not.toContain('7.0% APY');
  });
});
