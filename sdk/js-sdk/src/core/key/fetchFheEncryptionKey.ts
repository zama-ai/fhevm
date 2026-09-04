import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FheEncryptionKeyWasm } from '../types/fheEncryptionKey.js';
import type { TfheVersion } from '../types/moduleVersions.js';
import type { FheEncryptionKeyProvider, FheEncryptionKeyProviderParameters } from './FheEncryptionKeyProvider-p.js';
import { deserializeFheEncryptionKey } from './deserializeFheEncryptionKey.js';
import { globalFheEncryptionKeyWasmCache } from './FheEncryptionKeyCache-p.js';

export type FheEncryptionKeyWasmContext = {
  readonly runtime: WithEncrypt;
  readonly tfheVersion: TfheVersion;
  readonly fheEncryptionKeyProvider: FheEncryptionKeyProvider;
};

export async function fetchFheEncryptionKeyWasm(
  context: FheEncryptionKeyWasmContext,
  parameters: FheEncryptionKeyProviderParameters,
): Promise<FheEncryptionKeyWasm> {
  const bytes = await context.fheEncryptionKeyProvider.getAuthenticatedBytes(parameters);

  return globalFheEncryptionKeyWasmCache.getOrCreate({
    runtime: context.runtime,
    tfheVersion: context.tfheVersion,
    bytes,
    deserialize: () => deserializeFheEncryptionKey(context, bytes),
  });
}
