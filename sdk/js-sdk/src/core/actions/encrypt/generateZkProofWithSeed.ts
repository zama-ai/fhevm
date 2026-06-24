import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithEncrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { ZkProof } from '../../types/zkProof-p.js';
import type { BytesHex } from '../../types/primitives.js';
import { createTypedValue } from '../../base/typedValue.js';
import { createZkProofBuilder } from '../../coprocessor/ZkProofBuilder-p.js';
import { asFhevmWithTfheVersion } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

export type GenerateZkProofWithSeedParameters = {
  readonly contractAddress: string;
  readonly userAddress: string;
  readonly values: ReadonlyArray<{ readonly type: string; readonly value: boolean | bigint | number | string }>;
  readonly seed: Uint8Array;
};

export type GenerateZkProofWithSeedReturnType = ZkProof;

////////////////////////////////////////////////////////////////////////////////

export async function generateZkProofWithSeed(
  fhevm: Fhevm<FhevmChain, WithEncrypt>,
  parameters: GenerateZkProofWithSeedParameters,
): Promise<GenerateZkProofWithSeedReturnType> {
  const { values, contractAddress, userAddress, seed } = parameters;
  const hardCodedExtraData = '0x00' as BytesHex;

  const builder = createZkProofBuilder();
  for (const value of values) {
    builder.addTypedValue(createTypedValue(value));
  }

  const f = asFhevmWithTfheVersion(fhevm);

  return builder.buildSeeded(f, {
    contractAddress,
    userAddress,
    extraData: hardCodedExtraData,
    seed,
  });
}
