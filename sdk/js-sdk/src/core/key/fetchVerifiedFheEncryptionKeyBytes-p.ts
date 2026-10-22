import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { ResolvedFhevmOptions } from '../types/coreFhevmClient.js';
import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import type { FetchFheEncryptionKeyBytesParameters } from '../modules/relayer/types.js';
import { verifyFheEncryptionKeyBytes } from './verifyFheEncryptionKeyBytes-p.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * The only source of key / CRS bytes for the global FheEncryptionKey cache:
 * takes the caller-provided bytes (`options.fheEncryptionKey`) or downloads them
 * through the relayer, then checks them against KMSGeneration. Bytes that fail
 * the check are never returned, so never cached nor used.
 */
export async function fetchVerifiedFheEncryptionKeyBytes(
  context: {
    readonly chain: FhevmChain;
    readonly runtime: FhevmRuntime;
    readonly options: ResolvedFhevmOptions;
    readonly client: NonNullable<object> | undefined;
  },
  parameters: FetchFheEncryptionKeyBytesParameters,
): Promise<FheEncryptionKeyBytes> {
  const relayerUrl = context.chain.fhevm.relayerUrl;

  const bytes =
    context.options.fheEncryptionKey ??
    (await context.runtime.relayer.fetchFheEncryptionKeyBytes({ relayerUrl, chainId: context.chain.id }, parameters));

  if (context.runtime.relayer.servesPlaceholderFheEncryptionKey === true) {
    return bytes;
  }

  return verifyFheEncryptionKeyBytes(context, bytes);
}
