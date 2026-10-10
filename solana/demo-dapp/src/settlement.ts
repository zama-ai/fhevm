import { TOKEN_PROGRAM_ADDRESS } from '@solana-program/token';
import type { Bytes32Hex } from '@fhevm/sdk/types';
import type { Address, Signature, TransactionSigner } from '@solana/kit';
import {
  createFhevmPublicDecryptClient,
  defineFhevmSolanaChain,
  prepareTransientStore,
  setFhevmRuntimeConfig,
} from '@fhevm/sdk/solana';
import {
  buildCancelDispatchInstruction,
  buildDispatchBatchInstruction,
  buildQuitInstruction,
  dispatchableAt,
  getBatchJoinRecords,
  type VaultDemoRoots,
  getReclaimBatchAuthorityInstructionAsync,
  findJoinRecordPda,
  getBatchByIndex,
  getBatcher,
  getJoinRecord,
  getCloseJoinRecordInstructionAsync,
  readHostPolicy,
  settleBatch,
  settleDeadline,
} from './vault/index.js';

import {
  BatchStatus,
  type BatchLifecycle,
  type BatchTarget,
  type VaultDirection,
} from './batchTypes';
import { createFinalizedRpc } from '@fhevm/solana-zama-host';
import { fetchSysvarClock } from '@solana/sysvars';
import type { DemoConfig } from './demoConfig';
import { createDemoClient } from './demoClient';
import { vaultRoots } from './vaultRoots';

export type DemoOperatorSession = {
  readonly relayerApiKey: string;
  readonly config: DemoConfig;
  readonly keeper: TransactionSigner;
};
type DemoUserSession = {
  readonly config: DemoConfig;
  readonly signer: TransactionSigner;
};

const currentPinnedBatch = async (
  session: { readonly config: DemoConfig },
  position: BatchTarget,
  direction: VaultDirection,
) => {
  const rpc = createFinalizedRpc(session.config.rpcUrl);
  const batch = await getBatchByIndex(rpc, vaultRoots(session.config, direction), position.batchIndex);
  if (batch.index !== position.batchIndex || batch.addresses.batch !== position.batch) {
    throw new Error(`Batch reference ${position.batch} does not match index ${position.batchIndex}`);
  }
  return { rpc, batch };
};

/** The user's join record in `batch`, with its state while it exists: a quit or close removes it. */
const userJoinRecord = async (rpc: ReturnType<typeof createFinalizedRpc>, batch: Address, user: Address) => {
  const address = (await findJoinRecordPda({ batch, user }))[0];
  const exists = (await rpc.getAccountInfo(address, { encoding: 'base64' }).send()).value !== null;
  return { address, state: exists ? await getJoinRecord(rpc, address) : null };
};

export const readVaultLifecycle = async (
  session: DemoUserSession,
  position: BatchTarget,
  direction: VaultDirection,
): Promise<BatchLifecycle> => {
  const { rpc, batch } = await currentPinnedBatch(session, position, direction);
  if (batch.state.status === BatchStatus.Pending) {
    const batcher = await getBatcher(rpc, vaultRoots(session.config, direction).batcher);
    const { unixTimestamp } = await fetchSysvarClock(rpc);
    const from = dispatchableAt(batch.state, batcher);
    return {
      kind: 'awaiting-dispatch',
      remainingSecs: unixTimestamp >= from ? 0n : from - unixTimestamp,
    };
  }
  if (batch.state.status === BatchStatus.Dispatched) {
    return { kind: 'dispatched' };
  }
  if (batch.state.status === BatchStatus.Settled) {
    const joinRecord = (await userJoinRecord(rpc, position.batch, session.signer.address)).state;
    return {
      kind: 'settled',
      totalJoined: batch.state.totalJoined,
      payoutReceived: batch.state.payoutReceived,
      claimed: joinRecord === null || joinRecord.claimed,
    };
  }
  if (batch.state.status === BatchStatus.Canceled) return { kind: 'canceled' };
  if (batch.state.status === BatchStatus.Refunding) {
    const joinRecord = (await userJoinRecord(rpc, position.batch, session.signer.address)).state;
    return { kind: 'refunding', refunded: joinRecord === null };
  }
  throw new Error(`Unsupported batch status ${batch.state.status}`);
};

export const dispatchVaultBatch = async (
  session: DemoOperatorSession,
  position: BatchTarget,
  direction: VaultDirection,
): Promise<Signature | null> => {
  const roots = vaultRoots(session.config, direction);
  const { rpc, batch } = await currentPinnedBatch(session, position, direction);
  if (batch.state.status >= BatchStatus.Dispatched) return null;
  const batcher = await getBatcher(rpc, roots.batcher);
  if ((await fetchSysvarClock(rpc)).unixTimestamp < dispatchableAt(batch.state, batcher)) {
    throw new Error('The batch is not old enough to dispatch yet');
  }
  const transientStore = await prepareTransientStore({ payer: session.keeper, host: session.config.programs.host });
  const dispatch = await buildDispatchBatchInstruction({
    transientStore,
    payer: session.keeper,
    batcher: roots.batcher,
    batch: position.batch,
    joinConfidentialMint: roots.joinConfidentialMint,
    joinUnderlyingMint: roots.joinUnderlyingMint,
    tokenProgram: TOKEN_PROGRAM_ADDRESS,
    host: await readHostPolicy(rpc),
  });
  return (await createDemoClient(session.config, session.keeper).sendFheTransaction(transientStore, [dispatch])).context
    .signature;
};

/**
 * Runs `quit` for every participant still in a refunding batch, so each gets their exact contribution
 * back without signing: a refunding quit is permissionless and pays only the participant's own token
 * account. Each quit closes its join record, so a rerun refunds only who is left. One participant's
 * failure does not hold back the others; the failed participants are reported once all were tried.
 */
const refundParticipants = async (
  session: DemoOperatorSession,
  roots: VaultDemoRoots,
  batch: Address,
): Promise<Signature | null> => {
  const keeperClient = createDemoClient(session.config, session.keeper);
  const host = await readHostPolicy(keeperClient.rpc);
  let signature: Signature | null = null;
  const failed: Address[] = [];
  for (const { user } of await getBatchJoinRecords(keeperClient.rpc, batch)) {
    try {
      const transientStore = await prepareTransientStore({ payer: session.keeper, host: session.config.programs.host });
      const quit = await buildQuitInstruction({
        transientStore,
        user,
        payer: session.keeper,
        batcher: roots.batcher,
        batch,
        joinConfidentialMint: roots.joinConfidentialMint,
        joinUnderlyingMint: roots.joinUnderlyingMint,
        tokenProgram: TOKEN_PROGRAM_ADDRESS,
        host,
      });
      signature = (await keeperClient.sendFheTransaction(transientStore, [quit])).context.signature;
    } catch (error) {
      console.warn(`refunding ${user} failed: ${error instanceof Error ? error.message : String(error)}`);
      failed.push(user);
    }
  }
  if (failed.length > 0) throw new Error(`Refunds failed for ${failed.join(', ')}; a rerun retries them`);
  return signature;
};

/**
 * The keeper's pass over a dispatched batch: settles it, or cancels it once its settle deadline has
 * passed. A batch that ends refunding, cancelled or worth zero vault shares, then has every
 * participant refunded.
 */
export const settleOrCancelVaultBatch = async (
  session: DemoOperatorSession,
  position: BatchTarget,
  direction: VaultDirection,
): Promise<Signature | null> => {
  const roots = vaultRoots(session.config, direction);
  const { rpc, batch } = await currentPinnedBatch(session, position, direction);
  let status = batch.state.status;
  if (status === BatchStatus.Settled || status === BatchStatus.Canceled) return null;
  if (status !== BatchStatus.Dispatched && status !== BatchStatus.Refunding) {
    throw new Error('Dispatch the batch before settlement');
  }

  let signature: Signature | null = null;
  if (status === BatchStatus.Dispatched) {
    const keeperClient = createDemoClient(session.config, session.keeper);
    const batcher = await getBatcher(rpc, roots.batcher);
    if ((await fetchSysvarClock(rpc)).unixTimestamp >= settleDeadline(batch.state, batcher)) {
      // Settle is refused from the deadline on; cancelling opens the participants' refunds instead.
      const transientStore = await prepareTransientStore({ payer: session.keeper, host: session.config.programs.host });
      const cancel = await buildCancelDispatchInstruction({
        transientStore,
        payer: session.keeper,
        batcher: roots.batcher,
        batch: position.batch,
        joinConfidentialMint: roots.joinConfidentialMint,
        authorityFundingLamports: BigInt(session.config.authorityFundingLamports),
        host: await readHostPolicy(rpc),
      });
      signature = (await keeperClient.sendFheTransaction(transientStore, [cancel])).context.signature;
    } else {
      setFhevmRuntimeConfig({ auth: { type: 'ApiKeyHeader', value: session.relayerApiKey } });
      const chain = defineFhevmSolanaChain({
        id: BigInt(session.config.chainId),
        fhevm: { relayerUrl: session.config.relayerUrl, programs: { host: { address: session.config.aclProgram as Bytes32Hex } } },
      });
      const publicDecryptClient = createFhevmPublicDecryptClient({ chain, rpc });
      signature = await settleBatch(publicDecryptClient, keeperClient, {
        roots,
        batchIndex: position.batchIndex,
        authorityFundingLamports: BigInt(session.config.authorityFundingLamports),
        certificateOptions: { timeout: 60_000 },
        host: await readHostPolicy(rpc),
      });
      // The batch is settled or refunding, so its authority PDA has paid its last owner-charged rent: take its unspent
      // funding back. A failure here is a rent-hygiene miss, never a settlement failure, and not a
      // permanent one: the reclaim pass in prepareNextBatch drains any authority whose batch is finished.
      try {
        await keeperClient.sendTransaction([
          await getReclaimBatchAuthorityInstructionAsync({
            authority: session.keeper,
            batcher: roots.batcher,
            batch: batch.addresses.batch,
            batchAuthority: batch.addresses.batchAuthority,
            joinConfidentialMint: roots.joinConfidentialMint,
          }),
        ]);
      } catch (error) {
        console.warn(
          `settled, but reclaiming the batch authority failed (the next prepare retries): ${error instanceof Error ? error.message : String(error)}`,
        );
      }
    }
    // A cancel, or a deposit total worth zero vault shares, leaves the batch refunding.
    status = (await currentPinnedBatch(session, position, direction)).batch.state.status;
  }
  if (status === BatchStatus.Refunding) return (await refundParticipants(session, roots, position.batch)) ?? signature;
  return signature;
};

/** User-signed rent return after claim/cancel. A quit closes its record itself. */
export const closeSpentJoinRecord = async (session: DemoUserSession, position: BatchTarget): Promise<void> => {
  const rpc = createFinalizedRpc(session.config.rpcUrl);
  const { address, state } = await userJoinRecord(rpc, position.batch, session.signer.address);
  if (state === null) return;
  await createDemoClient(session.config, session.signer).sendTransaction([
    await getCloseJoinRecordInstructionAsync({ user: session.signer, batch: position.batch, joinRecord: address }),
  ]);
};
