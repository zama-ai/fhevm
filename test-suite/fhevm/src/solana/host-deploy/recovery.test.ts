import { test, expect } from 'bun:test';
import { mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { AccountRole, address, createKeyPairSignerFromBytes, getAddressEncoder, type Instruction } from '@solana/kit';
import { AccountState, TOKEN_PROGRAM_ADDRESS, getTokenEncoder } from '@solana-program/token';
import { recoverPreview } from '../../../../../solana/deploy/src/recover';
import { programDataAddressFor } from '../../../../../solana/deploy/src/bootstrap';
import { getVaultEncoder } from '../../../../../solana/demo-dapp/src/vault/internal/generated/demoVault/accounts/vault';
import { PREVIEW_CLOSE_TOKEN_DISCRIMINATOR } from '../../../../../solana/demo-dapp/src/vault/internal/generated/demoVault/instructions/previewCloseToken';
import { findVaultAuthorityPda } from '../../../../../solana/demo-dapp/src/vault/internal/generated/demoVault/pdas/vaultAuthority';
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

test('reset sweeps vault and wallet token accounts of preview mints under both token programs and retains foreign ones', async () => {
  const directory = await mkdtemp(path.join(tmpdir(), 'recovery-'));
  const namespace = process.env.SOLANA_PREVIEW_NAMESPACE;
  delete process.env.SOLANA_PREVIEW_NAMESPACE;
  try {
    const payer = await generateSolanaKeypair();
    const payerPath = path.join(directory, 'payer.json');
    await writeFile(payerPath, JSON.stringify([...payer.bytes]), { mode: 0o600 });
    const wallet = await generateSolanaKeypair();
    await writeFile(path.join(directory, 'demo-wallet.json'), JSON.stringify([...wallet.bytes]), { mode: 0o600 });
    const newAddress = async () => (await generateSolanaKeypair()).signer.address;
    const [previewMint, shareMint, foreignMint, vault] = [await newAddress(), await newAddress(), await newAddress(), await newAddress()];
    const vaultProgram = deployedProgramIds(DEFAULT_SOLANA_ENVIRONMENT).demo_vault;
    const [vaultAuthority, authorityBump] = await findVaultAuthorityPda({ vault }, { programAddress: vaultProgram });
    const vaultData = getVaultEncoder().encode({
      underlyingMint: previewMint,
      shareMint,
      vaultTokenAccount: await newAddress(),
      authorityBump,
    });
    const tokenProgram = TOKEN_PROGRAM_ADDRESS;
    const token2022Program = address('TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb');
    const tokenAccount = async (program: string, mint: string, owner: string, amount: bigint) => ({
      program,
      owner,
      pubkey: await newAddress(),
      data: Buffer.from(getTokenEncoder().encode({
        mint: address(mint),
        owner: address(owner),
        amount,
        delegate: null,
        state: AccountState.Initialized,
        isNative: null,
        delegatedAmount: 0n,
        closeAuthority: null,
      })).toString('base64'),
    });
    const vaultPreview = await tokenAccount(tokenProgram, previewMint, vaultAuthority, 4n);
    const vaultForeign = await tokenAccount(token2022Program, foreignMint, vaultAuthority, 0n);
    const walletPreview = await tokenAccount(token2022Program, previewMint, wallet.signer.address, 5n);
    const walletForeign = await tokenAccount(tokenProgram, foreignMint, wallet.signer.address, 3n);
    const tokenAccounts = [vaultPreview, vaultForeign, walletPreview, walletForeign];
    const result = (value: unknown) => ({ send: async () => value });
    const tokenProgramQueries: { program: string; filters: unknown }[] = [];
    const sent: { program: string; data: string; accounts: string[] }[] = [];
    const context = {
      rpc: {
        getGenesisHash: () => result('EtWTRABZaYq6iMfeYKouRu166VU2xqa1wcaWoxPkrZBG'),
        getBalance: () => result({ value: 0n }),
        getTokenAccountsByOwner: (owner: string, { programId }: { programId: string }) => result({
          value: tokenAccounts
            .filter((token) => token.owner === owner && token.program === programId)
            .map(({ pubkey, data }) => ({
              pubkey,
              account: { owner: programId, lamports: 2_039_280n, data: [data, 'base64'] },
            })),
        }),
        getProgramAccounts: (program: string, config: { filters?: unknown }) => {
          if (program === tokenProgram || program === token2022Program)
            tokenProgramQueries.push({ program, filters: config.filters });
          return result(program === vaultProgram
            ? [{ pubkey: vault, account: { lamports: 1n, data: [Buffer.from(vaultData).toString('base64'), 'base64'] } }]
            : []);
        },
        getAccountInfo: () => result({ value: null }),
      },
      sendTransaction: async (_payer: unknown, instructions: Instruction[]) => {
        const { programAddress, data, accounts } = instructions[0]!;
        sent.push({ program: programAddress, data: Buffer.from(data!).toString('hex'), accounts: accounts!.map((meta) => meta.address) });
      },
    } as unknown as HostDeployContext;
    await recoverPreview(context, payer.signer, DEFAULT_SOLANA_ENVIRONMENT, directory, true, payerPath);

    expect(sent).toEqual([
      {
        program: vaultProgram,
        data: expect.stringMatching(new RegExp(`^${Buffer.from(PREVIEW_CLOSE_TOKEN_DISCRIMINATOR).toString('hex')}`)),
        accounts: [payer.signer.address, await programDataAddressFor(vaultProgram), vaultAuthority, vaultPreview.pubkey, previewMint, tokenProgram],
      },
      // Burn 5 (tag 8) then close (tag 9), sent to the program that owns the account.
      { program: token2022Program, data: '080500000000000000', accounts: [walletPreview.pubkey, previewMint, wallet.signer.address] },
      { program: token2022Program, data: '09', accounts: [walletPreview.pubkey, payer.signer.address, wallet.signer.address] },
    ]);
    const { retained } = JSON.parse(await readFile(path.join(directory, 'report.json'), 'utf8'));
    expect(retained).toEqual([vaultForeign, walletForeign].map(({ pubkey }) => ({
      address: pubkey,
      lamports: '2039280',
      reason: 'token account of a mint outside this preview',
    })));
    // The RPC answers a scan of token accounts by mint from its index only in these shapes.
    const queriesFor = (mint: string) => {
      const ofMint = { memcmp: { offset: 0n, bytes: mint, encoding: 'base58' } };
      return [
        { program: tokenProgram, filters: [{ dataSize: 165n }, ofMint] },
        { program: token2022Program, filters: [{ dataSize: 165n }, ofMint] },
        { program: token2022Program, filters: [{ memcmp: { offset: 165n, bytes: '3', encoding: 'base58' } }, ofMint] },
      ];
    };
    expect(tokenProgramQueries).toHaveLength(6);
    expect(tokenProgramQueries).toEqual(expect.arrayContaining([...queriesFor(shareMint), ...queriesFor(previewMint)]));
  } finally {
    if (namespace !== undefined) process.env.SOLANA_PREVIEW_NAMESPACE = namespace;
    await rm(directory, { recursive: true, force: true });
  }
});
