import type { FetchInputProofPayload } from '../../../types/relayer-p.js';
import type { Logger } from '../../../types/logger.js';
import type { RelayerInputProofOptions } from '../../../types/relayer.js';
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

  const result = await submitInputProofPayload({
    relayerUrl: relayerClient.chain.fhevm.relayerUrl,
    payload: inputProofPayload,
    options,
    logger: relayerClient.runtime.config.logger,
  });

  return {
    handles: result.handles,
    coprocessorEip712Signatures: result.signatures,
    extraData: result.extraData,
  };
}

/** Sends a host-encoded input proof through the shared relayer request lifecycle. */
export async function submitInputProofPayload({
  relayerUrl,
  payload,
  options,
  logger,
}: {
  readonly relayerUrl: string;
  readonly payload: FetchInputProofPayload;
  readonly options?: RelayerInputProofOptions | undefined;
  readonly logger?: Logger | undefined;
}): Promise<FetchInputProofResult> {
  const hasAuth: boolean = options?.auth !== undefined;
  const relayerBaseUrl: URL = validateRelayerBaseUrl(relayerUrl, hasAuth);
  const url = buildRelayerUrlString(relayerBaseUrl, 'v2/input-proof');

  const request = new RelayerAsyncRequest({
    relayerOperation: 'INPUT_PROOF',
    url,
    payload,
    options,
    logger,
  });

  return (await request.run()) as FetchInputProofResult;
}
