import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import type { RelayerKeyUrlOptions } from '../types/relayer.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { globalFheEncryptionKeyCache } from './FheEncryptionKeyCache-p.js';
import { ensureFheEncryptionKeyDownload } from './ensureFheEncryptionKeyDownload.js';

////////////////////////////////////////////////////////////////////////////////

export async function fetchFheEncryptionKeyBytes(
  context: {
    readonly chain: FhevmChain;
    readonly runtime: FhevmRuntime;
  },
  parameters: {
    readonly fhevmContext: FhevmClientFrozenContext;
    readonly options?: RelayerKeyUrlOptions | undefined;
    readonly ignoreCache?: boolean | undefined;
  },
): Promise<FheEncryptionKeyBytes> {
  const relayerUrl = context.chain.fhevm.relayerUrl;

  // Ensure a fetch is in-flight
  ensureFheEncryptionKeyDownload(context, parameters);

  const bytes = await globalFheEncryptionKeyCache.resolveBytes({ relayerUrl });

  if (bytes === undefined) {
    throw new Error('Failed to fetch global FHE PKE params bytes');
  }

  return bytes;
}
