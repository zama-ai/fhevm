import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithEncrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import {
  type EncryptValueWithZkProofParameters,
  type EncryptValueWithZkProofReturnType,
  encryptValueWithZkProof,
} from './encryptValueWithZkProof.js';

////////////////////////////////////////////////////////////////////////////////

export type EncryptValueParameters = EncryptValueWithZkProofParameters;

export type EncryptValueReturnType = EncryptValueWithZkProofReturnType;

////////////////////////////////////////////////////////////////////////////////

/** Encrypts `value` with a ZK proof of knowledge: {@link encryptValueWithZkProof}. */
export async function encryptValue(
  fhevm: Fhevm<FhevmChain, WithEncrypt>,
  parameters: EncryptValueParameters,
): Promise<EncryptValueReturnType> {
  return encryptValueWithZkProof(fhevm, parameters);
}
