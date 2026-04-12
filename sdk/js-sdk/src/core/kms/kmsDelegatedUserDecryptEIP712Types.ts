////////////////////////////////////////////////////////////////////////////////
// KmsDelegateUserDecryptEIP712Types
////////////////////////////////////////////////////////////////////////////////

import type { KmsDelegateUserDecryptEIP712Types } from '../types/kms.js';

export const kmsDelegatedUserDecryptEIP712Types: KmsDelegateUserDecryptEIP712Types =
  {
    EIP712Domain: [
      { name: 'name', type: 'string' },
      { name: 'version', type: 'string' },
      { name: 'chainId', type: 'uint256' },
      { name: 'verifyingContract', type: 'address' },
    ] as const,
    // CRITICAL: Field order is authoritative — it determines the EIP-712 type hash.
    // Changing the order will produce a different signature and break on-chain verification.
    // Must match the Solidity struct definition exactly.
    DelegatedUserDecryptRequestVerification: [
      { name: 'publicKey', type: 'bytes' },
      { name: 'contractAddresses', type: 'address[]' },
      { name: 'delegatorAddress', type: 'address' },
      { name: 'startTimestamp', type: 'uint256' },
      { name: 'durationDays', type: 'uint256' },
      { name: 'extraData', type: 'bytes' },
    ] as const,
  } as const;

Object.freeze(kmsDelegatedUserDecryptEIP712Types);
Object.freeze(kmsDelegatedUserDecryptEIP712Types.EIP712Domain);
Object.freeze(
  kmsDelegatedUserDecryptEIP712Types.DelegatedUserDecryptRequestVerification,
);
kmsDelegatedUserDecryptEIP712Types.EIP712Domain.forEach(Object.freeze);
kmsDelegatedUserDecryptEIP712Types.DelegatedUserDecryptRequestVerification.forEach(
  Object.freeze,
);
