import { AccountRole, type Address, type Instruction } from '@solana/kit';
import { findDenyScopeRecordPda, type DenyScopeRecordSeeds } from '@fhevm/solana-zama-host';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './generated/confidentialBatcher/programAddress.js';

/** The application a mint's token executions run as. */
export const tokenApp = (mint: Address): DenyScopeRecordSeeds => ({
  appProgram: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
  scope: mint,
});

/** The application the batcher's own executions for `batch` run as. */
export const batchApp = (batch: Address): DenyScopeRecordSeeds => ({
  appProgram: CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS,
  scope: batch,
});

export type DenyListParameters = {
  /** The host's `grant_deny_list_enabled`: the instruction then carries its deny records. */
  readonly denyListEnabled?: boolean | undefined;
};

/**
 * Appends, while the host's deny list is on, the deny records a batcher instruction takes as its
 * remaining accounts: one per application each execution touches, in the order the instruction
 * documents (`confidential-batcher/src/lib.rs`).
 */
export async function withDenyRecords(
  instruction: Instruction,
  denyListEnabled: boolean | undefined,
  apps: readonly DenyScopeRecordSeeds[],
): Promise<Instruction> {
  if (denyListEnabled !== true) return instruction;
  const records = await Promise.all(apps.map((app) => findDenyScopeRecordPda(app)));
  return {
    ...instruction,
    accounts: [
      ...(instruction.accounts ?? []),
      ...records.map(([address]) => ({ address, role: AccountRole.READONLY })),
    ],
  };
}
