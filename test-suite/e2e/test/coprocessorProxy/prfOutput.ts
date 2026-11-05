// RFC-038 PRF outputs as prf-service serves them: the endpoint path, the `X-Prf-Attestation` header
// and its signed digest (mirror of shared/ciphertext-attestation/src/prf/sign.rs). The pad-worker
// that writes outputs does not exist yet (https://github.com/zama-ai/fhevm-internal/issues/2193), so
// scripts/seed-prf-pads.ts writes SEEDED_OUTPUT_REF the way it will, and the Coprocessor proxy suite
// requests it.
import { SigningKey, computeAddress, concat, getBytes, keccak256, recoverAddress, toBeHex, toUtf8Bytes } from 'ethers';

/** The only PRF instance prf-service serves (coprocessor/fhevm-engine/prf-service/src/instance.rs). */
export const PRF_ID = 0;
const ATTESTATION_VERSION = 1;
const PRF_DOMAIN_TAG = toUtf8Bytes('FHEVMPRF');

export interface PrfOutputRef {
  prfId: number;
  /** 64 lowercase hex characters, no `0x`. */
  label: string;
  padLength: number;
}

/** The output the seeder writes and the suite requests. */
export const SEEDED_OUTPUT_REF: PrfOutputRef = { prfId: PRF_ID, label: '01'.repeat(32), padLength: 1 };
/** Every byte value, so a client that decodes the body as text changes it. */
export const SEEDED_CIPHERTEXT128 = Uint8Array.from({ length: 256 }, (_, i) => i);
/** Stands for the digest of the pre-SnS output, which prf-service never serves. */
export const SEEDED_CIPHERTEXT_DIGEST = keccak256(toUtf8Bytes('seeded pre-SnS PRF output'));

export const prfOutputPath = ({ prfId, label, padLength }: PrfOutputRef) =>
  `/v1/prf/${prfId}/labels/${label}/${padLength}`;

/** The wire form of the `X-Prf-Attestation` header. */
export interface PrfOutputAttestation {
  version: number;
  ciphertext_digest: string;
  sns_ciphertext_digest: string;
  signer: string;
  signature: string;
}

/** The prehash the coprocessor signs: keccak256 of the canonical bytes, without an EIP-191 prefix. */
export const attestationDigest = (ref: PrfOutputRef, ciphertextDigest: string, ciphertext128Digest: string) =>
  keccak256(
    concat([
      PRF_DOMAIN_TAG,
      toBeHex(ATTESTATION_VERSION, 1),
      toBeHex(ref.prfId, 2),
      `0x${ref.label}`,
      toBeHex(ref.padLength, 2),
      ciphertextDigest,
      ciphertext128Digest,
    ]),
  );

/** Signs the output of `ref` as the pad-worker of the coprocessor whose key is `privateKey` will. */
export const signPrfOutput = (
  privateKey: string,
  ref: PrfOutputRef,
  ciphertextDigest: string,
  ciphertext128: Uint8Array,
): PrfOutputAttestation => {
  const key = new SigningKey(privateKey);
  const ciphertext128Digest = keccak256(ciphertext128);
  return {
    version: ATTESTATION_VERSION,
    ciphertext_digest: ciphertextDigest,
    sns_ciphertext_digest: ciphertext128Digest,
    signer: computeAddress(key.publicKey),
    signature: key.sign(attestationDigest(ref, ciphertextDigest, ciphertext128Digest)).serialized,
  };
};

/**
 * Checks `attestation` as a KMS connector does (`PrfOutputAttestation::verify`): it names
 * `expectedSigner`, its signature recovers to it over `ref`, and `body` is the output it names.
 */
export const verifyPrfOutput = (
  attestation: PrfOutputAttestation,
  ref: PrfOutputRef,
  body: Uint8Array,
  expectedSigner: string,
): string[] => {
  const problems: string[] = [];
  if (attestation.version !== ATTESTATION_VERSION) problems.push(`version ${attestation.version}`);
  if (attestation.signer.toLowerCase() !== expectedSigner.toLowerCase()) {
    problems.push(`signer ${attestation.signer}, expected ${expectedSigner}`);
  }
  if (keccak256(body) !== attestation.sns_ciphertext_digest) problems.push('body does not match sns_ciphertext_digest');
  if (getBytes(attestation.signature).length !== 65) {
    problems.push('signature is not 65 bytes');
    return problems;
  }
  const digest = attestationDigest(ref, attestation.ciphertext_digest, attestation.sns_ciphertext_digest);
  const recovered = recoverAddress(digest, attestation.signature);
  if (recovered.toLowerCase() !== attestation.signer.toLowerCase()) problems.push(`signature recovers ${recovered}`);
  return problems;
};
