import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { RelayerKeyUrlOptions } from '../types/relayer.js';
import type { FetchFheEncryptionKeyBytesParameters } from '../modules/relayer/types.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { globalFheEncryptionKeyCache } from './FheEncryptionKeyCache-p.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Starts the FHE encryption key download into the global cache, or joins the
 * one in flight. Builds the relayer request (e.g. auth) for both the bytes
 * and the encryption paths. Does not read or convert the key.
 *
 * @internal
 */
export function ensureFheEncryptionKeyDownload(
  context: {
    readonly chain: FhevmChain;
    readonly runtime: FhevmRuntime;
  },
  parameters: {
    readonly fhevmContext: FhevmClientFrozenContext;
    readonly options?: RelayerKeyUrlOptions | undefined;
    readonly ignoreCache?: boolean | undefined;
  },
): void {
  const relayerUrl = context.chain.fhevm.relayerUrl;
  const runtime = context.runtime;
  const logger = runtime.config.logger;

  // Caller-provided options override runtime config defaults (e.g. auth)
  const relayerParameters: FetchFheEncryptionKeyBytesParameters = {
    ...parameters,
    options: { auth: context.runtime.config.auth, ...parameters.options },
  };

  globalFheEncryptionKeyCache.ensureBytes({
    owner: runtime,
    relayerUrl,
    fetcher: async () => {
      logger?.debug?.(
        `Fetching FHE encryption key/CRS bytes from relayer "${relayerUrl}" (chainId=${context.chain.id}).`,
      );
      try {
        const bytes = await runtime.relayer.fetchFheEncryptionKeyBytes(
          { relayerUrl, chainId: context.chain.id },
          relayerParameters,
        );
        logger?.debug?.(`Fetched FHE encryption key/CRS bytes from relayer "${relayerUrl}".`);
        return bytes;
      } catch (e) {
        logger?.error?.(`Failed to fetch FHE encryption key/CRS bytes from relayer "${relayerUrl}".`, e);
        throw e;
      }
    },
    metadata: { chainId: context.chain.id, relayerUrl },
  });
}
