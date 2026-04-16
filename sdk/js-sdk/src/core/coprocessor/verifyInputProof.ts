import type { FhevmChain } from '../types/fhevmChain.js';
import type { InputProof, VerifiedInputProof } from '../types/inputProof.js';
import type { ChecksummedAddress } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { InputProofError } from '../errors/InputProofError.js';
import { verifyHandlesCoprocessorSignatures } from './verifyHandlesCoprocessorSignatures.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly chain: FhevmChain;
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly inputProof: InputProof;
  readonly signedHandleAccess?: {
    readonly userAddress: ChecksummedAddress;
    readonly contractAddress: ChecksummedAddress;
  };
};

type ReturnType = VerifiedInputProof;

////////////////////////////////////////////////////////////////////////////////

export async function verifyInputProof(context: Context, parameters: Parameters): Promise<ReturnType> {
  const signedHandleAccess = parameters.signedHandleAccess ?? parameters.inputProof.signedHandleAccess;

  if (signedHandleAccess === undefined) {
    throw new InputProofError({
      message: 'Missing signedHandleAccess argument.',
    });
  }

  const chainId = parameters.inputProof.inputHandles[0].chainId;

  await verifyHandlesCoprocessorSignatures(context, {
    chainId,
    coprocessorSignatures: parameters.inputProof.coprocessorSignatures,
    extraData: parameters.inputProof.extraData,
    handles: parameters.inputProof.inputHandles,
    userAddress: signedHandleAccess.userAddress,
    contractAddress: signedHandleAccess.contractAddress,
  });

  return parameters.inputProof as VerifiedInputProof;
}
