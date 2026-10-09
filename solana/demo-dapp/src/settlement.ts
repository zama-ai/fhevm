import { TOKEN_PROGRAM_ADDRESS } from '@solana-program/token';
import type { Bytes32Hex } from '@fhevm/sdk/types';
import type { Signature, TransactionSigner } from '@solana/kit';
import {
  createFhevmPublicDecryptClient,
  defineFhevmSolanaChain,
  prepareTransientStore,
  setFhevmRuntimeConfig,
} from '@fhevm/sdk/solana';
import {
  buildCancelDispatchInstruction,
  buildDispatchBatchInstruction,
  dispatchableAt,
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
    const recordAddress = (await findJoinRecordPda({ batch: position.batch, user: session.signer.address }))[0];
    const exists = (await rpc.getAccountInfo(recordAddress, { encoding: "base64" }).send()).value !== null;
    const joinRecord = exists ? await getJoinRecord(rpc, recordAddress) : null;
    return {
      kind: 'settled',
      totalJoined: batch.state.totalJoined,
      payoutReceived: batch.state.payoutReceived,
      claimed: joinRecord === null || joinRecord.claimed,
    };
  }
  if (batch.state.status === BatchStatus.Canceled) return { kind: 'canceled' };
  if (batch.state.status === BatchStatus.Refunding) return { kind: 'refunding' };
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

export const settleOrCancelVaultBatch = async (
  session: DemoOperatorSession,
  position: BatchTarget,
  direction: VaultDirection,
): Promise<Signature | null> => {
  const roots = vaultRoots(session.config, direction);
  const { rpc, batch } = await currentPinnedBatch(session, position, direction);
  if (
    batch.state.status === BatchStatus.Settled ||
    batch.state.status === BatchStatus.Canceled ||
    batch.state.status === BatchStatus.Refunding
  )
    return null;
  if (batch.state.status !== BatchStatus.Dispatched) throw new Error('Dispatch the batch before settlement');

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
    return (await keeperClient.sendFheTransaction(transientStore, [cancel])).context.signature;
  }
  setFhevmRuntimeConfig({ auth: { type: 'ApiKeyHeader', value: session.relayerApiKey } });
  const chain = defineFhevmSolanaChain({
    id: BigInt(session.config.chainId),
    fhevm: { relayerUrl: session.config.relayerUrl, programs: { host: { address: session.config.aclProgram as Bytes32Hex } } },
  });
  const publicDecryptClient = createFhevmPublicDecryptClient({ chain, rpc });
  const signature = await settleBatch(publicDecryptClient, keeperClient, {
    roots,
    batchIndex: position.batchIndex,
    authorityFundingLamports: BigInt(session.config.authorityFundingLamports),
    certificateOptions: { timeout: 60_000 },
    host: await readHostPolicy(rpc),
  });
  // The batch is settled, so its authority PDA has paid its last owner-charged rent: take its unspent
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
  return signature;
};

/** User-signed rent return after claim/cancel. A quit closes its record itself. */
export const closeSpentJoinRecord = async (session: DemoUserSession, position: BatchTarget): Promise<void> => {
  const rpc = createFinalizedRpc(session.config.rpcUrl);
  const record = (await findJoinRecordPda({ batch: position.batch, user: session.signer.address }))[0];
  if ((await rpc.getAccountInfo(record, { encoding: 'base64' }).send()).value === null) return;
  await createDemoClient(session.config, session.signer).sendTransaction([
    await getCloseJoinRecordInstructionAsync({ user: session.signer, batch: position.batch, joinRecord: record }),
  ]);
};
