import {
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  getAddressEncoder,
  type Address,
  type Signature,
  type TransactionSigner,
} from '@solana/kit';
import { createFhevmPublicDecryptClient, defineFhevmSolanaChain, setFhevmRuntimeConfig } from '@fhevm/sdk/solana';
import {
  buildDispatchBatchInstruction,
  burnedAmountValueAccount,
  deriveJoinRecordAddress,
  getBatchByIndex,
  getBatcher,
  getDeactivateLookupTableInstruction,
  getJoinRecord,
  settleBatch,
} from './vault/index.js';

import {
  BatchStatus,
  type BatchLifecycle,
  type BatchTarget,
  type VaultDirection,
} from './batchTypes';
import type { DemoConfig } from './demoConfig';
import { sendTransaction } from './sendTransaction';
import { vaultRoots } from './vaultRoots';

const DISPATCH_COMPUTE_UNIT_LIMIT = 600_000;
const DEACTIVATE_LOOKUP_TABLE_COMPUTE_UNIT_LIMIT = 50_000;

const addressEncoder = getAddressEncoder();
type Bytes32Hex = Parameters<typeof defineFhevmSolanaChain>[0]['fhevm']['acl']['domainKeys'][number];
export type DemoOperatorSession = {
  readonly config: DemoConfig;
  readonly keeper: TransactionSigner;
};
type DemoUserSession = {
  readonly config: DemoConfig;
  readonly signer: TransactionSigner;
};

const asBytes32Hex = (value: Address): Bytes32Hex =>
  `0x${Array.from(addressEncoder.encode(value), (byte) => byte.toString(16).padStart(2, '0')).join('')}` as Bytes32Hex;

const asBytes32BigEndian = (decimal: string): Uint8Array => {
  const bytes = new Uint8Array(32);
  let value = BigInt(decimal);
  for (let index = 31; index >= 0 && value > 0n; index -= 1) {
    bytes[index] = Number(value & 0xffn);
    value >>= 8n;
  }
  if (value > 0n) throw new Error(`${decimal} does not fit in 32 bytes`);
  return bytes;
};

const currentPinnedBatch = async (
  session: { readonly config: DemoConfig },
  position: BatchTarget,
  direction: VaultDirection,
) => {
  const rpc = createSolanaRpc(session.config.rpcUrl);
  const batch = await getBatchByIndex(rpc, vaultRoots(session.config, direction), position.batchIndex, {
    commitment: 'confirmed',
  });
  if (batch.index !== position.batchIndex || batch.addresses.batch !== position.batch) {
    throw new Error(`Batch reference ${position.batch} does not match index ${position.batchIndex}`);
  }
  return { rpc, batch };
};

type ProofReadinessBody = {
  readonly verified?: unknown;
  readonly status?: unknown;
  readonly code?: unknown;
};

export const classifyProofReadiness = (httpStatus: number, body: ProofReadinessBody | null): boolean => {
  if (httpStatus === 503 && body?.status === 'lagging') return false;
  if (httpStatus < 200 || httpStatus >= 300) {
    const reason =
      typeof body?.status === 'string'
        ? body.status
        : typeof body?.code === 'string'
          ? body.code
          : `HTTP ${httpStatus}`;
    throw new Error(`proof readiness check failed: ${reason}`);
  }
  if (body === null || typeof body.verified !== 'boolean') {
    throw new Error('proof readiness response is malformed');
  }
  return body.verified;
};

const hasReadyProof = async (
  session: { readonly config: DemoConfig },
  batch: Awaited<ReturnType<typeof getBatchByIndex>>,
  direction: VaultDirection,
): Promise<boolean> => {
  const roots = vaultRoots(session.config, direction);
  const burned = await burnedAmountValueAccount(roots.joinConfidentialMint, batch.addresses.batchJoinTokenAccount);
  const burnedTotalHandle = new Uint8Array(batch.state.burnedTotalHandle);
  const handle = `0x${Array.from(burnedTotalHandle, (byte) => byte.toString(16).padStart(2, '0')).join('')}`;
  const query = new URLSearchParams({
    encrypted_value: burned.encryptedValueAddress,
    handle,
  });
  const response = await fetch(`${session.config.proofServiceUrl}/internal/solana/public-proof?${query.toString()}`, {
    headers: { accept: 'application/json' },
    signal: AbortSignal.timeout(5_000),
  });
  const body = (await response.json().catch(() => null)) as ProofReadinessBody | null;
  return classifyProofReadiness(response.status, body);
};

export const readVaultLifecycle = async (
  session: DemoUserSession,
  position: BatchTarget,
  direction: VaultDirection,
): Promise<BatchLifecycle> => {
  const { rpc, batch } = await currentPinnedBatch(session, position, direction);
  if (batch.state.status === BatchStatus.Pending) {
    const batcher = await getBatcher(rpc, vaultRoots(session.config, direction).batcher, { commitment: 'confirmed' });
    const currentSlot = await rpc.getSlot({ commitment: 'confirmed' }).send();
    const dispatchableAt = batch.state.openedSlot + batcher.minBatchAgeSlots;
    return {
      kind: 'awaiting-dispatch',
      remainingSlots: currentSlot >= dispatchableAt ? 0n : dispatchableAt - currentSlot,
    };
  }
  if (batch.state.status === BatchStatus.Dispatched) {
    return { kind: 'proving', proofReady: await hasReadyProof(session, batch, direction) };
  }
  if (batch.state.status === BatchStatus.Settled) {
    const joinRecord = await getJoinRecord(rpc, await deriveJoinRecordAddress(position.batch, session.signer.address), {
      commitment: 'confirmed',
    });
    return {
      kind: 'settled',
      totalJoined: batch.state.totalJoined,
      payoutReceived: batch.state.payoutReceived,
      claimed: joinRecord.claimed,
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
  const batcher = await getBatcher(rpc, roots.batcher, { commitment: 'confirmed' });
  const currentSlot = await rpc.getSlot({ commitment: 'confirmed' }).send();
  if (currentSlot < batch.state.openedSlot + batcher.minBatchAgeSlots) {
    throw new Error('The batch is not old enough to dispatch yet');
  }
  return sendTransaction(
    session.config,
    session.keeper,
    [
      await buildDispatchBatchInstruction({
        payer: session.keeper,
        batcher: roots.batcher,
        batch: position.batch,
        joinConfidentialMint: roots.joinConfidentialMint,
        hostConfig: session.config.hostConfig,
      }),
    ],
    DISPATCH_COMPUTE_UNIT_LIMIT,
  );
};

export const settleVaultBatch = async (
  session: DemoOperatorSession,
  position: BatchTarget,
  direction: VaultDirection,
  // Required, not defaulted to the config's table: this address also names the table this
  // function deactivates on success, so falling back to the batch-0 table would retire a table
  // belonging to a different batch. Callers get it from `lookupTableForBatch`.
  lookupTableAddress: Address,
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
  if (!(await hasReadyProof(session, batch, direction))) throw new Error('The private proof is not ready yet');

  const rpcSubscriptions = createSolanaRpcSubscriptions(session.config.wsUrl);
  setFhevmRuntimeConfig({ auth: { type: 'ApiKeyHeader', value: 'local' } });
  const chain = defineFhevmSolanaChain({
    id: BigInt(session.config.chainId),
    fhevm: {
      relayerUrl: session.config.relayerUrl,
      acl: { domainKeys: [asBytes32Hex(roots.joinConfidentialMint)] },
    },
  });
  const publicDecryptClient = createFhevmPublicDecryptClient({ chain });
  const signature = await settleBatch(chain, { proofServiceUrl: session.config.proofServiceUrl }, session.keeper, {
    rpc,
    rpcSubscriptions,
    runtime: publicDecryptClient.runtime,
    roots,
    batchIndex: position.batchIndex,
    contextId: asBytes32BigEndian(session.config.userDecryptContextId),
    lookupTableAddress,
    authorityFundingLamports: BigInt(session.config.authorityFundingLamports),
    certificateOptions: { timeout: 60_000 },
  });
  // The batch is settled, so its per-batch table has served its one purpose: deactivate it now and
  // the crank in prepareNextBatch reclaims the rent once the cooldown has elapsed. A failed
  // deactivation is a rent-hygiene miss, never a settlement failure — and no longer a permanent
  // one either: the crank deactivates any table whose batch is settled or canceled, so this eager
  // attempt is a shortcut on the happy path rather than the only chance the table gets.
  try {
    await sendTransaction(
      session.config,
      session.keeper,
      [getDeactivateLookupTableInstruction({ lookupTable: lookupTableAddress, authority: session.keeper })],
      DEACTIVATE_LOOKUP_TABLE_COMPUTE_UNIT_LIMIT,
    );
  } catch (error) {
    console.warn(
      `settled, but deactivating lookup table ${lookupTableAddress} failed: ${error instanceof Error ? error.message : String(error)}`,
    );
  }
  return signature;
};
