import { createInputProofFromComponents } from '../../coprocessor/InputProof-p.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { VerifiedInputProof } from '../../types/inputProof.js';
import type {
  Bytes32,
  Bytes32Hex,
  Bytes32HexAble,
  Bytes65Hex,
  BytesHex,
  ChecksummedAddress,
} from '../../types/primitives.js';
import { verifyInputProof } from './verifyInputProof.js';

export type CreateVerifiedInputProofFromComponentsParameters = {
  readonly coprocessorEIP712Signatures: readonly Bytes65Hex[];
  readonly inputHandles:
    | readonly Bytes32Hex[]
    | readonly Bytes32[]
    | readonly Bytes32HexAble[];
  readonly extraData: BytesHex;
  readonly signedHandleAccess: {
    readonly userAddress: ChecksummedAddress;
    readonly contractAddress: ChecksummedAddress;
  };
};

export type CreateVerifiedInputProofFromComponentsReturnType =
  VerifiedInputProof;

export async function createVerifiedInputProofFromComponents(
  fhevm: Fhevm<FhevmChain>,
  parameters: CreateVerifiedInputProofFromComponentsParameters,
): Promise<CreateVerifiedInputProofFromComponentsReturnType> {
  const inputProof = createInputProofFromComponents(parameters);
  return await verifyInputProof(fhevm, { inputProof });
}
