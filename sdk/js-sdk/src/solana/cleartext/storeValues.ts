// Reads the plaintext a cleartext host recorded for a handle, from the `EncryptedStore` the handle
// lives in, at the offsets `zama_host::cleartext::layout` renders into `hostConstants.ts`.
import { fetchEncodedAccount, getAddressDecoder, type Address } from '@solana/kit';
import type { SolanaRpc } from '../encryptedStore.js';
import { bytesToHex } from '../../core/base/bytes.js';
import { decodeSolanaEncryptedStore } from '../encryptedStore.js';
import {
  ENTRY_LEN,
  ENTRY_VALUE_OFFSET,
  HISTORY_RECORD_LEN,
  MAGIC,
  RECORDED,
  STORE_ACCOUNT_SIZE,
  STORE_HISTORY_COUNT_OFFSET,
  STORE_HISTORY_LEN,
  STORE_HISTORY_OFFSET,
  STORE_SECTION_OFFSET,
  STORE_SLOTS_OFFSET,
} from './hostConstants.js';

////////////////////////////////////////////////////////////////////////////////

const MAGIC_BYTES = new TextEncoder().encode(MAGIC);

////////////////////////////////////////////////////////////////////////////////

const sameBytes = (a: Uint8Array, b: Uint8Array): boolean =>
  a.length === b.length && a.every((byte, index) => byte === b[index]);

/** The value of the entry at `offset` as a 32-byte big-endian word, or undefined if none was recorded. */
function entryValue(data: Uint8Array, offset: number): Uint8Array | undefined {
  if (data[offset] !== RECORDED) return undefined;
  const word = new Uint8Array(32);
  word.set(data.subarray(offset + ENTRY_VALUE_OFFSET, offset + ENTRY_LEN), ENTRY_VALUE_OFFSET);
  return word;
}

/**
 * The plaintext of `handle` as a 32-byte big-endian word: the value of the slot that holds it, or
 * of one of the latest results written to the store. Throws when the store recorded none, which
 * includes every store a production host wrote.
 */
export function cleartextStoreValue(data: Uint8Array, accountName: string, handle: Uint8Array): Uint8Array {
  const store = decodeSolanaEncryptedStore(data, accountName);
  if (
    data.length < STORE_ACCOUNT_SIZE ||
    !sameBytes(data.subarray(STORE_SECTION_OFFSET, STORE_SLOTS_OFFSET), MAGIC_BYTES)
  ) {
    throw new Error(`EncryptedStore ${accountName} holds no plaintexts: a cleartext host did not write it`);
  }
  const slot = store.slots.findIndex((entry) => sameBytes(entry.handle, handle));
  const current = slot < 0 ? undefined : entryValue(data, STORE_SLOTS_OFFSET + slot * ENTRY_LEN);
  if (current !== undefined) return current;

  const view = new DataView(data.buffer, data.byteOffset, data.byteLength);
  const count = view.getBigUint64(STORE_HISTORY_COUNT_OFFSET, true);
  const kept = count < BigInt(STORE_HISTORY_LEN) ? count : BigInt(STORE_HISTORY_LEN);
  for (let age = 0n; age < kept; age += 1n) {
    const offset = STORE_HISTORY_OFFSET + Number((count - 1n - age) % BigInt(STORE_HISTORY_LEN)) * HISTORY_RECORD_LEN;
    if (sameBytes(data.subarray(offset, offset + 32), handle)) {
      const value = entryValue(data, offset + 32);
      if (value !== undefined) return value;
    }
  }
  throw new Error(`EncryptedStore ${accountName} recorded no plaintext for handle ${bytesToHex(handle)}`);
}

/** Fetches `encryptedStore` from `programAddress` and reads the plaintext of `handle` from it. */
export async function fetchCleartextStoreValue(
  rpc: SolanaRpc,
  programAddress: Address,
  encryptedStore: Uint8Array,
  handle: Uint8Array,
): Promise<Uint8Array> {
  const address = getAddressDecoder().decode(encryptedStore);
  const account = await fetchEncodedAccount(rpc, address, { commitment: 'confirmed' });
  if (!account.exists || account.programAddress !== programAddress) {
    throw new Error(`No EncryptedStore of ${programAddress} at ${address}`);
  }
  return cleartextStoreValue(new Uint8Array(account.data), address, handle);
}
