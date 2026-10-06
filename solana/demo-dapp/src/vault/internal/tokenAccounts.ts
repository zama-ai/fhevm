import { getAddressEncoder, getProgramDerivedAddress, type Address } from '@solana/kit';
import { ZAMA_HOST_PROGRAM_ADDRESS, findTotalSupplyAuthorityPda, findEventAuthorityPda } from '@fhevm/confidential-token';

// Slot key shared with confidential_token::state.
export { BALANCE_KEY } from '@fhevm/confidential-token';

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
  findEventAuthorityPda().then(([address]) => address);

/** The zama-host program's Anchor event-authority PDA (the instruction `zamaEventAuthority`). */
export const zamaEventAuthorityAddress = (): Promise<Address> => findEventAuthorityPda({ programAddress: ZAMA_HOST_PROGRAM_ADDRESS }).then(([address]) => address);

export const TOKEN_PROGRAM_ADDRESS = SPL_TOKEN_PROGRAM_ADDRESS;

/**
 * Associated token account for `owner` and SPL `mint` under `tokenProgram`
 * (`get_associated_token_address_with_program_id`).
 */
export const associatedTokenAddress = (owner: Address, mint: Address, tokenProgram: Address): Promise<Address> =>
  pda(ASSOCIATED_TOKEN_PROGRAM_ADDRESS, [encodeAddress(owner), encodeAddress(tokenProgram), encodeAddress(mint)]);
