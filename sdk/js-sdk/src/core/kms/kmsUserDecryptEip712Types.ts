import type { KmsUserDecryptEip712Types } from '../types/kms.js';

////////////////////////////////////////////////////////////////////////////////
// KmsUserDecryptEIP712Types
////////////////////////////////////////////////////////////////////////////////

export const kmsUserDecryptEip712Types: KmsUserDecryptEip712Types = {
  EIP712Domain: [
    { name: 'name', type: 'string' },
    { name: 'version', type: 'string' },
    { name: 'chainId', type: 'uint256' },
    { name: 'verifyingContract', type: 'address' },
  ] as const,
  // CRITICAL: Field order is authoritative — it determines the EIP-712 type hash.
  // Changing the order will produce a different signature and break on-chain verification.
  // Must match the Solidity struct definition exactly.
  UserDecryptRequestVerification: [
    { name: 'publicKey', type: 'bytes' },
    { name: 'contractAddresses', type: 'address[]' },
    { name: 'startTimestamp', type: 'uint256' },
    { name: 'durationDays', type: 'uint256' },
    { name: 'extraData', type: 'bytes' },
  ] as const,
} as const;

Object.freeze(kmsUserDecryptEip712Types);
Object.freeze(kmsUserDecryptEip712Types.EIP712Domain);
Object.freeze(kmsUserDecryptEip712Types.UserDecryptRequestVerification);
kmsUserDecryptEip712Types.EIP712Domain.forEach(Object.freeze);
kmsUserDecryptEip712Types.UserDecryptRequestVerification.forEach(Object.freeze);
