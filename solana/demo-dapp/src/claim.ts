import {
  appendTransactionMessageInstructions,
  assertIsFullySignedTransaction,
  assertIsTransactionWithBlockhashLifetime,
  assertIsTransactionWithinSizeLimit,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  createTransactionMessage,
  sendAndConfirmTransactionFactory,
  setTransactionMessageComputeUnitLimit,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  type Instruction,
} from "@solana/kit";
import {
  buildClaimInstruction,
  buildInitializeTokenAccountInstruction,
  deriveJoinRecordAddress,
  getBatchByIndex,
  getJoinRecord,
  tokenAccountAddress,
} from "@fhevm/sdk/solana/vault";

import type { DemoSession } from "./demoSession";
import type { DepositResult } from "./deposit";
import { vaultRoots, type VaultDirection } from "./vaultRoots";

const BATCH_SETTLED = 2;
const CLAIM_COMPUTE_UNIT_LIMIT = 1_200_000;

export const claimBatchPayout = async (
  session: DemoSession,
  deposit: DepositResult,
  direction: VaultDirection,
): Promise<void> => {
  const rpc = createSolanaRpc(session.config.rpcUrl);
  const roots = vaultRoots(session.config, direction);
  const batch = await getBatchByIndex(rpc, roots, deposit.batchIndex, { commitment: "confirmed" });
  if (batch.index !== deposit.batchIndex || batch.addresses.batch !== deposit.batch) {
    throw new Error(`Deposit batch ${deposit.batch} is no longer the current demo batch`);
  }
  if (batch.state.status !== BATCH_SETTLED) throw new Error("The batch has not settled yet");

  const joinRecordAddress = await deriveJoinRecordAddress(deposit.batch, session.signer.address);
  const joinRecord = await getJoinRecord(rpc, joinRecordAddress, { commitment: "confirmed" });
  if (joinRecord.claimed) return;

  const payoutTokenAccount = await tokenAccountAddress(
    roots.payoutConfidentialMint,
    session.signer.address,
  );
  const payoutTokenAccountInfo = await rpc
    .getAccountInfo(payoutTokenAccount, { commitment: "confirmed", encoding: "base64" })
    .send();
  const instructions: Instruction[] =
    payoutTokenAccountInfo.value === null
      ? [
          await buildInitializeTokenAccountInstruction({
            owner: session.signer,
            mint: roots.payoutConfidentialMint,
            hostConfig: session.config.hostConfig,
          }),
        ]
      : [];
  instructions.push(
    await buildClaimInstruction({
      payer: session.signer,
      user: session.signer.address,
      batcher: roots.batcher,
      batch: deposit.batch,
      payoutConfidentialMint: roots.payoutConfidentialMint,
      hostConfig: session.config.hostConfig,
    }),
  );

  const rpcSubscriptions = createSolanaRpcSubscriptions(session.config.wsUrl);
  const sendAndConfirm = sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions });
  const { value: latestBlockhash } = await rpc.getLatestBlockhash({ commitment: "confirmed" }).send();
  const base = setTransactionMessageFeePayerSigner(session.signer, createTransactionMessage({ version: 0 }));
  const withLifetime = setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, base);
  const withComputeLimit = setTransactionMessageComputeUnitLimit(CLAIM_COMPUTE_UNIT_LIMIT, withLifetime);
  const message = appendTransactionMessageInstructions(instructions, withComputeLimit);
  session.assertActive();
  const transaction = await signTransactionMessageWithSigners(message);
  session.assertActive();
  assertIsFullySignedTransaction(transaction);
  assertIsTransactionWithBlockhashLifetime(transaction);
  assertIsTransactionWithinSizeLimit(transaction);
  await sendAndConfirm(transaction, { commitment: "confirmed" });
};
