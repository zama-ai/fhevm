import { remove0x, removeSuffix } from "../../../base/string.js";
import type { KmsSigncryptedShare } from "../../../types/kms-p.js";
import type {
  Bytes65HexNo0x,
  BytesHexNo0x,
} from "../../../types/primitives.js";
import type { FetchUserDecryptPayload } from "../../../types/relayer-p.js";
import type {
  FetchUserDecryptResult,
  RelayerUserDecryptOptions,
} from "../../../types/relayer.js";
import type {
  FetchUserDecryptParameters,
  FetchUserDecryptReturnType,
  RelayerClient,
} from "../types.js";
import { RelayerAsyncRequest } from "./RelayerAsyncRequest.js";

//////////////////////////////////////////////////////////////////////////////
// fetchUserDecrypt
//////////////////////////////////////////////////////////////////////////////

export async function fetchUserDecrypt(
  relayerClient: RelayerClient,
  parameters: FetchUserDecryptParameters,
): Promise<FetchUserDecryptReturnType> {
  const { options, payload } = parameters;
  const relayerOptions = options as RelayerUserDecryptOptions | undefined;

  const firstHandleContractPair = payload.handleContractPairs[0];
  if (firstHandleContractPair === undefined) {
    throw new Error("Empty handle contract pairs");
  }

  // retreive chainId using handles
  const contractsChainId = firstHandleContractPair.handle.chainId.toString();

  const userDecryptPayload: FetchUserDecryptPayload = {
    handleContractPairs: payload.handleContractPairs.map((pair) => {
      return {
        handle: pair.handle.bytes32Hex,
        contractAddress: pair.contractAddress,
      };
    }),
    requestValidity: {
      startTimestamp: payload.kmsUserDecryptEIP712Message.startTimestamp,
      durationDays: payload.kmsUserDecryptEIP712Message.durationDays,
    },
    contractsChainId,
    contractAddresses: payload.kmsUserDecryptEIP712Message.contractAddresses,
    userAddress: payload.kmsUserDecryptEIP712Signer,
    signature: remove0x(
      payload.kmsUserDecryptEIP712Signature,
    ) as Bytes65HexNo0x,
    extraData: payload.kmsUserDecryptEIP712Message.extraData,
    publicKey: remove0x(
      payload.kmsUserDecryptEIP712Message.publicKey,
    ) as BytesHexNo0x,
  };

  const request = new RelayerAsyncRequest({
    relayerOperation: "USER_DECRYPT",
    url: `${removeSuffix(relayerClient.relayerUrl, "/")}/user-decrypt`,
    payload: userDecryptPayload,
    options: relayerOptions,
  });

  const result = (await request.run()) as FetchUserDecryptResult;

  return result as readonly KmsSigncryptedShare[];
}
