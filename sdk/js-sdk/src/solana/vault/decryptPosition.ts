import type { SolanaUserDecryptSigner } from '../signer.js';
import {
  userDecrypt,
  type SolanaUserDecryptContext,
  type SolanaUserDecryptParameters,
  type SolanaUserDecryptResult,
} from '../actions/userDecrypt.js';

/**
 * Decrypts a batch position for its owner. This is a deliberately THIN wrapper over the shared
 * {@link userDecrypt} action: it owns no permit/proof logic, so the pending decrypt-v1 swap
 * (exact-handle signing → reusable permit + unsigned request, #1689) stays contained to
 * `userDecrypt` and this module inherits it without change.
 *
 * The parameters are `userDecrypt`'s verbatim, so the shape does NOT bake in the current
 * one-handle-per-request limit — when decrypt-v1 allows up to 32 handles, batching a joined amount
 * and a share balance in one request will need no API break here.
 *
 * Supply the `aclValueKey` naming the encrypted value account being decrypted:
 * - the batcher encrypted value accounts `pending_join_value` / `claim_amount_value` — see `pendingJoinValueAccount` /
 *   `claimAmountValueAccount` in `./internal/batcherPdas` — for a pending joined amount or a claimed payout, or
 * - a confidential-token balance encrypted value account (`balance_encrypted_value_address`) for a wrapped balance.
 */
export async function decryptPosition(
  context: SolanaUserDecryptContext,
  signer: SolanaUserDecryptSigner,
  parameters: SolanaUserDecryptParameters,
): Promise<SolanaUserDecryptResult> {
  return userDecrypt(context, signer, parameters);
}
