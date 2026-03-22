import type {
  DecryptAndReconstructUserModuleFunction,
  GetTkmsPublicKeyHexUserModuleFunction,
} from "../modules/decrypt/types.js";

// Use interface instead of type to keep the type name instead of its expansion
export interface FhevmDecryptionKey
  extends GetTkmsPublicKeyHexUserModuleFunction,
    DecryptAndReconstructUserModuleFunction {}
