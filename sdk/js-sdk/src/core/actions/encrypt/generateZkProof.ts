import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithEncrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { ZkProof } from '../../types/zkProof-p.js';
import type { BytesHex } from '../../types/primitives.js';
import { createTypedValue } from '../../base/typedValue.js';
import { createZkProofBuilder } from '../../coprocessor/ZkProofBuilder-p.js';
import { hyperWasmResolveTfheModuleVersion } from '../../runtime/HyperWasmSolver-p.js';

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

  const tfheVersion = await hyperWasmResolveTfheModuleVersion(fhevm);

  const builder = createZkProofBuilder();
  for (const value of values) {
    builder.addTypedValue(createTypedValue(value));
  }
  return builder.build(
    {
      chain: fhevm.chain,
      runtime: fhevm.runtime,
      tfheVersion,
    },
    {
      contractAddress,
      userAddress,
      extraData: hardCodedExtraData,
    },
  );
}
