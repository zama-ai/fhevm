import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { FheEncryptionKeyBytes } from '../../types/fheEncryptionKey.js';
import type { RelayerKeyUrlOptions } from '../../types/relayer.js';
import { fetchFheEncryptionKeyBytes as fetchFheEncryptionKeyBytes_ } from '../../key/fetchFheEncryptionKeyBytes.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';
import { copyFheEncryptionKeyBytes } from '../../key/copyFheEncryptionKeyBytes-p.js';

////////////////////////////////////////////////////////////////////////////////

export type FetchFheEncryptionKeyBytesParameters = {
  readonly options?: RelayerKeyUrlOptions | undefined;
  readonly ignoreCache?: boolean | undefined;
};

export type FetchFheEncryptionKeyBytesReturnType = FheEncryptionKeyBytes;

////////////////////////////////////////////////////////////////////////////////

/**
 * The verified FHE public key and CRS bytes, fetched from the relayer once
 * and cached. Returns a copy of the cached bytes: changing it never changes
 * the key the clients encrypt with.
 */
export async function fetchFheEncryptionKeyBytes(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters?: FetchFheEncryptionKeyBytesParameters,
): Promise<FetchFheEncryptionKeyBytesReturnType> {
  const fhevmContext = await initPublicAction(fhevm);
  return copyFheEncryptionKeyBytes(await fetchFheEncryptionKeyBytes_(fhevm, { ...parameters, fhevmContext }));
}
