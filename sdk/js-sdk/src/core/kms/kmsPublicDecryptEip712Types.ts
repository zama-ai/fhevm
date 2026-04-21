import type { KmsPublicDecryptEip712Types } from '../types/kms.js';

////////////////////////////////////////////////////////////////////////////////
// KmsPublicDecryptEIP712Types
////////////////////////////////////////////////////////////////////////////////

export const kmsPublicDecryptEip712Types: KmsPublicDecryptEip712Types = {
  EIP712Domain: [
    { name: 'name', type: 'string' },
    { name: 'version', type: 'string' },
    { name: 'chainId', type: 'uint256' },
    { name: 'verifyingContract', type: 'address' },
  ] as const,
  // CRITICAL: Field order is authoritative — it determines the EIP-712 type hash.
  // Changing the order will produce a different signature and break on-chain verification.
  // Must match the Solidity struct definition exactly.
  PublicDecryptVerification: [
    { name: 'ctHandles', type: 'bytes32[]' },
    { name: 'decryptedResult', type: 'bytes' },
    { name: 'extraData', type: 'bytes' },
  ] as const,
} as const;

Object.freeze(kmsPublicDecryptEip712Types);
Object.freeze(kmsPublicDecryptEip712Types.EIP712Domain);
Object.freeze(kmsPublicDecryptEip712Types.PublicDecryptVerification);
kmsPublicDecryptEip712Types.EIP712Domain.forEach(Object.freeze);
kmsPublicDecryptEip712Types.PublicDecryptVerification.forEach(Object.freeze);
