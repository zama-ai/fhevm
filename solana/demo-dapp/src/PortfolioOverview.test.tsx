import { act, create, type ReactTestRenderer } from 'react-test-renderer';
import { beforeEach, describe, expect, test, vi } from 'vitest';

import type { BatchPosition } from './batchTypes';
import type { DemoSession } from './demoSession';
import { PortfolioOverview } from './PortfolioOverview';
import { initialDemoState, type DemoController } from './useDemoController';

const controller = (
  deposit: ReturnType<typeof vi.fn>,
  claimed: boolean,
): DemoController =>
  ({
    state: {
      ...initialDemoState,
      generation: 1,
      connection: { kind: 'ready', session: { wallet: { kind: 'burner' } } },
      deposit: claimed ? { kind: 'joined', result: {} } : { kind: 'idle' },
      depositLifecycle: claimed
        ? { kind: 'settled', totalJoined: 100_000_000n, payoutReceived: 100_000_000n, claimed: true }
        : null,
      depositClaiming: false,
      depositClaimError: null,
      hasConfidentialUsdc: true,
      hasConfidentialShares: claimed,
      publicUsdcBalance: 900_000_000n,
      revealedShares: null,
      revealingShares: false,
      revealSharesError: null,
    },
    derived: {
      connected: true,
      depositRunning: false,
      hasPrivateShares: claimed,
      hasConfidentialShares: claimed,
      sharePrice: 1,
    },
    actions: {
      deposit,
      revealUsdc: vi.fn(),
      hideUsdc: vi.fn(),
      revealShares: vi.fn(),
      hideShares: vi.fn(),
    },
  }) as unknown as DemoController;

describe('PortfolioOverview', () => {
  beforeEach(() => {
    Object.assign(globalThis, { IS_REACT_ACT_ENVIRONMENT: true });
    vi.clearAllMocks();
  });

  test('clears the completed amount and shows transient success feedback', () => {
    vi.useFakeTimers();
    const deposit = vi.fn();
    let renderer: ReturnType<typeof create>;
    act(() => {
      renderer = create(<PortfolioOverview controller={controller(deposit, false)} />);
    });
    const submit = renderer!.root.findAllByType('button').find((button) => button.children.includes('Deposit USDC'));
    act(() => submit!.props.onClick());
    expect(deposit).toHaveBeenCalledWith(100, 'usdc');

    act(() => {
      renderer!.update(<PortfolioOverview controller={controller(deposit, true)} />);
    });
    const status = renderer!.root.findByProps({ role: 'status' });
    expect(status.findByType('strong').children).toEqual(['Deposit complete · cShares received']);
    expect(renderer!.root.findByProps({ id: 'deposit-amount' }).props.value).toBe('');
    act(() => {
      vi.advanceTimersByTime(5_000);
    });
    expect(renderer!.root.findAllByProps({ role: 'status' })).toHaveLength(0);
    act(() => renderer!.unmount());
    vi.useRealTimers();
  });

  test('does not prefill a new deposit for an existing position', () => {
    const deposit = vi.fn();
    let renderer: ReturnType<typeof create>;
    act(() => {
      renderer = create(<PortfolioOverview controller={controller(deposit, true)} />);
    });
    expect(
      renderer!.root.findAllByType('h2').some((heading) => heading.children.includes('Deposit USDC')),
    ).toBe(true);
    expect(JSON.stringify(renderer!.toJSON())).not.toContain('Deposit more USDC');
    const input = renderer!.root.findByProps({ id: 'deposit-amount' });
    expect(input.props.value).toBe('');
    act(() => input.props.onChange({ target: { value: '25' } }));
    const submit = renderer!.root.findAllByType('button').find((button) => button.children.includes('Deposit USDC'));
    act(() => submit!.props.onClick());
    expect(deposit).toHaveBeenCalledWith(25, 'usdc');
    act(() => renderer!.unmount());
  });

  test('does not expose an unvalidated retry action after a deposit error', () => {
    const deposit = vi.fn();
    const value = controller(deposit, false);
    const errorController = {
      ...value,
      state: { ...value.state, deposit: { kind: 'error', message: 'Wallet request cancelled' } },
    } as DemoController;
    let renderer: ReturnType<typeof create>;
    act(() => {
      renderer = create(<PortfolioOverview controller={errorController} />);
    });

    expect(renderer!.root.findByProps({ role: 'alert' }).findAllByType('button')).toHaveLength(0);
    expect(
      renderer!.root.findAllByType('button').some((button) => button.children.includes('Deposit USDC')),
    ).toBe(true);
    act(() => renderer!.unmount());
  });

  test('offers a direct one-transaction cUSDC deposit when the account exists', () => {
    const deposit = vi.fn();
    const value = controller(deposit, true);
    let renderer: ReturnType<typeof create>;
    act(() => {
      renderer = create(
        <PortfolioOverview
          controller={{
            ...value,
            state: {
              ...value.state,
              revealedUsdc: { handle: '0xcurrent', value: 50_000_000n },
            },
          }}
        />,
      );
    });
    const cUsdcSource = renderer!.root.findByProps({ name: 'deposit-source', value: 'cusdc' });
    act(() => cUsdcSource.props.onChange());

    expect(renderer!.root.findAllByType('h2').some((heading) => heading.children.includes('Deposit cUSDC'))).toBe(
      true,
    );
    expect(renderer!.root.findByProps({ className: 'approval-count' }).children).toEqual(['1 transaction']);
    expect(renderer!.root.findByProps({ className: 'transaction-preview' }).children).toHaveLength(1);
    const input = renderer!.root.findByProps({ id: 'deposit-amount' });
    act(() => input.props.onChange({ target: { value: '25' } }));
    const submit = renderer!.root.findAllByType('button').find((button) => button.children.includes('Deposit cUSDC'));
    act(() => submit!.props.onClick());
    expect(deposit).toHaveBeenCalledWith(25, 'cusdc');
    act(() => renderer!.unmount());
  });

  test('reveals cUSDC before allowing a direct deposit', () => {
    const deposit = vi.fn();
    const value = controller(deposit, true);
    let renderer: ReturnType<typeof create>;
    act(() => {
      renderer = create(<PortfolioOverview controller={value} />);
    });
    const cUsdcSource = renderer!.root.findByProps({ name: 'deposit-source', value: 'cusdc' });
    act(() => cUsdcSource.props.onChange());

    expect(renderer!.root.findByProps({ className: 'approval-count' }).children).toEqual([
      '1 signature · 1 transaction',
    ]);
    const reveal = renderer!.root
      .findAllByType('button')
      .find((button) => button.children.includes('Decrypt cUSDC balance to continue'));
    act(() => reveal!.props.onClick());
    expect(value.actions.revealUsdc).toHaveBeenCalledOnce();
    expect(deposit).not.toHaveBeenCalled();
    act(() => renderer!.unmount());
  });

  test('keeps cUSDC selected while account state refreshes', () => {
    const deposit = vi.fn();
    const value = controller(deposit, true);
    let renderer: ReturnType<typeof create>;
    act(() => {
      renderer = create(<PortfolioOverview controller={value} />);
    });
    act(() => renderer!.root.findByProps({ name: 'deposit-source', value: 'cusdc' }).props.onChange());
    act(() => {
      renderer!.update(
        <PortfolioOverview
          controller={{
            ...value,
            state: { ...value.state, hasConfidentialUsdc: null },
          }}
        />,
      );
    });

    expect(renderer!.root.findAllByType('h2').some((heading) => heading.children.includes('Deposit cUSDC'))).toBe(
      true,
    );
    expect(renderer!.root.findByProps({ name: 'deposit-source', value: 'usdc' })).toBeDefined();
    act(() => renderer!.unmount());
  });

  test('blocks a cUSDC amount above the revealed balance', () => {
    const deposit = vi.fn();
    const value = controller(deposit, true);
    let renderer: ReturnType<typeof create>;
    act(() => {
      renderer = create(
        <PortfolioOverview
          controller={{
            ...value,
            state: {
              ...value.state,
              revealedUsdc: { handle: '0xcurrent', value: 10_000_000n },
            },
          }}
        />,
      );
    });
    act(() => renderer!.root.findByProps({ name: 'deposit-source', value: 'cusdc' }).props.onChange());
    const input = renderer!.root.findByProps({ id: 'deposit-amount' });
    act(() => input.props.onChange({ target: { value: '25' } }));
    const submit = renderer!.root.findAllByType('button').find((button) => button.children.includes('Deposit cUSDC'));

    expect(submit!.props.disabled).toBe(true);
    expect(renderer!.root.findByProps({ role: 'alert' }).children.join('')).toContain('too low');
    expect(deposit).not.toHaveBeenCalled();
    act(() => renderer!.unmount());
  });
});

const actions = {
  hideShares: vi.fn(),
  hideUsdc: vi.fn(),
  revealShares: vi.fn(),
  revealUsdc: vi.fn(),
  deposit: vi.fn(),
};

const walletController = (
  wallet: DemoSession['wallet'],
  deposit: DemoController['state']['deposit'] = { kind: 'idle' },
): DemoController =>
  ({
    state: {
      ...initialDemoState,
      connection: { kind: 'ready', session: { wallet } as DemoSession },
      deposit,
      hasConfidentialShares: false,
    },
    derived: {
      connected: true,
      depositJoined: deposit.kind === 'joined',
      depositRunning: deposit.kind === 'running',
      hasPrivateShares: false,
      hasConfidentialShares: false,
      sharePrice: null,
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
    const renderer = render(walletController({ kind: 'burner', name: 'Demo wallet' }));
    const metrics = renderer.root.findByProps({ className: 'vault-stats' });
    expect(metrics.findAllByType('strong').some((node) => node.children.join('') === '7.0%')).toBe(true);
    expect(
      metrics.findAllByType('small').some((node) => node.children.join('') === '30-day average · annualized'),
    ).toBe(true);
    act(() => renderer.unmount());
  });

  test('keeps public, shielded, and vault balances distinct', () => {
    const value = walletController({ kind: 'burner', name: 'Demo wallet' });
    const renderer = render({
      ...value,
      state: {
        ...value.state,
        publicUsdcBalance: 900_000_000n,
        hasConfidentialUsdc: true,
        hasConfidentialShares: true,
        hasPrivateShares: true,
      },
      derived: { ...value.derived, hasPrivateShares: true, hasConfidentialShares: true },
    });

    const inventory = renderer.root.findByProps({ className: 'asset-inventory' });
    expect(inventory.findAllByType('span').map((node) => node.children.join(''))).toEqual(
      expect.arrayContaining(['Wallet · Public', 'Shielded balance · Private', 'Vault position · Private']),
    );
    expect(inventory.findAllByType('strong').map((node) => node.children.join(''))).toEqual(
      expect.arrayContaining(['900 USDC', '•••• cUSDC', '•••• cShares']),
    );
    const revealButtons = inventory.findAllByType('button');
    expect(revealButtons).toHaveLength(2);
    act(() => revealButtons[0].props.onClick());
    act(() => revealButtons[1].props.onClick());
    expect(actions.revealUsdc).toHaveBeenCalledOnce();
    expect(actions.revealShares).toHaveBeenCalledOnce();
    act(() => renderer.unmount());
  });

  test('disables private balance actions while an automatic claim is running', () => {
    const value = walletController({ kind: 'burner', name: 'Demo wallet' });
    const renderer = render({
      ...value,
      state: {
        ...value.state,
        hasConfidentialUsdc: true,
        hasConfidentialShares: true,
        depositOperatorAction: 'claim',
      },
      derived: { ...value.derived, hasConfidentialShares: true },
    });

    const inventory = renderer.root.findByProps({ className: 'asset-inventory' });
    expect(inventory.findAllByType('button').every((button) => button.props.disabled)).toBe(true);
    act(() => renderer.unmount());
  });

  test.each([
    ['idle', { kind: 'idle' }],
    ['error', { kind: 'error', message: 'Wallet request cancelled' }],
  ] as const)('shows passive developer-mode guidance while the Phantom deposit is %s', (_name, deposit) => {
    const renderer = render(walletController(phantom, deposit));
    const notes = renderer.root.findAll((node) => node.props.className === 'wallet-scan-note');

    expect(notes).toHaveLength(1);
    expect(notes[0].findAllByType('button')).toHaveLength(0);
    expect(notes[0].findAllByType('a')).toHaveLength(0);
    expect(notes[0].children.join(' ')).toContain('its scanner cannot reach this local validator');
    expect(renderer.root.findByProps({ className: 'approval-count' }).children).toEqual(['Up to 2 approvals']);
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
      'a running Phantom deposit',
      phantom,
      { kind: 'running', stage: 'shielding' } as const,
    ],
    [
      'a completed Phantom deposit',
      phantom,
      { kind: 'joined', result: {} as BatchPosition } as const,
    ],
  ])('does not show the localnet note for %s', (_name, wallet, deposit) => {
    const renderer = render(walletController(wallet, deposit as DemoController['state']['deposit']));

    expect(renderer.root.findAll((node) => node.props.className === 'wallet-scan-note')).toHaveLength(0);
    act(() => renderer.unmount());
  });
});
