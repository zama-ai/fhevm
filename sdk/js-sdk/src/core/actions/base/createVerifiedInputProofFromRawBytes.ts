import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { VerifiedInputProof } from '../../types/inputProof.js';
import type { Bytes, ChecksummedAddress } from '../../types/primitives.js';
import { createInputProofFromRawBytes } from '../../coprocessor/InputProof-p.js';
import { verifyInputProof } from './verifyInputProof.js';

export type CreateVerifiedInputProofFromRawBytesParameters = {
  readonly inputProofBytes: Bytes;
  readonly signedHandleAccess: {
    readonly userAddress: ChecksummedAddress;
    readonly contractAddress: ChecksummedAddress;
  };
};

export type CreateVerifiedInputProofFromRawBytesReturnType = VerifiedInputProof;

export async function createVerifiedInputProofFromRawBytes(
  fhevm: Fhevm<FhevmChain>,
  parameters: CreateVerifiedInputProofFromRawBytesParameters,
): Promise<CreateVerifiedInputProofFromRawBytesReturnType> {
  const inputProof = createInputProofFromRawBytes(parameters);
  return await verifyInputProof(fhevm, { inputProof });
}
