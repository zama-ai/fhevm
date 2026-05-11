import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FheEncryptionKeyBytes, FheEncryptionKeyWasm } from '../types/fheEncryptionKey.js';
import type { RelayerKeyUrlOptions } from '../types/relayer.js';
import type { FetchFheEncryptionKeyBytesParameters } from '../modules/relayer/types.js';
import { globalFheEncryptionKeyCache } from './FheEncryptionKeyCache-p.js';
import { serializeFheEncryptionKeyWasm } from './serializeFheEncryptionKey.js';
import { asFhevmRuntimeWith } from '../runtime/CoreFhevmRuntime-p.js';

////////////////////////////////////////////////////////////////////////////////

export async function fetchFheEncryptionKeyBytes(
  context: { readonly chain: FhevmChain; readonly runtime: FhevmRuntime },
  parameters?: {
    readonly options?: RelayerKeyUrlOptions | undefined;
    readonly ignoreCache?: boolean | undefined;
  },
): Promise<FheEncryptionKeyBytes> {
  const relayerUrl = context.chain.fhevm.relayerUrl;
  const runtime = context.runtime;

  // Caller-provided options override runtime config defaults (e.g. auth)
  const relayerParameters: FetchFheEncryptionKeyBytesParameters = {
    ...parameters,
    options: { auth: context.runtime.config.auth, ...parameters?.options },
  };

  // Ensure a fetch is in-flight
  globalFheEncryptionKeyCache.ensureBytes({
    owner: runtime,
    relayerUrl,
    fetcher: () =>
      runtime.relayer.fetchFheEncryptionKeyBytes({ relayerUrl, chainId: context.chain.id }, relayerParameters),
    metadata: { chainId: context.chain.id, relayerUrl },
  });

  const bytes = await globalFheEncryptionKeyCache.resolveBytes({
    relayerUrl,
    serializeFn: _getSerializeFn(context),
  });

  if (bytes === undefined) {
    throw new Error('Failed to fetch global FHE PKE params bytes');
  }

  return bytes;
}

/**
 * Returns a serialize function that converts wasm params to bytes,
 * or `undefined` if the encrypt module is not available on the runtime.
 */
function _getSerializeFn(context: {
  readonly chain: FhevmChain;
  readonly runtime: FhevmRuntime;
}): ((args: FheEncryptionKeyWasm) => Promise<FheEncryptionKeyBytes>) | undefined {
  // Try to get a serialize fn if the encrypt module is available
  let serializeFn: ((args: FheEncryptionKeyWasm) => Promise<FheEncryptionKeyBytes>) | undefined;

  try {
    // check if the 'encrypt' module is available
    const r = asFhevmRuntimeWith(context.runtime, 'encrypt');
    serializeFn = (args) => serializeFheEncryptionKeyWasm({ runtime: r }, args);
  } catch {
    // encrypt module not available
  }

  return serializeFn;
}
