import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import type { RelayerKeyUrlOptions } from '../types/relayer.js';
import type { FetchFheEncryptionKeyBytesParameters } from '../modules/relayer/types.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { globalFheEncryptionKeyCache } from './FheEncryptionKeyCache-p.js';

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
  const runtime = context.runtime;

  // Caller-provided options override runtime config defaults (e.g. auth)
  const relayerParameters: FetchFheEncryptionKeyBytesParameters = {
    ...parameters,
    options: { auth: context.runtime.config.auth, ...parameters.options },
  };

  // Ensure a fetch is in-flight
  globalFheEncryptionKeyCache.ensureBytes({
    owner: runtime,
    relayerUrl,
    fetcher: () =>
      runtime.relayer.fetchFheEncryptionKeyBytes({ relayerUrl, chainId: context.chain.id }, relayerParameters),
    metadata: { chainId: context.chain.id, relayerUrl },
  });

  const bytes = await globalFheEncryptionKeyCache.resolveBytes({ relayerUrl });

  if (bytes === undefined) {
    throw new Error('Failed to fetch global FHE PKE params bytes');
  }

  return bytes;
}
