import type { FetchInputProofPayload } from '../../../types/relayer-p.js';
import type { FetchInputProofResult } from '../../../types/relayer.js';
import type {
  FetchCoprocessorSignaturesParameters,
  FetchCoprocessorSignaturesReturnType,
  RelayerClient,
} from '../types.js';
import { bytesToHexNo0x } from '../../../base/bytes.js';
import { removeSuffix } from '../../../base/string.js';
import { uintToHex0x } from '../../../base/uint.js';
import { RelayerAsyncRequest } from './RelayerAsyncRequest.js';

////////////////////////////////////////////////////////////////////////////////
// fetchCoprocessorSignatures
////////////////////////////////////////////////////////////////////////////////

export async function fetchCoprocessorSignatures(
  relayerClient: RelayerClient,
  parameters: FetchCoprocessorSignaturesParameters,
): Promise<FetchCoprocessorSignaturesReturnType> {
  const { options, payload } = parameters;

  const inputProofPayload: FetchInputProofPayload = {
    ciphertextWithInputVerification: bytesToHexNo0x(payload.zkProof.ciphertextWithZkProof),
    contractAddress: payload.zkProof.contractAddress,
    contractChainId: uintToHex0x(payload.zkProof.chainId),
    extraData: payload.extraData,
    userAddress: payload.zkProof.userAddress,
  };

  const request = new RelayerAsyncRequest({
    relayerOperation: 'INPUT_PROOF',
    url: `${removeSuffix(relayerClient.relayerUrl, '/')}/v2/input-proof`,
    payload: inputProofPayload,
    options,
  });

  const result = (await request.run()) as FetchInputProofResult;

  return {
    handles: result.handles,
    coprocessorEip712Signatures: result.signatures,
    extraData: result.extraData,
  };
}
