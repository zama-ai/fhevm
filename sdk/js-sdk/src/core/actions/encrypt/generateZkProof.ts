import { createTypedValue } from '../../base/typedValue.js';
import { createZkProofBuilder } from '../../coprocessor/ZkProofBuilder-p.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithEncrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { TypedValueLike } from '../../types/primitives.js';
import type { ZkProof } from '../../types/zkProof.js';

////////////////////////////////////////////////////////////////////////////////

export type GenerateZkProofParameters = {
  readonly contractAddress: string;
  readonly userAddress: string;
  readonly values: readonly TypedValueLike[];
};

export type GenerateZkProofReturnType = ZkProof;

////////////////////////////////////////////////////////////////////////////////

export async function generateZkProof(
  fhevm: Fhevm<FhevmChain, WithEncrypt>,
  parameters: GenerateZkProofParameters,
): Promise<GenerateZkProofReturnType> {
  const { values, contractAddress, userAddress } = parameters;
  const builder = createZkProofBuilder();
  for (const value of values) {
    builder.addTypedValue(createTypedValue(value));
  }
  return builder.build(fhevm, {
    contractAddress,
    userAddress,
  });
}
