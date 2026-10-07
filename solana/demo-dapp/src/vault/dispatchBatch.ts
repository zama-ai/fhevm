import { tokenStoreAddress } from './internal/encryptedStores.js';
import { findEventAuthorityPda as findZamaEventAuthorityPda } from '@fhevm/solana-zama-host';
import {
  findPendingBurnPda,
  findTokenAccountPda,
  findEventAuthorityPda as findTokenEventAuthorityPda,
  findTotalSupplyAuthorityPda,
} from '@fhevm/confidential-token';
import { findAssociatedTokenPda } from '@solana-program/token';
import { INSTRUCTIONS_SYSVAR_ADDRESS, type TransientStore } from '@fhevm/sdk/solana';
import type { Address, Instruction, TransactionSigner } from '@solana/kit';
import { getDispatchInstructionAsync } from './internal/generated/confidentialBatcher/instructions/dispatch.js';
import { findBatchAuthorityPda } from './internal/generated/confidentialBatcher/pdas/index.js';


/**
 * Semantic roots for the batcher `dispatch` instruction. Every other account the on-chain handler
 * validates (`dispatch.rs`) — the batch authority, the join mint's total-supply
 * authority, the batch's join token account, the balance / total-supply / burned-amount encrypted stores,
 * and both event authorities — is derived internally from these, so callers never hand-build the
 * account map.
 */
export type SolanaVaultDispatchParameters = {
  readonly transientStore: TransientStore;
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
  const batchJoinTokenAccount = (await findTokenAccountPda({ mint: joinConfidentialMint, owner: batchAuthority }))[0];
  const totalSupplyAuthority = (await findTotalSupplyAuthorityPda({ mint: joinConfidentialMint }))[0];
  return getDispatchInstructionAsync({
    transientStore: parameters.transientStore.address,
    instructions: INSTRUCTIONS_SYSVAR_ADDRESS,
    payer: parameters.payer,
    batcher: parameters.batcher,
    batch: parameters.batch,
    batchAuthority,
    joinConfidentialMint,
    joinUnderlyingMint: parameters.joinUnderlyingMint,
    batchAuthorityAta: (await findAssociatedTokenPda({
      owner: batchAuthority,
      tokenProgram: parameters.tokenProgram,
      mint: parameters.joinUnderlyingMint,
    }))[0],
    totalSupplyAuthority,
    batchJoinTokenAccount,
    batchBalanceStore: await tokenStoreAddress(joinConfidentialMint, batchJoinTokenAccount),
    totalSupplyStore: await tokenStoreAddress(joinConfidentialMint, totalSupplyAuthority),
    pendingBurn: (await findPendingBurnPda({ mint: joinConfidentialMint, tokenAccount: batchJoinTokenAccount }))[0],
    zamaEventAuthority: (await findZamaEventAuthorityPda())[0],
    hostConfig: parameters.hostConfig,
    confidentialTokenEventAuthority: (await findTokenEventAuthorityPda())[0],
  });
}
