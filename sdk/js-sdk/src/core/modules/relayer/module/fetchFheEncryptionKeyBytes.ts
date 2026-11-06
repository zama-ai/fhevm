import type {
  FetchFheEncryptionKeyBytesParameters,
  FetchFheEncryptionKeyBytesReturnType,
  RelayerClient,
} from '../types.js';
import { fetchFheEncryptionKeySource } from './fetchFheEncryptionKeySource.js';
import { fetchFheEncryptionKeyBytesWithSource as fetchFheEncryptionKeyBytesWithSource_ } from '../../../key/fetchFheEncryptionKeyBytesWithSource.js';

////////////////////////////////////////////////////////////////////////////////
// fetchFheEncryptionKeyBytes
////////////////////////////////////////////////////////////////////////////////

export async function fetchFheEncryptionKeyBytes(
  relayerClient: RelayerClient,
  parameters: FetchFheEncryptionKeyBytesParameters,
): Promise<FetchFheEncryptionKeyBytesReturnType> {
  const { options, beforeDownload } = parameters;

  // 1. Ask the relayer for the URLs where the keys are hosted
  const source = await fetchFheEncryptionKeySource(relayerClient, {
    options,
  });

  // 2. Let the caller stop a download that cannot be used
  await beforeDownload?.(source);

  const init: RequestInit | undefined = options?.signal !== undefined ? { signal: options.signal } : undefined;

  // 3. Download the actual keys from those URLs
  const paramsBytes = await fetchFheEncryptionKeyBytesWithSource_(source, {
    retries: options?.fetchRetries,
    retryDelayMs: options?.fetchRetryDelayInMilliseconds,
    init,
  });

  return paramsBytes;
}
