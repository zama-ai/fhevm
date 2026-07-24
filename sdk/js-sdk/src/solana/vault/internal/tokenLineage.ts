import { getAddressEncoder, getProgramDerivedAddress, type Address } from '@solana/kit';

import {
  CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
  ZAMA_HOST_PROGRAM_ADDRESS,
} from '../../internal/generated/confidentialToken/programAddress.js';
import { findComputeSignerPda } from '../../internal/generated/confidentialToken/pdas/computeSigner.js';
import { findTotalSupplyAuthorityPda } from '../../internal/generated/confidentialToken/pdas/totalSupplyAuthority.js';
// The `__event_authority` seed and the canonical `EncryptedValue` derivation are owned by
// batcherPdas (its `encryptedValueAddress(aclDomain, appAccount, label)`); import them rather than
// re-declaring the seed / re-implementing the derivation here.
import { EVENT_AUTHORITY_SEED, encryptedValueAddress } from './batcherPdas.js';

// Fixed 32-byte encrypted-value labels, byte-identical to `confidential_token::state`:
//   balance_label()            = b"balance_________________________"
//   transferred_amount_label() = b"transferred_amount______________"
//   total_supply_label()       = b"total_supply____________________"
const BALANCE_LABEL = new TextEncoder().encode('balance_________________________');
const TRANSFERRED_AMOUNT_LABEL = new TextEncoder().encode('transferred_amount______________');
const TOTAL_SUPPLY_LABEL = new TextEncoder().encode('total_supply____________________');

const SPL_TOKEN_PROGRAM_ADDRESS = 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA' as Address;
const ASSOCIATED_TOKEN_PROGRAM_ADDRESS = 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL' as Address;

const addressEncoder = getAddressEncoder();
const encodeAddress = (value: Address): Uint8Array => new Uint8Array(addressEncoder.encode(value));

const pda = async (programAddress: Address, seeds: Uint8Array[]): Promise<Address> =>
  (await getProgramDerivedAddress({ programAddress, seeds }))[0];

/** The confidential balance lineage account for `tokenAccount` under `mint` (label `balance`). */
export const balanceValueAddress = (mint: Address, tokenAccount: Address): Promise<Address> =>
  encryptedValueAddress(mint, tokenAccount, BALANCE_LABEL);

/** The transferred-amount lineage account for `tokenAccount` under `mint` (label `transferred_amount`). */
export const transferredAmountValueAddress = (mint: Address, tokenAccount: Address): Promise<Address> =>
  encryptedValueAddress(mint, tokenAccount, TRANSFERRED_AMOUNT_LABEL);

/** The encrypted total-supply lineage account for `mint` (app account = its total-supply authority). */
export const totalSupplyValueAddress = (mint: Address, totalSupplyAuthority: Address): Promise<Address> =>
  encryptedValueAddress(mint, totalSupplyAuthority, TOTAL_SUPPLY_LABEL);

/** The mint's total-supply authority PDA under the compiled confidential-token program. */
export const totalSupplyAuthorityAddress = async (mint: Address): Promise<Address> =>
  (await findTotalSupplyAuthorityPda({ mint }))[0];

/**
 * The mint's `fhe-compute` compute-signer PDA — the confidential-token contract identity that
 * input proofs bind to and that FHE evals over the mint's lineages run under.
 */
export const computeSignerAddress = async (mint: Address): Promise<Address> =>
  (await findComputeSignerPda({ mint }))[0];

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
  pda(ASSOCIATED_TOKEN_PROGRAM_ADDRESS, [
    encodeAddress(owner),
    encodeAddress(SPL_TOKEN_PROGRAM_ADDRESS),
    encodeAddress(mint),
  ]);
