import type { Instruction } from '@solana/kit';

import {
  getCloseJoinRecordInstructionAsync,
  type CloseJoinRecordAsyncInput,
} from './internal/generated/confidentialBatcher/instructions/closeJoinRecord.js';

/**
 * Accounts for the batcher `close_join_record` instruction. `joinRecord` defaults to its PDA from
 * `batch` and the signing `user`, who receives the record's rent.
 */
export type SolanaVaultCloseJoinRecordParameters = CloseJoinRecordAsyncInput;

/**
 * Builds the user-signed `close_join_record` instruction: a record whose payout is claimed, or
 * whose batch was canceled, is closed and its rent returned. Records in a refunding batch stay
 * open because they still authorize `quit`.
 */
export async function buildCloseJoinRecordInstruction(
  parameters: SolanaVaultCloseJoinRecordParameters,
): Promise<Instruction> {
  return getCloseJoinRecordInstructionAsync(parameters);
}
