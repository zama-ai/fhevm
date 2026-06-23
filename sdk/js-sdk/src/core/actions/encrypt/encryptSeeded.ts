import type { RelayerInputProofOptions } from '../../types/relayer.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithEncrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { Bytes, BytesHex } from '../../types/primitives.js';
import type { EncryptedValue } from '../../types/encryptedTypes.js';
import { addressToChecksummedAddress, assertIsAddress } from '../../base/address.js';
import { assertIsEncryptionSeed, generateEncryptionSeed } from '../../base/random.js';
import { createTypedValue } from '../../base/typedValue.js';
import { encrypt as encrypt_ } from '../../coprocessor/encrypt.js';
import { asFhevmWithTfheVersion } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////
//
// encryptSeeded — deterministic ("seeded") public encryption.
//
// This is the distinct, explicitly-named "advanced/custody" entry point for
// seeded encryption. It is intentionally SEPARATE from `encryptValues` so the
// seeded path cannot be reached by accident on the default encrypt path.
//
// ⚠️  SECURITY: the returned `seed` is plaintext-equivalent. Anyone holding
// (seed, ciphertext, public key) can recover the plaintext WITHOUT the FHE
// secret key. Generate a fresh seed per encryption, never reuse it, never
// derive it from the message, and transmit it only over a secure out-of-band
// channel to a mutually-trusted verifier. The SDK never logs or persists it.
//
////////////////////////////////////////////////////////////////////////////////

export type EncryptSeededParameters = {
  readonly values: ReadonlyArray<{ readonly type: string; readonly value: boolean | bigint | number | string }>;
  readonly contractAddress: string;
  readonly userAddress: string;
  readonly options?: RelayerInputProofOptions | undefined;
  /**
   * Optional seed (>= 16 bytes). When omitted, a fresh CSPRNG seed is generated.
   * Either way the seed actually used is returned and MUST be protected by the
   * caller. Requires TFHE version `1.6.1`.
   */
  readonly seed?: Uint8Array | undefined;
};

export type EncryptSeededReturnType = {
  readonly encryptedValues: readonly EncryptedValue[];
  readonly inputProof: BytesHex;
  /** The seed actually used (caller-supplied or CSPRNG-generated). Plaintext-equivalent. */
  readonly seed: Bytes;
};

////////////////////////////////////////////////////////////////////////////////

export async function encryptSeeded(
  fhevm: Fhevm<FhevmChain, WithEncrypt>,
  parameters: EncryptSeededParameters,
): Promise<EncryptSeededReturnType> {
  const { contractAddress, userAddress, options } = parameters;

  // CSPRNG by default; validate caller-supplied seeds (length / type).
  const seed: Bytes = parameters.seed ?? generateEncryptionSeed();
  assertIsEncryptionSeed(seed);

  // Validates `values`
  const values = parameters.values.map(createTypedValue);

  assertIsAddress(contractAddress, {});
  assertIsAddress(userAddress, {});

  const f = asFhevmWithTfheVersion(fhevm);

  const result = await encrypt_(f, {
    contractAddress: addressToChecksummedAddress(contractAddress),
    userAddress: addressToChecksummedAddress(userAddress),
    values,
    options,
    seed,
  });

  return {
    encryptedValues: result.inputHandles.map(
      (encryptedValue) => encryptedValue.bytes32Hex as unknown as EncryptedValue,
    ),
    inputProof: result.inputProof,
    seed,
  };
}
