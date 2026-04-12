import type { CoprocessorEIP712Types } from '../types/coprocessor.js';

////////////////////////////////////////////////////////////////////////////////
// CoprocessorEIP712Types
////////////////////////////////////////////////////////////////////////////////

/*
    const EIP712DomainType = [
      { name: 'name', type: 'string' },
      { name: 'version', type: 'string' },
      { name: 'chainId', type: 'uint256' },
      { name: 'verifyingContract', type: 'address' },
    ];
*/

export const coprocessorEIP712PrimaryType =
  'CiphertextVerification' satisfies keyof CoprocessorEIP712Types;
export const coprocessorEIP712Types: CoprocessorEIP712Types = {
  // EIP712Domain: [
  //   { name: 'name', type: 'string' },
  //   { name: 'version', type: 'string' },
  //   { name: 'chainId', type: 'uint256' },
  //   { name: 'verifyingContract', type: 'address' },
  // ] as const,
  // CRITICAL: Field order is authoritative — it determines the EIP-712 type hash.
  // Changing the order will produce a different signature and break on-chain verification.
  // Must match the Solidity struct definition exactly.
  CiphertextVerification: [
    { name: 'ctHandles', type: 'bytes32[]' },
    { name: 'userAddress', type: 'address' },
    { name: 'contractAddress', type: 'address' },
    { name: 'contractChainId', type: 'uint256' },
    { name: 'extraData', type: 'bytes' },
  ],
} as const;
Object.freeze(coprocessorEIP712Types);
Object.freeze(coprocessorEIP712Types.CiphertextVerification);
coprocessorEIP712Types.CiphertextVerification.forEach(Object.freeze);
