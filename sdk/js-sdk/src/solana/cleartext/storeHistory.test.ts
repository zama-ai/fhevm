import { describe, expect, it } from 'vitest';
import { address, getBase58Decoder, type Address } from '@solana/kit';
import type { SolanaRpc } from '../encryptedStore.js';
import { getMakeStoreHandlePublicInstructionDataEncoder } from '../internal/generated/zamaHost/instructions/makeStoreHandlePublic.js';
import { createSolanaStoreHistoryReader } from './storeHistory.js';

const host = address('11111111111111111111111111111112');
const store = address('SysvarC1ock11111111111111111111111111111111');
const other = address('SysvarRent111111111111111111111111111111111');

/**
 * A validator whose only writes to the store are `make_store_handle_public`s, one per leaf count in
 * `writes`, oldest first. `reset` replaces the ledger, as restarting a validator does.
 */
function ledger(initial: readonly bigint[]) {
  let epoch = 0;
  let writes = [...initial];
  const hidden = new Set<number>();
  const fetched: string[] = [];
  const signatureOf = (index: number) => `e${epoch}sig${index}`;
  const indexOf = (signature: string): number | undefined => {
    const match = /^e(\d+)sig(\d+)$/.exec(signature);
    return match && Number(match[1]) === epoch && Number(match[2]) < writes.length ? Number(match[2]) : undefined;
  };
  const transaction = (previousLeafCount: bigint) => ({
    transaction: {
      message: {
        header: { numRequiredSignatures: 1, numReadonlySignedAccounts: 0, numReadonlyUnsignedAccounts: 1 },
        accountKeys: [host, other, other, store],
        instructions: [
          {
            programIdIndex: 0,
            // payer, authority, encryptedStore, hostConfig, denyScopeRecord, systemProgram
            accounts: [1, 2, 3, 1, 1, 1],
            data: getBase58Decoder().decode(
              getMakeStoreHandlePublicInstructionDataEncoder().encode({
                key: new Uint8Array(32).fill(1),
                handle: new Uint8Array(32).fill(Number(previousLeafCount) + 2),
                previousLeafCount,
              }),
            ),
          },
        ],
      },
    },
    meta: {
      err: null,
      innerInstructions: [],
      loadedAddresses: { writable: [] as Address[], readonly: [] as Address[] },
    },
  });
  const rpc = {
    // Newest first, in one page.
    getSignaturesForAddress: (_: Address, { before, until }: { before?: string; until?: string }) => ({
      send: () => {
        const after = until === undefined ? -1 : (indexOf(until) ?? -1);
        const newer = writes.map((_, index) => index).filter((index) => index > after && !hidden.has(index));
        return Promise.resolve(
          before === undefined ? newer.reverse().map((index) => ({ signature: signatureOf(index), err: null })) : [],
        );
      },
    }),
    getSignatureStatuses: ([signature]: [string]) => ({
      send: () => Promise.resolve({ value: [indexOf(signature) === undefined ? null : {}] }),
    }),
    getTransaction: (signature: string) => ({
      send: () => {
        fetched.push(signature);
        const index = indexOf(signature);
        return Promise.resolve(index === undefined ? null : transaction(writes[index] ?? 0n));
      },
    }),
  } as unknown as SolanaRpc;
  return {
    read: createSolanaStoreHistoryReader(rpc, host),
    fetched,
    append: (previousLeafCount: bigint) => writes.push(previousLeafCount),
    /** Leaves write `index` out of the signature listing, as an RPC that has not indexed it yet. */
    hide: (index: number) => hidden.add(index),
    reveal: (index: number) => hidden.delete(index),
    reset: (next: readonly bigint[]) => {
      epoch += 1;
      writes = [...next];
    },
  };
}

const handles = (history: readonly { readonly handle: Uint8Array }[]) => history.map((event) => event.handle[0]);

describe('createSolanaStoreHistoryReader', () => {
  it('places writes at the leaf count they carry, whatever order the RPC lists them in', async () => {
    expect(handles(await ledger([1n, 0n]).read(store))).toEqual([2, 3]);
  });

  it('refuses a history with a missing or a doubly claimed leaf', async () => {
    await expect(ledger([0n, 2n]).read(store)).rejects.toThrow(/records leaf 1/);
    await expect(ledger([0n, 0n]).read(store)).rejects.toThrow(/claim leaf 0/);
  });

  it('reads only the transactions newer than the last one it saw', async () => {
    const chain = ledger([0n, 1n]);
    await chain.read(store);
    chain.append(2n);
    expect(handles(await chain.read(store))).toEqual([2, 3, 4]);
    expect(chain.fetched).toEqual(['e0sig1', 'e0sig0', 'e0sig2']);
  });

  it('reads the store from the start again after a validator reset', async () => {
    const chain = ledger([0n, 1n]);
    await chain.read(store);
    chain.reset([0n]);
    expect(handles(await chain.read(store))).toEqual([2]);
  });

  it('keeps no history with a hole, so a later read looks behind it again', async () => {
    const chain = ledger([0n, 1n]);
    chain.hide(0);
    await expect(chain.read(store)).rejects.toThrow(/records leaf 0/);
    chain.reveal(0);
    expect(handles(await chain.read(store))).toEqual([2, 3]);
  });
});
