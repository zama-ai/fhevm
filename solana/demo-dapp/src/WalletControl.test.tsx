import { act, create, type ReactTestRenderer } from 'react-test-renderer';
import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';
import type { UiWallet } from '@wallet-standard/react';

const installedWallets = vi.hoisted(() => ({ current: [] as readonly UiWallet[] }));
vi.mock('@wallet-standard/react', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@wallet-standard/react')>()),
  useWallets: () => installedWallets.current,
}));

import { testUiWallet } from './testWallet';
import { WalletControl } from './WalletControl';

const NO_WALLET_NOTE = 'No installed wallet signs Solana version 1 transactions yet.';

const render = (): ReactTestRenderer => {
  let renderer!: ReactTestRenderer;
  act(() => {
    renderer = create(
      <WalletControl
        connection={{ kind: 'disconnected' }}
        disabled={false}
        onBurnerConnect={vi.fn()}
        onConnect={vi.fn()}
        onDisconnect={vi.fn()}
      />,
    );
  });
  return renderer;
};

const texts = (renderer: ReactTestRenderer): string[] =>
  renderer.root.findAll((node) => typeof node.type === 'string').map((node) => node.children.filter((child) => typeof child === 'string').join(''));

describe('WalletControl', () => {
  beforeEach(() => Object.assign(globalThis, { IS_REACT_ACT_ENVIRONMENT: true }));
  afterEach(() => {
    installedWallets.current = [];
  });

  test('lists only installed wallets that sign version 1 transactions, next to the demo wallet', () => {
    installedWallets.current = [
      testUiWallet({ name: 'Legacy wallet', supportedTransactionVersions: ['legacy', 0] }),
      testUiWallet({ name: 'V1 wallet', supportedTransactionVersions: [0, 1] }),
      testUiWallet({ name: 'Full wallet', supportedTransactionVersions: ['legacy', 0, 1] }),
    ];
    const renderer = render();

    expect(texts(renderer)).toEqual(
      expect.arrayContaining(['Connect V1 wallet', 'Connect Full wallet', 'Demo wallet']),
    );
    expect(texts(renderer)).not.toContain('Connect Legacy wallet');
    expect(texts(renderer)).not.toContain(NO_WALLET_NOTE);
    act(() => renderer.unmount());
  });

  test('says no installed wallet qualifies and offers the demo wallet', () => {
    installedWallets.current = [testUiWallet({ name: 'Legacy wallet', supportedTransactionVersions: ['legacy', 0] })];
    const renderer = render();

    expect(texts(renderer)).toEqual(expect.arrayContaining([NO_WALLET_NOTE, 'Start demo']));
    expect(texts(renderer).some((text) => text.startsWith('Connect'))).toBe(false);
    act(() => renderer.unmount());
  });
});
