import type { RelayerKeyUrlOptions } from '../types/relayer.js';
import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmBase, OptionalNativeClient } from '../types/coreFhevmClient.js';
import type { FheEncryptionKeyWasm } from '../types/fheEncryptionKey.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import type { SolanaProofContext } from '../types/zkProofBuilder-p.js';
import type { CacheEntry } from './FheEncryptionKeyCache-p.js';
import { deserializeFheEncryptionKey } from './deserializeFheEncryptionKey.js';
import { globalFheEncryptionKeyCache } from './FheEncryptionKeyCache-p.js';
import {
  ensureFheEncryptionKeyDownload,
  ensureSolanaFheEncryptionKeyDownload,
} from './ensureFheEncryptionKeyDownload.js';

export async function fetchFheEncryptionKeyWasm(
  context: FhevmBase<FhevmChain, WithEncrypt, OptionalNativeClient>,
  parameters: {
    readonly options?: RelayerKeyUrlOptions | undefined;
    readonly ignoreCache?: boolean | undefined;
    readonly fhevmContext: FhevmClientFrozenContext;
  },
): Promise<FheEncryptionKeyWasm> {
  // Download (or join the download), then check the key for this client
  const key = await ensureFheEncryptionKeyDownload(context, parameters);
  return _toFheEncryptionKeyWasm(context.runtime, key, parameters.fhevmContext);
}

/**
 * The Solana counterpart of {@link fetchFheEncryptionKeyWasm}: the key the client was given, or
 * the relayer's, not checked against KMSGeneration (see
 * {@link ensureSolanaFheEncryptionKeyDownload}).
 */
export async function fetchSolanaFheEncryptionKeyWasm(
  context: SolanaProofContext,
  parameters: { readonly fhevmContext: FhevmClientFrozenContext },
): Promise<FheEncryptionKeyWasm> {
  const key = ensureSolanaFheEncryptionKeyDownload(context, parameters);
  return _toFheEncryptionKeyWasm(context.runtime, key, parameters.fhevmContext);
}

async function _toFheEncryptionKeyWasm(
  runtime: WithEncrypt,
  { slot, entry }: { readonly slot: string; readonly entry: CacheEntry },
  fhevmContext: FhevmClientFrozenContext,
): Promise<FheEncryptionKeyWasm> {
  // Upgrade bytes → wasm, on the entry checked (not the slot's current one)
  globalFheEncryptionKeyCache.ensureWasm({
    owner: runtime,
    slot,
    entry,
    deserializeFn: (bytes) => deserializeFheEncryptionKey({ runtime }, { keyBytes: bytes, fhevmContext }),
  });

  await entry.ready;

  if (entry.resolvedKind !== 'wasm') {
    throw new Error('Expected wasm params but got ' + JSON.stringify(entry.resolvedKind));
  }

  return entry.value as FheEncryptionKeyWasm;
}
