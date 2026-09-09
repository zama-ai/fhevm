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

/** The PDA seed prefix of an encrypted state: `[seed, program, authority, scope]`. */
export const SOLANA_ENCRYPTED_STATE_SEED = new TextEncoder().encode('encrypted-state');

/** The three fields that, with the host program, name one encrypted state. */
export interface SolanaEncryptedStateSeeds {
  /** The 32-byte application program the value belongs to. */
  readonly program: Uint8Array;
  /** The 32-byte PDA of `program` that controls the value. */
  readonly authority: Uint8Array;
  /** The 32-byte program-declared scope within `program`. */
  readonly scope: Uint8Array;
}

/**
 * The canonical address of an encrypted state: the same
 * `find_program_address([seed, program, authority, scope], host_program)` the host program
 * and the Connector run. Matches `zama_solana_acl::encrypted_state_seeds`.
 *
 * @param hostProgramId - The 32-byte zama-host program id.
 * @param seeds - The three identity fields of the state.
 */
export async function solanaEncryptedStateAddress(
  hostProgramId: Uint8Array,
  seeds: SolanaEncryptedStateSeeds,
): Promise<Address> {
  const [address] = await getProgramDerivedAddress({
    programAddress: getAddressDecoder().decode(hostProgramId),
    seeds: [SOLANA_ENCRYPTED_STATE_SEED, seeds.program, seeds.authority, seeds.scope],
  });
  return address;
}

/** The decoded account: identity fields as base58 addresses, value state as bytes. */
export interface SolanaEncryptedState {
  readonly program: Address;
  readonly authority: Address;
  readonly scope: Uint8Array;
  readonly slots: ReadonlyArray<{ readonly key: Uint8Array; readonly handle: Uint8Array }>;
  readonly leafCount: bigint;
  readonly peaks: readonly Uint8Array[];
  readonly bump: number;
}

const encryptedStateBodyDecoder = getStructDecoder([
  ['program', fixDecoderSize(getBytesDecoder(), 32)],
  ['authority', fixDecoderSize(getBytesDecoder(), 32)],
  ['scope', fixDecoderSize(getBytesDecoder(), 32)],
  [
    'slots',
    getArrayDecoder(
      getStructDecoder([
        ['key', fixDecoderSize(getBytesDecoder(), 32)],
        ['handle', fixDecoderSize(getBytesDecoder(), 32)],
      ]),
      { size: getU32Decoder() },
    ),
  ],
  ['leafCount', getU64Decoder()],
  ['peaks', getArrayDecoder(fixDecoderSize(getBytesDecoder(), 32), { size: getU32Decoder() })],
  ['bump', getU8Decoder()],
]);

const DISCRIMINATOR_SIZE = 8;
const VECTOR_ELEMENT_SIZE = 32;
/** `sha256("account:EncryptedState")[..8]` — the crate's `encrypted_state_discriminator()`. */
const ENCRYPTED_STATE_DISCRIMINATOR = new Uint8Array([40, 50, 14, 85, 213, 243, 124, 173]);

/**
 * Decodes an account's raw data, discriminator included, into its state.
 *
 * @param data - The account data exactly as the RPC returned it.
 * @param accountName - How to name the account in an error; the fetch wrapper passes its address.
 * @throws If the bytes do not decode as the assumed layout, or decode into an impossible MMR.
 */
export function decodeSolanaEncryptedState(data: Uint8Array, accountName: string): SolanaEncryptedState {
  for (let index = 0; index < ENCRYPTED_STATE_DISCRIMINATOR.length; index += 1) {
    if (data[index] !== ENCRYPTED_STATE_DISCRIMINATOR[index]) {
      throw new Error(`account ${accountName} does not carry the EncryptedState discriminator`);
    }
  }
  const body = data.slice(DISCRIMINATOR_SIZE);
  const [decoded, offset] = encryptedStateBodyDecoder.read(body, 0);
  const trailingCapacity = body.length - offset;
  if (
    trailingCapacity < 0 ||
    trailingCapacity % VECTOR_ELEMENT_SIZE !== 0 ||
    decoded.peaks.length !== popcount(decoded.leafCount) ||
    decoded.slots.length > 32 ||
    new Set(decoded.slots.map((slot) => Array.from(slot.key).join(','))).size !== decoded.slots.length
  ) {
    throw new Error(
      `EncryptedState account ${accountName}: decoded ${decoded.peaks.length} MMR peaks for leaf count ` +
        `${decoded.leafCount} and consumed ${offset} of ${body.length} body bytes (after the ` +
        `${DISCRIMINATOR_SIZE}-byte discriminator) — the on-chain layout has drifted from this decoder. ` +
        `Re-check the crate's EncryptedState struct and update this module in lockstep.`,
    );
  }
  const addressDecoder = getAddressDecoder();
  return {
    program: addressDecoder.decode(decoded.program),
    authority: addressDecoder.decode(decoded.authority),
    scope: new Uint8Array(decoded.scope),
    slots: decoded.slots.map((slot) => ({ key: new Uint8Array(slot.key), handle: new Uint8Array(slot.handle) })),
    leafCount: decoded.leafCount,
    peaks: decoded.peaks.map((peak) => new Uint8Array(peak)),
    bump: decoded.bump,
  };
}

/**
 * Reads one EncryptedState account and decodes it.
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
export async function fetchSolanaEncryptedState(
  rpc: SolanaRpc,
  address: Address,
  config?: FetchAccountConfig,
  expectedOwner?: Address,
): Promise<SolanaEncryptedState> {
  const account = await fetchEncodedAccount(rpc, address, config);
  if (!account.exists) {
    throw new Error(`EncryptedState account ${address} does not exist`);
  }
  if (expectedOwner !== undefined && account.programAddress !== expectedOwner) {
    throw new Error(
      `account ${address} is owned by ${account.programAddress}, not the zama-host program ` +
        `${expectedOwner} — no EncryptedState account lives at this address`,
    );
  }
  return decodeSolanaEncryptedState(account.data, address);
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

/** Returns the handle currently bound to a named slot in this snapshot. */
export function encryptedStateHandle(state: SolanaEncryptedState, key: Uint8Array): Uint8Array {
  const slot = state.slots.find(
    (entry) => entry.key.length === key.length && entry.key.every((byte, index) => byte === key[index]),
  );
  if (!slot) throw new Error('encrypted state does not contain the requested slot');
  return slot.handle;
}
