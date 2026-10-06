import type { SolanaPublicDecryptCertificateClaim } from '@fhevm/sdk/solana';

/**
 * The KMS public-decrypt certificate the SDK action returns — the glossary term is "certificate"
 * (the SDK's type name keeps its historical "claim" suffix). Type-only import: the SDK workspace
 * need not be materialized to run the offline suites.
 */
export type PublicDecryptCertificate = SolanaPublicDecryptCertificateClaim;
/**
 * Interprets the certificate cleartext as a number. `abiEncodedCleartext` is UNPREFIXED ABI hex
 * (a 32-byte big-endian uint256), so it must be parsed as hex explicitly — `BigInt(...)` on the
 * raw string reads all-digit hex like "46" as decimal 46 instead of 0x46 = 70.
 */
export const certificateCleartext = (certificate: Pick<PublicDecryptCertificate, 'abiEncodedCleartext'>): bigint =>
  BigInt(`0x${certificate.abiEncodedCleartext.replace(/^0x/, '')}`);
