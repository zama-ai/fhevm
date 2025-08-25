export const enum KmsRequestType {
  OldDecryptions = 0,
  PublicDecrypt = 1,
  UserDecrypt = 2,
  PrepKeygen = 3,
  Keygen = 4,
  Crsgen = 5,
}

export function getKmsRequestIds(counter: number, kmsRequestType: KmsRequestType): BigInt {
  if (counter < 0) {
    throw new Error("Counter must be non-negative");
  }
  // Use BigInt to simulate uint256 arithmetic and left shift as in Solidity
  return BigInt(counter) + (BigInt(kmsRequestType) << 248n);
}
