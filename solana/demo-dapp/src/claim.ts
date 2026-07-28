import { createSolanaRpc, type Address, type Instruction, type TransactionSigner } from '@solana/kit';
import {
  buildClaimInstruction,
  buildInitializeTokenAccountInstruction,
  deriveJoinRecordAddress,
  getBatchByIndex,
  getJoinRecord,
  tokenAccountAddress,
} from '@fhevm/sdk/solana/vault';

import type { BatchTarget, VaultDirection } from './batchTypes';
import type { DemoConfig } from './demoConfig';
import { sendTransaction } from './sendTransaction';
import { vaultRoots } from './vaultRoots';

const BATCH_SETTLED = 2;
const CLAIM_COMPUTE_UNIT_LIMIT = 1_200_000;
const SYSTEM_PROGRAM_ADDRESS = '11111111111111111111111111111111';

type ClaimSession = {
  readonly config: DemoConfig;
  readonly keeper: TransactionSigner;
};

const readClaimState = async (
  session: ClaimSession,
  position: BatchTarget,
  direction: VaultDirection,
  user: Address,
) => {
  const rpc = createSolanaRpc(session.config.rpcUrl);
  const roots = vaultRoots(session.config, direction);
  const batch = await getBatchByIndex(rpc, roots, position.batchIndex, { commitment: 'confirmed' });
  if (batch.index !== position.batchIndex || batch.addresses.batch !== position.batch) {
    throw new Error(`Batch reference ${position.batch} does not match index ${position.batchIndex}`);
  }
  if (batch.state.status !== BATCH_SETTLED) throw new Error('The batch has not settled yet');

  const joinRecord = await getJoinRecord(rpc, await deriveJoinRecordAddress(position.batch, user), {
    commitment: 'confirmed',
  });
  if (joinRecord.batch !== position.batch || joinRecord.user !== user) {
    throw new Error('The join record does not match the requested batch and user');
  }
  return { rpc, roots, claimed: joinRecord.claimed };
};

const buildClaimInstructions = async (
  session: ClaimSession,
  position: BatchTarget,
  direction: VaultDirection,
  user: Address,
): Promise<{ readonly instructions: readonly Instruction[]; readonly initializesAccount: boolean } | null> => {
  const { rpc, roots, claimed } = await readClaimState(session, position, direction, user);
  if (claimed) return null;

  const payoutTokenAccount = await tokenAccountAddress(roots.payoutConfidentialMint, user);
  const account = (
    await rpc.getAccountInfo(payoutTokenAccount, { commitment: 'confirmed', encoding: 'base64' }).send()
  ).value;
  if (
    account !== null &&
    account.owner !== session.config.programs.token &&
    account.owner !== SYSTEM_PROGRAM_ADDRESS
  ) {
    throw new Error(`Payout account ${payoutTokenAccount} is owned by an unexpected program`);
  }

  const initializesAccount = account === null || account.owner === SYSTEM_PROGRAM_ADDRESS;
  const instructions: Instruction[] = [];
  if (initializesAccount) {
    instructions.push(
      await buildInitializeTokenAccountInstruction({
        payer: session.keeper,
        owner: user,
        mint: roots.payoutConfidentialMint,
        hostConfig: session.config.hostConfig,
      }),
    );
  }
  instructions.push(
    await buildClaimInstruction({
      payer: session.keeper,
      user,
      batcher: roots.batcher,
      batch: position.batch,
      payoutConfidentialMint: roots.payoutConfidentialMint,
      hostConfig: session.config.hostConfig,
    }),
  );
  return { instructions, initializesAccount };
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
): Promise<void> => {
  const plan = await buildClaimInstructions(session, position, direction, user);
  if (plan === null) return;
  try {
    await sendTransaction(session.config, session.keeper, plan.instructions, CLAIM_COMPUTE_UNIT_LIMIT);
  } catch (error) {
    if (!plan.initializesAccount) throw error;
    const retryPlan = await buildClaimInstructions(session, position, direction, user);
    if (retryPlan === null) return;
    if (retryPlan.initializesAccount) throw error;
    await sendTransaction(session.config, session.keeper, retryPlan.instructions, CLAIM_COMPUTE_UNIT_LIMIT);
  }
};
