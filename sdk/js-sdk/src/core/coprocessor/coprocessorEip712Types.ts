import type { CoprocessorEip712Types } from '../types/coprocessor.js';

////////////////////////////////////////////////////////////////////////////////
// CoprocessorEip712Types
////////////////////////////////////////////////////////////////////////////////

/*
    const EIP712DomainType = [
      { name: 'name', type: 'string' },
      { name: 'version', type: 'string' },
      { name: 'chainId', type: 'uint256' },
      { name: 'verifyingContract', type: 'address' },
    ];
*/

export const coprocessorEip712PrimaryType = 'CiphertextVerification' satisfies keyof CoprocessorEip712Types;

export const coprocessorEip712Types: CoprocessorEip712Types = {
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

Object.freeze(coprocessorEip712Types);
Object.freeze(coprocessorEip712Types.CiphertextVerification);
coprocessorEip712Types.CiphertextVerification.forEach(Object.freeze);
