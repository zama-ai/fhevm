import { getProgramDerivedAddress, getU64Encoder, type Address } from '@solana/kit';
import { base58 } from '@scure/base';
import { sha256 } from '@noble/hashes/sha2.js';

import { solanaEncryptedValueAccountAddress } from '@sdk-src/solana/encryptedValueAccount.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from './generated/confidentialToken/programAddress.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@sdk-src/solana/internal/generated/zamaHost/programAddress.js';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './generated/confidentialBatcher/programAddress.js';

const encoder = new TextEncoder();
const BATCH_SEED = encoder.encode('batch');
const TOKEN_ACCOUNT_SEED = encoder.encode('token-account');
const PENDING_BURN_SEED = encoder.encode('pending-burn');
/** Fixed confidential-token label for the all-or-zero burned amount (`encrypted_burned_amount_label`). */
const ENCRYPTED_BURNED_AMOUNT_LABEL = encoder.encode('burned_amount___________________');
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
 * The canonical `EncryptedValue` PDA of a confidential-token value: the token program's value,
 * scoped to its mint, controlled by `authority` (a token account, or a mint's total-supply
 * authority), under one of the program's fixed labels (`token_value_id` in the token program).
 */
export function tokenValueAddress(mint: Address, authority: Address, label: Uint8Array): Promise<Address> {
  return solanaEncryptedValueAccountAddress(addressBytes(ZAMA_HOST_PROGRAM_ADDRESS), {
    program: addressBytes(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS),
    encryptedValueAccountAuthority: addressBytes(authority),
    scope: addressBytes(mint),
    label,
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

/**
 * The batch's burned-amount value on the join mint: the token program's `burned_amount` value of
 * the batch's join token account. It is `batchBurnedAmountValue` in the settle account set and the
 * account a settle certificate is requested for.
 */
export function burnedAmountValueAddress(joinMint: Address, batchJoinTokenAccount: Address): Promise<Address> {
  return tokenValueAddress(joinMint, batchJoinTokenAccount, ENCRYPTED_BURNED_AMOUNT_LABEL);
}

function concatBytes(...parts: Uint8Array[]): Uint8Array {
  const out = new Uint8Array(parts.reduce((n, p) => n + p.length, 0));
  let offset = 0;
  for (const part of parts) {
    out.set(part, offset);
    offset += part.length;
  }
  return out;
}

/**
 * A batcher-owned per-user value (`pending_join_value` or `claim_amount_value`). Batcher values
 * belong to the batcher program scoped to the batch (`batch_app`), are controlled by the batch
 * authority, and are labeled `sha256(purpose_prefix || user)` (`batcher_encrypted_value_id`).
 */
function batcherValueAddress(batch: Address, batchAuthority: Address, purposePrefix: string, user: Address): Promise<Address> {
  return solanaEncryptedValueAccountAddress(addressBytes(ZAMA_HOST_PROGRAM_ADDRESS), {
    program: addressBytes(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS),
    encryptedValueAccountAuthority: addressBytes(batchAuthority),
    scope: addressBytes(batch),
    label: sha256(concatBytes(encoder.encode(purposePrefix), addressBytes(user))),
  });
}

/** The user's pending joined-amount value for a batch (`encrypted_pending_join_label`). */
export function pendingJoinValueAddress(batch: Address, batchAuthority: Address, user: Address): Promise<Address> {
  return batcherValueAddress(batch, batchAuthority, 'batcher-pending-join', user);
}

/** The user's claimed-payout value for a batch (`encrypted_claim_amount_label`). */
export function claimAmountValueAddress(batch: Address, batchAuthority: Address, user: Address): Promise<Address> {
  return batcherValueAddress(batch, batchAuthority, 'batcher-claim-amount', user);
}

export {
  findBatchAuthorityPda,
  findJoinRecordPda,
  findBatchJoinUnderlyingPda,
  findBatchPayoutUnderlyingPda,
} from './generated/confidentialBatcher/pdas/index.js';
