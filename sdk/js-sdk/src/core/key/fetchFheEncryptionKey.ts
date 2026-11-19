import type { RelayerKeyUrlOptions } from '../types/relayer.js';
import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmBase, OptionalNativeClient } from '../types/coreFhevmClient.js';
import type { FheEncryptionKeyBytes, FheEncryptionKeyWasm } from '../types/fheEncryptionKey.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { deserializeFheEncryptionKey } from './deserializeFheEncryptionKey.js';
import { ensureFheEncryptionKeyDownload, memoizeUntilRejected } from './ensureFheEncryptionKeyDownload.js';

// The wasm form of each key (bytes object), converted once and shared by the
// clients using it. A failed conversion is forgotten: the next call retries.
const wasmKeys = new WeakMap<FheEncryptionKeyBytes, Promise<FheEncryptionKeyWasm>>();

type Context = FhevmBase<FhevmChain, WithEncrypt, OptionalNativeClient>;

type Parameters = {
  readonly options?: RelayerKeyUrlOptions | undefined;
  readonly fhevmContext: FhevmClientFrozenContext;
};

/** The key (wasm form), from the key cache. */
export async function fetchFheEncryptionKeyWasm(
  context: Context,
  parameters: Parameters,
): Promise<FheEncryptionKeyWasm> {
  const { value } = await fetchVersionedFheEncryptionKeyWasm(context, parameters);
  return value;
}

/**
 * The key (wasm form), from the key cache, with its cache version
 * (`keyId:crsId`), for a request that records it (see `retryOnStaleCache`).
 */
export async function fetchVersionedFheEncryptionKeyWasm(
  context: Context,
  parameters: Parameters,
): Promise<{ readonly value: FheEncryptionKeyWasm; readonly version: string }> {
  // Download (or join the download), then check the key for this client
  const { bytes, version } = await ensureFheEncryptionKeyDownload(context, parameters);

  const value = await memoizeUntilRejected(
    wasmKeys,
    bytes,
    () => deserializeFheEncryptionKey(context, { keyBytes: bytes, fhevmContext: parameters.fhevmContext }),
    false,
  );
  return { value, version };
}
