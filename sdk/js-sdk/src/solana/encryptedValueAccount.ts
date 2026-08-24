// Reading the host's EncryptedValue account: the live handle, the MMR leaf count, and the live
// peaks a proof is verified against.
//
// Hand-rolled on purpose. The struct is defined in the `zama-solana-acl` crate and used as an
// account with a manually-computed discriminator; it is not declared in any program's `#[program]`
// block, never appears in an Anchor IDL, and so cannot be generated. The layout therefore lives in
// two places by construction — the crate and this decoder — and everything below exists to keep
// that duplication honest rather than silent.
//
// Assumed layout (borsh, after the 8-byte discriminator), mirroring the crate's field order:
//   [domain: 32][encrypted_value_account_authority: 32][label: 32][current_handle: 32]
//   [subjects: Vec<Pubkey>][leaf_count: u64][peaks: Vec<[u8; 32]>][bump: u8]
//
// Two structural checks keep a drifted layout from decoding into shifted-but-plausible fields:
// trailing realloc capacity must come in whole 32-byte vector elements (the account grows and
// never shrinks, one element per step), and the peak count must equal the leaf count's set bits —
// the MMR invariant, checked independently of the borsh walk. Each `subjects` element is a bare
// 32-byte pubkey; if the on-chain entry ever grows past that, this decoder misaligns everything
// after it, and whoever changes that layout must update this file in lockstep.
//
// Ported from the demo-dapp's `vault/reads.ts`, which is expected to consume this module once the
// consumers move to the permit path.

import {
  fetchEncodedAccount,
  fixDecoderSize,
  getAddressDecoder,
  getArrayDecoder,
  getBytesDecoder,
  getProgramDerivedAddress,
  getStructDecoder,
  getU32Decoder,
  getU64Decoder,
  getU8Decoder,
  type Address,
  type FetchAccountConfig,
  type Rpc,
  type SolanaRpcApi,
} from '@solana/kit';

/** The RPC shape this module reads through — `@solana/kit`'s standard API surface. */
export type SolanaRpc = Rpc<SolanaRpcApi>;

/** The PDA seed prefix of an encrypted value account: `[seed, encrypted_value_id]`. */
export const SOLANA_ENCRYPTED_VALUE_SEED = new TextEncoder().encode('encrypted-value');

/**
 * The canonical address of the account an encrypted value id names.
 *
 * Two different 32-byte values meet here, and conflating them is the mistake this function exists
 * to prevent: the *id* is the identity requests carry on the wire and the Connector re-derives, the
 * *account address* is the PDA the state lives at and the historical-access leaves bind. This is
 * the same `find_program_address([seed, id], host_program)` the host and the Connector run.
 *
 * @param hostProgramId - The 32-byte zama-host program id.
 * @param encryptedValueId - The 32-byte encrypted value identity.
 */
export async function solanaEncryptedValueAccountAddress(
  hostProgramId: Uint8Array,
  encryptedValueId: Uint8Array,
): Promise<Address> {
  const [address] = await getProgramDerivedAddress({
    programAddress: getAddressDecoder().decode(hostProgramId),
    seeds: [SOLANA_ENCRYPTED_VALUE_SEED, encryptedValueId],
  });
  return address;
}

/** The decoded account: identity fields as base58 addresses, value state as bytes. */
export interface SolanaEncryptedValueState {
  readonly domain: Address;
  readonly encryptedValueAccountAuthority: Address;
  readonly label: Uint8Array;
  readonly currentHandle: Uint8Array;
  readonly subjects: readonly Address[];
  readonly leafCount: bigint;
  readonly peaks: readonly Uint8Array[];
}

const encryptedValueBodyDecoder = getStructDecoder([
  ['domain', fixDecoderSize(getBytesDecoder(), 32)],
  ['encryptedValueAccountAuthority', fixDecoderSize(getBytesDecoder(), 32)],
  ['label', fixDecoderSize(getBytesDecoder(), 32)],
  ['currentHandle', fixDecoderSize(getBytesDecoder(), 32)],
  ['subjects', getArrayDecoder(fixDecoderSize(getBytesDecoder(), 32), { size: getU32Decoder() })],
  ['leafCount', getU64Decoder()],
  ['peaks', getArrayDecoder(fixDecoderSize(getBytesDecoder(), 32), { size: getU32Decoder() })],
  ['bump', getU8Decoder()],
]);

const DISCRIMINATOR_SIZE = 8;
const VECTOR_ELEMENT_SIZE = 32;

/**
 * Decodes an account's raw data, discriminator included, into its state.
 *
 * @param data - The account data exactly as the RPC returned it.
 * @param accountName - How to name the account in an error; the fetch wrapper passes its address.
 * @throws If the bytes do not decode as the assumed layout, or decode into an impossible MMR.
 */
export function decodeSolanaEncryptedValueState(data: Uint8Array, accountName: string): SolanaEncryptedValueState {
  const body = data.slice(DISCRIMINATOR_SIZE);
  const [decoded, offset] = encryptedValueBodyDecoder.read(body, 0);
  const trailingCapacity = body.length - offset;
  if (
    trailingCapacity < 0 ||
    trailingCapacity % VECTOR_ELEMENT_SIZE !== 0 ||
    decoded.peaks.length !== popcount(decoded.leafCount)
  ) {
    throw new Error(
      `EncryptedValue account ${accountName}: decoded ${decoded.peaks.length} MMR peaks for leaf count ` +
        `${decoded.leafCount} and consumed ${offset} of ${body.length} body bytes (after the ` +
        `${DISCRIMINATOR_SIZE}-byte discriminator) — the on-chain layout has drifted from this decoder. ` +
        `Re-check the crate's EncryptedValue struct and update this module in lockstep.`,
    );
  }
  const addressDecoder = getAddressDecoder();
  return {
    domain: addressDecoder.decode(decoded.domain),
    encryptedValueAccountAuthority: addressDecoder.decode(decoded.encryptedValueAccountAuthority),
    label: new Uint8Array(decoded.label),
    currentHandle: new Uint8Array(decoded.currentHandle),
    subjects: decoded.subjects.map((subject) => addressDecoder.decode(subject)),
    leafCount: decoded.leafCount,
    peaks: decoded.peaks.map((peak) => new Uint8Array(peak)),
  };
}

/**
 * Reads one EncryptedValue account and decodes it.
 *
 * One read is one snapshot: the current handle, the leaf count and the peaks come out of the same
 * account data, which is what lets a proof fetched for that snapshot be verified against these
 * peaks without a second read racing the first.
 *
 * @param rpc - The Solana RPC to read through.
 * @param address - The account's address.
 * @param config - Standard fetch passthrough, e.g. `{ commitment: 'confirmed' }`.
 * @throws If the account does not exist, or its data does not decode.
 */
export async function fetchSolanaEncryptedValueState(
  rpc: SolanaRpc,
  address: Address,
  config?: FetchAccountConfig,
): Promise<SolanaEncryptedValueState> {
  const account = await fetchEncodedAccount(rpc, address, config);
  if (!account.exists) {
    throw new Error(`EncryptedValue account ${address} does not exist`);
  }
  return decodeSolanaEncryptedValueState(account.data, address);
}

/**
 * The number of set bits: how many mountains an MMR of this many leaves has.
 *
 * @param value - The leaf count.
 */
function popcount(value: bigint): number {
  let count = 0;
  for (let remaining = value; remaining > 0n; remaining >>= 1n) {
    count += Number(remaining & 1n);
  }
  return count;
}
