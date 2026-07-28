import { act, create, type ReactTestRenderer } from 'react-test-renderer';
import { beforeEach, describe, expect, test, vi } from 'vitest';

import type { BatchPosition } from './batchTypes';
import type { DemoSession } from './demoSession';
import { PortfolioOverview } from './PortfolioOverview';
import { initialDemoState, type DemoController } from './useDemoController';

const actions = {
  hideShares: vi.fn(),
  revealShares: vi.fn(),
  shieldAndDeposit: vi.fn(),
};

const controller = (
  wallet: DemoSession['wallet'],
  deposit: DemoController['state']['deposit'] = { kind: 'idle' },
): DemoController =>
  ({
    state: {
      ...initialDemoState,
      connection: { kind: 'ready', session: { wallet } as DemoSession },
      deposit,
    },
    derived: {
      connected: true,
      depositJoined: deposit.kind === 'joined',
      depositRunning: deposit.kind === 'running',
    },
    actions,
  }) as unknown as DemoController;

const render = (value: DemoController): ReactTestRenderer => {
  let renderer!: ReactTestRenderer;
  act(() => {
    renderer = create(<PortfolioOverview controller={value} />);
  });
  return renderer;
};

const phantom = {
  kind: 'wallet-standard',
  name: 'Phantom',
  accountKey: 'phantom-account',
} as const;

describe('PortfolioOverview Phantom localnet guidance', () => {
  beforeEach(() => {
    Object.assign(globalThis, { IS_REACT_ACT_ENVIRONMENT: true });
    vi.clearAllMocks();
  });

  test('advertises the demo APY before deposit', () => {
    const renderer = render(controller({ kind: 'burner', name: 'Demo wallet' }));
    expect(
      renderer.root
        .findAllByType('p')
        .some((node) => node.children.join('') === '7.0% demo APY · 30-day rate, annualized'),
    ).toBe(true);
    act(() => renderer.unmount());
  });

  test.each([
    ['idle', { kind: 'idle' }],
    ['running', { kind: 'running', stage: 'shielding' }],
    ['error', { kind: 'error', message: 'Wallet request cancelled' }],
  ] as const)('shows passive developer-mode guidance while the Phantom deposit is %s', (_name, deposit) => {
    const renderer = render(controller(phantom, deposit));
    const notes = renderer.root.findAll((node) => node.props.className === 'wallet-scan-note');

    expect(notes).toHaveLength(1);
    expect(notes[0].findAllByType('button')).toHaveLength(0);
    expect(notes[0].findAllByType('a')).toHaveLength(0);
    expect(notes[0].children.join(' ')).toContain("Phantom's scanner cannot reach this local validator");
    const walletFlow = renderer.root.findByProps({ className: 'vault-metric' });
    expect(walletFlow.findByType('strong').children).toEqual(['2 required']);
    act(() => renderer.unmount());
  });

  test.each([
    ['the demo wallet', { kind: 'burner', name: 'Demo wallet' } as const, { kind: 'idle' } as const],
    [
      'another Wallet Standard wallet',
      { kind: 'wallet-standard', name: 'Solflare', accountKey: 'solflare-account' } as const,
      { kind: 'idle' } as const,
    ],
    [
      'a completed Phantom deposit',
      phantom,
      { kind: 'joined', result: {} as BatchPosition } as const,
    ],
  ])('does not show the localnet note for %s', (_name, wallet, deposit) => {
    const renderer = render(controller(wallet, deposit as DemoController['state']['deposit']));

    expect(renderer.root.findAll((node) => node.props.className === 'wallet-scan-note')).toHaveLength(0);
    act(() => renderer.unmount());
  });
});
