import type { FetchInputProofPayload } from '../../../types/relayer-p.js';
import type { FetchInputProofResult } from '../../../types/relayer.js';
import type {
  FetchCoprocessorSignaturesParameters,
  FetchCoprocessorSignaturesReturnType,
  RelayerClientWithRuntime,
} from '../types.js';
import { bytesToHexNo0x } from '../../../base/bytes.js';
import { uintToHex0x } from '../../../base/uint.js';
import { RelayerAsyncRequest } from './RelayerAsyncRequest.js';
import { buildRelayerUrlString, validateRelayerBaseUrl } from './relayerUrl.js';

////////////////////////////////////////////////////////////////////////////////
// fetchCoprocessorSignatures
////////////////////////////////////////////////////////////////////////////////

export async function fetchCoprocessorSignatures(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchCoprocessorSignaturesParameters,
): Promise<FetchCoprocessorSignaturesReturnType> {
  const { options, payload } = parameters;

  const inputProofPayload: FetchInputProofPayload = {
    ciphertextWithInputVerification: bytesToHexNo0x(payload.zkProof.ciphertextWithZkProof),
    contractAddress: payload.zkProof.contractAddress,
    contractChainId: uintToHex0x(payload.zkProof.chainId),
    extraData: payload.zkProof.getExtraData(),
    userAddress: payload.zkProof.userAddress,
  };

  const hasAuth: boolean = options?.auth !== undefined;
  const relayerBaseUrl: URL = validateRelayerBaseUrl(relayerClient.chain.fhevm.relayerUrl, hasAuth);
  const url = buildRelayerUrlString(relayerBaseUrl, 'v2/input-proof');

  const request = new RelayerAsyncRequest({
    relayerOperation: 'INPUT_PROOF',
    url,
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
