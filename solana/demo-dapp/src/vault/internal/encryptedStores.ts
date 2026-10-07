import type { Address } from '@solana/kit';
import { findEncryptedStorePda } from '@fhevm/solana-zama-host';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './generated/confidentialBatcher/programAddress.js';
import { findJoinRecordPda } from './generated/confidentialBatcher/pdas/joinRecord.js';

export async function joinStoreAddress(batch: Address, user: Address): Promise<Address> {
  const [record] = await findJoinRecordPda({ batch, user });
  const [store] = await findEncryptedStorePda({
    program: CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS,
    authority: record,
    scope: batch,
  });
  return store;
}

/** A confidential-token encrypted store: owned by the token program, scoped to its mint, controlled by `authority` (a token account or the mint's total-supply authority). */
export async function tokenStoreAddress(mint: Address, authority: Address): Promise<Address> {
  const [store] = await findEncryptedStorePda({
    program: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    authority,
    scope: mint,
  });
  return store;
}
