import { tokenStoreAddress } from './internal/encryptedStores.js';
import {
  findPendingBurnPda,
  findTokenAccountPda,
  findEventAuthorityPda as findTokenEventAuthorityPda,
  findTotalSupplyAuthorityPda,
} from '@fhevm/confidential-token';
import { INSTRUCTIONS_SYSVAR_ADDRESS, type TransientStore } from '@fhevm/sdk/solana';
import type { Address, Instruction, TransactionSigner } from '@solana/kit';
import { getCancelDispatchInstructionAsync } from './internal/generated/confidentialBatcher/instructions/cancelDispatch.js';
import { findBatchAuthorityPda } from './internal/generated/confidentialBatcher/pdas/index.js';

export type SolanaVaultCancelDispatchParameters = {
  readonly transientStore: TransientStore;
  /** Join-mint wrapper authority; also pays optional batch-authority funding. */
  readonly payer: TransactionSigner;
  readonly batcher: Address;
  readonly batch: Address;
  readonly joinConfidentialMint: Address;
  readonly authorityFundingLamports?: bigint;
};

/** Builds the wrapper-authorized dispatch cancellation that opens participant refunds. */
export async function buildCancelDispatchInstruction(
  parameters: SolanaVaultCancelDispatchParameters,
): Promise<Instruction> {
  const mint = parameters.joinConfidentialMint;
  const [batchAuthority] = await findBatchAuthorityPda({ batch: parameters.batch });
  const batchJoinTokenAccount = (await findTokenAccountPda({ mint, owner: batchAuthority }))[0];
  const totalSupplyAuthority = (await findTotalSupplyAuthorityPda({ mint }))[0];
  return getCancelDispatchInstructionAsync({
    transientStore: parameters.transientStore.address,
    instructions: INSTRUCTIONS_SYSVAR_ADDRESS,
    payer: parameters.payer,
    batcher: parameters.batcher,
    batch: parameters.batch,
    batchAuthority,
    joinConfidentialMint: mint,
    totalSupplyAuthority,
    batchJoinTokenAccount,
    batchBalanceStore: await tokenStoreAddress(mint, batchJoinTokenAccount),
    totalSupplyStore: await tokenStoreAddress(mint, totalSupplyAuthority),
    pendingBurn: (await findPendingBurnPda({ mint, tokenAccount: batchJoinTokenAccount }))[0],
    confidentialTokenEventAuthority: (await findTokenEventAuthorityPda())[0],
    authorityFundingLamports: parameters.authorityFundingLamports ?? 0n,
  });
}
