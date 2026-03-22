import { InputProofError } from "../../errors/InputProofError.js";
import type { Fhevm } from "../../types/coreFhevmClient.js";
import type { FhevmChain } from "../../types/fhevmChain.js";
import type { InputProof, VerifiedInputProof } from "../../types/inputProof.js";
import type { ChecksummedAddress } from "../../types/primitives.js";
import { verifyHandlesCoprocessorSignatures } from "./verifyHandlesCoprocessorSignatures.js";

export type VerifyInputProofParameters = {
  readonly inputProof: InputProof;
  readonly coprocessorSignedParams?: {
    readonly userAddress: ChecksummedAddress;
    readonly contractAddress: ChecksummedAddress;
  };
};

export type VerifyInputProofReturnType = VerifiedInputProof;

export async function verifyInputProof(
  fhevm: Fhevm<FhevmChain>,
  parameters: VerifyInputProofParameters,
): Promise<VerifyInputProofReturnType> {
  const coprocessorSignedParams =
    parameters.coprocessorSignedParams ??
    parameters.inputProof.coprocessorSignedParams;
  if (coprocessorSignedParams === undefined) {
    throw new InputProofError({
      message: "Missing coprocessorSignedParams argument.",
    });
  }

  const chainId = parameters.inputProof.externalHandles[0].chainId;

  await verifyHandlesCoprocessorSignatures(fhevm, {
    chainId,
    coprocessorSignatures: parameters.inputProof.coprocessorSignatures,
    extraData: parameters.inputProof.extraData,
    handles: parameters.inputProof.externalHandles,
    userAddress: coprocessorSignedParams.userAddress,
    contractAddress: coprocessorSignedParams.contractAddress,
  });

  return parameters.inputProof as VerifiedInputProof;
}
