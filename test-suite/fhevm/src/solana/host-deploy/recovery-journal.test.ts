import { test, expect } from 'bun:test';
import { mkdtemp, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import type { Signature } from '@solana/kit';
import { journalRecovery } from '../../../../../solana/deploy/src/recovery-journal';
import type { HostDeployContext } from '../../../../../solana/deploy/src/send';

test('a fresh recovery process retains receipts from an interrupted attempt', async () => {
  const directory = await mkdtemp(path.join(tmpdir(), 'recovery-journal-'));
  const namespace = process.env.SOLANA_PREVIEW_NAMESPACE;
  delete process.env.SOLANA_PREVIEW_NAMESPACE;
  try {
    const first = { rpc: {}, sendTransaction: async () => {} } as unknown as HostDeployContext;
    const journal = await journalRecovery(first, directory);
    await journal.persist('inventory-recovery.json', JSON.stringify({ mints: ['public-mint'] }));
    await first.beforeSubmit!('first-transaction' as Signature, 100n);
    // No report was written: simulate interruption immediately after submission.
    const next = { rpc: {}, sendTransaction: async () => {} } as unknown as HostDeployContext;
    const resumed = await journalRecovery(next, directory);
    await next.beforeSubmit!('second-transaction' as Signature, 200n);
    expect(await resumed.receipts()).toEqual(expect.arrayContaining([
      { signature: 'first-transaction', lastValidBlockHeight: '100' },
      { signature: 'second-transaction', lastValidBlockHeight: '200' },
    ]));
    expect((await resumed.receipts()).length).toBe(2);
  } finally {
    if (namespace !== undefined) process.env.SOLANA_PREVIEW_NAMESPACE = namespace;
    await rm(directory, { recursive: true, force: true });
  }
});
