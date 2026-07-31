import {
  AccountRole,
  getProgramDerivedAddress,
  getU64Encoder,
  type AccountMeta,
  type Address,
  type Instruction,
  type TransactionSigner,
} from '@solana/kit';
import { base58 } from '@scure/base';

/**
 * Minimal client for the native Address Lookup Table program. Kit ships no ALT program client and
 * the confidential-vault demo needs exactly two instructions — create and extend — to stand up the
 * per-batch settle table at `open_batch`, so they are built by hand here rather than pulling in a
 * whole dependency. Layout matches `solana_sdk::address_lookup_table::instruction`: a 4-byte
 * little-endian enum discriminant, then the fields.
 */
export const ADDRESS_LOOKUP_TABLE_PROGRAM_ADDRESS =
  'AddressLookupTab1e1111111111111111111111111' as Address<'AddressLookupTab1e1111111111111111111111111'>;

const SYSTEM_PROGRAM_ADDRESS = '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;

const CREATE_LOOKUP_TABLE_DISCRIMINANT = 0;
const EXTEND_LOOKUP_TABLE_DISCRIMINANT = 2;

function addressBytes(value: Address): Uint8Array {
  return base58.decode(value);
}

function u64le(value: bigint): Uint8Array {
  return new Uint8Array(getU64Encoder().encode(value));
}

/**
 * A signer account meta: the `signer` field rides along at runtime so `signTransactionMessageWithSigners`
 * can produce the signature, while the meta stays typed as a plain `AccountMeta`.
 */
function signerMeta(signer: TransactionSigner, role: AccountRole): AccountMeta {
  return { address: signer.address, role, signer } as unknown as AccountMeta;
}

/**
 * Derives the lookup table PDA for an authority and the slot it is created in. The table address is
 * `PDA([authority, recent_slot_le], ALT program)`; the returned bump is the `bump_seed` the create
 * instruction commits to.
 */
export async function deriveAddressLookupTableAddress(
  authority: Address,
  recentSlot: bigint,
): Promise<{ readonly address: Address; readonly bump: number }> {
  const [address, bump] = await getProgramDerivedAddress({
    programAddress: ADDRESS_LOOKUP_TABLE_PROGRAM_ADDRESS,
    seeds: [addressBytes(authority), u64le(recentSlot)],
  });
  return { address, bump };
}

/**
 * Builds `CreateLookupTable { recent_slot, bump_seed }` and returns it alongside the derived table
 * address. `recentSlot` must be a recent, finalized slot; the table's addresses become usable from
 * the next slot, so a table created at `open_batch` is always usable by the later `settle`.
 */
export async function getCreateLookupTableInstruction(input: {
  readonly authority: TransactionSigner;
  readonly payer: TransactionSigner;
  readonly recentSlot: bigint;
}): Promise<{ readonly instruction: Instruction; readonly lookupTableAddress: Address }> {
  const { address, bump } = await deriveAddressLookupTableAddress(input.authority.address, input.recentSlot);
  const data = new Uint8Array(4 + 8 + 1);
  const view = new DataView(data.buffer);
  view.setUint32(0, CREATE_LOOKUP_TABLE_DISCRIMINANT, true);
  data.set(u64le(input.recentSlot), 4);
  data[12] = bump;
  return {
    lookupTableAddress: address,
    instruction: {
      programAddress: ADDRESS_LOOKUP_TABLE_PROGRAM_ADDRESS,
      accounts: [
        { address, role: AccountRole.WRITABLE },
        signerMeta(input.authority, AccountRole.READONLY_SIGNER),
        signerMeta(input.payer, AccountRole.WRITABLE_SIGNER),
        { address: SYSTEM_PROGRAM_ADDRESS, role: AccountRole.READONLY },
      ],
      data,
    },
  };
}

/** Builds `ExtendLookupTable { new_addresses }`, appending settle's derivable accounts to the table. */
export function getExtendLookupTableInstruction(input: {
  readonly lookupTable: Address;
  readonly authority: TransactionSigner;
  readonly payer: TransactionSigner;
  readonly addresses: readonly Address[];
}): Instruction {
  if (input.addresses.length === 0) throw new Error('extend lookup table requires at least one address');
  const data = new Uint8Array(4 + 8 + input.addresses.length * 32);
  const view = new DataView(data.buffer);
  view.setUint32(0, EXTEND_LOOKUP_TABLE_DISCRIMINANT, true);
  data.set(u64le(BigInt(input.addresses.length)), 4);
  input.addresses.forEach((address, i) => {
    data.set(addressBytes(address), 12 + i * 32);
  });
  return {
    programAddress: ADDRESS_LOOKUP_TABLE_PROGRAM_ADDRESS,
    accounts: [
      { address: input.lookupTable, role: AccountRole.WRITABLE },
      signerMeta(input.authority, AccountRole.READONLY_SIGNER),
      signerMeta(input.payer, AccountRole.WRITABLE_SIGNER),
      { address: SYSTEM_PROGRAM_ADDRESS, role: AccountRole.READONLY },
    ],
    data,
  };
}
