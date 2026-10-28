import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import type { RelayerKeyUrlOptions } from '../types/relayer.js';
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

  const entry = globalFheEncryptionKeyCache.getOrCreate({
    owner: runtime,
    key: relayerUrl,
    relayerUrl,
    fill: () =>
      runtime.relayer.fetchFheEncryptionKeyBytes(
        { relayerUrl, chainId: context.chain.id },
        // Caller-provided options override runtime config defaults (e.g. auth)
        { ...parameters, options: { auth: runtime.config.auth, ...parameters.options } },
      ),
  });

  // The bytes as fetched, even once the entry holds the wasm key: a
  // re-serialization may re-encode them (tfhe upgrades old versions).
  return entry.bytes;
}
