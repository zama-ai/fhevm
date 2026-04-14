import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { InputProof, VerifiedInputProof } from '../../types/inputProof.js';
import type { ChecksummedAddress } from '../../types/primitives.js';
import { InputProofError } from '../../errors/InputProofError.js';
import { verifyHandlesCoprocessorSignatures } from './verifyHandlesCoprocessorSignatures.js';

////////////////////////////////////////////////////////////////////////////////

export type VerifyInputProofParameters = {
  readonly inputProof: InputProof;
  readonly signedHandleAccess?: {
    readonly userAddress: ChecksummedAddress;
    readonly contractAddress: ChecksummedAddress;
  };
};

export type VerifyInputProofReturnType = VerifiedInputProof;

////////////////////////////////////////////////////////////////////////////////

export async function verifyInputProof(
  fhevm: Fhevm<FhevmChain>,
  parameters: VerifyInputProofParameters,
): Promise<VerifyInputProofReturnType> {
  const signedHandleAccess = parameters.signedHandleAccess ?? parameters.inputProof.signedHandleAccess;
  if (signedHandleAccess === undefined) {
    throw new InputProofError({
      message: 'Missing signedHandleAccess argument.',
    });
  }

  const chainId = parameters.inputProof.inputHandles[0].chainId;

  await verifyHandlesCoprocessorSignatures(fhevm, {
    chainId,
    coprocessorSignatures: parameters.inputProof.coprocessorSignatures,
    extraData: parameters.inputProof.extraData,
    handles: parameters.inputProof.inputHandles,
    userAddress: signedHandleAccess.userAddress,
    contractAddress: signedHandleAccess.contractAddress,
  });

  return parameters.inputProof as VerifiedInputProof;
}
