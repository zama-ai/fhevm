import { test, expect } from 'bun:test';
import { mkdtemp, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { AccountRole, address, createKeyPairSignerFromBytes, getAddressEncoder, type Instruction } from '@solana/kit';
import { recoverPreview } from '../../../../../solana/deploy/src/recover';
import { DEFAULT_SOLANA_ENVIRONMENT, deployedProgramIds } from '../../../../../solana/deploy/src/environment';
import { uploadBufferBytes } from '../../../../../solana/deploy/src/deploy-programs';
import type { HostDeployContext } from '../../../../../solana/deploy/src/send';
import { generateSolanaKeypair } from '../provision';

for (const runId of [undefined, "current-run"] as const) {
  test(`recovery closes an interrupted upload buffer and spares the active browser wallet (${runId ?? 'all'})`, async () => {
    const directory = await mkdtemp(path.join(tmpdir(), 'recovery-'));
    const namespace = process.env.SOLANA_PREVIEW_NAMESPACE;
    delete process.env.SOLANA_PREVIEW_NAMESPACE;
    try {
      const payer = await generateSolanaKeypair();
      const payerPath = path.join(directory, 'payer.json');
      await writeFile(payerPath, JSON.stringify([...payer.bytes]), { mode: 0o600 });
      const buffer = await createKeyPairSignerFromBytes(await uploadBufferBytes(payerPath, deployedProgramIds(DEFAULT_SOLANA_ENVIRONMENT).zama_host));
      const loader = address('BPFLoaderUpgradeab1e11111111111111111111111');
      let bufferData = runId ? null : Buffer.from([1, 0, 0, 0, 1, ...getAddressEncoder().encode(payer.signer.address)]);
      const activeBrowser = await generateSolanaKeypair();
      if (runId) await writeFile(path.join(directory, 'browser-active.json'), JSON.stringify([...activeBrowser.bytes]), { mode: 0o600 });
      const events: string[] = [];
      const result = (value: unknown) => ({ send: async () => value });
      const context = {
        rpc: {
          getGenesisHash: () => result('EtWTRABZaYq6iMfeYKouRu166VU2xqa1wcaWoxPkrZBG'),
          getBalance: (account: string) => {
            expect(account).not.toBe(activeBrowser.signer.address);
            return result({ value: 0n });
          },
          getTokenAccountsByOwner: () => result({ value: [] }),
          getProgramAccounts: () => result([]),
          getAccountInfo: (account: string) => result({ value: account === buffer.address && bufferData
            ? { owner: loader, data: [bufferData.toString('base64'), 'base64'] } : null }),
        },
        sendTransaction: async (_payer: unknown, instructions: Instruction[]) => {
          const instruction = instructions[0]!;
          expect(instruction.programAddress).toBe(loader);
          expect(Buffer.from(instruction.data!).toString('hex')).toBe('05000000');
          expect(instruction.accounts!.slice(0, 3)).toMatchObject([
            { address: buffer.address, role: AccountRole.WRITABLE },
            { address: payer.signer.address, role: AccountRole.WRITABLE },
            { address: payer.signer.address, role: AccountRole.READONLY_SIGNER, signer: payer.signer },
          ]);
          events.push('buffer-close');
          bufferData = null;
        },
      } as unknown as HostDeployContext;
      await recoverPreview(context, payer.signer, DEFAULT_SOLANA_ENVIRONMENT, directory, !runId, payerPath, false, runId);
      expect(events).toEqual(runId ? [] : ['buffer-close']);
      expect(bufferData).toBeNull();
    } finally {
      if (namespace !== undefined) process.env.SOLANA_PREVIEW_NAMESPACE = namespace;
      await rm(directory, { recursive: true, force: true });
    }
  });
}
