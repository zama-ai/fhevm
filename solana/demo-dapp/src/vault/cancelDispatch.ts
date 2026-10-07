import { findEventAuthorityPda as findZamaEventAuthorityPda } from '@fhevm/solana-zama-host';
import { findEventAuthorityPda as findTokenEventAuthorityPda, findTotalSupplyAuthorityPda } from '@fhevm/confidential-token';
import { INSTRUCTIONS_SYSVAR_ADDRESS, type TransientStore } from '@fhevm/sdk/solana';
import type { Address, Instruction, TransactionSigner } from '@solana/kit';

import { getCancelDispatchInstructionAsync } from './internal/generated/confidentialBatcher/instructions/cancelDispatch.js';
import { findBatchAuthorityPda, pendingBurnAddress, tokenAccountAddress, tokenStateAddress } from './internal/batcherPdas.js';

export type SolanaVaultCancelDispatchParameters = {
  readonly transientStore: TransientStore;
  /** Join-mint wrapper authority; also pays optional batch-authority funding. */
  readonly payer: TransactionSigner;
  readonly batcher: Address;
  readonly batch: Address;
  readonly joinConfidentialMint: Address;
  readonly hostConfig: Address;
  readonly authorityFundingLamports?: bigint;
};

/** Builds the wrapper-authorized dispatch cancellation that opens participant refunds. */
export async function buildCancelDispatchInstruction(
  parameters: SolanaVaultCancelDispatchParameters,
): Promise<Instruction> {
  const mint = parameters.joinConfidentialMint;
  const [batchAuthority] = await findBatchAuthorityPda({ batch: parameters.batch });
  const batchJoinTokenAccount = await tokenAccountAddress(mint, batchAuthority);
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
    batchBalanceStore: await tokenStateAddress(mint, batchJoinTokenAccount),
    totalSupplyStore: await tokenStateAddress(mint, totalSupplyAuthority),
    pendingBurn: await pendingBurnAddress(mint, batchJoinTokenAccount),
    hostConfig: parameters.hostConfig,
    zamaEventAuthority: (await findZamaEventAuthorityPda())[0],
    confidentialTokenEventAuthority: (await findTokenEventAuthorityPda())[0],
    authorityFundingLamports: parameters.authorityFundingLamports ?? 0n,
  });
}
