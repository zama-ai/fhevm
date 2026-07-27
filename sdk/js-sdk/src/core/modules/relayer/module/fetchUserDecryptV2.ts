import type { KmsSigncryptedShare } from '../../../types/kms-p.js';
import type { FetchUserDecryptPayloadV2 } from '../../../types/relayer-p.js';
import type { FetchUserDecryptResult } from '../../../types/relayer.js';
import type { FetchUserDecryptParametersV2, FetchUserDecryptReturnType, RelayerClientWithRuntime } from '../types.js';
import { remove0x } from '../../../base/string.js';
import { RelayerAsyncRequest } from './RelayerAsyncRequest.js';
import { buildRelayerUrlString, validateRelayerBaseUrl } from './relayerUrl.js';

//////////////////////////////////////////////////////////////////////////////
// fetchUserDecryptV2 (protocol version >= 0.14.0)
//////////////////////////////////////////////////////////////////////////////

export async function fetchUserDecryptV2(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchUserDecryptParametersV2,
): Promise<FetchUserDecryptReturnType> {
  const { options, payload } = parameters;

  const firstHandleContractPair = payload.handleContractPairs[0];
  if (firstHandleContractPair === undefined) {
    throw new Error('Empty handle contract pairs');
  }

  // V3 envelope: AttestedUserDecryptRequestJson
  // publicKey and signature keep their 0x prefix (unlike the V1/V2 flat payload)
  const relayerPayload: FetchUserDecryptPayloadV2 = {
    attestationType: 'eip712-unified-user-decrypt-v1',
    attestedPayload: {
      version: '2.0',
      type: 'user_decryption',
      handles: payload.handleContractPairs.map((pair) => ({
        ctHandle: pair.handle.bytes32Hex,
        contractAddress: pair.contractAddress,
        ownerAddress: pair.ownerAddress,
      })),
      userAddress: payload.kmsDecryptEip712Message.userAddress,
      allowedContracts: payload.kmsDecryptEip712Message.allowedContracts,
      requestValidity: {
        startTimestamp: payload.kmsDecryptEip712Message.startTimestamp,
        durationSeconds: payload.kmsDecryptEip712Message.durationSeconds,
      },
      publicKey: payload.kmsDecryptEip712Message.publicKey,
      extraData: payload.kmsDecryptEip712Message.extraData,
    },
    signature: payload.kmsDecryptEip712Signature,
  };

  const hasAuth: boolean = options?.auth !== undefined;
  const relayerBaseUrl: URL = validateRelayerBaseUrl(relayerClient.chain.fhevm.relayerUrl, hasAuth);
  const url = buildRelayerUrlString(relayerBaseUrl, 'v3/user-decrypt');

  const request = new RelayerAsyncRequest({
    relayerOperation: 'USER_DECRYPT',
    url,
    payload: relayerPayload,
    options,
  });

  const result = (await request.run()) as FetchUserDecryptResult;

  const shares: KmsSigncryptedShare[] = result.map((r) => {
    const share: KmsSigncryptedShare = {
      signature: r.signature,
      payload: r.payload,
      extraData: remove0x(r.extraData),
    };
    return share;
  });

  return shares;
}
