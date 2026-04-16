import type { PublicDecryptionProof } from '../types/publicDecryptionProof-p.js';
import { PublicDecryptionProofImpl } from './PublicDecryptionProof-p.js';

////////////////////////////////////////////////////////////////////////////////
// isPublicDecryptionProof
////////////////////////////////////////////////////////////////////////////////

export function isPublicDecryptionProof(value: unknown): value is PublicDecryptionProof {
  return value instanceof PublicDecryptionProofImpl;
}
