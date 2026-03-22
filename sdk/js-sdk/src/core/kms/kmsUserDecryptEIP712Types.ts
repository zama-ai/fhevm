////////////////////////////////////////////////////////////////////////////////
// KmsUserDecryptEIP712Types
////////////////////////////////////////////////////////////////////////////////

import type { KmsUserDecryptEIP712Types } from "../types/kms.js";

export const kmsUserDecryptEIP712Types: KmsUserDecryptEIP712Types = {
  EIP712Domain: [
    { name: "name", type: "string" },
    { name: "version", type: "string" },
    { name: "chainId", type: "uint256" },
    { name: "verifyingContract", type: "address" },
  ] as const,
  UserDecryptRequestVerification: [
    { name: "publicKey", type: "bytes" },
    { name: "contractAddresses", type: "address[]" },
    { name: "startTimestamp", type: "uint256" },
    { name: "durationDays", type: "uint256" },
    { name: "extraData", type: "bytes" },
  ] as const,
} as const;

Object.freeze(kmsUserDecryptEIP712Types);
Object.freeze(kmsUserDecryptEIP712Types.EIP712Domain);
Object.freeze(kmsUserDecryptEIP712Types.UserDecryptRequestVerification);
kmsUserDecryptEIP712Types.EIP712Domain.forEach(Object.freeze);
kmsUserDecryptEIP712Types.UserDecryptRequestVerification.forEach(Object.freeze);
