import type { RelayerKeyUrlOptions } from '../types/relayer.js';
import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import { deserializeFheEncryptionKey } from './deserializeFheEncryptionKey.js';
import { globalFheEncryptionKeyCache } from './FheEncryptionKeyCache-p.js';
import type { FheEncryptionKeyWasm } from '../types/fheEncryptionKey.js';

export async function fetchFheEncryptionKeyWasm(
  context: { readonly chain: FhevmChain; readonly runtime: WithEncrypt },
  parameters?: {
    readonly options?: RelayerKeyUrlOptions | undefined;
    readonly ignoreCache?: boolean | undefined;
  },
): Promise<FheEncryptionKeyWasm> {
  const relayerUrl = context.chain.fhevm.relayerUrl;
  const runtime = context.runtime;

  // Ensure a bytes fetch is in-flight
  globalFheEncryptionKeyCache.ensureBytes({
    owner: runtime,
    relayerUrl,
    fetcher: () =>
      runtime.relayer.fetchFheEncryptionKeyBytes(
        { relayerUrl, chainId: context.chain.id },
        parameters ?? {},
      ),
    metadata: { chainId: context.chain.id, relayerUrl },
  });

  // Upgrade bytes → wasm (chains from pending bytes if still in-flight)
  globalFheEncryptionKeyCache.ensureWasm({
    owner: runtime,
    relayerUrl,
    deserializeFn: (bytes) => deserializeFheEncryptionKey(context, bytes),
  });

  const entry = globalFheEncryptionKeyCache.get(relayerUrl);
  if (entry === undefined) {
    throw new Error('Failed to fetch global FHE PKE params');
  }

  await entry.ready;

  if (entry.resolvedKind !== 'wasm') {
    throw new Error(
      'Expected wasm params but got ' + JSON.stringify(entry.resolvedKind),
    );
  }

  return entry.value as FheEncryptionKeyWasm;
}
