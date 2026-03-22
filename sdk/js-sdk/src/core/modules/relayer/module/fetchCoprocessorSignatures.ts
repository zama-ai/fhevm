import { bytesToHexNo0x } from "../../../base/bytes.js";
import { removeSuffix } from "../../../base/string.js";
import { uintToHex0x } from "../../../base/uint.js";
import type { FetchInputProofPayload } from "../../../types/relayer-p.js";
import type {
  FetchInputProofResult,
  RelayerInputProofOptions,
} from "../../../types/relayer.js";
import type {
  FetchCoprocessorSignaturesParameters,
  FetchCoprocessorSignaturesReturnType,
  RelayerClient,
} from "../types.js";
import { RelayerAsyncRequest } from "./RelayerAsyncRequest.js";

////////////////////////////////////////////////////////////////////////////////
// fetchCoprocessorSignatures
////////////////////////////////////////////////////////////////////////////////

export async function fetchCoprocessorSignatures(
  relayerClient: RelayerClient,
  parameters: FetchCoprocessorSignaturesParameters,
): Promise<FetchCoprocessorSignaturesReturnType> {
  const { options, payload } = parameters;
  const relayerOptions = options as RelayerInputProofOptions | undefined;

  const inputProofPayload: FetchInputProofPayload = {
    ciphertextWithInputVerification: bytesToHexNo0x(
      payload.zkProof.ciphertextWithZkProof,
    ),
    contractAddress: payload.zkProof.contractAddress,
    contractChainId: uintToHex0x(payload.zkProof.chainId),
    extraData: payload.extraData,
    userAddress: payload.zkProof.userAddress,
  };

  const request = new RelayerAsyncRequest({
    relayerOperation: "INPUT_PROOF",
    url: `${removeSuffix(relayerClient.relayerUrl, "/")}/input-proof`,
    payload: inputProofPayload,
    options: relayerOptions,
  });

  const result = (await request.run()) as FetchInputProofResult;

  return {
    handles: result.handles,
    coprocessorEIP712Signatures: result.signatures,
    extraData: result.extraData,
  };
}
