import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithEncrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { BytesHex } from '../../types/primitives.js';
import type { EncryptedValue } from '../../types/encryptedTypes.js';
import { bytesToHexLarge, toBytes } from '../../base/bytes.js';
import { assertIsEncryptionSeed } from '../../base/random.js';
import { EncryptionError } from '../../errors/EncryptionError.js';
import { generateZkProof } from './generateZkProof.js';

////////////////////////////////////////////////////////////////////////////////
//
// verifySeededEncryption
//
// Client-side encryption verifiability. An independent verifier re-runs the same
// seeded encryption (same seed + plaintext values + public key/CRS + addresses)
// and checks the reproduced ciphertext / input handles match the ones claimed by
// the (potentially untrusted) encrypting machine. A byte-for-byte match proves
// "this ciphertext encrypts exactly these values".
//
// Notes:
// - Purely local: this does NOT contact the relayer/coprocessor.
// - Compares against the CLIENT-side proven ciphertext and the locally-derived
//   handles. The on-chain ciphertext is re-randomized by the coprocessor and is
//   therefore not byte-comparable; the input handle (keccak over the client
//   ciphertext) is the stable identifier and is reproduced here.
//
////////////////////////////////////////////////////////////////////////////////

export type VerifySeededEncryptionParameters = {
  /** The secret seed shared between encryptor and verifier (>= 16 bytes). */
  readonly seed: Uint8Array;
  /** The plaintext values claimed to have been encrypted. */
  readonly values: ReadonlyArray<{ readonly type: string; readonly value: boolean | bigint | number | string }>;
  readonly contractAddress: string;
  readonly userAddress: string;
  /**
   * Claimed input handles (e.g. `encryptedValues` returned by `encryptValues`).
   * When provided, the reproduced handles must match these exactly.
   */
  readonly expectedEncryptedValues?: readonly EncryptedValue[] | undefined;
  /**
   * Claimed raw client-side proven ciphertext bytes (hex string or `Uint8Array`).
   * When provided, the reproduced ciphertext must match byte-for-byte.
   */
  readonly expectedCiphertext?: BytesHex | Uint8Array | undefined;
};

export type VerifySeededEncryptionReturnType = {
  /** `true` iff every provided expectation matched the reproduced artifacts. */
  readonly verified: boolean;
  /** The reproduced input handles. */
  readonly encryptedValues: readonly EncryptedValue[];
  /** The reproduced raw client-side proven ciphertext bytes, hex-encoded. */
  readonly ciphertext: BytesHex;
  /** Human-readable reasons for any mismatch (empty when `verified`). */
  readonly mismatches: readonly string[];
};

////////////////////////////////////////////////////////////////////////////////

export async function verifySeededEncryption(
  fhevm: Fhevm<FhevmChain, WithEncrypt>,
  parameters: VerifySeededEncryptionParameters,
): Promise<VerifySeededEncryptionReturnType> {
  const { seed, values, contractAddress, userAddress, expectedEncryptedValues, expectedCiphertext } = parameters;

  assertIsEncryptionSeed(seed);

  if (expectedEncryptedValues === undefined && expectedCiphertext === undefined) {
    throw new EncryptionError({
      message: 'verifySeededEncryption requires expectedEncryptedValues and/or expectedCiphertext to verify against.',
    });
  }

  const zkProof = await generateZkProof(fhevm, { seed, values, contractAddress, userAddress });

  const encryptedValues = zkProof.getInputHandles().map((h) => h.bytes32Hex as unknown as EncryptedValue);
  const ciphertext = bytesToHexLarge(zkProof.ciphertextWithZkProof);

  const mismatches: string[] = [];

  if (expectedEncryptedValues !== undefined) {
    if (expectedEncryptedValues.length !== encryptedValues.length) {
      mismatches.push(
        `handle count mismatch: expected ${expectedEncryptedValues.length}, reproduced ${encryptedValues.length}`,
      );
    } else {
      for (let i = 0; i < encryptedValues.length; i++) {
        if (!_eqHex(expectedEncryptedValues[i], encryptedValues[i])) {
          mismatches.push(`handle[${i}] mismatch`);
        }
      }
    }
  }

  if (expectedCiphertext !== undefined) {
    const expectedHex = bytesToHexLarge(toBytes(expectedCiphertext, { subject: 'expectedCiphertext' }));
    if (!_eqHex(expectedHex, ciphertext)) {
      mismatches.push('ciphertext bytes mismatch');
    }
  }

  return {
    verified: mismatches.length === 0,
    encryptedValues,
    ciphertext,
    mismatches,
  };
}

////////////////////////////////////////////////////////////////////////////////

/** Case-insensitive hex-string equality (handles differ only by `0x` casing). */
function _eqHex(a: string | undefined, b: string | undefined): boolean {
  return a !== undefined && a.toLowerCase() === b?.toLowerCase();
}
