import { fetchEncodedAccount, type Address, type ReadonlyUint8Array, type Rpc, type SolanaRpcApi } from '@solana/kit';

import { ENCRYPTED_STORE_DISCRIMINATOR, getEncryptedStoreDecoder, type EncryptedStore } from '@fhevm/solana-zama-host';

/** The RPC shape this module reads through — `@solana/kit`'s standard API surface. */
export type SolanaRpc = Rpc<SolanaRpcApi>;

/** The decoded account, exactly as the generated `EncryptedStore` decoder reads it. */
export type SolanaEncryptedStore = Readonly<EncryptedStore>;

const VECTOR_ELEMENT_SIZE = 32;

/** Whether `data` starts with the `EncryptedStore` discriminator. */
export function isSolanaEncryptedStoreData(data: ReadonlyUint8Array): boolean {
  return ENCRYPTED_STORE_DISCRIMINATOR.every((byte, index) => data[index] === byte);
}

/**
 * Decodes an account's raw data, discriminator included, into its state.
 *
 * @param data - The account data exactly as the RPC returned it.
 * @param accountName - How to name the account in an error; the fetch wrapper passes its address.
 * @throws If the bytes do not decode as the generated layout, or decode into an impossible MMR.
 */
export function decodeSolanaEncryptedStore(data: ReadonlyUint8Array, accountName: string): SolanaEncryptedStore {
  if (!isSolanaEncryptedStoreData(data)) {
    throw new Error(`account ${accountName} does not carry the EncryptedStore discriminator`);
  }
  const [store, offset] = getEncryptedStoreDecoder().read(data, 0);
  // The account keeps spare capacity after the borsh body, in whole 32-byte vector elements.
  const trailingCapacity = data.length - offset;
  if (
    trailingCapacity < 0 ||
    trailingCapacity % VECTOR_ELEMENT_SIZE !== 0 ||
    store.peaks.length !== popcount(store.leafCount) ||
    store.slots.length > 32 ||
    new Set(store.slots.map((slot) => Array.from(slot.key).join(','))).size !== store.slots.length
  ) {
    throw new Error(
      `EncryptedStore account ${accountName}: decoded ${store.peaks.length} MMR peaks for leaf count ` +
        `${store.leafCount} and consumed ${offset} of ${data.length} bytes — the on-chain layout has ` +
        `drifted from the committed zama-host IDL. Regenerate the client with \`codegen:solana\`.`,
    );
  }
  return store;
}

/**
 * Reads one EncryptedStore account at `finalized` and decodes it.
 *
 * @param rpc - The Solana RPC to read through.
 * @param address - The account's address.
 * @param expectedOwner - The host program expected to own the account. When given, an account
 * owned by anyone else — e.g. a system account somebody created by transferring lamports to the
 * PDA — is reported as such instead of failing deeper in the decoder as a phantom layout drift.
 * @throws If the account does not exist, is not owned by `expectedOwner`, or does not decode.
 */
export async function fetchSolanaEncryptedStore(
  rpc: SolanaRpc,
  address: Address,
  expectedOwner?: Address,
): Promise<SolanaEncryptedStore> {
  const account = await fetchEncodedAccount(rpc, address, { commitment: 'finalized' });
  if (!account.exists) {
    throw new Error(`EncryptedStore account ${address} does not exist`);
  }
  if (expectedOwner !== undefined && account.programAddress !== expectedOwner) {
    throw new Error(
      `account ${address} is owned by ${account.programAddress}, not the zama-host program ` +
        `${expectedOwner} — no EncryptedStore account lives at this address`,
    );
  }
  return decodeSolanaEncryptedStore(account.data, address);
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

/**
 * Returns a copy of the handle currently bound to a named slot in this snapshot, as the
 * `Uint8Array` the decrypt entries take.
 */
export function encryptedStoreHandle(state: SolanaEncryptedStore, key: ReadonlyUint8Array): Uint8Array {
  const slot = state.slots.find(
    (entry) => entry.key.length === key.length && entry.key.every((byte, index) => byte === key[index]),
  );
  if (!slot) throw new Error('encrypted store does not contain the requested slot');
  return new Uint8Array(slot.handle);
}
