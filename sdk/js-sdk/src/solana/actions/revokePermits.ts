import type { SolanaRpc } from '../encryptedStore.js';
import {
  createNoopSigner,
  fetchEncodedAccount,
  type Address,
  type Instruction,
  type MaybeEncodedAccount,
  type ProgramDerivedAddress,
} from '@solana/kit';

import {
  findInvalidationPda,
  getPermitInvalidationDecoder,
  getPermitInvalidationSize,
  getRevokePermitsInstruction,
  PERMIT_INVALIDATION_DISCRIMINATOR,
} from '@fhevm/solana-zama-host';

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
    params.invalidation ??
    (await findInvalidationPda({ user: params.user }, { programAddress: params.programAddress }))[0];
  return getRevokePermitsInstruction(
    {
      user: createNoopSigner(params.user),
      invalidation,
    },
    { programAddress: params.programAddress },
  );
}

/**
 * Reads the canonical watermark at `finalized`. Missing accounts have never invalidated a permit.
 */
export async function fetchSolanaPermitInvalidation(
  rpc: SolanaRpc,
  user: Address,
  config: { readonly programAddress: Address },
): Promise<bigint> {
  const { programAddress } = config;
  const pda = await findInvalidationPda({ user }, { programAddress });
  return solanaPermitInvalidationWatermark(
    await fetchEncodedAccount(rpc, pda[0], { commitment: 'finalized' }),
    pda,
    user,
    programAddress,
  );
}

/**
 * The watermark an account read at `user`'s invalidation address holds.
 *
 * @throws If the account is not the host's invalidation record of `user` at that address.
 */
export function solanaPermitInvalidationWatermark(
  account: MaybeEncodedAccount,
  [address, bump]: ProgramDerivedAddress,
  user: Address,
  programAddress: Address,
): bigint {
  if (!account.exists) return 0n;
  const data = account.data;
  // Anyone can fund a PDA before initialization; the host treats that account as watermark zero.
  if (!account.executable && account.programAddress === '11111111111111111111111111111111' && data.length === 0)
    return 0n;
  if (
    account.programAddress !== programAddress ||
    account.executable ||
    data.length !== getPermitInvalidationSize() ||
    !PERMIT_INVALIDATION_DISCRIMINATOR.every((byte, index) => data[index] === byte)
  ) {
    throw new Error(`Invalid permit invalidation account ${address}`);
  }
  const record = getPermitInvalidationDecoder().decode(data);
  if (record.user !== user || record.bump !== bump) {
    throw new Error(`Invalid permit invalidation account ${address}`);
  }
  return record.invalidationWatermark;
}
