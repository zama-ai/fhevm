import {
  findTokenAccountPda,
  findEventAuthorityPda as findTokenEventAuthorityPda,
} from '@fhevm/confidential-token';
import { findAssociatedTokenPda } from '@solana-program/token';
import { INSTRUCTIONS_SYSVAR_ADDRESS, type TransientStore } from '@fhevm/sdk/solana';
import type { Address, Instruction, TransactionSigner } from '@solana/kit';

import { getQuitInstructionAsync } from './internal/generated/confidentialBatcher/instructions/quit.js';
import { findBatchAuthorityPda } from './internal/generated/confidentialBatcher/pdas/index.js';
import { batchApp, tokenApp, withDenyRecords, type HostPolicyParameters } from './internal/hostPolicy.js';
import { joinStoreAddress, tokenStoreAddress } from './internal/encryptedStores.js';

/** Roots for a quit. The builder derives the batch and user token accounts, their Stores and the join record. */
export type SolanaVaultQuitParameters = HostPolicyParameters & {
  readonly transientStore: TransientStore;
  /** Signs a pending batch's quit; a refunding quit is permissionless, so a plain address suffices. */
  readonly user: Address | TransactionSigner;
  /** Pays the transfer output rent and the reset execution's ACL rent. */
  readonly payer: TransactionSigner;
  readonly batcher: Address;
  /** The pending or refunding batch being quit. */
  readonly batch: Address;
  readonly joinConfidentialMint: Address;
  /** SPL mint wrapped by `joinConfidentialMint`. Freeze checks the owners' ATAs on this mint. */
  readonly joinUnderlyingMint: Address;
  /** Token program that owns `joinUnderlyingMint` (`Tokenkeg` or Token-2022). */
  readonly tokenProgram: Address;
};

/**
 * Builds the batcher `quit` instruction: the user leaves a pending batch, or a refunding one, and
 * is refunded the exact recorded amount. On-chain this spends the user's joined encrypted value account via
 * `confidential_transfer_from_value` (the from-value arm) and resets it to zero — the SDK only
 * builds the batcher instruction; the from-value transfer is a CPI the program makes internally.
 */
export async function buildQuitInstruction(parameters: SolanaVaultQuitParameters): Promise<Instruction> {
  const { user, joinConfidentialMint: mint, joinUnderlyingMint, tokenProgram } = parameters;
  const userAddress = typeof user === 'string' ? user : user.address;
  const [batchAuthority] = await findBatchAuthorityPda({ batch: parameters.batch });
  const batchJoinTokenAccount = (await findTokenAccountPda({ mint, owner: batchAuthority }))[0];
  const userTokenAccount = (await findTokenAccountPda({ mint, owner: userAddress }))[0];
  const joinMint = tokenApp(mint);
  const batch = batchApp(parameters.batch);
  const [joinMintHcu, batchHcu] = await Promise.all([
    parameters.host.hcuAccounts(joinMint),
    parameters.host.hcuAccounts(batch),
  ]);
  const instruction = await getQuitInstructionAsync({
    transientStore: parameters.transientStore.address,
    instructions: INSTRUCTIONS_SYSVAR_ADDRESS,
    user,
    payer: parameters.payer,
    batcher: parameters.batcher,
    batch: parameters.batch,
    batchAuthority,
    joinConfidentialMint: mint,
    joinUnderlyingMint,
    batchAuthorityAta: (await findAssociatedTokenPda({ owner: batchAuthority, tokenProgram, mint: joinUnderlyingMint }))[0],
    userAta: (await findAssociatedTokenPda({ owner: userAddress, tokenProgram, mint: joinUnderlyingMint }))[0],
    batchJoinTokenAccount,
    userTokenAccount,
    batchBalanceStore: await tokenStoreAddress(mint, batchJoinTokenAccount),
    userBalanceStore: await tokenStoreAddress(mint, userTokenAccount),
    joinStore: await joinStoreAddress(parameters.batch, userAddress),
    confidentialTokenEventAuthority: (await findTokenEventAuthorityPda())[0],
    joinMintHcuBlockMeter: joinMintHcu.hcuBlockMeter,
    joinMintHcuTrustedAppRecord: joinMintHcu.hcuTrustedAppRecord,
    batchHcuBlockMeter: batchHcu.hcuBlockMeter,
    batchHcuTrustedAppRecord: batchHcu.hcuTrustedAppRecord,
  });
  return withDenyRecords(instruction, parameters.host.denyListEnabled, [joinMint, batch, batch]);
}
