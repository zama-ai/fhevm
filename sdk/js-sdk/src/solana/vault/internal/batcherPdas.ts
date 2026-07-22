import { getProgramDerivedAddress, getU64Encoder, type Address } from '@solana/kit';
import { base58 } from '@scure/base';
import { sha256 } from '@noble/hashes/sha2.js';

import { deriveValueKey } from '../../proof.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '../../internal/generated/confidentialToken/programAddress.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '../../internal/generated/zamaHost/programAddress.js';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './generated/confidentialBatcher/programAddress.js';

const encoder = new TextEncoder();
const BATCH_SEED = encoder.encode('batch');
const TOKEN_ACCOUNT_SEED = encoder.encode('token-account');
const BURN_REDEMPTION_SEED = encoder.encode('burn-redemption');
const ENCRYPTED_VALUE_SEED = encoder.encode('encrypted-value');
/** Fixed confidential-token label for the all-or-zero burned amount (`burned_amount_label`). */
const BURNED_AMOUNT_LABEL = encoder.encode('burned_amount___________________');

async function pda(programAddress: Address, seeds: Uint8Array[]): Promise<Address> {
  return (await getProgramDerivedAddress({ programAddress, seeds }))[0];
}

function addressBytes(value: Address): Uint8Array {
  return base58.decode(value);
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

/**
 * The per-handle redemption replay marker for a settle (`burn_redemption_address`). Seeded by the
 * burned handle, it only exists after dispatch — this is why it cannot ride in the settle ALT
 * frozen at open_batch and must stay a static account in the v0 message.
 */
export async function burnRedemptionAddress(mint: Address, burnedHandle: Uint8Array): Promise<Address> {
  if (burnedHandle.length !== 32) throw new Error(`burned handle must be 32 bytes, got ${burnedHandle.length}`);
  return pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [BURN_REDEMPTION_SEED, addressBytes(mint), burnedHandle]);
}

/** A confidential-value lineage: its value key (naming the lineage) and its canonical `EncryptedValue` PDA. */
export type SolanaValueLineage = {
  readonly aclValueKey: Uint8Array;
  readonly encryptedValueAddress: Address;
};

/**
 * The batch's burned-amount lineage on the join mint (acl domain = join mint, app account = the
 * batch's join token account, label = `burned_amount`). Its value key is the certificate's
 * `aclValueKey` and its PDA is both `batchBurnedAmountValue` and the proof-service `encrypted_value`.
 */
export async function burnedAmountLineage(
  joinMint: Address,
  batchJoinTokenAccount: Address,
): Promise<SolanaValueLineage> {
  const aclValueKey = deriveValueKey(addressBytes(joinMint), addressBytes(batchJoinTokenAccount), BURNED_AMOUNT_LABEL);
  return {
    aclValueKey,
    encryptedValueAddress: await pda(ZAMA_HOST_PROGRAM_ADDRESS, [ENCRYPTED_VALUE_SEED, aclValueKey]),
  };
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
 * A batcher-owned per-user lineage (`pending_join_value` or `claim_amount_value`). Batcher lineages
 * live in the batch's own ACL domain: acl domain = batch, app account = batch authority, label =
 * `sha256(purpose_prefix || user)` (`batcher_encrypted_value_address`). Use the returned
 * `aclValueKey` for a `decryptPosition` call and `encryptedValueAddress` as the account.
 */
async function batcherLineage(
  batch: Address,
  batchAuthority: Address,
  purposePrefix: string,
  user: Address,
): Promise<SolanaValueLineage> {
  const label = sha256(concatBytes(encoder.encode(purposePrefix), addressBytes(user)));
  const aclValueKey = deriveValueKey(addressBytes(batch), addressBytes(batchAuthority), label);
  return {
    aclValueKey,
    encryptedValueAddress: await pda(ZAMA_HOST_PROGRAM_ADDRESS, [ENCRYPTED_VALUE_SEED, aclValueKey]),
  };
}

/** The user's pending joined-amount lineage for a batch (`pending_join_label`). */
export async function pendingJoinLineage(
  batch: Address,
  batchAuthority: Address,
  user: Address,
): Promise<SolanaValueLineage> {
  return batcherLineage(batch, batchAuthority, 'batcher-pending-join', user);
}

/** The user's claimed-payout lineage for a batch (`claim_amount_label`). */
export async function claimAmountLineage(
  batch: Address,
  batchAuthority: Address,
  user: Address,
): Promise<SolanaValueLineage> {
  return batcherLineage(batch, batchAuthority, 'batcher-claim-amount', user);
}

export {
  findBatchAuthorityPda,
  findJoinRecordPda,
  findBatchJoinUnderlyingPda,
  findBatchPayoutUnderlyingPda,
} from './generated/confidentialBatcher/pdas/index.js';
