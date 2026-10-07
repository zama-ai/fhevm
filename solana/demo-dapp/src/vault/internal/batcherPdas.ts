import type { Address } from '@solana/kit';
import { findBatchPda } from './generated/confidentialBatcher/pdas/batch.js';
import { findJoinRecordPda } from './generated/confidentialBatcher/pdas/joinRecord.js';

import { solanaEncryptedStoreAddress } from '@fhevm/sdk/solana';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './generated/confidentialBatcher/programAddress.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, findPendingBurnPda, findTokenAccountPda } from '@fhevm/confidential-token';

/**
 * The canonical `EncryptedStore` PDA of a confidential-token value: the token program's value,
 * scoped to its mint, controlled by `authority` (a token account, or a mint's total-supply
 * authority), under one of the program's fixed labels (`token_slot` in the token program).
 */
export function tokenStateAddress(mint: Address, authority: Address): Promise<Address> {
  return solanaEncryptedStoreAddress(ZAMA_HOST_PROGRAM_ADDRESS, {
    program: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    authority,
    scope: mint,
  });
}

/** The batch PDA for a batcher config and zero-based index (`batch_address`). */
export async function batchAddress(batcher: Address, index: bigint): Promise<Address> {
  return (await findBatchPda({ batcher, index }))[0];
}

/** The canonical confidential token account for one owner and mint (`token_account_address`). */
export async function tokenAccountAddress(mint: Address, owner: Address): Promise<Address> {
  return (await findTokenAccountPda({ mint, owner }))[0];
}

/** The single PendingBurn for a confidential token account (`pending_burn_address`). */
export async function pendingBurnAddress(mint: Address, tokenAccount: Address): Promise<Address> {
  return (await findPendingBurnPda({ mint, tokenAccount }))[0];
}

export async function joinStoreAddress(batch: Address, user: Address): Promise<Address> {
  const [record] = await findJoinRecordPda({ batch, user });
  return solanaEncryptedStoreAddress(ZAMA_HOST_PROGRAM_ADDRESS, {
    program: CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS,
    authority: record,
    scope: batch,
  });
}


export {
  findBatchAuthorityPda,
  findJoinRecordPda,
  findBatchJoinUnderlyingPda,
  findBatchPayoutUnderlyingPda,
} from './generated/confidentialBatcher/pdas/index.js';
