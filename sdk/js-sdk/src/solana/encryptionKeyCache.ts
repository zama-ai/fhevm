import { globalFheEncryptionKeyCache } from '../core/key/FheEncryptionKeyCache-p.js';

/**
 * Invalidates cached Solana encryption material for one relayer.
 *
 * Local validators commonly restart at the same URL while publishing a new KMS key/CRS. A browser
 * integration that detects changed material ids must clear the URL-keyed SDK cache before injecting
 * the replacement bytes.
 */
export function clearSolanaEncryptionKeyCache(relayerUrl: string): boolean {
  return globalFheEncryptionKeyCache.remove(relayerUrl);
}
