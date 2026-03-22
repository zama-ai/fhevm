import type { FhevmDecryptionKey } from "./fhevmDecryptionKey.js";
import type { ChecksummedAddress } from "./primitives.js";

export type FhevmUser = {
  readonly address: ChecksummedAddress;
  readonly decryptionKey: FhevmDecryptionKey;
};
