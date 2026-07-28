import {
  address,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  getAddressEncoder,
  type Address,
  type Signature,
} from '@solana/kit';
import { createFhevmEncryptClient, defineFhevmSolanaChain, setFhevmRuntimeConfig } from '@fhevm/sdk/solana';
import {
  computeSignerAddress,
  deriveBatchAddresses,
  deriveJoinRecordAddress,
  getCurrentBatch,
  joinBatch,
} from '@fhevm/sdk/solana/vault';

import type { BatchPosition } from './batchTypes';
import type { DemoSession } from './demoSession';
import { loadDemoEncryptionKey } from './encryptionKey';
import { readClaimedSharesHandle } from './revealShares';
import { vaultRoots } from './vaultRoots';

export type RedeemStage = 'proving' | 'joining' | 'joined';

type Bytes32Hex = Parameters<typeof joinBatch>[0]['aclProgramAddress'];
const JOIN_COMPUTE_UNIT_LIMIT = 800_000;
const RECENT_BATCH_SCAN_LIMIT = 32n;
const addressEncoder = getAddressEncoder();

const asBytes32Hex = (value: Address): Bytes32Hex =>
  `0x${Array.from(addressEncoder.encode(value), (byte) => byte.toString(16).padStart(2, '0')).join('')}` as Bytes32Hex;

const activeRedeemKey = (session: DemoSession): string =>
  `fhevm-solana-demo:active-redeem:${session.config.chainId}:${session.config.batchers.redeem.batcher}:${session.signer.address}`;

export type RedeemIntent = BatchPosition & {
  readonly sourceHandle: string;
};

type StoredRedeem = BatchPosition & {
  readonly transaction?: {
    readonly signature: string;
    readonly blockhash: string;
    readonly lastValidBlockHeight: string;
  };
};

export const redactRedeemPosition = (position: Pick<BatchPosition, 'batchIndex' | 'batch'>): BatchPosition => ({
  ...position,
  amountBaseUnits: 0n,
});

export const assertBalanceHandleIsCurrent = (expected: string | undefined, current: string): void => {
  if (expected !== undefined && expected !== current) {
    throw new Error('Your private share balance changed before submission. Try the redeem again.');
  }
};

const writeActiveRedeem = (session: DemoSession, position: StoredRedeem): void => {
  localStorage.setItem(
    activeRedeemKey(session),
    JSON.stringify({
      batchIndex: position.batchIndex.toString(),
      batch: position.batch,
      amountBaseUnits: '0',
      transaction: position.transaction,
    }),
  );
};

const clearActiveRedeem = (session: DemoSession): void => {
  localStorage.removeItem(activeRedeemKey(session));
};

const readActiveRedeem = (session: DemoSession): StoredRedeem | null => {
  try {
    const value = localStorage.getItem(activeRedeemKey(session));
    if (value === null) return null;
    const parsed = JSON.parse(value) as Record<string, unknown>;
    if (
      typeof parsed.batchIndex !== 'string' ||
      typeof parsed.batch !== 'string' ||
      typeof parsed.amountBaseUnits !== 'string'
    ) {
      return null;
    }
    return {
      batchIndex: BigInt(parsed.batchIndex),
      batch: address(parsed.batch),
      amountBaseUnits: 0n,
      transaction:
        typeof parsed.transaction === 'object' &&
        parsed.transaction !== null &&
        typeof (parsed.transaction as Record<string, unknown>).signature === 'string' &&
        typeof (parsed.transaction as Record<string, unknown>).blockhash === 'string' &&
        typeof (parsed.transaction as Record<string, unknown>).lastValidBlockHeight === 'string'
          ? {
              signature: (parsed.transaction as Record<string, string>).signature,
              blockhash: (parsed.transaction as Record<string, string>).blockhash,
              lastValidBlockHeight: (parsed.transaction as Record<string, string>).lastValidBlockHeight,
            }
          : undefined,
    };
  } catch {
    return null;
  }
};

const reconcileSubmittedIntent = async (
  rpc: ReturnType<typeof createSolanaRpc>,
  session: DemoSession,
  intent: StoredRedeem,
): Promise<void> => {
  if (intent.transaction === undefined) {
    clearActiveRedeem(session);
    return;
  }
  const status = (
    await rpc
      .getSignatureStatuses([intent.transaction.signature as Signature], { searchTransactionHistory: true })
      .send()
  ).value[0];
  if (status !== null && status.err !== null) {
    clearActiveRedeem(session);
    return;
  }
  if (status === null) {
    const currentBlockHeight = await rpc.getBlockHeight({ commitment: 'confirmed' }).send();
    if (currentBlockHeight > BigInt(intent.transaction.lastValidBlockHeight)) {
      clearActiveRedeem(session);
      return;
    }
  }
  throw new Error('The previous redeem transaction is still being confirmed. Try again shortly.');
};

const recoverHandleChange = async (
  rpc: ReturnType<typeof createSolanaRpc>,
  session: DemoSession,
  intent: RedeemIntent,
  joinRecord: Address,
  currentHandle: string,
): Promise<BatchPosition | null> => {
  try {
    assertBalanceHandleIsCurrent(intent.sourceHandle, currentHandle);
    return null;
  } catch (handleError) {
    const account = await rpc.getAccountInfo(joinRecord, { commitment: 'confirmed', encoding: 'base64' }).send();
    if (account.value !== null) {
      const saved = readActiveRedeem(session);
      const position =
        saved !== null && saved.batchIndex === intent.batchIndex && saved.batch === intent.batch
          ? redactRedeemPosition(saved)
          : redactRedeemPosition(intent);
      writeActiveRedeem(session, position);
      return position;
    }
    const saved = readActiveRedeem(session);
    if (saved?.transaction !== undefined && saved.batchIndex === intent.batchIndex && saved.batch === intent.batch) {
      await reconcileSubmittedIntent(rpc, session, saved);
    } else {
      clearActiveRedeem(session);
    }
    throw handleError;
  }
};

export const findExistingRedeem = async (session: DemoSession): Promise<BatchPosition | null> => {
  const rpc = createSolanaRpc(session.config.rpcUrl);
  const roots = vaultRoots(session.config, 'redeem');
  const saved = readActiveRedeem(session);
  const current = await getCurrentBatch(rpc, roots, { commitment: 'confirmed' });
  const candidates: Array<{ readonly batchIndex: bigint; readonly batch: Address }> = [];
  if (saved !== null) {
    const savedAddresses = await deriveBatchAddresses(roots, saved.batchIndex);
    if (savedAddresses.batch === saved.batch) candidates.push(saved);
  }
  const oldestIndex =
    current.index >= RECENT_BATCH_SCAN_LIMIT - 1n ? current.index - (RECENT_BATCH_SCAN_LIMIT - 1n) : 0n;
  for (let index = current.index; ; index -= 1n) {
    const addresses = await deriveBatchAddresses(roots, index);
    if (!candidates.some((candidate) => candidate.batch === addresses.batch)) {
      candidates.push({ batchIndex: index, batch: addresses.batch });
    }
    if (index === oldestIndex) break;
  }
  for (const candidate of candidates) {
    const joinRecord = await deriveJoinRecordAddress(candidate.batch, session.signer.address);
    const account = await rpc.getAccountInfo(joinRecord, { commitment: 'confirmed', encoding: 'base64' }).send();
    if (account.value === null) continue;
    const position =
      saved !== null && saved.batchIndex === candidate.batchIndex && saved.batch === candidate.batch
        ? redactRedeemPosition(saved)
        : redactRedeemPosition(candidate);
    writeActiveRedeem(session, position);
    return position;
  }
  if (saved !== null) await reconcileSubmittedIntent(rpc, session, saved);
  return null;
};

export const joinRedeemBatch = async (
  session: DemoSession,
  amountBaseUnits: bigint,
  sourceHandle: string,
  onStage: (stage: RedeemStage) => void,
): Promise<BatchPosition> => {
  session.assertActive();
  if (amountBaseUnits <= 0n) throw new Error('Redeem amount must be positive');
  const { config, signer } = session;
  const rpc = createSolanaRpc(config.rpcUrl);
  const rpcSubscriptions = createSolanaRpcSubscriptions(config.wsUrl);
  const roots = vaultRoots(config, 'redeem');
  const saved = readActiveRedeem(session);
  if (saved !== null) {
    const savedAddresses = await deriveBatchAddresses(roots, saved.batchIndex);
    if (savedAddresses.batch === saved.batch) {
      const savedJoinRecord = await deriveJoinRecordAddress(saved.batch, signer.address);
      const savedJoin = await rpc
        .getAccountInfo(savedJoinRecord, { commitment: 'confirmed', encoding: 'base64' })
        .send();
      if (savedJoin.value !== null) {
        const position = redactRedeemPosition(saved);
        writeActiveRedeem(session, position);
        return position;
      }
    }
    if (saved.transaction !== undefined) await reconcileSubmittedIntent(rpc, session, saved);
    else clearActiveRedeem(session);
  }
  const batch = await getCurrentBatch(rpc, roots, { commitment: 'confirmed' });
  const joinRecord = await deriveJoinRecordAddress(batch.addresses.batch, signer.address);
  const existing = await rpc.getAccountInfo(joinRecord, { commitment: 'confirmed', encoding: 'base64' }).send();
  if (existing.value !== null) {
    const position = redactRedeemPosition({ batchIndex: batch.index, batch: batch.addresses.batch });
    writeActiveRedeem(session, position);
    return position;
  }
  if (batch.state.status !== 0) throw new Error('The current redeem batch is no longer accepting redemptions');

  const currentHandle = await readClaimedSharesHandle(session);
  const intent = {
    batchIndex: batch.index,
    batch: batch.addresses.batch,
    amountBaseUnits,
    sourceHandle,
  };
  const recoveredBeforeProof = await recoverHandleChange(rpc, session, intent, joinRecord, currentHandle);
  if (recoveredBeforeProof !== null) return recoveredBeforeProof;

  const supportsThreads = globalThis.crossOriginIsolated === true && typeof SharedArrayBuffer !== 'undefined';
  setFhevmRuntimeConfig({
    auth: { type: 'ApiKeyHeader', value: 'local' },
    singleThread: !supportsThreads,
  });
  const chain = defineFhevmSolanaChain({
    id: BigInt(config.chainId),
    fhevm: {
      relayerUrl: config.relayerUrl,
      acl: { domainKeys: [asBytes32Hex(roots.joinConfidentialMint)] },
    },
  });
  const aclProgramAddress = config.aclProgram as Bytes32Hex;
  const encryptClient = createFhevmEncryptClient({
    chain,
    aclProgramAddress,
    options: { fheEncryptionKey: await loadDemoEncryptionKey(config) },
  });
  const computeSigner = await computeSignerAddress(roots.joinConfidentialMint);

  onStage('proving');
  session.assertActive();
  const inputProof = await encryptClient.buildInputProof({
    contractAddress: asBytes32Hex(computeSigner),
    userAddress: asBytes32Hex(signer.address),
    values: [{ type: 'uint64', value: intent.amountBaseUnits }],
  });
  const inputProofResult = await encryptClient.submitInputProof({ inputProof });

  const handleBeforeJoin = await readClaimedSharesHandle(session);
  const recoveredBeforeJoin = await recoverHandleChange(rpc, session, intent, joinRecord, handleBeforeJoin);
  if (recoveredBeforeJoin !== null) return recoveredBeforeJoin;

  onStage('joining');
  session.assertActive();
  await joinBatch(
    { solanaChain: chain, aclProgramAddress },
    {
      rpc,
      rpcSubscriptions,
      inputProof,
      inputProofResult,
      inputIndex: 0,
      user: signer,
      payer: signer,
      batcher: roots.batcher,
      batch: batch.addresses.batch,
      joinConfidentialMint: roots.joinConfidentialMint,
      hostConfig: config.hostConfig,
      computeUnitLimit: JOIN_COMPUTE_UNIT_LIMIT,
      onTransactionSigned: (transaction) => {
        session.assertActive();
        writeActiveRedeem(session, {
          ...redactRedeemPosition(intent),
          transaction: {
            signature: transaction.signature,
            blockhash: transaction.blockhash,
            lastValidBlockHeight: transaction.lastValidBlockHeight.toString(),
          },
        });
      },
    },
  );

  onStage('joined');
  const position = redactRedeemPosition(intent);
  writeActiveRedeem(session, position);
  return position;
};
