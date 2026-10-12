/**
 * Derives the on-chain certificate threshold (matching signatures a certificate needs) from the
 * KMS corruption threshold t: 2t+1 matching signatures. The KMS core runs a committee of exactly
 * 3t+1 parties, so the gateway's signer set must have that size. t=0 is the single signer of a
 * cleartext host.
 */
export const kmsCertificateThreshold = (kmsCorruptionThreshold: number, registeredSignerCount: number): number => {
  if (!Number.isSafeInteger(kmsCorruptionThreshold) || kmsCorruptionThreshold < 0 || kmsCorruptionThreshold > 255) {
    throw new Error('KMS corruption threshold t must be an unsigned byte');
  }
  const committeeSize = 3 * kmsCorruptionThreshold + 1;
  if (registeredSignerCount !== committeeSize) {
    throw new Error(
      `KMS corruption threshold t=${kmsCorruptionThreshold} needs a committee of 3t+1=${committeeSize} ` +
        `KMS signers; the gateway has ${registeredSignerCount} registered`,
    );
  }
  return 2 * kmsCorruptionThreshold + 1;
};
