import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { FheEncryptionKeyBytes } from '../../types/fheEncryptionKey.js';
import type { RelayerKeyUrlOptions } from '../../types/relayer.js';
import { fetchFheEncryptionKeyBytes as fetchFheEncryptionKeyBytes_ } from '../../key/fetchFheEncryptionKeyBytes.js';

////////////////////////////////////////////////////////////////////////////////

export type FetchFheEncryptionKeyBytesParameters = {
  readonly options?: RelayerKeyUrlOptions | undefined;
  readonly ignoreCache?: boolean | undefined;
};

export type FetchFheEncryptionKeyBytesReturnType = FheEncryptionKeyBytes;

////////////////////////////////////////////////////////////////////////////////

export async function fetchFheEncryptionKeyBytes(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters?: FetchFheEncryptionKeyBytesParameters,
): Promise<FetchFheEncryptionKeyBytesReturnType> {
  return fetchFheEncryptionKeyBytes_(fhevm, parameters);
}
