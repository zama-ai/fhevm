import { prepareTransientStore } from '@fhevm/sdk/solana';
import type { Address, TransactionSigner } from '@solana/kit';
import {
  getReclaimBatchAuthorityInstructionAsync,
  getBatchByIndex,
  getBatcher,
  getCurrentBatch,
  openBatchForBatcher,
  readHostPolicy,
} from './vault/index.js';

import { BatchStatus, isBatchFinished, type VaultDirection } from './batchTypes';
import { createDemoClient } from './demoClient';
import type { DemoConfig } from './demoConfig';
import { vaultRoots } from './vaultRoots';

/**
 * How many of the most recent batches the reclaim pass inspects. It runs on the page's join path
 * (two RPC reads per batch), so it is bounded; settle reclaims eagerly, and a batch that finishes
 * later than this window (a long-canceled straggler) is reclaimed by hand.
 */
export const RECLAIM_SCAN_WINDOW = 8n;

/**
 * Rent hygiene: every batch's authority PDA is
 * funded at open (and again at cancel/settle) to pay the rent token CPIs charge to the account
 * owner, and keeps whatever is left. Once the batch is settled, canceled or refunding nothing
 * charges the authority any more, so the operator takes the remainder back with
 * `reclaim_batch_authority`. `settleVaultBatch` reclaims eagerly on the happy path; this pass
 * catches every batch that finished another way (a canceled dispatch, a zero-total cancel at
 * settle, a process that exited between settle and reclaim) and is idempotent: a drained
 * authority is skipped. One batch read and one balance read per batch in the last
 * `RECLAIM_SCAN_WINDOW` the direction has opened. Returns how many batches were reclaimed;
 * failures are logged and retried by a later prepare.
 */
export const reclaimFinishedBatchAuthorities = async (
  config: DemoConfig,
  keeper: TransactionSigner,
  direction: VaultDirection,
): Promise<number> => {
  const client = createDemoClient(config, keeper);
  const { rpc } = client;
  const roots = vaultRoots(config, direction);
  const batcher = await getBatcher(rpc, roots.batcher);
  let reclaimed = 0;
  const first = batcher.nextBatchIndex > RECLAIM_SCAN_WINDOW ? batcher.nextBatchIndex - RECLAIM_SCAN_WINDOW : 0n;
  for (let index = first; index < batcher.nextBatchIndex; index += 1n) {
    try {
      const batch = await getBatchByIndex(rpc, roots, index);
      if (!isBatchFinished(batch.state.status)) continue;
      const { value: lamports } = await rpc
        .getBalance(batch.addresses.batchAuthority)
        .send();
      if (lamports === 0n) continue;
      await client.sendTransaction([
        await getReclaimBatchAuthorityInstructionAsync({
          authority: keeper,
          batcher: roots.batcher,
          batch: batch.addresses.batch,
          batchAuthority: batch.addresses.batchAuthority,
          joinConfidentialMint: roots.joinConfidentialMint,
        }),
      ]);
      reclaimed += 1;
    } catch (error) {
      console.warn(
        `reclaiming the authority funding of ${direction} batch ${index} failed (a later prepare retries): ${error instanceof Error ? error.message : String(error)}`,
      );
    }
  }
  return reclaimed;
};

export type PreparedBatch = {
  readonly batchIndex: bigint;
  readonly batch: Address;
};

export const prepareNextBatch = async (
  config: DemoConfig,
  keeper: TransactionSigner,
  direction: VaultDirection,
): Promise<PreparedBatch> => {
  const client = createDemoClient(config, keeper);
  const { rpc } = client;
  const roots = vaultRoots(config, direction);
  const current = await getCurrentBatch(rpc, roots);

  const batchIndex = current.state.status === BatchStatus.Pending ? current.index : current.index + 1n;
  if (current.state.status !== BatchStatus.Pending) {
    const transientStore = await prepareTransientStore({ payer: keeper, host: config.programs.host });
    const openBatchInstruction = await openBatchForBatcher({
      transientStore,
      roots,
      batchIndex,
      payer: keeper,
      authorityFundingLamports: BigInt(config.authorityFundingLamports),
      host: await readHostPolicy(rpc),
    });
    await client.sendFheTransaction(transientStore, [openBatchInstruction]);
  }
  const batch =
    current.state.status === BatchStatus.Pending
      ? current.addresses.batch
      : (await getCurrentBatch(rpc, roots)).addresses.batch;

  // Rent hygiene: drain this direction's finished batch authorities while we are here.
  await reclaimFinishedBatchAuthorities(config, keeper, direction);
  return { batchIndex, batch };
};
