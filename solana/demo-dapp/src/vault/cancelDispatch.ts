import type { Address, Instruction, TransactionSigner } from '@solana/kit';

import { getCancelDispatchInstructionAsync } from './internal/generated/confidentialBatcher/instructions/cancelDispatch.js';
import {
  burnedAmountValueAccount,
  findBatchAuthorityPda,
  pendingBurnAddress,
  tokenAccountAddress,
} from './internal/batcherPdas.js';
import {
  balanceValueAddress,
  computeSignerAddress,
  tokenEventAuthorityAddress,
  totalSupplyAuthorityAddress,
  totalSupplyValueAddress,
  zamaEventAuthorityAddress,
} from './internal/tokenValueAccount.js';

export type SolanaVaultCancelDispatchParameters = {
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
  const totalSupplyAuthority = await totalSupplyAuthorityAddress(mint);
  return getCancelDispatchInstructionAsync({
    payer: parameters.payer,
    batcher: parameters.batcher,
    batch: parameters.batch,
    batchAuthority,
    joinConfidentialMint: mint,
    joinComputeSigner: await computeSignerAddress(mint),
    totalSupplyAuthority,
    batchJoinTokenAccount,
    batchBalanceValue: await balanceValueAddress(mint, batchJoinTokenAccount),
    totalSupplyValue: await totalSupplyValueAddress(mint, totalSupplyAuthority),
    batchBurnedAmountValue: (await burnedAmountValueAccount(mint, batchJoinTokenAccount)).encryptedValueAddress,
    pendingBurn: await pendingBurnAddress(mint, batchJoinTokenAccount),
    hostConfig: parameters.hostConfig,
    zamaEventAuthority: await zamaEventAuthorityAddress(),
    confidentialTokenEventAuthority: await tokenEventAuthorityAddress(),
    authorityFundingLamports: parameters.authorityFundingLamports ?? 0n,
  });
}
