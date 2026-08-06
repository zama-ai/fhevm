/**
 * Decodes the batch total the KMS certified for a settle.
 *
 * The relayer returns the certified cleartext as a 32-byte big-endian `uint256` — the exact bytes
 * the KMS signed over — but the on-chain `settle` argument is a `u64`. The token program re-expands
 * that `u64` to the 32-byte form and compares it against the signed value
 * (`confidential-token`'s `redeem_burned_amount.rs`), so the low 8 bytes must carry the whole value
 * and the high 24 bytes must be zero. Extracting the low 8 bytes big-endian AND asserting the high
 * 24 are zero rejects any certified total that does not fit `u64` before it is sent (rather than
 * silently truncating). This is intentionally NOT `verifyPublicDecryptArgsFromClaim`, which keeps
 * the 32-byte field verbatim for the host verifier.
 */
export function settleTotalFromCleartext(cleartext: Uint8Array): bigint {
  if (cleartext.length !== 32) {
    throw new Error(`certified cleartext must be a 32-byte uint256, got ${cleartext.length} bytes`);
  }
  for (let i = 0; i < 24; i++) {
    if (cleartext[i] !== 0) {
      throw new Error('certified batch total exceeds u64 (high 24 bytes of the uint256 are non-zero)');
    }
  }
  // High 24 bytes are zero, so the value is exactly the low 8 bytes read big-endian.
  return new DataView(cleartext.buffer, cleartext.byteOffset, cleartext.byteLength).getBigUint64(24, false);
}
