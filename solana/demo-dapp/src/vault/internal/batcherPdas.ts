import { getProgramDerivedAddress, getU64Encoder, type Address } from '@solana/kit';
import { base58 } from '@scure/base';
import { findJoinRecordPda } from './generated/confidentialBatcher/pdas/joinRecord.js';

import { solanaEncryptedStateAddress } from '@sdk-src/solana/encryptedState.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from './generated/confidentialToken/programAddress.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@sdk-src/solana/internal/generated/zamaHost/programAddress.js';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './generated/confidentialBatcher/programAddress.js';

const encoder = new TextEncoder();
const BATCH_SEED = encoder.encode('batch');
const TOKEN_ACCOUNT_SEED = encoder.encode('token-account');
const PENDING_BURN_SEED = encoder.encode('pending-burn');
/** Fixed confidential-token label for the all-or-zero burned amount (`burned_amount_key`). */
/**
 * Anchor event-CPI authority seed (`__event_authority`). Both the zama-host and confidential-token
 * programs derive their event authority from this seed, so the vault builders that emit through
 * those programs (join, settle) share this one constant instead of re-encoding the literal.
 */
export const EVENT_AUTHORITY_SEED = encoder.encode('__event_authority');

async function pda(programAddress: Address, seeds: Uint8Array[]): Promise<Address> {
  return (await getProgramDerivedAddress({ programAddress, seeds }))[0];
}

function addressBytes(value: Address): Uint8Array {
  return base58.decode(value);
}

/**
 * The canonical `EncryptedState` PDA of a confidential-token value: the token program's value,
 * scoped to its mint, controlled by `authority` (a token account, or a mint's total-supply
 * authority), under one of the program's fixed labels (`token_slot` in the token program).
 */
export function tokenStateAddress(mint: Address, authority: Address): Promise<Address> {
  return solanaEncryptedStateAddress(addressBytes(ZAMA_HOST_PROGRAM_ADDRESS), {
    program: addressBytes(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS),
    authority: addressBytes(authority),
    scope: addressBytes(mint),
  });
}

/** The batch PDA for a batcher config and zero-based index (`batch_address`). */
export async function batchAddress(batcher: Address, index: bigint): Promise<Address> {
  return pda(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, [
    BATCH_SEED,
    addressBytes(batcher),
    new Uint8Array(getU64Encoder().encode(index)),
  ]);
}

/** The canonical confidential token account for one owner and mint (`token_account_address`). */
export async function tokenAccountAddress(mint: Address, owner: Address): Promise<Address> {
  return pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [TOKEN_ACCOUNT_SEED, addressBytes(mint), addressBytes(owner)]);
}

/** The single PendingBurn for a confidential token account (`pending_burn_address`). */
export async function pendingBurnAddress(mint: Address, tokenAccount: Address): Promise<Address> {
  return pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [PENDING_BURN_SEED, addressBytes(mint), addressBytes(tokenAccount)]);
}

export async function joinStateAddress(batch: Address, user: Address): Promise<Address> {
  const [record] = await findJoinRecordPda({ batch, user });
  return solanaEncryptedStateAddress(addressBytes(ZAMA_HOST_PROGRAM_ADDRESS), {
    program: addressBytes(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS),
    authority: addressBytes(record),
    scope: addressBytes(batch),
  });
}

export async function scratchAddress(state: Address): Promise<Address> {
  return pda(ZAMA_HOST_PROGRAM_ADDRESS, [encoder.encode('transient'), addressBytes(state)]);
}

export {
  findBatchAuthorityPda,
  findJoinRecordPda,
  findBatchJoinUnderlyingPda,
  findBatchPayoutUnderlyingPda,
} from './generated/confidentialBatcher/pdas/index.js';
