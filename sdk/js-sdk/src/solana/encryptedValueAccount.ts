// Reading the host's EncryptedValue account: who owns the value, the live handle, the MMR leaf
// count, and the live peaks a proof is verified against.
//
// Hand-rolled on purpose. The struct is defined in the `zama-solana-acl` crate and used as an
// account with a manually-computed discriminator; it is not declared in any program's `#[program]`
// block, never appears in an Anchor IDL, and so cannot be generated. The layout therefore lives in
// two places by construction — the crate and this decoder — and everything below exists to keep
// that duplication honest rather than silent.
//
// Assumed layout (borsh, after the 8-byte discriminator), mirroring the crate's field order:
//   [program: 32][encrypted_value_account_authority: 32][scope: 32][label: 32][current_handle: 32]
//   [leaf_count: u64][peaks: Vec<[u8; 32]>][bump: u8]
//
// Two structural checks keep a drifted layout from decoding into shifted-but-plausible fields:
// trailing realloc capacity must come in whole 32-byte vector elements (the account grows and
// never shrinks, one peak per step), and the peak count must equal the leaf count's set bits —
// the MMR invariant, checked independently of the borsh walk.

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

/** The PDA seed prefix of an encrypted value account: `[seed, program, authority, scope, label]`. */
export const SOLANA_ENCRYPTED_VALUE_SEED = new TextEncoder().encode('encrypted-value');

/** The four fields that, with the host program, name one encrypted value account. */
export interface SolanaEncryptedValueSeeds {
  /** The 32-byte application program the value belongs to. */
  readonly program: Uint8Array;
  /** The 32-byte PDA of `program` that controls the value. */
  readonly encryptedValueAccountAuthority: Uint8Array;
  /** The 32-byte program-declared scope within `program`. */
  readonly scope: Uint8Array;
  /** Which of the authority's values this is. */
  readonly label: Uint8Array;
}

/**
 * The canonical address of an encrypted value account: the same
 * `find_program_address([seed, program, authority, scope, label], host_program)` the host program
 * and the Connector run. Matches `zama_solana_acl::encrypted_value_seeds`.
 *
 * @param hostProgramId - The 32-byte zama-host program id.
 * @param seeds - The four identity fields of the value.
 */
export async function solanaEncryptedValueAccountAddress(
  hostProgramId: Uint8Array,
  seeds: SolanaEncryptedValueSeeds,
): Promise<Address> {
  const [address] = await getProgramDerivedAddress({
    programAddress: getAddressDecoder().decode(hostProgramId),
    seeds: [SOLANA_ENCRYPTED_VALUE_SEED, seeds.program, seeds.encryptedValueAccountAuthority, seeds.scope, seeds.label],
  });
  return address;
}

/** The decoded account: identity fields as base58 addresses, value state as bytes. */
export interface SolanaEncryptedValueState {
  readonly program: Address;
  readonly encryptedValueAccountAuthority: Address;
  readonly scope: Uint8Array;
  readonly label: Uint8Array;
  readonly currentHandle: Uint8Array;
  readonly leafCount: bigint;
  readonly peaks: readonly Uint8Array[];
  readonly bump: number;
}

const encryptedValueBodyDecoder = getStructDecoder([
  ['program', fixDecoderSize(getBytesDecoder(), 32)],
  ['encryptedValueAccountAuthority', fixDecoderSize(getBytesDecoder(), 32)],
  ['scope', fixDecoderSize(getBytesDecoder(), 32)],
  ['label', fixDecoderSize(getBytesDecoder(), 32)],
  ['currentHandle', fixDecoderSize(getBytesDecoder(), 32)],
  ['leafCount', getU64Decoder()],
  ['peaks', getArrayDecoder(fixDecoderSize(getBytesDecoder(), 32), { size: getU32Decoder() })],
  ['bump', getU8Decoder()],
]);

const DISCRIMINATOR_SIZE = 8;
const VECTOR_ELEMENT_SIZE = 32;
/** `sha256("account:EncryptedValue")[..8]` — the crate's `encrypted_value_discriminator()`. */
const ENCRYPTED_VALUE_DISCRIMINATOR = new Uint8Array([0x9b, 0x03, 0x95, 0x3a, 0x84, 0x67, 0xc8, 0xa1]);

/**
 * Decodes an account's raw data, discriminator included, into its state.
 *
 * @param data - The account data exactly as the RPC returned it.
 * @param accountName - How to name the account in an error; the fetch wrapper passes its address.
 * @throws If the bytes do not decode as the assumed layout, or decode into an impossible MMR.
 */
export function decodeSolanaEncryptedValueState(data: Uint8Array, accountName: string): SolanaEncryptedValueState {
  for (let index = 0; index < ENCRYPTED_VALUE_DISCRIMINATOR.length; index += 1) {
    if (data[index] !== ENCRYPTED_VALUE_DISCRIMINATOR[index]) {
      throw new Error(`account ${accountName} does not carry the EncryptedValue discriminator`);
    }
  }
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
    program: addressDecoder.decode(decoded.program),
    encryptedValueAccountAuthority: addressDecoder.decode(decoded.encryptedValueAccountAuthority),
    scope: new Uint8Array(decoded.scope),
    label: new Uint8Array(decoded.label),
    currentHandle: new Uint8Array(decoded.currentHandle),
    leafCount: decoded.leafCount,
    peaks: decoded.peaks.map((peak) => new Uint8Array(peak)),
    bump: decoded.bump,
  };
}

/**
 * Reads one EncryptedValue account and decodes it.
 *
 * One read is one snapshot: the current handle, the leaf count and the peaks come out of the same
 * account data, which is what lets a proof built for that snapshot be verified against these peaks
 * without a second read racing the first.
 *
 * @param rpc - The Solana RPC to read through.
 * @param address - The account's address.
 * @param config - Standard fetch passthrough, e.g. `{ commitment: 'confirmed' }`.
 * @param expectedOwner - The host program expected to own the account. When given, an account
 * owned by anyone else — e.g. a system account somebody created by transferring lamports to the
 * PDA — is reported as such instead of failing deeper in the decoder as a phantom layout drift.
 * @throws If the account does not exist, is not owned by `expectedOwner`, or does not decode.
 */
export async function fetchSolanaEncryptedValueState(
  rpc: SolanaRpc,
  address: Address,
  config?: FetchAccountConfig,
  expectedOwner?: Address,
): Promise<SolanaEncryptedValueState> {
  const account = await fetchEncodedAccount(rpc, address, config);
  if (!account.exists) {
    throw new Error(`EncryptedValue account ${address} does not exist`);
  }
  if (expectedOwner !== undefined && account.programAddress !== expectedOwner) {
    throw new Error(
      `account ${address} is owned by ${account.programAddress}, not the zama-host program ` +
        `${expectedOwner} — no EncryptedValue account lives at this address`,
    );
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
