import type { SolanaRpc } from '../encryptedStore.js';
import {
  createNoopSigner,
  fetchEncodedAccount,
  getAddressDecoder,
  type FetchAccountConfig,
  getAddressEncoder,
  getProgramDerivedAddress,
  type Address,
  type Instruction,
} from '@solana/kit';

import { getRevokePermitsInstruction } from '../internal/generated/zamaHost/instructions/revokePermits.js';

/** Seed of the per-user permit invalidation watermark PDA. */
export const SOLANA_PERMIT_INVALIDATION_SEED = new TextEncoder().encode('permit-invalidation');

/**
 * The canonical permit invalidation watermark address of a user. A missing account reads as
 * watermark zero: a user who has never revoked anything simply has no account.
 */
export async function solanaPermitInvalidationAddress(user: Address, programAddress: Address): Promise<Address> {
  const [derived] = await getProgramDerivedAddress({
    programAddress,
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
  /** The zama-host program id of the deployment. */
  readonly programAddress: Address;
}): Promise<Instruction> {
  const invalidation =
    params.invalidation ?? (await solanaPermitInvalidationAddress(params.user, params.programAddress));
  return getRevokePermitsInstruction(
    {
      user: createNoopSigner(params.user),
      invalidation,
    },
    { programAddress: params.programAddress },
  );
}

/** Reads the canonical watermark. Missing accounts have never invalidated a permit. */
export async function fetchSolanaPermitInvalidation(
  rpc: SolanaRpc,
  user: Address,
  config: FetchAccountConfig & { readonly programAddress: Address },
): Promise<bigint> {
  const { programAddress, ...fetchConfig } = config;
  const [address, bump] = await getProgramDerivedAddress({
    programAddress,
    seeds: [SOLANA_PERMIT_INVALIDATION_SEED, getAddressEncoder().encode(user)],
  });
  const account = await fetchEncodedAccount(rpc, address, fetchConfig);
  if (!account.exists) return 0n;
  // PermitInvalidation is an unchecked Anchor account and therefore absent from the IDL.
  // Its discriminator and 49-byte layout are pinned in state/permit_invalidation.rs.
  const data = account.data;
  // Anyone can fund a PDA before initialization; the host treats that account as watermark zero.
  if (!account.executable && account.programAddress === '11111111111111111111111111111111' && data.length === 0)
    return 0n;
  if (
    account.programAddress !== programAddress ||
    account.executable ||
    data.length !== 49 ||
    ![0xec, 0x8b, 0xdb, 0xa9, 0xb9, 0x22, 0xe9, 0x88].every((byte, index) => data[index] === byte) ||
    getAddressDecoder().decode(data.subarray(8, 40)) !== user ||
    data[48] !== bump
  ) {
    throw new Error(`Invalid permit invalidation account ${address}`);
  }
  return new DataView(data.buffer, data.byteOffset, data.byteLength).getBigUint64(40, true);
}
