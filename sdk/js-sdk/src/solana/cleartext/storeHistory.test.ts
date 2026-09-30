import { describe, expect, it } from 'vitest';
import { address, getBase58Decoder, type Address } from '@solana/kit';
import type { SolanaRpc } from '../encryptedStore.js';
import { getMakeStoreHandlePublicInstructionDataEncoder } from '../internal/generated/zamaHost/instructions/makeStoreHandlePublic.js';
import { fetchSolanaStoreHistory } from './storeHistory.js';

const host = address('11111111111111111111111111111112');
const store = address('SysvarC1ock11111111111111111111111111111111');
const other = address('SysvarRent111111111111111111111111111111111');

/** An RPC whose history for the store is one `make_store_handle_public` per leaf count given. */
function rpcWithPublicWrites(previousLeafCounts: readonly bigint[]): SolanaRpc {
  const transaction = (previousLeafCount: bigint) => ({
    transaction: {
      message: {
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
  return {
    getSignaturesForAddress: (_: Address, { before }: { before?: string }) => ({
      send: () =>
        Promise.resolve(
          before === undefined ? previousLeafCounts.map((_, index) => ({ signature: `sig${index}`, err: null })) : [],
        ),
    }),
    getTransaction: (signature: string) => ({
      send: () => Promise.resolve(transaction(previousLeafCounts[Number(signature.slice(3))] ?? 0n)),
    }),
  } as unknown as SolanaRpc;
}

describe('fetchSolanaStoreHistory', () => {
  it('places writes at the leaf count they carry, whatever order the RPC lists them in', async () => {
    const history = await fetchSolanaStoreHistory(rpcWithPublicWrites([1n, 0n]), store, host);
    expect(history.map((event) => event.handle[0])).toEqual([2, 3]);
  });

  it('refuses a history with a missing or a doubly claimed leaf', async () => {
    await expect(fetchSolanaStoreHistory(rpcWithPublicWrites([0n, 2n]), store, host)).rejects.toThrow(/records leaf 1/);
    await expect(fetchSolanaStoreHistory(rpcWithPublicWrites([0n, 0n]), store, host)).rejects.toThrow(/claim leaf 0/);
  });
});
