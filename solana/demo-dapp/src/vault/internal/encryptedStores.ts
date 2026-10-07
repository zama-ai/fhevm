import type { Address } from '@solana/kit';
import { solanaEncryptedStoreAddress } from '@fhevm/sdk/solana';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './generated/confidentialBatcher/programAddress.js';
import { findJoinRecordPda } from './generated/confidentialBatcher/pdas/joinRecord.js';

export async function joinStoreAddress(batch: Address, user: Address): Promise<Address> {
  const [record] = await findJoinRecordPda({ batch, user });
  return solanaEncryptedStoreAddress(ZAMA_HOST_PROGRAM_ADDRESS, {
    program: CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS,
    authority: record,
    scope: batch,
  });
}

/** A confidential-token encrypted store: owned by the token program, scoped to its mint, controlled by `authority` (a token account or the mint's total-supply authority). */
export function tokenStoreAddress(mint: Address, authority: Address): Promise<Address> {
  return solanaEncryptedStoreAddress(ZAMA_HOST_PROGRAM_ADDRESS, {
    program: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    authority,
    scope: mint,
  });
}
