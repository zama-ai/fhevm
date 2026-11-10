import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmBase, OptionalNativeClient } from '../types/coreFhevmClient.js';
import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import type { RelayerKeyUrlOptions } from '../types/relayer.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { ensureFheEncryptionKeyDownload } from './ensureFheEncryptionKeyDownload.js';

////////////////////////////////////////////////////////////////////////////////

export async function fetchFheEncryptionKeyBytes(
  context: FhevmBase<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: {
    readonly fhevmContext: FhevmClientFrozenContext;
    readonly options?: RelayerKeyUrlOptions | undefined;
    readonly ignoreCache?: boolean | undefined;
  },
): Promise<FheEncryptionKeyBytes> {
  // Download (or join the download), then check the key for this client:
  // the bytes checked, not the slot's current ones
  const { bytes } = await ensureFheEncryptionKeyDownload(context, parameters);

  return bytes;
}
