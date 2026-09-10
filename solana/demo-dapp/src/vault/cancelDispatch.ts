import type { SolanaFheTransactionAccounts } from '@fhevm/sdk/solana';
import type { Address, Instruction, TransactionSigner } from '@solana/kit';

import { getCancelDispatchInstructionAsync } from './internal/generated/confidentialBatcher/instructions/cancelDispatch.js';
import { findBatchAuthorityPda, pendingBurnAddress, tokenAccountAddress } from './internal/batcherPdas.js';
import {
  tokenStateAddress,
  tokenEventAuthorityAddress,
  totalSupplyAuthorityAddress,
  zamaEventAuthorityAddress,
} from './internal/tokenAccounts.js';

export type SolanaVaultCancelDispatchParameters = {
  readonly fhe: SolanaFheTransactionAccounts;
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
    ...parameters.fhe,
    payer: parameters.payer,
    batcher: parameters.batcher,
    batch: parameters.batch,
    batchAuthority,
    joinConfidentialMint: mint,
    totalSupplyAuthority,
    batchJoinTokenAccount,
    batchBalanceState: await tokenStateAddress(mint, batchJoinTokenAccount),
    totalSupplyState: await tokenStateAddress(mint, totalSupplyAuthority),
    pendingBurn: await pendingBurnAddress(mint, batchJoinTokenAccount),
    hostConfig: parameters.hostConfig,
    zamaEventAuthority: await zamaEventAuthorityAddress(),
    confidentialTokenEventAuthority: await tokenEventAuthorityAddress(),
    authorityFundingLamports: parameters.authorityFundingLamports ?? 0n,
  });
}
