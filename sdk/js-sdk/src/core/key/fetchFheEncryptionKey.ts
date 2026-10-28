import type { RelayerKeyUrlOptions } from '../types/relayer.js';
import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmBase, OptionalNativeClient } from '../types/coreFhevmClient.js';
import type { FheEncryptionKeyWasm } from '../types/fheEncryptionKey.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { deserializeFheEncryptionKey } from './deserializeFheEncryptionKey.js';
import {
  fetchVerifiedFheEncryptionKeyBytes,
  getFheEncryptionKeyEntry,
} from './fetchVerifiedFheEncryptionKeyBytes-p.js';

export async function fetchFheEncryptionKeyWasm(
  context: FhevmBase<FhevmChain, WithEncrypt, OptionalNativeClient>,
  parameters: {
    readonly options?: RelayerKeyUrlOptions | undefined;
    readonly ignoreCache?: boolean | undefined;
    readonly fhevmContext: FhevmClientFrozenContext;
  },
): Promise<FheEncryptionKeyWasm> {
  const relayerUrl = context.chain.fhevm.relayerUrl;
  const runtime = context.runtime;
  const logger = runtime.config.logger;

  const entry = await getFheEncryptionKeyEntry(context, {
    fetcher: async () => {
      logger?.debug?.(
        `Fetching FHE encryption key/CRS bytes from relayer "${relayerUrl}" (chainId=${context.chain.id}).`,
      );
      try {
        const bytes = await fetchVerifiedFheEncryptionKeyBytes(context, parameters);
        logger?.debug?.(`Fetched FHE encryption key/CRS bytes from relayer "${relayerUrl}".`);
        return bytes;
      } catch (e) {
        logger?.error?.(`Failed to fetch FHE encryption key/CRS bytes from relayer "${relayerUrl}".`, e);
        throw e;
      }
    },
  });

  // Deserialized once per entry, from its bytes (pending or resolved).
  return entry.wasm((bytes) =>
    deserializeFheEncryptionKey(context, { keyBytes: bytes, fhevmContext: parameters.fhevmContext }),
  );
}
