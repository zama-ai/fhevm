import { renderToStaticMarkup } from 'react-dom/server';
import { act, create } from 'react-test-renderer';
import { describe, expect, test, vi } from 'vitest';

import { RedeemJourney } from './RedeemJourney';
import type { DemoController } from './useDemoController';

describe('RedeemJourney', () => {
  test('offers amount control without requiring yield to be fast-forwarded', () => {
    const controller = {
      state: {
        redeem: { kind: 'idle' },
        redeemLifecycle: null,
        redeemOperatorAction: null,
        redeemOperatorError: null,
        redeemClaiming: false,
        redeemClaimError: null,
        revealedShares: null,
      },
      derived: {
        connected: true,
        hasPrivateShares: false,
        hasConfidentialShares: true,
        redeemJoined: false,
      },
      actions: {
        redeem: vi.fn(),
      },
    } as unknown as DemoController;

    const markup = renderToStaticMarkup(<RedeemJourney controller={controller} />);
    expect(markup).toContain('Withdraw from the vault');
    expect(markup).toContain('type="range"');
    expect(markup).toContain('aria-label="Percentage to redeem"');
    expect(markup).toContain('value="50"');
    expect(markup).toContain('>Redeem</button>');
    expect(markup).not.toContain('Redeem half');
    expect(markup).not.toContain('Yield accrues');
  });

  test('submits the selected percentage', () => {
    const redeem = vi.fn();
    const controller = {
      state: {
        redeem: { kind: 'idle' },
        redeemLifecycle: null,
        redeemOperatorAction: null,
        redeemOperatorError: null,
        redeemClaiming: false,
        redeemClaimError: null,
        revealedShares: null,
      },
      derived: {
        connected: true,
        hasPrivateShares: true,
        hasConfidentialShares: true,
        redeemJoined: false,
      },
      actions: { redeem },
    } as unknown as DemoController;

    let renderer: ReturnType<typeof create>;
    act(() => {
      renderer = create(<RedeemJourney controller={controller} />);
    });
    const slider = renderer!.root.findByProps({ 'aria-label': 'Percentage to redeem' });
    act(() => slider.props.onChange({ target: { value: '25' } }));
    const button = renderer!.root.findAllByType('button').find((candidate) => candidate.children.includes('Redeem'));
    expect(button).toBeDefined();
    act(() => button!.props.onClick());
    expect(redeem).toHaveBeenCalledWith(25);
    act(() => renderer!.unmount());
  });

  test('disables redemption when the revealed cShare balance is zero', () => {
    const controller = {
      state: {
        redeem: { kind: 'idle' },
        redeemLifecycle: null,
        redeemOperatorAction: null,
        redeemOperatorError: null,
        revealedShares: { value: 0n, handle: '0x01' },
      },
      derived: {
        connected: true,
        hasConfidentialShares: true,
        redeemJoined: false,
      },
      actions: { redeem: vi.fn() },
    } as unknown as DemoController;

    const markup = renderToStaticMarkup(<RedeemJourney controller={controller} />);
    expect(markup).toContain('disabled=""');
    expect(markup).toContain('No cShares to redeem');
  });

  test('does not keep showing an in-progress action after redemption completes', () => {
    const controller = {
      state: {
        redeem: { kind: 'idle' },
        redeemLifecycle: null,
        completedRedeemLifecycle: {
          kind: 'settled',
          claimed: true,
          totalJoined: 50_000_000n,
          payoutReceived: 50_000_000n,
        },
        redeemOperatorAction: null,
        redeemOperatorError: null,
        revealedShares: null,
      },
      derived: {
        connected: true,
        hasPrivateShares: true,
        hasConfidentialShares: true,
        redeemJoined: false,
      },
      actions: {},
    } as unknown as DemoController;

    const markup = renderToStaticMarkup(<RedeemJourney controller={controller} />);
    expect(markup).toContain('Latest redemption');
    expect(markup).toContain('Redeem 50% of your position');
    expect(markup).not.toContain('Waiting for private settlement');
    expect(markup).not.toContain('Redemption joined</button>');
  });

  test('does not duplicate the completed timeline during the claimed-to-idle transition', () => {
    const controller = {
      state: {
        redeem: { kind: 'joined', result: {} },
        redeemLifecycle: {
          kind: 'settled',
          claimed: true,
          totalJoined: 50_000_000n,
          payoutReceived: 50_000_000n,
        },
        completedRedeemLifecycle: null,
        redeemOperatorAction: null,
        redeemOperatorError: null,
        revealedShares: null,
      },
      derived: {
        connected: true,
        hasPrivateShares: true,
        hasConfidentialShares: true,
        redeemJoined: true,
      },
      actions: {},
    } as unknown as DemoController;

    const markup = renderToStaticMarkup(<RedeemJourney controller={controller} />);
    expect(markup).toContain('Latest redemption');
    expect(markup).not.toContain('One signature · one transaction');
  });
});
