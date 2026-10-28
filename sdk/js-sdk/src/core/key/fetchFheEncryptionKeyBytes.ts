import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmBase, OptionalNativeClient } from '../types/coreFhevmClient.js';
import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import type { RelayerKeyUrlOptions } from '../types/relayer.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import {
  fetchVerifiedFheEncryptionKeyBytes,
  getFheEncryptionKeyEntry,
} from './fetchVerifiedFheEncryptionKeyBytes-p.js';

////////////////////////////////////////////////////////////////////////////////

export async function fetchFheEncryptionKeyBytes(
  context: FhevmBase<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: {
    readonly fhevmContext: FhevmClientFrozenContext;
    readonly options?: RelayerKeyUrlOptions | undefined;
    readonly ignoreCache?: boolean | undefined;
  },
): Promise<FheEncryptionKeyBytes> {
  const entry = await getFheEncryptionKeyEntry(context, {
    fetcher: () => fetchVerifiedFheEncryptionKeyBytes(context, parameters),
  });

  // The verified bytes as fetched, even once the entry holds the wasm key: a
  // re-serialization may differ and no longer match the KMSGeneration digest.
  return entry.bytes;
}
