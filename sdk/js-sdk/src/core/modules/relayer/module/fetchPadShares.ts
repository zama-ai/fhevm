import type { FetchPadSharesParameters, FetchPadSharesReturnType, RelayerClientWithRuntime } from '../types.js';

////////////////////////////////////////////////////////////////////////////////
// fetchPadShares
////////////////////////////////////////////////////////////////////////////////

// RFC-030 pad decryption has no relayer route yet.
// eslint-disable-next-line @typescript-eslint/require-await
export async function fetchPadShares(
  _relayerClient: RelayerClientWithRuntime,
  _parameters: FetchPadSharesParameters,
): Promise<FetchPadSharesReturnType> {
  throw new Error('Not yet implemented');
}
