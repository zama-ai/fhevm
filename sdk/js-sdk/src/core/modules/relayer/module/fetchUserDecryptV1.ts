import type { KmsSigncryptedShare } from '../../../types/kms-p.js';
import type { FetchUserDecryptPayload as FetchUserDecryptPayloadV1 } from '../../../types/relayer-p.js';
import type { FetchUserDecryptResult } from '../../../types/relayer.js';
import type { FetchUserDecryptParametersV1, FetchUserDecryptReturnType, RelayerClientWithRuntime } from '../types.js';
import { remove0x } from '../../../base/string.js';
import { RelayerAsyncRequest } from './RelayerAsyncRequest.js';
import { buildRelayerUrlString, validateRelayerBaseUrl } from './relayerUrl.js';

//////////////////////////////////////////////////////////////////////////////
// fetchUserDecryptV1 (protocol version < 0.14.0)
//////////////////////////////////////////////////////////////////////////////

export async function fetchUserDecryptV1(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchUserDecryptParametersV1,
): Promise<FetchUserDecryptReturnType> {
  const { options, payload } = parameters;

  const firstHandleContractPair = payload.handleContractPairs[0];
  if (firstHandleContractPair === undefined) {
    throw new Error('Empty handle contract pairs');
  }

  // ---------------------------------------------------------------------------
  // Migration hazard: extraData v2 vs a lagging (v13) relayer
  // ---------------------------------------------------------------------------
  // This path forwards `extraData` to the relayer verbatim (see `relayerPayload`
  // below). Protocol upgrades roll out in stages, so a v13 -> v14 migration has a
  // window where the host-contracts are already v14 — the current KMS context is
  // encoded as extraData **v2** (`0x02 | contextId(32) | epochId(32)`) — while the
  // relayer still runs v13 and does not recognize the v2 encoding.
  //
  // In that window the relayer rejects a v2 `extraData` in its request validation,
  // BEFORE it ever reaches the gateway/KMS (the v13 relayer only accepts `0x00`,
  // `0x01`, and — where implemented — `0x02`; an unknown/older build fails it):
  //
  //   RelayerErrorBase: User decryption: Relayer API error
  //   [validation_failed]: Validation failed for 1 field in the request: extraData
  //   status: 400
  //   url: https://relayer.<env>.zama.cloud/v2/user-decrypt   (HTTP API v2, unrelated to extraData v2)
  //
  // Rollout ordering: the relayer must be upgraded to accept extraData v2 BEFORE
  // clients start emitting v2 permits. A v13-capped SDK avoids this by only ever
  // emitting v1 extraData; but a v2 permit produced elsewhere (a v14 client, or a
  // forced-v2 permit) routed through this v1 path will hit the 400 above.
  // ---------------------------------------------------------------------------

  // retrieve chainId using handles
  const contractsChainId = firstHandleContractPair.handle.chainId.toString();

  const relayerPayload: FetchUserDecryptPayloadV1 = {
    handleContractPairs: payload.handleContractPairs.map((pair) => {
      return {
        handle: pair.handle.bytes32Hex,
        contractAddress: pair.contractAddress,
      };
    }),
    requestValidity: {
      startTimestamp: payload.kmsDecryptEip712Message.startTimestamp,
      durationDays: payload.kmsDecryptEip712Message.durationDays,
    },
    contractsChainId,
    contractAddresses: payload.kmsDecryptEip712Message.contractAddresses,
    userAddress: payload.kmsDecryptEip712Signer,
    signature: remove0x(payload.kmsDecryptEip712Signature),
    extraData: payload.kmsDecryptEip712Message.extraData,
    publicKey: remove0x(payload.kmsDecryptEip712Message.publicKey),
  };

  const hasAuth: boolean = options?.auth !== undefined;
  const relayerBaseUrl: URL = validateRelayerBaseUrl(relayerClient.chain.fhevm.relayerUrl, hasAuth);
  const url = buildRelayerUrlString(relayerBaseUrl, 'v2/user-decrypt');

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
