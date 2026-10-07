import type { Address } from '@solana/kit';
import { findAssociatedTokenPda } from '@solana-program/token';
import { ZAMA_HOST_PROGRAM_ADDRESS, findTotalSupplyAuthorityPda, findEventAuthorityPda } from '@fhevm/confidential-token';

// Slot key shared with confidential_token::state.
export { BALANCE_KEY } from '@fhevm/confidential-token';

const SPL_TOKEN_PROGRAM_ADDRESS = 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA' as Address;
export { tokenStateAddress } from './batcherPdas.js';

/** The mint's total-supply authority PDA under the compiled confidential-token program. */
export const totalSupplyAuthorityAddress = async (mint: Address): Promise<Address> =>
  (await findTotalSupplyAuthorityPda({ mint }))[0];

/** The confidential-token program's own Anchor event-authority PDA (the instruction `eventAuthority`). */
export const tokenEventAuthorityAddress = (): Promise<Address> =>
  findEventAuthorityPda().then(([address]) => address);

/** The zama-host program's Anchor event-authority PDA (the instruction `zamaEventAuthority`). */
export const zamaEventAuthorityAddress = (): Promise<Address> => findEventAuthorityPda({ programAddress: ZAMA_HOST_PROGRAM_ADDRESS }).then(([address]) => address);

export const TOKEN_PROGRAM_ADDRESS = SPL_TOKEN_PROGRAM_ADDRESS;

/**
 * Associated token account for `owner` and SPL `mint` under `tokenProgram`
 * (`get_associated_token_address_with_program_id`).
 */
export const associatedTokenAddress = async (owner: Address, mint: Address, tokenProgram: Address): Promise<Address> =>
  (await findAssociatedTokenPda({ owner, tokenProgram, mint }))[0];
