import type { DecryptedFhevmHandle } from "./decryptedFhevmHandle.js";
import type { BytesHex } from "./primitives.js";

////////////////////////////////////////////////////////////////////////////////
//
// PublicDecryptionProof
//
////////////////////////////////////////////////////////////////////////////////

export interface PublicDecryptionProof {
  readonly decryptionProof: BytesHex;
  readonly orderedDecryptedHandles: readonly DecryptedFhevmHandle[];
  readonly orderedAbiEncodedClearValues: BytesHex;
  readonly extraData: BytesHex;
}
