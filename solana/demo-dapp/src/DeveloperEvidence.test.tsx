import { act, create, type ReactTestRenderer } from 'react-test-renderer';
import { expect, test, vi } from 'vitest';

import { DeveloperEvidence } from './DeveloperEvidence';
import { initialDemoState, type DemoController } from './useDemoController';

vi.mock('@solana/kit', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@solana/kit')>()),
  createSolanaRpc: () => ({
    getSignaturesForAddress: () => ({ send: async () => [] }),
    getTransaction: () => ({ send: async () => null }),
  }),
}));
vi.mock('./evidenceStore', () => ({
  readTransactionEvidence: () => [{ label: 'Deposit', signature: '1'.repeat(64) }],
  readDecryptionEvidence: () => [],
}));
vi.mock('./revealShares', () => ({
  readConfidentialBalanceEvidence: async () => {
    throw new Error('account does not exist');
  },
}));

test('keeps the recorded signature visible when the RPC has pruned the transaction', async () => {
  Object.assign(globalThis, { IS_REACT_ACT_ENVIRONMENT: true });
  const controller = {
    state: {
      ...initialDemoState,
      connection: {
        kind: 'ready',
        session: {
          signer: { address: '1'.repeat(32) },
          config: { rpcUrl: 'http://127.0.0.1:8899', mints: {} },
          assertActive: vi.fn(),
        },
      },
    },
  } as unknown as DemoController;
  let renderer: ReactTestRenderer;
  await act(async () => {
    renderer = create(<DeveloperEvidence controller={controller} />);
  });
  await act(async () => {
    renderer!.root.findByProps({ className: 'developer-evidence' }).props.onToggle({
      currentTarget: { open: true },
    });
  });
  const activity = renderer!.root.findByProps({ className: 'evidence-transactions' });
  expect(activity.findByType('small').children.join('')).toBe('Unavailable from RPC');
  expect(activity.findByType('a').props.href).toContain(`/tx/${'1'.repeat(64)}?`);
  expect(activity.findByProps({ 'aria-label': 'Copy transaction signature' })).toBeDefined();
  expect(activity.findAllByType('details')).toHaveLength(0);
  await act(async () => renderer!.unmount());
});
