import type { FheEncryptionKeyBytes, FheEncryptionKeySource } from '../types/fheEncryptionKey.js';
import type { Bytes } from '../types/primitives.js';
import { FetchError } from '../base/errors/FetchError.js';
import { fetchWithRetry, getResponseBytes, type FetchWithRetryParameters } from '../base/fetch.js';

////////////////////////////////////////////////////////////////////////////////
// fetchFheEncryptionKeyBytesWithSource
////////////////////////////////////////////////////////////////////////////////

export async function fetchFheEncryptionKeyBytesWithSource(
  source: FheEncryptionKeySource,
  options?: FetchWithRetryParameters,
): Promise<FheEncryptionKeyBytes> {
  if (source.crsSource.capacity !== 2048) {
    throw new FetchError({
      url: source.crsSource.url,
      message: `Invalid pke crs capacity ${source.crsSource.capacity.toString()}. Expecting 2048.`,
    });
  }

  const [publicKeyBytes, pkeCrsBytes]: [Bytes, Bytes] = await Promise.all([
    _fetchBytes({ url: source.publicKeySource.url, ...options }),
    _fetchBytes({ url: source.crsSource.url, ...options }),
  ]);

  return Object.freeze({
    publicKeyBytes: Object.freeze({
      id: source.publicKeySource.id,
      bytes: publicKeyBytes,
    }),
    crsBytes: Object.freeze({
      id: source.crsSource.id,
      capacity: source.crsSource.capacity,
      bytes: pkeCrsBytes,
    }),
    metadata: Object.freeze({ ...source.metadata }),
  }) as FheEncryptionKeyBytes;
}

////////////////////////////////////////////////////////////////////////////////
// _fetchBytes
////////////////////////////////////////////////////////////////////////////////

async function _fetchBytes(params: { url: string } & FetchWithRetryParameters): Promise<Bytes> {
  const url = params.url;

  // Fetching a public key must use GET (the default method)
  if (params.init?.method !== undefined && params.init.method !== 'GET') {
    throw new FetchError({
      url,
      message: `Invalid fetch method: expected 'GET', got '${params.init.method}'`,
    });
  }

  const response = await fetchWithRetry({
    url,
    init: params.init,
    retries: params.retries,
    retryDelayMs: params.retryDelayMs,
  });

  if (!response.ok) {
    throw new FetchError({
      url,
      message: `HTTP error! status: ${response.status} on ${response.url}`,
    });
  }

  const compactPkeCrsBytes: Uint8Array = await getResponseBytes(response);

  return compactPkeCrsBytes;
}
