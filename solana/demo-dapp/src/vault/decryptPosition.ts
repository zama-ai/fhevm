// The vault module reaches the SDK through @sdk-src (source-mapped in every consumer's tsconfig),
// like the rest of src/vault: the published package's types need a built SDK, and the test-suite
// typechecks these files from a clean checkout.
import type { FhevmSolanaPermitDecryptClient, SolanaUserDecryptParameters } from '@sdk-src/solana/index.js';

type ClearValues = Awaited<ReturnType<FhevmSolanaPermitDecryptClient['userDecrypt']>>;

/**
 * Decrypts a batch position for its owner. This is a deliberately THIN wrapper over the permit
 * client's `userDecrypt`: it owns no permit/proof logic, so protocol evolution stays contained to
 * the SDK and this module inherits it without change — exactly as it did across the v0 → permit
 * swap, where only this file's one line moved.
 *
 * The parameters are `userDecrypt`'s verbatim: a signed permit session plus one entry per handle,
 * each naming the `EncryptedValue` account its value lives in —
 * `pendingJoinValueAccount`/`claimAmountValueAccount` (see `./internal/batcherPdas`) for a pending
 * joined amount or a claimed payout, or a confidential-token balance account's `aclValueKey` for a
 * wrapped balance. Evidence — the account read, and a historical-access proof when an update
 * replaced the handle mid-flight — is resolved by the SDK itself.
 */
export async function decryptPosition(
  client: FhevmSolanaPermitDecryptClient,
  parameters: SolanaUserDecryptParameters,
): Promise<ClearValues> {
  return client.userDecrypt(parameters);
}
