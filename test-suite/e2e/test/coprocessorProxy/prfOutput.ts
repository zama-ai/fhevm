// PRF outputs as prf-service serves them, and the attestation in their `x-prf-attestation` header, as
// the PRF endpoint spec defines them:
// https://github.com/zama-ai/tech-spec/blob/main/rfcs/038-coprocessor-prf-output-endpoint.md#attestation-format
// The signed digest mirrors shared/ciphertext-attestation/src/prf/sign.rs; the "Coprocessor proxy"
// suite pins it to that crate's test vector.
import {
  SigningKey,
  computeAddress,
  concat,
  getAddress,
  keccak256,
  recoverAddress,
  toBeHex,
  toUtf8Bytes,
} from 'ethers';

const VERSION = 1;
const PRF_DOMAIN_TAG = toUtf8Bytes('FHEVMPRF');

export interface PrfOutputRef {
  prfId: number;
  /** 32 bytes as 0x-prefixed lowercase hex. */
  label: string;
  padLength: number;
}

/** The wire form of the `x-prf-attestation` header. */
export interface PrfOutputAttestation {
  version: number;
  ciphertext_digest: string;
  sns_ciphertext_digest: string;
  signer: string;
  signature: string;
}

/** prf-service accepts only canonical parameters: decimal numbers and a 64-lowercase-hex label. */
export const prfPath = ({ prfId, label, padLength }: PrfOutputRef) =>
  `/v1/prf/${prfId}/labels/${label.slice(2)}/${padLength}`;

/** `canonical_digest`: the prehash the coprocessor signs, without an EIP-191 prefix. */
export const attestationDigest = (ref: PrfOutputRef, ciphertextDigest: string, snsCiphertextDigest: string) =>
  keccak256(
    concat([
      PRF_DOMAIN_TAG,
      toBeHex(VERSION, 1),
      toBeHex(ref.prfId, 2),
      ref.label,
      toBeHex(ref.padLength, 2),
      ciphertextDigest,
      snsCiphertextDigest,
    ]),
  );

/** Attests `body` as the output of `ref`, as the pad-worker of the coprocessor holding `privateKey` will. */
export function signPrfOutput(
  privateKey: string,
  ref: PrfOutputRef,
  ciphertextDigest: string,
  body: Uint8Array,
): PrfOutputAttestation {
  const key = new SigningKey(privateKey);
  const snsCiphertextDigest = keccak256(body);
  return {
    version: VERSION,
    ciphertext_digest: ciphertextDigest,
    sns_ciphertext_digest: snsCiphertextDigest,
    signer: computeAddress(key.publicKey),
    signature: key.sign(attestationDigest(ref, ciphertextDigest, snsCiphertextDigest)).serialized,
  };
}

/**
 * What a KMS connector refuses in `attestation` for `body`, requested as `ref` from coprocessor
 * `signer` (`PrfOutputAttestation::verify`, then the body against its digest). Empty when it accepts.
 */
export function verifyPrfOutput(
  attestation: PrfOutputAttestation,
  ref: PrfOutputRef,
  body: Uint8Array,
  signer: string,
): string[] {
  const problems: string[] = [];
  if (attestation.version !== VERSION) problems.push(`version ${attestation.version}`);
  if (getAddress(attestation.signer) !== getAddress(signer)) {
    problems.push(`signer ${attestation.signer}, expected ${signer}`);
  }
  const digest = attestationDigest(ref, attestation.ciphertext_digest, attestation.sns_ciphertext_digest);
  const recovered = recoverAddress(digest, attestation.signature);
  if (recovered !== getAddress(attestation.signer)) problems.push(`signature recovers ${recovered}`);
  if (keccak256(body) !== attestation.sns_ciphertext_digest) problems.push('body does not match sns_ciphertext_digest');
  return problems;
}
