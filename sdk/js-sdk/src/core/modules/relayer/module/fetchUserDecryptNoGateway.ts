import type { KmsSigncryptedShare } from '../../../types/kms-p.js';
import type { FetchUserDecryptPayloadV2NoGateway } from '../../../types/relayer-p.js';
import type { FetchUserDecryptParametersV2, FetchUserDecryptReturnType, RelayerClientWithRuntime } from '../types.js';
import { remove0x } from '../../../base/string.js';
import { RelayerHttpRequest } from './RelayerHttpRequest.js';
import { buildRelayerUrlString, validateRelayerBaseUrl } from './relayerUrl.js';

// Target route: relayer-http's `POST /v4/user-decrypt` (bypasses the Gateway chain).
//
// Request body (application/json):
// {
//   "attestationType": "eip712-unified-user-decrypt-v1",
//   "payload": {
//     "handles": [
//       { "ctHandle": "0x<32-byte handle>", "contractAddress": "0x<20-byte address>", "ownerAddress": "0x<20-byte address>" }
//     ],
//     "userAddress": "0x<20-byte address>",
//     "allowedContracts": ["0x<20-byte address>", ...],   // at most 10
//     "requestValidity": { "startTimestamp": "<decimal string or number>", "durationSeconds": "<decimal string or number>" },
//     "publicKey": "0x<bytes>",
//     "extraData": "0x<bytes>"   // 0x00 (v0), or 0x01 + 32-byte context id (v1), or 0x02 + context id + 32-byte epoch id (v2)
//   },
//   "signature": "0x<bytes>"   // the user's EIP-712 signature; empty on the ERC-1271 path
// }
//
// Success response, 200:
// { "status": "succeeded", "requestId": "<uuid>", "result": [ { "payload": "<hex, no 0x>", "signature": "<hex, no 0x>", "extraData": "0x..." }, ... ] }
//
// Source: relayer-http/crates/relayer-http/src/endpoint/{docs.md,flows/user_decrypt.rs}

//////////////////////////////////////////////////////////////////////////////
// fetchUserDecryptNoGateway (protocol version >= 0.14.0)
//////////////////////////////////////////////////////////////////////////////

export async function fetchUserDecryptNoGateway(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchUserDecryptParametersV2,
): Promise<FetchUserDecryptReturnType> {
  const { options, payload } = parameters;

  const firstHandleContractPair = payload.handleContractPairs[0];
  if (firstHandleContractPair === undefined) {
    throw new Error('Empty handle contract pairs');
  }

  // relayer-http's v4 UserDecryptRequest (attestationType, payload, signature).
  // publicKey and signature keep their 0x prefix (unlike the V1/V2 flat payload)
  const relayerPayload: FetchUserDecryptPayloadV2NoGateway = {
    attestationType: 'eip712-unified-user-decrypt-v1',
    payload: {
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
  const url = buildRelayerUrlString(relayerBaseUrl, 'v4/user-decrypt');

  const request = new RelayerHttpRequest({
    relayerOperation: 'USER_DECRYPT',
    url,
    payload: relayerPayload,
    options,
    logger: relayerClient.runtime.config.logger,
  });

  const result = await request.run();

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
