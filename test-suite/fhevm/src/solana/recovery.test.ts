import { test, expect } from 'bun:test';
import { mkdtemp, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { generateSolanaKeypair } from './provision';
import { requireRecoverableWallet } from './recovery';

test('funding requires a matching private recovery copy and a successful persistent mirror', async () => {
  const directory = await mkdtemp(path.join(tmpdir(), 'funding-recovery-'));
  const previous = { directory: process.env.SOLANA_RECOVERY_DIR, namespace: process.env.SOLANA_PREVIEW_NAMESPACE, path: process.env.PATH };
  process.env.SOLANA_RECOVERY_DIR = directory;
  process.env.SOLANA_PREVIEW_NAMESPACE = 'fhevm-ci-recovery-test';
  process.env.PATH = `${directory}:${process.env.PATH}`;
  try {
    const wallet = await generateSolanaKeypair();
    await expect(requireRecoverableWallet(wallet.signer.address)).rejects.toThrow('external wallets must self-fund');
    await writeFile(path.join(directory, 'browser-test.json'), JSON.stringify([...wallet.bytes]), { mode: 0o600 });
    const kubectl = path.join(directory, 'kubectl');
    await writeFile(kubectl, '#!/bin/sh\ncat >/dev/null\nexit 1\n', { mode: 0o700 });
    await expect(requireRecoverableWallet(wallet.signer.address)).rejects.toThrow('refusing to fund');
    await writeFile(kubectl, '#!/bin/sh\ncat >/dev/null\nexit 0\n', { mode: 0o700 });
    await requireRecoverableWallet(wallet.signer.address);
  } finally {
    for (const [key, value] of Object.entries({ SOLANA_RECOVERY_DIR: previous.directory, SOLANA_PREVIEW_NAMESPACE: previous.namespace, PATH: previous.path })) {
      if (value === undefined) delete process.env[key]; else process.env[key] = value;
    }
    await rm(directory, { recursive: true, force: true });
  }
});
