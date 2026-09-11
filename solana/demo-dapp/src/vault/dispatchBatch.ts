import type { SolanaFheTransactionAccounts } from '@fhevm/sdk/solana';
import type { Address, Instruction, TransactionSigner } from '@solana/kit';

import { getDispatchInstructionAsync } from './internal/generated/confidentialBatcher/instructions/dispatch.js';

import { findBatchAuthorityPda, pendingBurnAddress, tokenAccountAddress } from './internal/batcherPdas.js';
import {
  associatedTokenAddress,
  tokenStateAddress,
  tokenEventAuthorityAddress,
  totalSupplyAuthorityAddress,
  zamaEventAuthorityAddress,
} from './internal/tokenAccounts.js';

/**
 * Semantic roots for the batcher `dispatch` instruction. Every other account the on-chain handler
 * validates (`dispatch.rs`) — the batch authority, the join mint's total-supply
 * authority, the batch's join token account, the balance / total-supply / burned-amount encrypted stores,
 * and both event authorities — is derived internally from these, so callers never hand-build the
 * account map.
 */
export type SolanaVaultDispatchParameters = {
  readonly fhe: SolanaFheTransactionAccounts;
  /** Pays the rent for the burn's output encrypted store. Anyone — dispatch is permissionless. */
  readonly payer: TransactionSigner;
  /** Batcher config account. */
  readonly batcher: Address;
  /** The full batch being dispatched. */
  readonly batch: Address;
  /** Confidential mint the batch total is burned on (`batcher.join_confidential_mint`). */
  readonly joinConfidentialMint: Address;
  /** SPL mint wrapped by `joinConfidentialMint`. Freeze checks the batch authority's ATA. */
  readonly joinUnderlyingMint: Address;
  /** Token program that owns `joinUnderlyingMint` (`Tokenkeg` or Token-2022). */
  readonly tokenProgram: Address;
  /** ZamaHost config PDA (demo-config `hostConfig`). */
  readonly hostConfig: Address;
};

/**
 * Builds the permissionless `dispatch` instruction: once a batch is old enough, it burns the batch
 * account's full encrypted balance and records the created-public burned handle the KMS will certify
 * at settle.
 */
export async function buildDispatchBatchInstruction(parameters: SolanaVaultDispatchParameters): Promise<Instruction> {
  const { joinConfidentialMint } = parameters;
  const [batchAuthority] = await findBatchAuthorityPda({ batch: parameters.batch });
  const batchJoinTokenAccount = await tokenAccountAddress(joinConfidentialMint, batchAuthority);
  const totalSupplyAuthority = await totalSupplyAuthorityAddress(joinConfidentialMint);
  return getDispatchInstructionAsync({
    ...parameters.fhe,
    payer: parameters.payer,
    batcher: parameters.batcher,
    batch: parameters.batch,
    batchAuthority,
    joinConfidentialMint,
    joinUnderlyingMint: parameters.joinUnderlyingMint,
    batchAuthorityAta: await associatedTokenAddress(
      batchAuthority,
      parameters.joinUnderlyingMint,
      parameters.tokenProgram,
    ),
    totalSupplyAuthority,
    batchJoinTokenAccount,
    batchBalanceStore: await tokenStateAddress(joinConfidentialMint, batchJoinTokenAccount),
    totalSupplyStore: await tokenStateAddress(joinConfidentialMint, totalSupplyAuthority),
    pendingBurn: await pendingBurnAddress(joinConfidentialMint, batchJoinTokenAccount),
    zamaEventAuthority: await zamaEventAuthorityAddress(),
    hostConfig: parameters.hostConfig,
    confidentialTokenEventAuthority: await tokenEventAuthorityAddress(),
  });
}
