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
        revealedUsdc: null,
        revealingUsdc: false,
        revealUsdcError: null,
      },
      derived: {
        connected: true,
        hasPrivateShares: true,
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
        revealedUsdc: null,
        revealingUsdc: false,
        revealUsdcError: null,
      },
      derived: {
        connected: true,
        hasPrivateShares: true,
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
});
