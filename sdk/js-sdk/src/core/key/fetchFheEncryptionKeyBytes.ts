import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import type { FheEncryptionKeyProvider, FheEncryptionKeyProviderParameters } from './FheEncryptionKeyProvider-p.js';

////////////////////////////////////////////////////////////////////////////////

export async function fetchFheEncryptionKeyBytes(
  provider: FheEncryptionKeyProvider,
  parameters?: FheEncryptionKeyProviderParameters,
): Promise<FheEncryptionKeyBytes> {
  return provider.getAuthenticatedBytes(parameters);
}
