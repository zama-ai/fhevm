import { TOKEN_PROGRAM_ADDRESS } from '@solana-program/token';
import { prepareTransientStore } from '@fhevm/sdk/solana';
import { type Address, type Signature, type TransactionSigner } from '@solana/kit';
import {
  buildClaimInstruction as buildVaultClaimInstruction,
  buildInitializeTokenAccountInstruction,
  findJoinRecordPda,
  getBatchByIndex,
  getJoinRecord,
} from './vault/index.js';
import { BatchStatus, type BatchTarget, type VaultDirection } from './batchTypes';
import { createFinalizedRpc } from '@fhevm/solana-zama-host';
import type { DemoConfig } from './demoConfig';
import { createDemoClient } from './demoClient';
import { vaultRoots } from './vaultRoots';

type ClaimSession = {
  readonly config: DemoConfig;
  readonly keeper: TransactionSigner;
};

const readClaimStore = async (
  session: ClaimSession,
  position: BatchTarget,
  direction: VaultDirection,
  user: Address,
) => {
  const rpc = createFinalizedRpc(session.config.rpcUrl);
  const roots = vaultRoots(session.config, direction);
  const batch = await getBatchByIndex(rpc, roots, position.batchIndex);
  if (batch.index !== position.batchIndex || batch.addresses.batch !== position.batch) {
    throw new Error(`Batch reference ${position.batch} does not match index ${position.batchIndex}`);
  }
  if (batch.state.status !== BatchStatus.Settled) throw new Error('The batch has not settled yet');

  const joinRecord = await getJoinRecord(rpc, (await findJoinRecordPda({ batch: position.batch, user: user }))[0]);
  if (joinRecord.batch !== position.batch || joinRecord.user !== user) {
    throw new Error('The join record does not match the requested batch and user');
  }
  return { roots, claimed: joinRecord.claimed };
};

/**
 * Sponsors the connected local-demo user's canonical payout account and permissionless claim.
 * Accounts and instructions are derived server-side; the keeper never signs browser-provided messages.
 */
export const claimBatchPayout = async (
  session: ClaimSession,
  position: BatchTarget,
  direction: VaultDirection,
  user: Address,
): Promise<Signature | null> => {
  const { roots, claimed } = await readClaimStore(session, position, direction, user);
  if (claimed) return null;

  const transientStore = await prepareTransientStore({ payer: session.keeper, host: session.config.programs.host });
  const instructions = [
    await buildInitializeTokenAccountInstruction({
      transientStore: transientStore,
      payer: session.keeper,
      owner: user,
      mint: roots.payoutConfidentialMint,
    }),
    await buildVaultClaimInstruction({
      transientStore: transientStore,
      payer: session.keeper,
      user,
      batcher: roots.batcher,
      batch: position.batch,
      payoutConfidentialMint: roots.payoutConfidentialMint,
      payoutUnderlyingMint: roots.payoutUnderlyingMint,
      tokenProgram: TOKEN_PROGRAM_ADDRESS,
    }),
  ];
  return (await createDemoClient(session.config, session.keeper).sendFheTransaction(transientStore, instructions)).context
    .signature;
};
