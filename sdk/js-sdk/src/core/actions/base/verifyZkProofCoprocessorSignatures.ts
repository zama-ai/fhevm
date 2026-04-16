import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { Bytes65Hex, BytesHex } from '../../types/primitives.js';
import type { ZkProof } from '../../types/zkProof-p.js';
import { assertIsZkProof } from '../../coprocessor/ZkProof-p.js';
import { verifyHandlesCoprocessorSignatures } from './verifyHandlesCoprocessorSignatures.js';

////////////////////////////////////////////////////////////////////////////////
// verifyZKProofCoprocessorSignatures
////////////////////////////////////////////////////////////////////////////////

export type VerifyZkProofCoprocessorSignaturesParameters = {
  readonly zkProof: ZkProof;
  readonly coprocessorSignatures: readonly Bytes65Hex[];
  readonly extraData: BytesHex;
};

export async function verifyZkProofCoprocessorSignatures(
  fhevm: Fhevm<FhevmChain>,
  parameters: VerifyZkProofCoprocessorSignaturesParameters,
): Promise<void> {
  assertIsZkProof(parameters.zkProof, {});

  return verifyHandlesCoprocessorSignatures(fhevm, {
    inputHandles: parameters.zkProof.getInputHandles(),
    userAddress: parameters.zkProof.userAddress,
    contractAddress: parameters.zkProof.contractAddress,
    chainId: parameters.zkProof.chainId,
    extraData: parameters.extraData,
    coprocessorSignatures: parameters.coprocessorSignatures,
  });
}
