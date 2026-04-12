import type { FetchDelegatedUserDecryptPayload } from '../../../types/relayer-p.js';
import type { FetchDelegatedUserDecryptResult } from '../../../types/relayer.js';
import type {
  FetchDelegatedUserDecryptParameters,
  FetchDelegatedUserDecryptReturnType,
  RelayerClient,
} from '../types.js';
import { remove0x, removeSuffix } from '../../../base/string.js';
import { RelayerAsyncRequest } from './RelayerAsyncRequest.js';
import type { KmsSigncryptedShare } from '../../../types/kms-p.js';

//////////////////////////////////////////////////////////////////////////////
// fetchDelegatedUserDecrypt
//////////////////////////////////////////////////////////////////////////////

export async function fetchDelegatedUserDecrypt(
  relayerClient: RelayerClient,
  parameters: FetchDelegatedUserDecryptParameters,
): Promise<FetchDelegatedUserDecryptReturnType> {
  const { options, payload } = parameters;

  const firstHandleContractPair = payload.handleContractPairs[0];
  if (firstHandleContractPair === undefined) {
    throw new Error('Empty handle contract pairs');
  }

  // retreive chainId using handles
  const contractsChainId = firstHandleContractPair.handle.chainId.toString();

  const relayerPayload: FetchDelegatedUserDecryptPayload = {
    handleContractPairs: payload.handleContractPairs.map((pair) => {
      return {
        handle: pair.handle.bytes32Hex,
        contractAddress: pair.contractAddress,
      };
    }),
    startTimestamp: payload.kmsDecryptEip712Message.startTimestamp,
    durationDays: payload.kmsDecryptEip712Message.durationDays,
    contractsChainId,
    contractAddresses: payload.kmsDecryptEip712Message.contractAddresses,
    delegatorAddress: payload.kmsDecryptEip712Message.delegatorAddress,
    delegateAddress: payload.kmsDecryptEip712Signer,
    signature: remove0x(payload.kmsDecryptEip712Signature),
    extraData: payload.kmsDecryptEip712Message.extraData,
    publicKey: remove0x(payload.kmsDecryptEip712Message.publicKey),
  };

  const request = new RelayerAsyncRequest({
    relayerOperation: 'DELEGATED_USER_DECRYPT',
    url: `${removeSuffix(relayerClient.relayerUrl, '/')}/v2/delegated-user-decrypt`,
    payload: relayerPayload,
    options,
  });

  const result: FetchDelegatedUserDecryptResult =
    (await request.run()) as FetchDelegatedUserDecryptResult;

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
