import type { KmsUserDecryptEip712V2Types } from '../types/kms.js';

////////////////////////////////////////////////////////////////////////////////
// KmsUserDecryptEIP712V2Types (RFC-016)
////////////////////////////////////////////////////////////////////////////////

// Careful: do not change any fields in this structure
export const kmsUserDecryptEip712V2Types: KmsUserDecryptEip712V2Types = {
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
    { name: 'userAddress', type: 'address' },
    { name: 'publicKey', type: 'bytes' },
    { name: 'allowedContracts', type: 'address[]' },
    { name: 'startTimestamp', type: 'uint256' },
    { name: 'durationSeconds', type: 'uint256' },
    { name: 'extraData', type: 'bytes' },
  ] as const,
} as const;

Object.freeze(kmsUserDecryptEip712V2Types);
Object.freeze(kmsUserDecryptEip712V2Types.EIP712Domain);
Object.freeze(kmsUserDecryptEip712V2Types.UserDecryptRequestVerification);
kmsUserDecryptEip712V2Types.EIP712Domain.forEach(Object.freeze);
kmsUserDecryptEip712V2Types.UserDecryptRequestVerification.forEach(Object.freeze);
