import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithEncrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { ZkProof } from '../../types/zkProof-p.js';
import type { BytesHex } from '../../types/primitives.js';
import { createTypedValue } from '../../base/typedValue.js';
import { createZkProofBuilder } from '../../coprocessor/ZkProofBuilder-p.js';
import { asFhevmWithTfheVersion } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

export type GenerateZkProofParameters = {
  readonly contractAddress: string;
  readonly userAddress: string;
  readonly values: ReadonlyArray<{ readonly type: string; readonly value: boolean | bigint | number | string }>;
  /**
   * Optional seed for deterministic ("seeded") public encryption. When provided,
   * the returned proof's ciphertext and input handles are byte-for-byte
   * reproducible from the same seed + inputs — the basis for verify-by-reproduction.
   * Requires TFHE version `1.6.1` and a seed of at least 16 bytes.
   */
  readonly seed?: Uint8Array | undefined;
};

export type GenerateZkProofReturnType = ZkProof;

////////////////////////////////////////////////////////////////////////////////

export async function generateZkProof(
  fhevm: Fhevm<FhevmChain, WithEncrypt>,
  parameters: GenerateZkProofParameters,
): Promise<GenerateZkProofReturnType> {
  const { values, contractAddress, userAddress, seed } = parameters;
  const hardCodedExtraData = '0x00' as BytesHex;

  const builder = createZkProofBuilder();
  for (const value of values) {
    builder.addTypedValue(createTypedValue(value));
  }

  const f = asFhevmWithTfheVersion(fhevm);

  return builder.build(f, {
    contractAddress,
    userAddress,
    extraData: hardCodedExtraData,
    seed,
  });
}
