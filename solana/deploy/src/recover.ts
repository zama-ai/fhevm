// Exhaustive preview reset. Public account inventory is written before any state is closed.
import { createHash } from 'node:crypto';
import { mkdir, readFile, readdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import {
  AccountRole,
  address,
  createKeyPairSignerFromBytes,
  getAddressDecoder,
  getAddressEncoder,
  getProgramDerivedAddress,
  type Address,
  type Instruction,
  type TransactionSigner,
} from '@solana/kit';
import { uploadBufferBytes } from './deploy-programs';
import { programDataAddressFor } from './bootstrap';
import { deployedProgramIds, type SolanaEnvironment } from './environment';
import { loadKeypairSigner } from './keypair';
import type { HostDeployContext } from './send';
import { journalRecovery } from './recovery-journal';
import { wipeZamaHost } from './wipe';

import { getVaultDecoder } from '../../demo-dapp/src/vault/internal/generated/demoVault/accounts/vault';
import { getBatchDecoder } from '../../demo-dapp/src/vault/internal/generated/confidentialBatcher/accounts/batch';
import { getBatcherDecoder } from '../../demo-dapp/src/vault/internal/generated/confidentialBatcher/accounts/batcher';
import { getJoinRecordDecoder } from '../../demo-dapp/src/vault/internal/generated/confidentialBatcher/accounts/joinRecord';
import { BatchStatus } from '../../demo-dapp/src/vault/internal/generated/confidentialBatcher/types/batchStatus';
import { getReclaimBatchAuthorityInstructionAsync } from '../../demo-dapp/src/vault/internal/generated/confidentialBatcher/instructions/reclaimBatchAuthority';
import { getCloseJoinRecordInstruction } from '../../demo-dapp/src/vault/internal/generated/confidentialBatcher/instructions/closeJoinRecord';

const TOKEN = address('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
const SYSTEM = address('11111111111111111111111111111111');
const LOADER = address('BPFLoaderUpgradeab1e11111111111111111111111');
const ALT = address('AddressLookupTab1e1111111111111111111111111');
const discriminator = (name: string) => createHash('sha256').update(name).digest().subarray(0, 8);
const u32 = (n: number) => {
  const b = Buffer.alloc(4);
  b.writeUInt32LE(n);
  return b;
};
const writable = (a: Address) => ({ address: a, role: AccountRole.WRITABLE });
const signer = (s: TransactionSigner) => ({ address: s.address, role: AccountRole.READONLY_SIGNER, signer: s });
const decodeAddress = (b: Uint8Array, offset: number) => getAddressDecoder().decode(b, offset);
const encodeSeeds = (seeds: readonly Uint8Array[]) =>
  Buffer.concat([u32(seeds.length), ...seeds.flatMap((s) => [u32(s.length), Buffer.from(s)])]);
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
  const before = (await context.rpc.getBalance(payer.address, { commitment: 'confirmed' }).send()).value;
  const inventory = [];
  const finishedBatches = new Set<string>();
  const liveBatches = new Set<string>();
  for (const [name, program] of Object.entries(fundingOnly ? {} : programs)) {
    for (const item of await context.rpc
      .getProgramAccounts(program, { encoding: 'base64', commitment: 'confirmed' })
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
    if (item.name === 'confidential_batcher' && bytes.subarray(0, 8).equals(discriminator('account:Batch'))) {
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
    if (
      (item.name === 'confidential_token' && bytes.subarray(0, 8).equals(discriminator('account:ConfidentialMint'))) ||
      (item.name === 'demo_vault' && bytes.subarray(0, 8).equals(discriminator('account:Vault')))
    )
      knownMints.add(decodeAddress(bytes, 40));
    if (item.name === 'demo_vault' && bytes.subarray(0, 8).equals(discriminator('account:Vault')))
      knownMints.add(getVaultDecoder().decode(bytes).underlyingMint);
  }
  await journal.persist('inventory-recovery.json', JSON.stringify({ mints: [...knownMints] }));
  const send = (instruction: Instruction) => context.sendTransaction(payer, [instruction]);
  const retained: { address: string; lamports: string; reason: string }[] = [];
  const authorities: { program: Address; address: Address; seeds: Uint8Array[] }[] = [];
  if (reset) {
    // Derive all external signing authorities while their discovery accounts still exist.
    for (const item of inventory) {
      const data = Buffer.from(item.data, 'base64');
      const roots =
        item.name === 'confidential_token' && data.subarray(0, 8).equals(discriminator('account:ConfidentialMint'))
          ? ['vault-authority', 'total-supply']
          : item.name === 'demo_vault' && data.subarray(0, 8).equals(discriminator('account:Vault'))
            ? ['authority']
            : item.name === 'confidential_batcher' && data.subarray(0, 8).equals(discriminator('account:Batch'))
              ? ['batch-authority']
              : [];
      for (const root of roots) {
        const seeds = [new TextEncoder().encode(root), Uint8Array.from(getAddressEncoder().encode(item.address))];
        const [derived, bump] = await getProgramDerivedAddress({ programAddress: item.program, seeds });
        authorities.push({ program: item.program, address: derived, seeds: [...seeds, new Uint8Array([bump])] });
      }
      const mint =
        item.name === 'demo_vault' && roots.length
          ? decodeAddress(data, 40)
          : item.name === 'confidential_token' && roots.length
            ? decodeAddress(data, 40)
            : undefined;
      if (mint) {
        const info = (await context.rpc.getAccountInfo(mint, { encoding: 'base64', commitment: 'confirmed' }).send())
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
      const admin = [
        { ...signer(payer), role: AccountRole.WRITABLE_SIGNER },
        { address: await programDataAddressFor(authority.program), role: AccountRole.READONLY },
      ];
      const tokens = await context.rpc
        .getTokenAccountsByOwner(
          authority.address,
          { programId: TOKEN },
          { encoding: 'base64', commitment: 'confirmed' },
        )
        .send();
      for (const token of tokens.value) {
        const data = Buffer.from(token.account.data[0], 'base64');
        await send({
          programAddress: authority.program,
          accounts: [
            ...admin,
            { address: authority.address, role: AccountRole.READONLY },
            writable(token.pubkey),
            writable(decodeAddress(data, 0)),
            { address: TOKEN, role: AccountRole.READONLY },
          ],
          data: Buffer.concat([discriminator('global:preview_close_token'), encodeSeeds(authority.seeds)]),
        });
      }
      const info = (
        await context.rpc.getAccountInfo(authority.address, { encoding: 'base64', commitment: 'confirmed' }).send()
      ).value;
      if (info?.owner === SYSTEM && info.lamports > 0n) {
        await send({
          programAddress: authority.program,
          accounts: [...admin, writable(authority.address), { address: SYSTEM, role: AccountRole.READONLY }],
          data: Buffer.concat([discriminator('global:preview_drain'), encodeSeeds(authority.seeds)]),
        });
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
        if (!batcherBytes.subarray(0, 8).equals(discriminator('account:Batcher')))
          throw new Error('Invalid batcher account');
        const mint = getBatcherDecoder().decode(batcherBytes).joinConfidentialMint;
        const mintData = byAddress.get(mint);
        if (!mintData) throw new Error(`missing confidential mint ${mint}`);
        const mintBytes = Buffer.from(mintData.data, 'base64');
        if (mintBytes.length < 72 || !mintBytes.subarray(0, 8).equals(discriminator('account:ConfidentialMint')))
          throw new Error('Invalid confidential mint');
        const keeper = wallets.get(decodeAddress(mintBytes, 8));
        if (!keeper) {
          retained.push({ address: item.address, lamports: item.lamports, reason: 'missing batch reclaim signer' });
          continue;
        }
        const [authority] = await getProgramDerivedAddress({
          programAddress: programs.confidential_batcher,
          seeds: [new TextEncoder().encode('batch-authority'), getAddressEncoder().encode(item.address)],
        });
        if ((await context.rpc.getBalance(authority, { commitment: 'confirmed' }).send()).value > 0n)
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
      if (bytes.subarray(0, 8).equals(discriminator('account:JoinRecord'))) {
        const join = getJoinRecordDecoder().decode(bytes);
        const user = wallets.get(join.user);
        const batchData = byAddress.get(join.batch);
        if (!user || !batchData) continue;
        const state = getBatchDecoder().decode(Buffer.from(batchData.data, 'base64')).status;
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
      .getTokenAccountsByOwner(wallet.address, { programId: TOKEN }, { encoding: 'base64', commitment: 'confirmed' })
      .send();
    for (const token of tokens.value) {
      const data = Buffer.from(token.account.data[0], 'base64');
      if (legacyWallets.has(wallet.address) && !knownMints.has(decodeAddress(data, 0))) continue;
      const amount = data.readBigUInt64LE(64);
      if (amount > 0n && !reset) {
        retained.push({
          address: token.pubkey,
          lamports: token.account.lamports.toString(),
          reason: 'live token balance; retained until reset',
        });
        continue;
      }
      if (amount > 0n)
        await send({
          programAddress: TOKEN,
          accounts: [writable(token.pubkey), writable(decodeAddress(data, 0)), signer(wallet)],
          data: Buffer.concat([Buffer.from([8]), data.subarray(64, 72)]),
        });
      await send({
        programAddress: TOKEN,
        accounts: [writable(token.pubkey), writable(payer.address), signer(wallet)],
        data: new Uint8Array([9]),
      });
    }
    {
      const tables = await context.rpc
        .getProgramAccounts(ALT, {
          encoding: 'base64',
          commitment: 'confirmed',
          filters: [{ memcmp: { offset: 22n, bytes: wallet.address, encoding: 'base58' } }],
        })
        .send();
      for (const table of tables) {
        const bytes = Buffer.from(table.account.data[0], 'base64');
        if (bytes[21] !== 1 || decodeAddress(bytes, 22) !== wallet.address) continue;
        const members = [];
        for (let offset = 56; offset + 32 <= bytes.length; offset += 32) members.push(decodeAddress(bytes, offset));
        const finished = members.some((a) => finishedBatches.has(a)) && !members.some((a) => liveBatches.has(a));
        if (!reset && !finished && bytes.readBigUInt64LE(4) === 0xffffffffffffffffn) {
          retained.push({
            address: table.pubkey,
            lamports: table.account.lamports.toString(),
            reason: 'active lookup table; retained until batch completion or reset',
          });
          continue;
        }
        if (bytes.readBigUInt64LE(4) === 0xffffffffffffffffn)
          await send({ programAddress: ALT, accounts: [writable(table.pubkey), signer(wallet)], data: u32(3) });
        pendingTables.set(table.pubkey, wallet);
      }
    }
  }
  // Start every cooldown before waiting, including tables owned by different wallets.
  const tableDeadline = Date.now() + 10 * 60_000;
  while (pendingTables.size > 0) {
    const finalizedSlot = await context.rpc.getSlot({ commitment: 'finalized' }).send();
    for (const [table, wallet] of pendingTables) {
      const info = (
        await context.rpc.getAccountInfo(table, { encoding: 'base64', commitment: 'confirmed' }).send()
      ).value;
      if (!info) {
        pendingTables.delete(table);
        continue;
      }
      const deactivated = Buffer.from(info.data[0], 'base64').readBigUInt64LE(4);
      if (finalizedSlot > deactivated + 513n) {
        await send({
          programAddress: ALT,
          accounts: [writable(table), signer(wallet), writable(payer.address)],
          data: u32(4),
        });
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
        await context.rpc.getAccountInfo(buffer.address, { encoding: 'base64', commitment: 'confirmed' }).send()
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
    const balance = (await context.rpc.getBalance(wallet.address, { commitment: 'confirmed' }).send()).value;
    if (balance > 0n) {
      const amount = Buffer.alloc(8);
      amount.writeBigUInt64LE(balance);
      await send({
        programAddress: SYSTEM,
        accounts: [{ ...signer(wallet), role: AccountRole.WRITABLE_SIGNER }, writable(payer.address)],
        data: Buffer.concat([u32(2), amount]),
      });
    }
    if ((await context.rpc.getBalance(wallet.address, { commitment: 'confirmed' }).send()).value !== 0n)
      throw new Error(`wallet ${wallet.address} still holds recoverable SOL`);
  }
  for (const name of await readdir(directory)) {
    if (!/^inventory-.+\.json$/.test(name)) continue;
    const entry = JSON.parse(await readFile(path.join(directory, name), 'utf8')) as { mints?: string[] };
    for (const mint of entry.mints ?? []) {
      const info = (
        await context.rpc.getAccountInfo(address(mint), { encoding: 'base64', commitment: 'confirmed' }).send()
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
      const info = (await context.rpc.getAccountInfo(account, { encoding: 'base64', commitment: 'confirmed' }).send())
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
        await context.rpc.getAccountInfo(item.address, { encoding: 'base64', commitment: 'confirmed' }).send()
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
          commitment: 'confirmed',
          filters: [{ dataSize: 165n }, { memcmp: { offset: 0n, bytes: mint, encoding: 'base58' } }],
        })
        .send();
      for (const item of remaining)
        retained.push({
          address: item.pubkey,
          lamports: item.account.lamports.toString(),
          reason: 'preview mint token account has no recovered signing key',
        });
    }
  const after = (await context.rpc.getBalance(payer.address, { commitment: 'confirmed' }).send()).value;
  const transactions = [];
  let fees = 0n;
  let netRecovered = 0n;
  for (const { signature, lastValidBlockHeight } of await journal.receipts()) {
    let transaction = await context.rpc
      .getTransaction(signature, { commitment: 'confirmed', encoding: 'json', maxSupportedTransactionVersion: 0 })
      .send();
    for (let attempt = 0; transaction === null && attempt < 20; attempt++) {
      await sleep(1_000);
      transaction = await context.rpc
        .getTransaction(signature, { commitment: 'confirmed', encoding: 'json', maxSupportedTransactionVersion: 0 })
        .send();
    }
    if (
      !transaction &&
      (await context.rpc.getBlockHeight({ commitment: 'finalized' }).send()) > BigInt(lastValidBlockHeight)
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
