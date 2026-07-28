import { beforeEach, describe, expect, test } from 'vitest';
import type { Signature } from '@solana/kit';

import type { DemoSession } from './demoSession';
import { readTransactionEvidence, recordTransactionEvidence } from './evidenceStore';

const session = {
  config: { demoBootId: 'boot-a', chainId: 'localnet' },
  signer: { address: '11111111111111111111111111111111' },
} as unknown as DemoSession;
const transactionSignature =
  '5h6xBEauJ3PK6WsSJZuHmEdmHdGzXJnbccQkWc9s3E8fPJ8mgLPJgGbu49Bv4J2M5z7yV1ycK4XoLZ4qsVQxHzDP' as Signature;

describe('public transaction evidence', () => {
  beforeEach(() => {
    const storage = new Map<string, string>();
    Object.assign(globalThis, {
      localStorage: {
        getItem: (key: string) => storage.get(key) ?? null,
        setItem: (key: string, value: string) => storage.set(key, value),
      },
    });
  });

  test('keeps the latest label for a signature without duplicating it', () => {
    recordTransactionEvidence(session, { label: 'Deposit dispatch', signature: transactionSignature });
    recordTransactionEvidence(session, { label: 'Deposit settle', signature: transactionSignature });

    expect(readTransactionEvidence(session)).toEqual([
      { label: 'Deposit settle', signature: transactionSignature },
    ]);
  });

  test('ignores malformed stored evidence', () => {
    localStorage.setItem('fhevm-solana-demo:evidence:boot-a:localnet:11111111111111111111111111111111', '{');
    expect(readTransactionEvidence(session)).toEqual([]);
  });

  test('ignores a stored value that is not a Solana signature', () => {
    localStorage.setItem(
      'fhevm-solana-demo:evidence:boot-a:localnet:11111111111111111111111111111111',
      JSON.stringify([{ label: 'Deposit', signature: 'not-a-signature' }]),
    );
    expect(readTransactionEvidence(session)).toEqual([]);
  });

  test('isolates evidence from a previous demo boot', () => {
    recordTransactionEvidence(session, {
      label: 'Deposit',
      signature: transactionSignature,
    });

    const restarted = {
      ...session,
      config: { ...session.config, demoBootId: 'boot-b' },
    } as DemoSession;
    expect(readTransactionEvidence(restarted)).toEqual([]);
  });
});
