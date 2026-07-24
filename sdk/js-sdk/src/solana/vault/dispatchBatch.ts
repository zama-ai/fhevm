import type { Address, Instruction, TransactionSigner } from '@solana/kit';

import { getDispatchInstructionAsync } from './internal/generated/confidentialBatcher/instructions/dispatch.js';
import { burnedAmountLineage, findBatchAuthorityPda, tokenAccountAddress } from './internal/batcherPdas.js';
import {
  balanceValueAddress,
  computeSignerAddress,
  tokenEventAuthorityAddress,
  totalSupplyAuthorityAddress,
  totalSupplyValueAddress,
  zamaEventAuthorityAddress,
} from './internal/tokenLineage.js';

/**
 * Semantic roots for the batcher `dispatch` instruction. Every other account the on-chain handler
 * validates (`dispatch.rs`) — the batch authority, the join mint's compute signer and total-supply
 * authority, the batch's join token account, the balance / total-supply / burned-amount value accounts,
 * and both event authorities — is derived internally from these, so callers never hand-build the
 * account map.
 */
export type SolanaVaultDispatchParameters = {
  /** Pays the rent for the burn's output value account. Anyone — dispatch is permissionless. */
  readonly payer: TransactionSigner;
  /** Batcher config account. */
  readonly batcher: Address;
  /** The full batch being dispatched. */
  readonly batch: Address;
  /** Confidential mint the batch total is burned on (`batcher.join_confidential_mint`). */
  readonly joinConfidentialMint: Address;
  /** ZamaHost config PDA (demo-config `hostConfig`). */
  readonly hostConfig: Address;
};

/**
 * Builds the permissionless `dispatch` instruction: once a batch is old enough, it burns the batch
 * account's full encrypted balance and records the born-public burned handle the KMS will certify
 * at settle.
 */
export async function buildDispatchBatchInstruction(parameters: SolanaVaultDispatchParameters): Promise<Instruction> {
  const { joinConfidentialMint } = parameters;
  const [batchAuthority] = await findBatchAuthorityPda({ batch: parameters.batch });
  const batchJoinTokenAccount = await tokenAccountAddress(joinConfidentialMint, batchAuthority);
  const totalSupplyAuthority = await totalSupplyAuthorityAddress(joinConfidentialMint);
  return getDispatchInstructionAsync({
    payer: parameters.payer,
    batcher: parameters.batcher,
    batch: parameters.batch,
    batchAuthority,
    joinConfidentialMint,
    joinComputeSigner: await computeSignerAddress(joinConfidentialMint),
    totalSupplyAuthority,
    batchJoinTokenAccount,
    batchBalanceValue: await balanceValueAddress(joinConfidentialMint, batchJoinTokenAccount),
    totalSupplyValue: await totalSupplyValueAddress(joinConfidentialMint, totalSupplyAuthority),
    batchBurnedAmountValue: (await burnedAmountLineage(joinConfidentialMint, batchJoinTokenAccount))
      .encryptedValueAddress,
    zamaEventAuthority: await zamaEventAuthorityAddress(),
    hostConfig: parameters.hostConfig,
    confidentialTokenEventAuthority: await tokenEventAuthorityAddress(),
  });
}
