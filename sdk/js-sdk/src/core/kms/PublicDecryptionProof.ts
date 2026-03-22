import type { PublicDecryptionProof } from "../types/publicDecryptionProof.js";
import { PublicDecryptionProofImpl } from "./PublicDecryptionProof-p.js";

////////////////////////////////////////////////////////////////////////////////
// isPublicDecryptionProof
////////////////////////////////////////////////////////////////////////////////

export function isPublicDecryptionProof(
  value: unknown,
): value is PublicDecryptionProof {
  return value instanceof PublicDecryptionProofImpl;
}
