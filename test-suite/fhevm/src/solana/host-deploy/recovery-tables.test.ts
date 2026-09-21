import { test, expect } from 'bun:test';
import { mkdtemp, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { getAddressEncoder, type Instruction } from '@solana/kit';
import { recoverPreview } from '../../../../../solana/deploy/src/recover';
import { DEFAULT_SOLANA_ENVIRONMENT } from '../../../../../solana/deploy/src/environment';
import type { HostDeployContext } from '../../../../../solana/deploy/src/send';
import { generateSolanaKeypair } from '../provision';

for (const resumed of [false, true]) {
  test(`recovery overlaps cooldowns across wallets (${resumed ? 'resumed' : 'fresh'})`, async () => {
    const directory = await mkdtemp(path.join(tmpdir(), 'recovery-tables-'));
    const namespace = process.env.SOLANA_PREVIEW_NAMESPACE;
    delete process.env.SOLANA_PREVIEW_NAMESPACE;
    try {
      const payer = await generateSolanaKeypair();
      const payerPath = path.join(directory, 'payer.json');
      await writeFile(payerPath, JSON.stringify([...payer.bytes]), { mode: 0o600 });
      const tables = new Map<string, { owner: string; data: Buffer }>();
      for (let i = 0; i < 2; i++) {
        const wallet = await generateSolanaKeypair();
        await writeFile(path.join(directory, `run-${i}.json`), JSON.stringify([...wallet.bytes]), { mode: 0o600 });
        const table = await generateSolanaKeypair();
        const data = Buffer.alloc(56);
        data.writeBigUInt64LE(resumed ? 100n : 0xffffffffffffffffn, 4);
        data[21] = 1;
        data.set(getAddressEncoder().encode(wallet.signer.address), 22);
        tables.set(table.signer.address, { owner: wallet.signer.address, data });
      }
      const events: string[] = [];
      const result = (value: unknown) => ({ send: async () => value });
      const context = {
        rpc: {
          getGenesisHash: () => result('EtWTRABZaYq6iMfeYKouRu166VU2xqa1wcaWoxPkrZBG'),
          getBalance: () => result({ value: 0n }),
          getTokenAccountsByOwner: () => result({ value: [] }),
          getProgramAccounts: (program: string, options: { filters: { memcmp: { bytes: string } }[] }) => result(
            program === 'AddressLookupTab1e1111111111111111111111111'
              ? [...tables].filter(([, table]) => table.owner === options.filters[0].memcmp.bytes)
                .map(([pubkey, table]) => ({ pubkey, account: { data: [table.data.toString('base64'), 'base64'] } }))
              : [],
          ),
          getAccountInfo: (account: string) => result({ value: tables.has(account)
            ? { data: [tables.get(account)!.data.toString('base64'), 'base64'] } : null }),
          getSlot: () => {
            // Polling must not begin while another wallet's table is still active.
            expect([...tables.values()].every(table => table.data.readBigUInt64LE(4) === 100n)).toBe(true);
            events.push('poll');
            return result(614n);
          },
        },
        sendTransaction: async (_payer: unknown, instructions: Instruction[]) => {
          const instruction = instructions[0]!;
          const table = instruction.accounts![0]!.address;
          const operation = Buffer.from(instruction.data!).readUInt32LE();
          if (operation === 3) {
            events.push('deactivate');
            tables.get(table)!.data.writeBigUInt64LE(100n, 4);
          } else {
            expect(operation).toBe(4);
            events.push('close');
            tables.delete(table);
          }
        },
      } as unknown as HostDeployContext;
      await recoverPreview(context, payer.signer, DEFAULT_SOLANA_ENVIRONMENT, directory, true, payerPath);
      expect(events).toEqual(resumed ? ['poll', 'close', 'close'] : ['deactivate', 'deactivate', 'poll', 'close', 'close']);
      expect(tables.size).toBe(0);
    } finally {
      if (namespace !== undefined) process.env.SOLANA_PREVIEW_NAMESPACE = namespace;
      await rm(directory, { recursive: true, force: true });
    }
  });
}
