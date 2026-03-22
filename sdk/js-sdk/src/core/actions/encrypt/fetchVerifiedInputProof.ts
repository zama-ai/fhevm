import { InputProofError } from "../../errors/InputProofError.js";
import { assertFhevmHandleArrayEquals } from "../../handle/FhevmHandle.js";
import type { RelayerFetchOptions } from "../../modules/relayer/types.js";
import type { Fhevm } from "../../types/coreFhevmClient.js";
import type { WithRelayer } from "../../types/coreFhevmRuntime.js";
import type { FhevmChain } from "../../types/fhevmChain.js";
import type { FhevmHandle } from "../../types/fhevmHandle.js";
import type { VerifiedInputProof } from "../../types/inputProof.js";
import type { BytesHex } from "../../types/primitives.js";
import type { ZkProof } from "../../types/zkProof.js";
import { createVerifiedInputProofFromComponents } from "./createVerifiedInputProofFromComponents.js";

export type FetchInputProofParameters = {
  readonly zkProof: ZkProof;
  readonly extraData: BytesHex;
  readonly options?: RelayerFetchOptions;
};

export type FetchInputProofReturnType = VerifiedInputProof;

export async function fetchVerifiedInputProof(
  fhevm: Fhevm<FhevmChain, WithRelayer>,
  parameters: FetchInputProofParameters,
): Promise<FetchInputProofReturnType> {
  const { zkProof, extraData, options } = parameters;

  // 1. extract FhevmHandles from the given ZK proof
  const fhevmHandles: readonly FhevmHandle[] = zkProof.getFhevmHandles();

  if (fhevmHandles.length === 0) {
    throw new InputProofError({
      message: `Input proof must contain at least one external handle`,
    });
  }

  // 2. Request coprocessor signatures from the relayer for the given ZK proof
  const {
    handles: coprocessorSignedHandles,
    coprocessorEIP712Signatures: coprocessorSignatures,
  } = await fhevm.runtime.relayer.fetchCoprocessorSignatures(
    { relayerUrl: fhevm.chain.fhevm.relayerUrl },
    {
      payload: {
        zkProof,
        extraData,
      },
      options,
    },
  );

  // 3. Check that the handles and the one in the fetch result
  // Note: this check is theoretically unecessary
  // We prefer to perform this test since we do not trust the relayer
  // The purpose is to check if the relayer is possibly malicious
  assertFhevmHandleArrayEquals(fhevmHandles, coprocessorSignedHandles);

  // 4. Verify ZK proof and Compute the final Input proof
  return await createVerifiedInputProofFromComponents(fhevm, {
    coprocessorEIP712Signatures: coprocessorSignatures,
    externalHandles: fhevmHandles,
    extraData: extraData,
    coprocessorSignedParams: {
      userAddress: zkProof.userAddress,
      contractAddress: zkProof.contractAddress,
    },
  });
}
