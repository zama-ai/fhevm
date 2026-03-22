import { ensure0x, removeSuffix } from "../../../base/string.js";
import type { Bytes65Hex, BytesHex } from "../../../types/primitives.js";
import type { RelayerFetchPublicDecryptPayload } from "../../../types/relayer-p.js";
import type {
  FetchPublicDecryptResult,
  RelayerPublicDecryptOptions,
} from "../../../types/relayer.js";
import type {
  FetchPublicDecryptParameters,
  FetchPublicDecryptReturnType,
  RelayerClient,
} from "../types.js";
import { RelayerAsyncRequest } from "./RelayerAsyncRequest.js";

//////////////////////////////////////////////////////////////////////////////
// fetchPublicDecrypt
//////////////////////////////////////////////////////////////////////////////

export async function fetchPublicDecrypt(
  relayerClient: RelayerClient,
  parameters: FetchPublicDecryptParameters,
): Promise<FetchPublicDecryptReturnType> {
  const { options, payload } = parameters;
  const relayerOptions = options as RelayerPublicDecryptOptions | undefined;

  // Convert payload argument to relayer payload
  const p: RelayerFetchPublicDecryptPayload = {
    ciphertextHandles: payload.orderedHandles.map((h) => {
      return h.bytes32Hex;
    }),
    extraData: payload.extraData,
  };

  const request = new RelayerAsyncRequest({
    relayerOperation: "PUBLIC_DECRYPT",
    url: `${removeSuffix(relayerClient.relayerUrl, "/")}/v2/public-decrypt`,
    payload: p,
    options: relayerOptions,
  });

  const result = (await request.run()) as FetchPublicDecryptResult;

  return {
    orderedAbiEncodedClearValues: ensure0x(result.decryptedValue) as BytesHex,
    kmsPublicDecryptEIP712Signatures: result.signatures.map(
      ensure0x,
    ) as Bytes65Hex[],
  };
}
