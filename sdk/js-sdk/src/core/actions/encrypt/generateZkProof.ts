import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithEncrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { ZkProof } from '../../types/zkProof-p.js';
import type { BytesHex } from '../../types/primitives.js';
import { createTypedValue } from '../../base/typedValue.js';
import { createZkProofBuilder } from '../../coprocessor/ZkProofBuilder-p.js';
import { resolveRawValueTypeName } from '../../handle/FheType.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

export type GenerateZkProofParameters = {
  readonly contractAddress: string;
  readonly userAddress: string;
  readonly values: ReadonlyArray<{ readonly type: string; readonly value: boolean | bigint | number | string }>;
};

export type GenerateZkProofReturnType = ZkProof;

////////////////////////////////////////////////////////////////////////////////

export async function generateZkProof(
  fhevm: Fhevm<FhevmChain, WithEncrypt>,
  parameters: GenerateZkProofParameters,
): Promise<GenerateZkProofReturnType> {
  const { values, contractAddress, userAddress } = parameters;
  const hardCodedExtraData = '0x00' as BytesHex;

  const builder = createZkProofBuilder();
  for (const value of values) {
    builder.addTypedValue(createTypedValue({ type: resolveRawValueTypeName(value.type), value: value.value }));
  }

  const fhevmContext = await initPublicAction(fhevm);

  return builder.build(fhevm, {
    contractAddress,
    userAddress,
    extraData: hardCodedExtraData,
    fhevmContext,
  });
}
