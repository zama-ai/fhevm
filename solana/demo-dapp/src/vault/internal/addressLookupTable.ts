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
 * the confidential-vault demo needs exactly four instructions — create and extend to stand up the
 * per-batch settle table at `open_batch`, deactivate and close to reclaim its rent once the batch
 * has settled — so they are built by hand here rather than pulling in a whole dependency. Layout
 * matches `solana_sdk::address_lookup_table::instruction`: a 4-byte little-endian enum
 * discriminant, then the fields.
 */
export const ADDRESS_LOOKUP_TABLE_PROGRAM_ADDRESS =
  'AddressLookupTab1e1111111111111111111111111' as Address<'AddressLookupTab1e1111111111111111111111111'>;

const SYSTEM_PROGRAM_ADDRESS = '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;

const CREATE_LOOKUP_TABLE_DISCRIMINANT = 0;
const EXTEND_LOOKUP_TABLE_DISCRIMINANT = 2;
const DEACTIVATE_LOOKUP_TABLE_DISCRIMINANT = 3;
const CLOSE_LOOKUP_TABLE_DISCRIMINANT = 4;

/**
 * Most addresses one `ExtendLookupTable` may carry and still fit a transaction: 32 addresses are
 * ~1274 bytes of instruction data alone, over the 1232-byte v0 wire limit, and 20 keeps an extend
 * comfortably under the limit even when it shares a transaction with the table's create.
 */
export const MAX_EXTEND_ADDRESSES_PER_TRANSACTION = 20;

/**
 * Slots a deactivated table stays addressable before `CloseLookupTable` is allowed — the runtime's
 * `LOOKUP_TABLE_MAX_ADDRESSES`-independent deactivation cooldown (one full slot-hashes window), so
 * in-flight transactions referencing the table cannot be invalidated by an early close.
 */
export const LOOKUP_TABLE_DEACTIVATION_COOLDOWN_SLOTS = 513n;

/** `deactivation_slot` value marking a live (never deactivated) table. */
export const LOOKUP_TABLE_STILL_ACTIVE = 0xffff_ffff_ffff_ffffn;

/**
 * Reads `deactivation_slot` out of a lookup-table account's raw data (`LookupTableMeta` starts
 * after the 4-byte program-state discriminant). {@link LOOKUP_TABLE_STILL_ACTIVE} means the table
 * has not been deactivated.
 */
export function decodeLookupTableDeactivationSlot(accountData: Uint8Array): bigint {
  if (accountData.length < 12) throw new Error('lookup table account data is too short');
  return new DataView(accountData.buffer, accountData.byteOffset, accountData.byteLength).getBigUint64(4, true);
}

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

/**
 * Builds one `ExtendLookupTable { new_addresses }`, appending settle's derivable accounts to the
 * table. Refuses more than {@link MAX_EXTEND_ADDRESSES_PER_TRANSACTION} addresses — a bigger
 * extend cannot fit a transaction; use {@link getExtendLookupTableInstructions} to chunk.
 */
export function getExtendLookupTableInstruction(input: {
  readonly lookupTable: Address;
  readonly authority: TransactionSigner;
  readonly payer: TransactionSigner;
  readonly addresses: readonly Address[];
}): Instruction {
  if (input.addresses.length === 0) throw new Error('extend lookup table requires at least one address');
  if (input.addresses.length > MAX_EXTEND_ADDRESSES_PER_TRANSACTION) {
    throw new Error(
      `a single extend of ${input.addresses.length} addresses cannot fit the 1232-byte transaction wire limit; ` +
        `use getExtendLookupTableInstructions to chunk at ${MAX_EXTEND_ADDRESSES_PER_TRANSACTION}`,
    );
  }
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

/**
 * Builds the `ExtendLookupTable` instructions for an address set of any size, chunked at
 * {@link MAX_EXTEND_ADDRESSES_PER_TRANSACTION} so no returned instruction can overflow the
 * transaction wire limit. Send each instruction in its own transaction (the first may share one
 * with the table's create) and confirm it before the next, in order.
 */
export function getExtendLookupTableInstructions(input: {
  readonly lookupTable: Address;
  readonly authority: TransactionSigner;
  readonly payer: TransactionSigner;
  readonly addresses: readonly Address[];
}): Instruction[] {
  const instructions: Instruction[] = [];
  for (let index = 0; index < input.addresses.length; index += MAX_EXTEND_ADDRESSES_PER_TRANSACTION) {
    instructions.push(
      getExtendLookupTableInstruction({
        ...input,
        addresses: input.addresses.slice(index, index + MAX_EXTEND_ADDRESSES_PER_TRANSACTION),
      }),
    );
  }
  return instructions;
}

/**
 * Builds `DeactivateLookupTable`: the table stops being extendable and starts the
 * {@link LOOKUP_TABLE_DEACTIVATION_COOLDOWN_SLOTS} cooldown after which it can be closed.
 */
export function getDeactivateLookupTableInstruction(input: {
  readonly lookupTable: Address;
  readonly authority: TransactionSigner;
}): Instruction {
  const data = new Uint8Array(4);
  new DataView(data.buffer).setUint32(0, DEACTIVATE_LOOKUP_TABLE_DISCRIMINANT, true);
  return {
    programAddress: ADDRESS_LOOKUP_TABLE_PROGRAM_ADDRESS,
    accounts: [
      { address: input.lookupTable, role: AccountRole.WRITABLE },
      signerMeta(input.authority, AccountRole.READONLY_SIGNER),
    ],
    data,
  };
}

/**
 * Builds `CloseLookupTable`, refunding the table's rent to `recipient`. Only valid once the table
 * has been deactivated and the cooldown has elapsed; the runtime rejects an early close.
 */
export function getCloseLookupTableInstruction(input: {
  readonly lookupTable: Address;
  readonly authority: TransactionSigner;
  readonly recipient: Address;
}): Instruction {
  const data = new Uint8Array(4);
  new DataView(data.buffer).setUint32(0, CLOSE_LOOKUP_TABLE_DISCRIMINANT, true);
  return {
    programAddress: ADDRESS_LOOKUP_TABLE_PROGRAM_ADDRESS,
    accounts: [
      { address: input.lookupTable, role: AccountRole.WRITABLE },
      signerMeta(input.authority, AccountRole.READONLY_SIGNER),
      { address: input.recipient, role: AccountRole.WRITABLE },
    ],
    data,
  };
}
