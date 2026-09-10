import { AccountRole, type Address, type Instruction } from '@solana/kit';
import { getCloseScratchInstruction } from '@sdk-src/solana/internal/generated/zamaHost/instructions/closeScratch.js';

export const INSTRUCTIONS_SYSVAR = 'Sysvar1nstructions1111111111111111111111111' as Address;

/** Must be the final instruction; the host refunds only the payer recorded at open. */
export function closeScratchInstruction(scratch: Address, refund: Address): Instruction {
  const close = getCloseScratchInstruction({ instructions: INSTRUCTIONS_SYSVAR });
  return {
    ...close,
    accounts: [...close.accounts, { address: scratch, role: AccountRole.WRITABLE }, { address: refund, role: AccountRole.WRITABLE }],
  };
}
