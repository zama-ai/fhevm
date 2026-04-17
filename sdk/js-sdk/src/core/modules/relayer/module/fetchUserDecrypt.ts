import type { KmsSigncryptedShare } from '../../../types/kms-p.js';
import type { Bytes65HexNo0x } from '../../../types/primitives.js';
import type { FetchUserDecryptPayload } from '../../../types/relayer-p.js';
import type { FetchUserDecryptResult } from '../../../types/relayer.js';
import type { FetchUserDecryptParameters, FetchUserDecryptReturnType, RelayerClient } from '../types.js';
import { remove0x, removeSuffix } from '../../../base/string.js';
import { RelayerAsyncRequest } from './RelayerAsyncRequest.js';

//////////////////////////////////////////////////////////////////////////////
// fetchUserDecrypt
//////////////////////////////////////////////////////////////////////////////

export async function fetchUserDecrypt(
  relayerClient: RelayerClient,
  parameters: FetchUserDecryptParameters,
): Promise<FetchUserDecryptReturnType> {
  const { options, payload } = parameters;

  const firstHandleContractPair = payload.handleContractPairs[0];
  if (firstHandleContractPair === undefined) {
    throw new Error('Empty handle contract pairs');
  }

  // retrieve chainId using handles
  const contractsChainId = firstHandleContractPair.handle.chainId.toString();

  const relayerPayload: FetchUserDecryptPayload = {
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
    signature: remove0x(payload.kmsDecryptEip712Signature) as Bytes65HexNo0x,
    extraData: payload.kmsDecryptEip712Message.extraData,
    publicKey: remove0x(payload.kmsDecryptEip712Message.publicKey),
  };

  const request = new RelayerAsyncRequest({
    relayerOperation: 'USER_DECRYPT',
    url: `${removeSuffix(relayerClient.relayerUrl, '/')}/v2/user-decrypt`,
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
