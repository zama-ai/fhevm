import type { RelayerKeyUrlOptions } from '../types/relayer.js';
import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmBase, OptionalNativeClient } from '../types/coreFhevmClient.js';
import type { FheEncryptionKeyBytes, FheEncryptionKeyWasm } from '../types/fheEncryptionKey.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import type { UsedCacheVersions } from '../caching/usedCacheVersions-p.js';
import { deserializeFheEncryptionKey } from './deserializeFheEncryptionKey.js';
import { ensureFheEncryptionKeyDownload, memoizeUntilRejected } from './ensureFheEncryptionKeyDownload.js';
import { recordUsedCacheVersion } from '../caching/usedCacheVersions-p.js';

// The wasm form of each key (bytes object), converted once and shared by the
// clients using it. A failed conversion is forgotten: the next call retries.
const wasmKeys = new WeakMap<FheEncryptionKeyBytes, Promise<FheEncryptionKeyWasm>>();

export async function fetchFheEncryptionKeyWasm(
  context: FhevmBase<FhevmChain, WithEncrypt, OptionalNativeClient>,
  parameters: {
    readonly options?: RelayerKeyUrlOptions | undefined;
    readonly fhevmContext: FhevmClientFrozenContext;
    readonly usedVersions?: UsedCacheVersions | undefined;
  },
): Promise<FheEncryptionKeyWasm> {
  // Download (or join the download), then check the key for this client
  const { bytes, version } = await ensureFheEncryptionKeyDownload(context, parameters);
  recordUsedCacheVersion(parameters.usedVersions, 'keyMaterial', version);

  return memoizeUntilRejected(
    wasmKeys,
    bytes,
    () => deserializeFheEncryptionKey(context, { keyBytes: bytes, fhevmContext: parameters.fhevmContext }),
    false,
  );
}
