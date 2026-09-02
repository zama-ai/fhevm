import {
  createNoopSigner,
  getAddressEncoder,
  getProgramDerivedAddress,
  type Address,
  type Instruction,
} from '@solana/kit';

import { getRevokePermitsInstruction } from '../internal/generated/zamaHost/instructions/revokePermits.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '../internal/generated/zamaHost/programAddress.js';

/** Seed of the per-user permit invalidation watermark PDA. */
export const SOLANA_PERMIT_INVALIDATION_SEED = new TextEncoder().encode('permit-invalidation');

/**
 * The canonical permit invalidation watermark address of a user. A missing account reads as
 * watermark zero: a user who has never revoked anything simply has no account.
 */
export async function solanaPermitInvalidationAddress(user: Address): Promise<Address> {
  const [derived] = await getProgramDerivedAddress({
    programAddress: ZAMA_HOST_PROGRAM_ADDRESS,
    seeds: [SOLANA_PERMIT_INVALIDATION_SEED, getAddressEncoder().encode(user)],
  });
  return derived;
}

/**
 * Builds the `zama_host::revoke_permits` instruction: kills every outstanding permit whose
 * validity window opened at or before now, in one transaction of constant work. This is the
 * requester-side lever — a delegator revoking a *delegation* uses
 * `buildRevokeDelegationForUserDecryptionInstruction` instead; their own watermark is never read
 * for delegated requests.
 */
export async function buildRevokePermitsInstruction(params: {
  /** The user whose permits die. Signs the transaction and pays rent for the watermark. */
  readonly user: Address;
  /** The watermark address; defaults to the canonical PDA of the user when omitted. */
  readonly invalidation?: Address | undefined;
}): Promise<Instruction> {
  const invalidation = params.invalidation ?? (await solanaPermitInvalidationAddress(params.user));
  return getRevokePermitsInstruction({
    user: createNoopSigner(params.user),
    invalidation,
  });
}
