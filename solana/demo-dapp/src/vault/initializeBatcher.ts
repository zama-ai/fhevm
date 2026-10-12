import { findEventAuthorityPda as findTokenEventAuthorityPda } from '@fhevm/confidential-token';
import { INSTRUCTIONS_SYSVAR_ADDRESS, type TransientStore } from '@fhevm/sdk/solana';
import type { Address, Instruction, TransactionSigner } from '@solana/kit';
import { tokenApp, withDenyRecords, type HostPolicyParameters } from './internal/hostPolicy.js';
import { getInitializeBatcherInstructionAsync } from './internal/generated/confidentialBatcher/instructions/initializeBatcher.js';
import type { BatchDirection } from './internal/generated/confidentialBatcher/types/batchDirection.js';
import { deriveBatchAddresses, type VaultDemoRoots } from './derive.js';

export type SolanaVaultInitializeBatcherParameters = HostPolicyParameters & {
  readonly transientStore: TransientStore;
  /** Pays the batcher's and batch 0's rent and batch 0's authority funding. */
  readonly payer: TransactionSigner;
  /** The new batcher account's keypair. */
  readonly batcher: TransactionSigner;
  /** The batcher's topology, other than the batcher itself. */
  readonly roots: Omit<VaultDemoRoots, 'batcher'>;
  readonly minBatchAgeSecs: bigint;
  readonly settleDeadlineSecs: bigint;
  readonly direction: BatchDirection;
  /** Key that may pause the batcher, set its policy and hand the role on. */
  readonly admin: Address;
  /** Lamports batch 0's authority is funded with to pay its owner-charged rent. */
  readonly authorityFundingLamports: bigint;
};

/**
 * Initializes a batcher, which opens batch 0 — the single call the demo seeder makes per batcher. It
 * derives batch 0's accounts (its authority, its join/payout token accounts and their balance
 * encrypted stores, and its SPL accounts) from the roots, and appends the deny records of both mints.
 */
export async function buildInitializeBatcherInstruction(
  parameters: SolanaVaultInitializeBatcherParameters,
): Promise<Instruction> {
  const roots = { ...parameters.roots, batcher: parameters.batcher.address };
  const batch = await deriveBatchAddresses(roots, 0n);
  const joinMint = tokenApp(roots.joinConfidentialMint);
  const payoutMint = tokenApp(roots.payoutConfidentialMint);
  const [joinMintHcu, payoutMintHcu] = await Promise.all([
    parameters.host.hcuAccounts(joinMint),
    parameters.host.hcuAccounts(payoutMint),
  ]);
  const instruction = await getInitializeBatcherInstructionAsync({
    transientStore: parameters.transientStore.address,
    instructions: INSTRUCTIONS_SYSVAR_ADDRESS,
    payer: parameters.payer,
    batcher: parameters.batcher,
    vault: roots.vault,
    batch: batch.batch,
    joinConfidentialMint: roots.joinConfidentialMint,
    batchJoinTokenAccount: batch.batchJoinTokenAccount,
    // A token account has one encrypted store, holding both its balance and its burned amount.
    batchJoinBalanceStore: batch.batchBurnedAmountStore,
    payoutConfidentialMint: roots.payoutConfidentialMint,
    batchPayoutTokenAccount: batch.batchPayoutTokenAccount,
    batchPayoutBalanceStore: batch.batchPayoutBalanceStore,
    joinUnderlyingMint: roots.joinUnderlyingMint,
    payoutUnderlyingMint: roots.payoutUnderlyingMint,
    confidentialTokenEventAuthority: (await findTokenEventAuthorityPda())[0],
    minBatchAgeSecs: parameters.minBatchAgeSecs,
    settleDeadlineSecs: parameters.settleDeadlineSecs,
    direction: parameters.direction,
    admin: parameters.admin,
    authorityFundingLamports: parameters.authorityFundingLamports,
    joinMintHcuBlockMeter: joinMintHcu.hcuBlockMeter,
    joinMintHcuTrustedAppRecord: joinMintHcu.hcuTrustedAppRecord,
    payoutMintHcuBlockMeter: payoutMintHcu.hcuBlockMeter,
    payoutMintHcuTrustedAppRecord: payoutMintHcu.hcuTrustedAppRecord,
  });
  return withDenyRecords(instruction, parameters.host.denyListEnabled, [joinMint, payoutMint]);
}
