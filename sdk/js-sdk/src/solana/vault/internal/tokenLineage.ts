import { getAddressEncoder, getProgramDerivedAddress, type Address } from '@solana/kit';
import { base58 } from '@scure/base';

import { deriveValueKey } from '../../proof.js';
import {
  CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
  ZAMA_HOST_PROGRAM_ADDRESS,
} from '../../internal/generated/confidentialToken/programAddress.js';

// Anchor `emit_cpi!` event-authority PDA seed, shared by the confidential-token and zama-host
// programs (each derives its own under its own id).
const EVENT_AUTHORITY_SEED = new TextEncoder().encode('__event_authority');
// zama-host `EncryptedValue` account seed: the durable-lineage account keyed by a value key.
const ENCRYPTED_VALUE_SEED = new TextEncoder().encode('encrypted-value');

// Fixed 32-byte encrypted-value labels, byte-identical to `confidential_token::state`:
//   balance_label()      = b"balance_________________________"
//   total_supply_label() = b"total_supply____________________"
const BALANCE_LABEL = new TextEncoder().encode('balance_________________________');
const TOTAL_SUPPLY_LABEL = new TextEncoder().encode('total_supply____________________');

const SPL_TOKEN_PROGRAM_ADDRESS = 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA' as Address;
const ASSOCIATED_TOKEN_PROGRAM_ADDRESS = 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL' as Address;

const addressBytes = (value: Address): Uint8Array => base58.decode(value);

const addressEncoder = getAddressEncoder();
const encodeAddress = (value: Address): Uint8Array => new Uint8Array(addressEncoder.encode(value));

const pda = async (programAddress: Address, seeds: Uint8Array[]): Promise<Address> =>
  (await getProgramDerivedAddress({ programAddress, seeds }))[0];

/**
 * The zama-host `EncryptedValue` account for a `(mint, appAccount, label)` lineage — the TS mirror
 * of `confidential_token::state::encrypted_value_address`: derive the value key from the ACL domain
 * (mint), the app account, and the fixed label, then take the host program's `encrypted-value` PDA.
 */
const encryptedValueAddress = async (mint: Address, appAccount: Address, label: Uint8Array): Promise<Address> => {
  const valueKey = deriveValueKey(addressBytes(mint), addressBytes(appAccount), label);
  return pda(ZAMA_HOST_PROGRAM_ADDRESS, [ENCRYPTED_VALUE_SEED, valueKey]);
};

/** The confidential balance lineage account for `tokenAccount` under `mint` (label `balance`). */
export const balanceValueAddress = (mint: Address, tokenAccount: Address): Promise<Address> =>
  encryptedValueAddress(mint, tokenAccount, BALANCE_LABEL);

/** The encrypted total-supply lineage account for `mint` (app account = its total-supply authority). */
export const totalSupplyValueAddress = (mint: Address, totalSupplyAuthority: Address): Promise<Address> =>
  encryptedValueAddress(mint, totalSupplyAuthority, TOTAL_SUPPLY_LABEL);

/** The confidential-token program's own Anchor event-authority PDA (the instruction `eventAuthority`). */
export const tokenEventAuthorityAddress = (): Promise<Address> =>
  pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [EVENT_AUTHORITY_SEED]);

/** The zama-host program's Anchor event-authority PDA (the instruction `zamaEventAuthority`). */
export const zamaEventAuthorityAddress = (): Promise<Address> => pda(ZAMA_HOST_PROGRAM_ADDRESS, [EVENT_AUTHORITY_SEED]);

/**
 * The canonical associated token account for `owner` and SPL `mint` on the classic token program
 * (`get_associated_token_address_with_program_id`) — the same derivation `derive.ts` uses for the
 * batcher's underlying vaults.
 */
export const associatedTokenAddress = (owner: Address, mint: Address): Promise<Address> =>
  pda(ASSOCIATED_TOKEN_PROGRAM_ADDRESS, [encodeAddress(owner), encodeAddress(SPL_TOKEN_PROGRAM_ADDRESS), encodeAddress(mint)]);
