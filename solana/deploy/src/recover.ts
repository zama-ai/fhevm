import {
  CONFIDENTIAL_MINT_DISCRIMINATOR,
  getConfidentialMintDecoder,
  findVaultAuthorityPda,
  findTotalSupplyAuthorityPda,
  getVaultAuthorityPdaSeeds,
  getTotalSupplyAuthorityPdaSeeds,
} from '@fhevm/confidential-token';
// Exhaustive preview reset. Public account inventory is written before any state is closed.
import { mkdir, readFile, readdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import {
  AccountRole,
  address,
  createKeyPairSignerFromBytes,
  getAddressDecoder,
  isSome,
  type Address,
  type Instruction,
  type TransactionSigner,
} from '@solana/kit';
import {
  TOKEN_PROGRAM_ADDRESS as TOKEN,
  getBurnInstruction,
  getCloseAccountInstruction,
  getTokenDecoder,
  getTokenSize,
} from '@solana-program/token';
import {
  ADDRESS_LOOKUP_TABLE_PROGRAM_ADDRESS as ALT,
  getAddressLookupTableDecoder,
  getDeactivateLookupTableInstruction,
  getCloseLookupTableInstruction,
} from '@solana-program/address-lookup-table';
import { uploadBufferBytes } from './deploy-programs';
import { programDataAddressFor } from './bootstrap';
import { deployedProgramIds, type SolanaEnvironment } from './environment';
import { loadKeypairSigner } from './keypair';
import type { HostDeployContext } from './send';
import { journalRecovery } from './recovery-journal';
import { wipeZamaHost } from './wipe';
import { VAULT_DISCRIMINATOR, getVaultDecoder } from '../../demo-dapp/src/vault/internal/generated/demoVault/accounts/vault';
import {
  BATCH_DISCRIMINATOR,
  getBatchDecoder,
} from '../../demo-dapp/src/vault/internal/generated/confidentialBatcher/accounts/batch';
import {
  BATCHER_DISCRIMINATOR,
  getBatcherDecoder,
} from '../../demo-dapp/src/vault/internal/generated/confidentialBatcher/accounts/batcher';
import {
  JOIN_RECORD_DISCRIMINATOR,
  getJoinRecordDecoder,
} from '../../demo-dapp/src/vault/internal/generated/confidentialBatcher/accounts/joinRecord';
import { BatchStatus } from '../../demo-dapp/src/vault/internal/generated/confidentialBatcher/types/batchStatus';
import {
  getReclaimBatchAuthorityInstructionAsync,
} from '../../demo-dapp/src/vault/internal/generated/confidentialBatcher/instructions/reclaimBatchAuthority';
import {
  getCloseJoinRecordInstruction,
} from '../../demo-dapp/src/vault/internal/generated/confidentialBatcher/instructions/closeJoinRecord';
import {
  findBatchAuthorityPda,
  getBatchAuthorityPdaSeeds,
} from '../../demo-dapp/src/vault/internal/generated/confidentialBatcher/pdas/batchAuthority';
import {
  findVaultAuthorityPda as findDemoVaultAuthorityPda,
  getVaultAuthorityPdaSeeds as getDemoVaultAuthorityPdaSeeds,
} from '../../demo-dapp/src/vault/internal/generated/demoVault/pdas/vaultAuthority';
import {
  getPreviewCloseTokenInstruction as getBatcherPreviewCloseTokenInstruction,
} from '../../demo-dapp/src/vault/internal/generated/confidentialBatcher/instructions/previewCloseToken';
import {
  getPreviewDrainInstruction as getBatcherPreviewDrainInstruction,
} from '../../demo-dapp/src/vault/internal/generated/confidentialBatcher/instructions/previewDrain';
import {
  getPreviewCloseTokenInstruction as getVaultPreviewCloseTokenInstruction,
} from '../../demo-dapp/src/vault/internal/generated/demoVault/instructions/previewCloseToken';
import {
  getPreviewDrainInstruction as getVaultPreviewDrainInstruction,
} from '../../demo-dapp/src/vault/internal/generated/demoVault/instructions/previewDrain';

const SYSTEM = address('11111111111111111111111111111111');
const LOADER = address('BPFLoaderUpgradeab1e11111111111111111111111');
const u32 = (n: number) => {
  const b = Buffer.alloc(4);
  b.writeUInt32LE(n);
  return b;
};
const writable = (a: Address) => ({ address: a, role: AccountRole.WRITABLE });
const signer = (s: TransactionSigner) => ({ address: s.address, role: AccountRole.READONLY_SIGNER, signer: s });
const decodeAddress = (b: Uint8Array, offset: number) => getAddressDecoder().decode(b, offset);
const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

export async function recoverPreview(
  context: HostDeployContext,
  payer: TransactionSigner,
  environment: SolanaEnvironment,
  directory: string,
  reset: boolean,
  deployerPath: string,
  fundingOnly = false,
  runId?: string,
) {
  if (runId && (reset || !/^[a-zA-Z0-9-]+$/.test(runId)))
    throw new Error('run recovery requires a valid run id and cannot reset shared state');
  if ((await context.rpc.getGenesisHash().send()) !== 'EtWTRABZaYq6iMfeYKouRu166VU2xqa1wcaWoxPkrZBG')
    throw new Error('preview recovery requires Solana devnet');
  await mkdir(directory, { recursive: true, mode: 0o700 });
  const programs = deployedProgramIds(environment);
  const journal = await journalRecovery(context, directory);
  const before = (await context.rpc.getBalance(payer.address).send()).value;
  const inventory = [];
  const finishedBatches = new Set<string>();
  const liveBatches = new Set<string>();
  for (const [name, program] of Object.entries(fundingOnly ? {} : programs)) {
    for (const item of await context.rpc
      .getProgramAccounts(program, { encoding: 'base64' })
      .send()) {
      inventory.push({
        name,
        program,
        address: item.pubkey,
        lamports: item.account.lamports.toString(),
        data: item.account.data[0],
      });
    }
  }
  for (const item of inventory) {
    const bytes = Buffer.from(item.data, 'base64');
    if (item.name === 'confidential_batcher' && bytes.subarray(0, 8).equals(Buffer.from(BATCH_DISCRIMINATOR))) {
      const batch = getBatchDecoder().decode(bytes);
      ([BatchStatus.Settled, BatchStatus.Canceled, BatchStatus.Refunding].includes(batch.status)
        ? finishedBatches
        : liveBatches
      ).add(item.address);
    }
  }
  await writeFile(path.join(directory, 'inventory.json'), JSON.stringify(inventory, null, 2), { mode: 0o600 });
  const knownMints = new Set<Address>();
  for (const name of await readdir(directory)) {
    if (!/^inventory-.+\.json$/.test(name)) continue;
    for (const mint of JSON.parse(await readFile(path.join(directory, name), 'utf8')).mints ?? [])
      knownMints.add(address(mint));
  }
  for (const item of inventory) {
    const bytes = Buffer.from(item.data, 'base64');
    if (item.name === 'confidential_token' && bytes.subarray(0, 8).equals(Buffer.from(CONFIDENTIAL_MINT_DISCRIMINATOR)))
      knownMints.add(getConfidentialMintDecoder().decode(bytes).underlyingMint);
    if (item.name === 'demo_vault' && bytes.subarray(0, 8).equals(Buffer.from(VAULT_DISCRIMINATOR))) {
      const vault = getVaultDecoder().decode(bytes);
      knownMints.add(vault.shareMint);
      knownMints.add(vault.underlyingMint);
    }
  }
  await journal.persist('inventory-recovery.json', JSON.stringify({ mints: [...knownMints] }));
  const send = (instruction: Instruction) => context.sendTransaction(payer, [instruction]);
  const retained: { address: string; lamports: string; reason: string }[] = [];
  const authorities: { program: Address; address: Address; seeds: Uint8Array[] }[] = [];
  if (reset) {
    // Derive all external signing authorities while their discovery accounts still exist.
    for (const item of inventory) {
      const data = Buffer.from(item.data, 'base64');
      const isTokenMint = item.name === 'confidential_token' && data.subarray(0, 8).equals(Buffer.from(CONFIDENTIAL_MINT_DISCRIMINATOR));
      const isVault = item.name === 'demo_vault' && data.subarray(0, 8).equals(Buffer.from(VAULT_DISCRIMINATOR));
      const isBatch = item.name === 'confidential_batcher' && data.subarray(0, 8).equals(Buffer.from(BATCH_DISCRIMINATOR));
      if (isTokenMint) {
        for (const [finder, seedEncoder] of [
          [findVaultAuthorityPda, getVaultAuthorityPdaSeeds],
          [findTotalSupplyAuthorityPda, getTotalSupplyAuthorityPdaSeeds],
        ] as const) {
          const seeds = { mint: item.address };
          const [derived, bump] = await finder(seeds, { programAddress: item.program });
          authorities.push({ program: item.program, address: derived, seeds: [...seedEncoder(seeds).map(bytes => Uint8Array.from(bytes)), new Uint8Array([bump])] });
        }
      }
      if (isVault) {
        const seeds = { vault: item.address };
        const [derived, bump] = await findDemoVaultAuthorityPda(seeds, { programAddress: item.program });
        authorities.push({ program: item.program, address: derived, seeds: [...getDemoVaultAuthorityPdaSeeds(seeds).map(bytes => Uint8Array.from(bytes)), new Uint8Array([bump])] });
      }
      if (isBatch) {
        const seeds = { batch: item.address };
        const [derived, bump] = await findBatchAuthorityPda(seeds, { programAddress: item.program });
        authorities.push({ program: item.program, address: derived, seeds: [...getBatchAuthorityPdaSeeds(seeds).map(bytes => Uint8Array.from(bytes)), new Uint8Array([bump])] });
      }
      const mint = isVault
        ? getVaultDecoder().decode(data).shareMint
        : isTokenMint
          ? getConfidentialMintDecoder().decode(data).underlyingMint
          : undefined;
      if (mint) {
        const info = (await context.rpc.getAccountInfo(mint, { encoding: 'base64' }).send())
          .value;
        if (info?.owner === TOKEN)
          retained.push({
            address: mint,
            lamports: info.lamports.toString(),
            reason: 'classic SPL mint has no close instruction',
          });
      }
    }
    for (const authority of authorities) {
      const programData = await programDataAddressFor(authority.program);
      // Confidential-token shares this ABI; check-zama-host-idl.sh compares its admin-sweep IDL.
      const closeToken = authority.program === programs.confidential_batcher
        ? getBatcherPreviewCloseTokenInstruction
        : getVaultPreviewCloseTokenInstruction;
      const drain = authority.program === programs.confidential_batcher
        ? getBatcherPreviewDrainInstruction
        : getVaultPreviewDrainInstruction;
      const tokens = await context.rpc
        .getTokenAccountsByOwner(
          authority.address,
          { programId: TOKEN },
          { encoding: 'base64' },
        )
        .send();
      for (const token of tokens.value) {
        const { mint } = getTokenDecoder().decode(Buffer.from(token.account.data[0], 'base64'));
        await send(closeToken({
          admin: payer,
          programData,
          authority: authority.address,
          account: token.pubkey,
          mint,
          tokenProgram: TOKEN,
          seeds: authority.seeds,
        }, { programAddress: authority.program }));
      }
      const info = (
        await context.rpc.getAccountInfo(authority.address, { encoding: 'base64' }).send()
      ).value;
      if (info?.owner === SYSTEM && info.lamports > 0n) {
        await send(drain({
          admin: payer,
          programData,
          authority: authority.address,
          systemProgram: SYSTEM,
          seeds: authority.seeds,
        }, { programAddress: authority.program }));
      }
    }
  }
  const wallets = new Map<string, TransactionSigner>();
  const legacyWallets = new Set<string>();
  for (const name of await readdir(directory)) {
    if (!/^(run-|demo-|browser-).+\.json$/.test(name)) continue;
    if (runId && !name.startsWith(`run-${runId}-`)) continue;
    const wallet = await loadKeypairSigner(path.join(directory, name));
    wallets.set(wallet.address, wallet);
    if (name.startsWith('demo-legacy-')) legacyWallets.add(wallet.address);
  }
  if (!reset && !fundingOnly) {
    const byAddress = new Map(inventory.map((item) => [item.address, item]));
    for (const item of inventory) {
      const bytes = Buffer.from(item.data, 'base64');
      if (item.name !== 'confidential_batcher') continue;
      if (finishedBatches.has(item.address)) {
        const batch = getBatchDecoder().decode(bytes);
        const batcherData = byAddress.get(batch.batcher);
        if (!batcherData) throw new Error(`missing batcher ${batch.batcher}`);
        const batcherBytes = Buffer.from(batcherData.data, 'base64');
        if (!batcherBytes.subarray(0, 8).equals(Buffer.from(BATCHER_DISCRIMINATOR)))
          throw new Error('Invalid batcher account');
        const mint = getBatcherDecoder().decode(batcherBytes).joinConfidentialMint;
        const mintData = byAddress.get(mint);
        if (!mintData) throw new Error(`missing confidential mint ${mint}`);
        const mintBytes = Buffer.from(mintData.data, 'base64');
        if (!mintBytes.subarray(0, 8).equals(Buffer.from(CONFIDENTIAL_MINT_DISCRIMINATOR)))
          throw new Error('Invalid confidential mint');
        const keeper = wallets.get(getConfidentialMintDecoder().decode(mintBytes).authority);
        if (!keeper) {
          retained.push({ address: item.address, lamports: item.lamports, reason: 'missing batch reclaim signer' });
          continue;
        }
        const [authority] = await findBatchAuthorityPda(
          { batch: item.address },
          { programAddress: item.program },
        );
        if ((await context.rpc.getBalance(authority).send()).value > 0n)
          await send(
            await getReclaimBatchAuthorityInstructionAsync(
              {
                authority: keeper,
                batcher: batch.batcher,
                batch: item.address,
                batchAuthority: authority,
                joinConfidentialMint: mint,
              },
              { programAddress: programs.confidential_batcher },
            ),
          );
      }
      if (bytes.subarray(0, 8).equals(Buffer.from(JOIN_RECORD_DISCRIMINATOR))) {
        const join = getJoinRecordDecoder().decode(bytes);
        const user = wallets.get(join.user);
        const batchData = byAddress.get(join.batch);
        if (!user || !batchData) continue;
        const batchBytes = Buffer.from(batchData.data, 'base64');
        if (!batchBytes.subarray(0, 8).equals(Buffer.from(BATCH_DISCRIMINATOR)))
          throw new Error('Invalid batch account');
        const state = getBatchDecoder().decode(batchBytes).status;
        if (state === BatchStatus.Canceled || (state === BatchStatus.Settled && join.claimed))
          await send(
            getCloseJoinRecordInstruction(
              { user, batch: join.batch, joinRecord: item.address },
              { programAddress: programs.confidential_batcher },
            ),
          );
      }
    }
  }
  const pendingTables = new Map<Address, TransactionSigner>();
  for (const wallet of fundingOnly ? [] : wallets.values()) {
    const tokens = await context.rpc
      .getTokenAccountsByOwner(wallet.address, { programId: TOKEN }, { encoding: 'base64' })
      .send();
    for (const token of tokens.value) {
      const { mint, amount } = getTokenDecoder().decode(Buffer.from(token.account.data[0], 'base64'));
      if (legacyWallets.has(wallet.address) && !knownMints.has(mint)) continue;
      if (amount > 0n && !reset) {
        retained.push({
          address: token.pubkey,
          lamports: token.account.lamports.toString(),
          reason: 'live token balance; retained until reset',
        });
        continue;
      }
      if (amount > 0n)
        await send(getBurnInstruction({ account: token.pubkey, mint, authority: wallet, amount }));
      await send(getCloseAccountInstruction({ account: token.pubkey, destination: payer.address, owner: wallet }));
    }
    {
      const tables = await context.rpc
        .getProgramAccounts(ALT, {
          encoding: 'base64',
          // LookupTableMeta.authority's address starts at byte 22; the client exposes no field offsets.
          filters: [{ memcmp: { offset: 22n, bytes: wallet.address, encoding: 'base58' } }],
        })
        .send();
      for (const table of tables) {
        const { authority, addresses: members, deactivationSlot } = getAddressLookupTableDecoder().decode(
          Buffer.from(table.account.data[0], 'base64'),
        );
        if (!isSome(authority) || authority.value !== wallet.address) continue;
        const finished = members.some((a) => finishedBatches.has(a)) && !members.some((a) => liveBatches.has(a));
        if (!reset && !finished && deactivationSlot === 0xffffffffffffffffn) {
          retained.push({
            address: table.pubkey,
            lamports: table.account.lamports.toString(),
            reason: 'active lookup table; retained until batch completion or reset',
          });
          continue;
        }
        if (deactivationSlot === 0xffffffffffffffffn)
          await send(getDeactivateLookupTableInstruction({ address: table.pubkey, authority: wallet }));
        pendingTables.set(table.pubkey, wallet);
      }
    }
  }
  // Start every cooldown before waiting, including tables owned by different wallets.
  const tableDeadline = Date.now() + 10 * 60_000;
  while (pendingTables.size > 0) {
    const finalizedSlot = await context.rpc.getSlot().send();
    for (const [table, wallet] of pendingTables) {
      const info = (
        await context.rpc.getAccountInfo(table, { encoding: 'base64' }).send()
      ).value;
      if (!info) {
        pendingTables.delete(table);
        continue;
      }
      const { deactivationSlot: deactivated } = getAddressLookupTableDecoder().decode(Buffer.from(info.data[0], 'base64'));
      if (finalizedSlot > deactivated + 513n) {
        await send(getCloseLookupTableInstruction({ address: table, authority: wallet, recipient: payer.address }));
        pendingTables.delete(table);
      }
    }
    if (pendingTables.size === 0) break;
    if (Date.now() > tableDeadline)
      throw new Error(`${pendingTables.size} lookup tables still cooling down; retry recovery`);
    await sleep(5_000);
  }
  // Recover interrupted uploads; these are loader Buffer accounts, never ProgramData.
  {
    for (const program of runId ? [] : Object.values(programs)) {
      const buffer = await createKeyPairSignerFromBytes(await uploadBufferBytes(deployerPath, program));
      const account = (
        await context.rpc.getAccountInfo(buffer.address, { encoding: 'base64' }).send()
      ).value;
      if (!account) continue;
      const data = Buffer.from(account.data[0], 'base64');
      if (
        account.owner !== LOADER ||
        data.readUInt32LE(0) !== 1 ||
        data[4] !== 1 ||
        decodeAddress(data, 5) !== payer.address
      )
        throw new Error(`unexpected upload buffer ${buffer.address}`);
      await send({
        programAddress: LOADER,
        accounts: [writable(buffer.address), writable(payer.address), signer(payer)],
        data: u32(5),
      });
    }
  }
  if (reset) {
    // Last: callers no longer need application roots or host encrypted state.
    for (const name of ['confidential_batcher', 'confidential_token', 'demo_vault', 'zama_host'] as const) {
      if ((await context.rpc.getAccountInfo(programs[name], { encoding: 'base64' }).send()).value) {
        await wipeZamaHost(context, { payer, programAddress: programs[name] });
      }
    }
  }
  for (const wallet of wallets.values()) {
    if (wallet.address === payer.address) continue;
    const balance = (await context.rpc.getBalance(wallet.address).send()).value;
    if (balance > 0n) {
      const amount = Buffer.alloc(8);
      amount.writeBigUInt64LE(balance);
      await send({
        programAddress: SYSTEM,
        accounts: [{ ...signer(wallet), role: AccountRole.WRITABLE_SIGNER }, writable(payer.address)],
        data: Buffer.concat([u32(2), amount]),
      });
    }
    if ((await context.rpc.getBalance(wallet.address).send()).value !== 0n)
      throw new Error(`wallet ${wallet.address} still holds recoverable SOL`);
  }
  for (const name of await readdir(directory)) {
    if (!/^inventory-.+\.json$/.test(name)) continue;
    const entry = JSON.parse(await readFile(path.join(directory, name), 'utf8')) as { mints?: string[] };
    for (const mint of entry.mints ?? []) {
      const info = (
        await context.rpc.getAccountInfo(address(mint), { encoding: 'base64' }).send()
      ).value;
      if (info?.owner === TOKEN)
        retained.push({
          address: mint,
          lamports: info.lamports.toString(),
          reason: 'classic SPL mint has no close instruction',
        });
    }
  }
  for (const program of Object.values(programs)) {
    for (const account of [program, await programDataAddressFor(program)]) {
      const info = (await context.rpc.getAccountInfo(account, { encoding: 'base64' }).send())
        .value;
      if (info)
        retained.push({
          address: account,
          lamports: info.lamports.toString(),
          reason: 'program identity and executable intentionally retained for reuse',
        });
    }
  }
  if (!reset)
    for (const item of inventory) {
      const info = (
        await context.rpc.getAccountInfo(item.address, { encoding: 'base64' }).send()
      ).value;
      if (info)
        retained.push({
          address: item.address,
          lamports: info.lamports.toString(),
          reason: 'application state retained until reset; lifecycle cleanup may still be required',
        });
    }
  if (reset)
    for (const mint of knownMints) {
      const remaining = await context.rpc
        .getProgramAccounts(TOKEN, {
          encoding: 'base64',
          // Token.mint starts at byte 0; the client exposes no field offsets.
          filters: [{ dataSize: BigInt(getTokenSize()) }, { memcmp: { offset: 0n, bytes: mint, encoding: 'base58' } }],
        })
        .send();
      for (const item of remaining)
        retained.push({
          address: item.pubkey,
          lamports: item.account.lamports.toString(),
          reason: 'preview mint token account has no recovered signing key',
        });
    }
  const after = (await context.rpc.getBalance(payer.address).send()).value;
  const transactions = [];
  let fees = 0n;
  let netRecovered = 0n;
  for (const { signature, lastValidBlockHeight } of await journal.receipts()) {
    let transaction = await context.rpc
      .getTransaction(signature, { encoding: 'json', maxSupportedTransactionVersion: 1 })
      .send();
    for (let attempt = 0; transaction === null && attempt < 20; attempt++) {
      await sleep(1_000);
      transaction = await context.rpc
        .getTransaction(signature, { encoding: 'json', maxSupportedTransactionVersion: 1 })
        .send();
    }
    if (
      !transaction &&
      (await context.rpc.getBlockHeight().send()) > BigInt(lastValidBlockHeight)
    ) {
      const status = (await context.rpc.getSignatureStatuses([signature], { searchTransactionHistory: true }).send())
        .value[0];
      if (!status) {
        transactions.push({ signature, feeLamports: '0', outcome: 'expired without landing' });
        continue;
      }
    }
    if (!transaction?.meta) throw new Error(`transaction accounting unavailable for ${signature}; retry recovery`);
    fees += transaction.meta.fee;
    netRecovered += transaction.meta.postBalances[0]! - transaction.meta.preBalances[0]!;
    transactions.push({ signature, feeLamports: transaction.meta.fee.toString() });
  }
  const report = {
    reset,
    transactionFeesLamports: fees.toString(),
    grossRecoveredLamports: (netRecovered + fees).toString(),
    transactions,
    recipient: payer.address,
    netRecoveredLamports: netRecovered.toString(),
    attemptBalanceChangeLamports: (after - before).toString(),
    retained: [...new Map(retained.map((item) => [item.address, item])).values()],
    completedAt: new Date().toISOString(),
  };
  await writeFile(path.join(directory, 'report.json'), JSON.stringify(report, null, 2), { mode: 0o600 });
  console.log(JSON.stringify(report));
}
