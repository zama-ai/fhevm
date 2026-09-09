import { getAddressEncoder, getProgramDerivedAddress, type Address } from '@solana/kit';

import {
  CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
  ZAMA_HOST_PROGRAM_ADDRESS,
} from './generated/confidentialToken/programAddress.js';
import { findTotalSupplyAuthorityPda } from './generated/confidentialToken/pdas/totalSupplyAuthority.js';
// The `__event_authority` seed and the canonical token-value derivation are owned by batcherPdas;
// import them rather than re-declaring the seed / re-implementing the derivation here.
import { EVENT_AUTHORITY_SEED } from './batcherPdas.js';

// Slot key shared with confidential_token::state.
export const BALANCE_KEY = new TextEncoder().encode('balance_________________________');

const SPL_TOKEN_PROGRAM_ADDRESS = 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA' as Address;
const ASSOCIATED_TOKEN_PROGRAM_ADDRESS = 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL' as Address;

const addressEncoder = getAddressEncoder();
const encodeAddress = (value: Address): Uint8Array => new Uint8Array(addressEncoder.encode(value));

const pda = async (programAddress: Address, seeds: Uint8Array[]): Promise<Address> =>
  (await getProgramDerivedAddress({ programAddress, seeds }))[0];

export { tokenStateAddress } from './batcherPdas.js';

/** The mint's total-supply authority PDA under the compiled confidential-token program. */
export const totalSupplyAuthorityAddress = async (mint: Address): Promise<Address> =>
  (await findTotalSupplyAuthorityPda({ mint }))[0];

/** The confidential-token program's own Anchor event-authority PDA (the instruction `eventAuthority`). */
export const tokenEventAuthorityAddress = (): Promise<Address> =>
  pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [EVENT_AUTHORITY_SEED]);

/** The zama-host program's Anchor event-authority PDA (the instruction `zamaEventAuthority`). */
export const zamaEventAuthorityAddress = (): Promise<Address> => pda(ZAMA_HOST_PROGRAM_ADDRESS, [EVENT_AUTHORITY_SEED]);

export const TOKEN_PROGRAM_ADDRESS = SPL_TOKEN_PROGRAM_ADDRESS;

/**
 * Associated token account for `owner` and SPL `mint` under `tokenProgram`
 * (`get_associated_token_address_with_program_id`).
 */
export const associatedTokenAddress = (owner: Address, mint: Address, tokenProgram: Address): Promise<Address> =>
  pda(ASSOCIATED_TOKEN_PROGRAM_ADDRESS, [encodeAddress(owner), encodeAddress(tokenProgram), encodeAddress(mint)]);
