import type { RelayerKeyUrlOptions } from '../types/relayer.js';
import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmBase, OptionalNativeClient } from '../types/coreFhevmClient.js';
import type { FheEncryptionKeyWasm } from '../types/fheEncryptionKey.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { deserializeFheEncryptionKey } from './deserializeFheEncryptionKey.js';
import { globalFheEncryptionKeyCache } from './FheEncryptionKeyCache-p.js';
import { ensureFheEncryptionKeyDownload } from './ensureFheEncryptionKeyDownload.js';

export async function fetchFheEncryptionKeyWasm(
  context: FhevmBase<FhevmChain, WithEncrypt, OptionalNativeClient>,
  parameters: {
    readonly options?: RelayerKeyUrlOptions | undefined;
    readonly ignoreCache?: boolean | undefined;
    readonly fhevmContext: FhevmClientFrozenContext;
  },
): Promise<FheEncryptionKeyWasm> {
  const runtime = context.runtime;

  // Download (or join the download), then check the key for this client
  const { key, entry } = await ensureFheEncryptionKeyDownload(context, parameters);

  // Upgrade bytes → wasm, on the entry checked (not the slot's current one)
  globalFheEncryptionKeyCache.ensureWasm({
    owner: runtime,
    key,
    entry,
    deserializeFn: (bytes) =>
      deserializeFheEncryptionKey(context, { keyBytes: bytes, fhevmContext: parameters.fhevmContext }),
  });

  await entry.ready;

  if (entry.resolvedKind !== 'wasm') {
    throw new Error('Expected wasm params but got ' + JSON.stringify(entry.resolvedKind));
  }

  return entry.value as FheEncryptionKeyWasm;
}
