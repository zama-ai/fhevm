import {
  address,
  appendTransactionMessageInstructions,
  assertIsFullySignedTransaction,
  assertIsTransactionWithBlockhashLifetime,
  assertIsTransactionWithinSizeLimit,
  compileTransaction,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  createTransactionMessage,
  getAddressEncoder,
  getSignatureFromTransaction,
  sendAndConfirmTransactionFactory,
  setTransactionMessageComputeUnitLimit,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  type Address,
  type Instruction,
  type Signature,
} from '@solana/kit';
import { createFhevmEncryptClient, defineFhevmSolanaChain, setFhevmRuntimeConfig } from '@fhevm/sdk/solana';
import {
  buildWrapUsdcInstruction,
  computeSignerAddress,
  deriveBatchAddresses,
  deriveJoinRecordAddress,
  getBatchByIndex,
  getBatcher,
  getCurrentBatch,
  getOrCreateConfidentialTokenAccountInstruction,
  getJoinRecord,
  joinBatch,
} from './vault/index.js';

import type { BatchPosition, BatchTarget } from './batchTypes';
import type { DemoSession } from './demoSession';
import { loadDemoEncryptionKey } from './encryptionKey';
import { recordTransactionEvidence } from './evidenceStore';
import { readClaimedUsdcHandle } from './revealShares';
import { simulateSignedTransactionLocally, simulateUnsignedTransactionLocally } from './transactionSimulation';
import { vaultRoots } from './vaultRoots';

export type DepositStage = 'preparing' | 'shielding' | 'proving' | 'joining' | 'joined';

export type DepositSource = 'usdc' | 'cusdc';

export const needsShieldTransaction = (source: DepositSource): boolean => source === 'usdc';

export const assertDepositSourceHandle = (expectedSourceHandle: string | undefined, currentHandle: string): void => {
  if (expectedSourceHandle === undefined || currentHandle !== expectedSourceHandle) {
    throw new Error('Your private cUSDC balance changed. Reveal it again before depositing.');
  }
};

type Bytes32Hex = Parameters<typeof joinBatch>[0]['aclProgramAddress'];

const USDC_DECIMALS = 6;
const SHIELD_COMPUTE_UNIT_LIMIT = 1_200_000;
const JOIN_COMPUTE_UNIT_LIMIT = 800_000;

const addressEncoder = getAddressEncoder();

const asBytes32Hex = (value: Address): Bytes32Hex => {
  const bytes = new Uint8Array(addressEncoder.encode(value));
  return `0x${Array.from(bytes, (byte) => byte.toString(16).padStart(2, '0')).join('')}` as Bytes32Hex;
};

export const usdcToBaseUnits = (amount: number): bigint => {
  if (!Number.isFinite(amount) || amount <= 0 || amount > 1_000) {
    throw new Error('Deposit amount must be between 0 and 1,000 USDC');
  }
  return BigInt(Math.round(amount * 10 ** USDC_DECIMALS));
};

const depositRoots = (session: DemoSession) => vaultRoots(session.config, 'deposit');

const shieldJournalKey = (session: DemoSession): string =>
  `fhevm-solana-demo:shield:${session.config.chainId}:${session.config.batchers.deposit.batcher}:${session.signer.address}`;
const activeDepositKey = (session: DemoSession): string =>
  `fhevm-solana-demo:active-deposit:${session.config.chainId}:${session.config.batchers.deposit.batcher}:${session.signer.address}`;

type ShieldJournal = {
  readonly amountBaseUnits: string;
  readonly signature: string;
  readonly blockhash: string;
  readonly lastValidBlockHeight: string;
  readonly state: 'submitted' | 'confirmed';
};

export type SubmittedDepositTransaction = {
  readonly signature: string;
  readonly blockhash: string;
  readonly lastValidBlockHeight: string;
};

export type StoredDeposit = BatchPosition & {
  readonly transaction?: SubmittedDepositTransaction;
};

const readShieldJournal = (session: DemoSession): ShieldJournal | null => {
  try {
    const value = localStorage.getItem(shieldJournalKey(session));
    if (value === null) return null;
    const parsed = JSON.parse(value) as Partial<ShieldJournal>;
    if (
      typeof parsed.amountBaseUnits !== 'string' ||
      typeof parsed.signature !== 'string' ||
      typeof parsed.blockhash !== 'string' ||
      typeof parsed.lastValidBlockHeight !== 'string' ||
      (parsed.state !== 'submitted' && parsed.state !== 'confirmed')
    ) {
      return null;
    }
    return parsed as ShieldJournal;
  } catch {
    return null;
  }
};

const writeShieldJournal = (session: DemoSession, journal: ShieldJournal): void => {
  try {
    localStorage.setItem(shieldJournalKey(session), JSON.stringify(journal));
  } catch (error) {
    throw new Error(
      `Cannot safely persist the shield transaction before submission: ${
        error instanceof Error ? error.message : String(error)
      }`,
    );
  }
};

const clearShieldJournal = (session: DemoSession): void => {
  try {
    localStorage.removeItem(shieldJournalKey(session));
  } catch {
    // Storage may be blocked in hardened browser profiles; chain state remains authoritative.
  }
};

const writeActiveDeposit = (session: DemoSession, deposit: StoredDeposit): void => {
  localStorage.setItem(
    activeDepositKey(session),
    JSON.stringify({
      batchIndex: deposit.batchIndex.toString(),
      batch: deposit.batch,
      amountBaseUnits: deposit.amountBaseUnits.toString(),
      transaction: deposit.transaction,
    }),
  );
};

const clearActiveDeposit = (session: DemoSession): void => {
  localStorage.removeItem(activeDepositKey(session));
};

const readActiveDeposit = (session: DemoSession): StoredDeposit | null => {
  try {
    const value = localStorage.getItem(activeDepositKey(session));
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
      amountBaseUnits: BigInt(parsed.amountBaseUnits),
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

export const reconcileDepositTransaction = async (
  rpc: ReturnType<typeof createSolanaRpc>,
  transaction: SubmittedDepositTransaction,
): Promise<'pending' | 'retry'> => {
  const status = (
    await rpc.getSignatureStatuses([transaction.signature as Signature], { searchTransactionHistory: true }).send()
  ).value[0];
  if (status !== null && status.err !== null) return 'retry';
  if (status === null) {
    const currentBlockHeight = await rpc.getBlockHeight({ commitment: 'confirmed' }).send();
    if (currentBlockHeight > BigInt(transaction.lastValidBlockHeight)) return 'retry';
  }
  return 'pending';
};

export const reconcileSavedDeposit = async (
  rpc: ReturnType<typeof createSolanaRpc>,
  session: DemoSession,
  saved: StoredDeposit,
): Promise<BatchPosition | null> => {
  const expectedBatch = await deriveBatchAddresses(depositRoots(session), saved.batchIndex);
  if (expectedBatch.batch !== saved.batch) {
    clearActiveDeposit(session);
    clearShieldJournal(session);
    return null;
  }
  const joinRecord = await deriveJoinRecordAddress(saved.batch, session.signer.address);
  const account = await rpc.getAccountInfo(joinRecord, { commitment: 'confirmed', encoding: 'base64' }).send();
  if (account.value !== null) {
    if (saved.transaction !== undefined) {
      recordTransactionEvidence(session, { label: 'Deposit', signature: saved.transaction.signature as Signature });
    }
    const confirmed = {
      batchIndex: saved.batchIndex,
      batch: saved.batch,
      amountBaseUnits: saved.amountBaseUnits,
    };
    writeActiveDeposit(session, confirmed);
    clearShieldJournal(session);
    return confirmed;
  }
  if (saved.transaction !== undefined) {
    const outcome = await reconcileDepositTransaction(rpc, saved.transaction);
    if (outcome === 'pending') {
      throw new Error('The previous deposit transaction is still being confirmed. Try again shortly.');
    }
  }
  clearActiveDeposit(session);
  return null;
};

export async function findExistingDeposit(session: DemoSession): Promise<BatchPosition | null> {
  const rpc = createSolanaRpc(session.config.rpcUrl);
  const saved = readActiveDeposit(session);
  if (saved !== null) {
    const reconciled = await reconcileSavedDeposit(rpc, session, saved);
    if (reconciled !== null) return reconciled;
  }
  const batch = await getCurrentBatch(rpc, depositRoots(session), { commitment: 'confirmed' });
  const joinRecord = await deriveJoinRecordAddress(batch.addresses.batch, session.signer.address);
  const account = await rpc.getAccountInfo(joinRecord, { commitment: 'confirmed', encoding: 'base64' }).send();
  if (account.value === null) return null;
  clearShieldJournal(session);
  const result = { batchIndex: batch.index, batch: batch.addresses.batch, amountBaseUnits: 0n };
  writeActiveDeposit(session, result);
  return result;
}

export const hasClaimedDeposit = async (session: DemoSession): Promise<boolean> => {
  const rpc = createSolanaRpc(session.config.rpcUrl);
  const roots = depositRoots(session);
  const batcher = await getBatcher(rpc, roots.batcher, { commitment: 'confirmed' });
  for (let batchIndex = 0n; batchIndex < batcher.nextBatchIndex; batchIndex += 1n) {
    const batch = await deriveBatchAddresses(roots, batchIndex);
    const joinRecordAddress = await deriveJoinRecordAddress(batch.batch, session.signer.address);
    const account = await rpc.getAccountInfo(joinRecordAddress, { commitment: 'confirmed', encoding: 'base64' }).send();
    if (account.value !== null) {
      const joinRecord = await getJoinRecord(rpc, joinRecordAddress, { commitment: 'confirmed' });
      if (joinRecord.claimed) return true;
    }
  }
  return false;
};

export async function depositToVault(
  session: DemoSession,
  amount: number,
  onStage: (stage: DepositStage) => void,
  target?: BatchTarget,
  source: DepositSource = 'usdc',
  expectedSourceHandle?: string,
): Promise<BatchPosition> {
  session.assertActive();
  const { config, signer } = session;
  const amountBaseUnits = usdcToBaseUnits(amount);
  const rpc = createSolanaRpc(config.rpcUrl);
  const rpcSubscriptions = createSolanaRpcSubscriptions(config.wsUrl);
  const sendAndConfirm = sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions });
  const roots = depositRoots(session);
  const saved = readActiveDeposit(session);
  if (
    saved !== null &&
    (target === undefined || (saved.batchIndex === target.batchIndex && saved.batch === target.batch))
  ) {
    const reconciled = await reconcileSavedDeposit(rpc, session, saved);
    if (reconciled !== null) {
      onStage('joined');
      return reconciled;
    }
  }
  const batch =
    target === undefined
      ? await getCurrentBatch(rpc, roots, { commitment: 'confirmed' })
      : await getBatchByIndex(rpc, roots, target.batchIndex, { commitment: 'confirmed' });
  if (target !== undefined && (batch.index !== target.batchIndex || batch.addresses.batch !== target.batch)) {
    throw new Error('The prepared deposit batch no longer matches the requested batch');
  }
  const joinRecord = await deriveJoinRecordAddress(batch.addresses.batch, signer.address);
  const joinRecordAccount = await rpc
    .getAccountInfo(joinRecord, { commitment: 'confirmed', encoding: 'base64' })
    .send();
  if (joinRecordAccount.value !== null) {
    clearShieldJournal(session);
    throw new Error('This wallet already joined the current batch. Reconnect to recover the confirmed deposit.');
  }
  if (batch.state.status !== 0) throw new Error('The current deposit batch is no longer accepting deposits');
  if (source === 'cusdc') {
    const currentHandle = await readClaimedUsdcHandle(session);
    assertDepositSourceHandle(expectedSourceHandle, currentHandle);
  }

  const send = async (
    instructions: readonly Instruction[],
    computeUnitLimit: number,
    beforeSend?: (journal: Omit<ShieldJournal, 'amountBaseUnits' | 'state'>) => void,
  ): Promise<Signature> => {
    const { value: latestBlockhash } = await rpc.getLatestBlockhash({ commitment: 'confirmed' }).send();
    const base = setTransactionMessageFeePayerSigner(signer, createTransactionMessage({ version: 0 }));
    const withLifetime = setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, base);
    const withComputeLimit = setTransactionMessageComputeUnitLimit(computeUnitLimit, withLifetime);
    const message = appendTransactionMessageInstructions(instructions, withComputeLimit);
    session.assertActive();
    await simulateUnsignedTransactionLocally(rpc, compileTransaction(message), 'Shield transaction');
    session.assertActive();
    const transaction = await signTransactionMessageWithSigners(message);
    session.assertActive();
    assertIsFullySignedTransaction(transaction);
    assertIsTransactionWithBlockhashLifetime(transaction);
    assertIsTransactionWithinSizeLimit(transaction);
    await simulateSignedTransactionLocally(rpc, transaction, 'Signed shield transaction');
    session.assertActive();
    const signature = getSignatureFromTransaction(transaction);
    beforeSend?.({
      signature,
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight.toString(),
    });
    await sendAndConfirm(transaction, { commitment: 'confirmed', skipPreflight: true });
    return signature;
  };

  let shieldAlreadyConfirmed = false;
  const shieldJournal = needsShieldTransaction(source) ? readShieldJournal(session) : null;
  if (shieldJournal?.amountBaseUnits === amountBaseUnits.toString()) {
    if (shieldJournal.state === 'confirmed') {
      shieldAlreadyConfirmed = true;
    } else {
      const status = (
        await rpc
          .getSignatureStatuses([shieldJournal.signature as Signature], { searchTransactionHistory: true })
          .send()
      ).value[0];
      if (status !== null && status.err === null) {
        writeShieldJournal(session, { ...shieldJournal, state: 'confirmed' });
        shieldAlreadyConfirmed = true;
      } else if (status !== null) {
        clearShieldJournal(session);
      } else {
        const currentBlockHeight = await rpc.getBlockHeight({ commitment: 'confirmed' }).send();
        if (currentBlockHeight <= BigInt(shieldJournal.lastValidBlockHeight)) {
          throw new Error('The shield transaction is still being confirmed. Try again shortly.');
        }
        clearShieldJournal(session);
      }
    }
  } else if (shieldJournal !== null) {
    clearShieldJournal(session);
  }
  if (shieldAlreadyConfirmed && shieldJournal !== null) {
    recordTransactionEvidence(session, { label: 'Shield USDC', signature: shieldJournal.signature as Signature });
  }
  if (needsShieldTransaction(source) && !shieldAlreadyConfirmed) {
    onStage('preparing');
    const initializeJoinTokenAccount = await getOrCreateConfidentialTokenAccountInstruction(rpc, {
      payer: signer,
      owner: signer.address,
      mint: config.mints.joinConfidential,
      hostConfig: config.hostConfig,
    });
    const shieldInstructions: Instruction[] = initializeJoinTokenAccount === null ? [] : [initializeJoinTokenAccount];
    shieldInstructions.push(
      await buildWrapUsdcInstruction({
        owner: signer,
        mint: config.mints.joinConfidential,
        underlyingMint: config.mints.joinUnderlying,
        hostConfig: config.hostConfig,
        amount: amountBaseUnits,
      }),
    );

    onStage('shielding');
    let submittedJournal: ShieldJournal | undefined;
    const shieldSignature = await send(shieldInstructions, SHIELD_COMPUTE_UNIT_LIMIT, (submitted) => {
      submittedJournal = {
        ...submitted,
        amountBaseUnits: amountBaseUnits.toString(),
        state: 'submitted',
      };
      writeShieldJournal(session, submittedJournal);
    });
    if (submittedJournal === undefined) throw new Error('Shield transaction journal was not created');
    writeShieldJournal(session, { ...submittedJournal, state: 'confirmed' });
    recordTransactionEvidence(session, { label: 'Shield USDC', signature: shieldSignature });
  }

  const supportsThreads = globalThis.crossOriginIsolated === true && typeof SharedArrayBuffer !== 'undefined';
  setFhevmRuntimeConfig({
    auth: { type: 'ApiKeyHeader', value: 'local' },
    singleThread: !supportsThreads,
  });
  const chain = defineFhevmSolanaChain({
    id: BigInt(config.chainId),
    fhevm: {
      relayerUrl: config.relayerUrl,
      acl: { domainKeys: [asBytes32Hex(config.mints.joinConfidential)] },
    },
  });
  const aclProgramAddress = config.aclProgram as Bytes32Hex;
  const encryptClient = createFhevmEncryptClient({
    chain,
    aclProgramAddress,
    options: { fheEncryptionKey: await loadDemoEncryptionKey(config) },
  });
  const joinComputeSigner = await computeSignerAddress(config.mints.joinConfidential);

  onStage('proving');
  session.assertActive();
  const inputProof = await encryptClient.buildInputProof({
    contractAddress: asBytes32Hex(joinComputeSigner),
    userAddress: asBytes32Hex(signer.address),
    values: [{ type: 'uint64', value: amountBaseUnits }],
  });
  const inputProofResult = await encryptClient.submitInputProof({ inputProof });

  onStage('joining');
  session.assertActive();
  if (source === 'cusdc') {
    const currentHandle = await readClaimedUsdcHandle(session);
    assertDepositSourceHandle(expectedSourceHandle, currentHandle);
  }
  let joinSignature: Signature | undefined;
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
        joinSignature = transaction.signature;
        writeActiveDeposit(session, {
          batchIndex: batch.index,
          batch: batch.addresses.batch,
          amountBaseUnits,
          transaction: {
            signature: transaction.signature,
            blockhash: transaction.blockhash,
            lastValidBlockHeight: transaction.lastValidBlockHeight.toString(),
          },
        });
      },
    },
  );
  if (joinSignature !== undefined) {
    recordTransactionEvidence(session, { label: 'Deposit', signature: joinSignature });
  }

  clearShieldJournal(session);
  onStage('joined');
  const result = { batchIndex: batch.index, batch: batch.addresses.batch, amountBaseUnits };
  writeActiveDeposit(session, result);
  return result;
}
