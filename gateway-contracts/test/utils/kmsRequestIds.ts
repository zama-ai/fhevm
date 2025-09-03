// Build the KMS request types as defined in contracts
const enum KmsRequestType {
  OldDecryptions = 0, // DEPRECATED
  PublicDecrypt = 1,
  UserDecrypt = 2,
  PrepKeygen = 3,
  Keygen = 4,
  Crsgen = 5,
}

// Get the expected KMS request ID (uint256) for a counter and request type
export function getKmsRequestIds(counter: number, kmsRequestType: KmsRequestType): bigint {
  if (counter < 0) {
    throw new Error("Counter must be non-negative");
  }
  // Left shift 248 bits to put the expected value in the ID's most significant byte.
  // See `KmsRequestCounters.sol` for more details.
  return BigInt(counter) + (BigInt(kmsRequestType) << 248n);
}

// Get the expected decryptionId for a public decryption request
export function getPublicDecryptId(counter: number): bigint {
  return getKmsRequestIds(counter, KmsRequestType.PublicDecrypt);
}

// Get the expected decryptionId for a user decryption request
export function getUserDecryptId(counter: number): bigint {
  return getKmsRequestIds(counter, KmsRequestType.UserDecrypt);
}

// Get the expected prepKeygenId for a preprocessing keygen request
export function getPrepKeygenId(counter: number): bigint {
  return getKmsRequestIds(counter, KmsRequestType.PrepKeygen);
}

// Get the expected keyId for a keygen request
export function getKeyId(counter: number): bigint {
  return getKmsRequestIds(counter, KmsRequestType.Keygen);
}

// Get the expected crsId for a crsgen request
export function getCrsId(counter: number): bigint {
  return getKmsRequestIds(counter, KmsRequestType.Crsgen);
}
