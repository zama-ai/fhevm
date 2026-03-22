import type {
  FetchDelegatedUserDecryptParameters,
  FetchDelegatedUserDecryptReturnType,
  RelayerClient,
} from "../types.js";

//////////////////////////////////////////////////////////////////////////////
// fetchDelegatedUserDecrypt
//////////////////////////////////////////////////////////////////////////////

// eslint-disable-next-line @typescript-eslint/require-await
export async function fetchDelegatedUserDecrypt(
  _relayerClient: RelayerClient,
  _parameters: FetchDelegatedUserDecryptParameters,
): Promise<FetchDelegatedUserDecryptReturnType> {
  throw new Error("not yet implemented");
}
